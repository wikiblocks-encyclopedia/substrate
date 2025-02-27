// This file is part of a fork of Substrate which has had various changes.

// Copyright (C) Parity Technologies (UK) Ltd.
// Copyright (C) 2022-2023 Luke Parker
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License

use crate::construct_runtime::Pallet;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::Ident;

pub fn expand_outer_inherent(
	runtime: &Ident,
	block: &TokenStream,
	unchecked_extrinsic: &TokenStream,
	pallet_decls: &[Pallet],
	scrate: &TokenStream,
) -> TokenStream {
	let mut pallet_names = Vec::new();
	let mut pallet_attrs = Vec::new();
	let mut query_inherent_part_macros = Vec::new();

	for pallet_decl in pallet_decls {
		if pallet_decl.exists_part("Inherent") {
			let name = &pallet_decl.name;
			let path = &pallet_decl.path;
			let attr = pallet_decl.cfg_pattern.iter().fold(TokenStream::new(), |acc, pattern| {
				let attr = TokenStream::from_str(&format!("#[cfg({})]", pattern.original()))
					.expect("was successfully parsed before; qed");
				quote! {
					#acc
					#attr
				}
			});

			pallet_names.push(name);
			pallet_attrs.push(attr);
			query_inherent_part_macros.push(quote! {
				#path::__substrate_inherent_check::is_inherent_part_defined!(#name);
			});
		}
	}

	quote! {
		#( #query_inherent_part_macros )*

		trait InherentDataExt {
			fn create_extrinsics(&self) ->
				#scrate::sp_std::vec::Vec<<#block as #scrate::sp_runtime::traits::Block>::Extrinsic>;
			fn check_extrinsics(&self, block: &#block) -> #scrate::inherent::CheckInherentsResult;
		}

		impl InherentDataExt for #scrate::inherent::InherentData {
			fn create_extrinsics(&self) ->
				#scrate::sp_std::vec::Vec<<#block as #scrate::sp_runtime::traits::Block>::Extrinsic>
			{
				use #scrate::inherent::ProvideInherent;

				let mut inherents = #scrate::sp_std::vec::Vec::new();

				#(
					#pallet_attrs
					if let Some(inherent) = #pallet_names::create_inherent(self) {
						let inherent = <#unchecked_extrinsic as #scrate::sp_runtime::traits::Extrinsic>::new(
							inherent.into(),
							None,
						).expect("Runtime UncheckedExtrinsic is not Opaque, so it has to return \
							`Some`; qed");

						inherents.push(inherent);
					}
				)*

				inherents
			}

			fn check_extrinsics(&self, block: &#block) -> #scrate::inherent::CheckInherentsResult {
				use #scrate::inherent::{ProvideInherent, IsFatalError};
				use #scrate::traits::{IsSubType, ExtrinsicCall};
				use #scrate::sp_runtime::traits::Block as _;
				use #scrate::_private::sp_inherents::Error;
				use #scrate::log;

				let mut result = #scrate::inherent::CheckInherentsResult::new();

				// This handle assume we abort on the first fatal error.
				fn handle_put_error_result(res: Result<(), Error>) {
					const LOG_TARGET: &str = "runtime::inherent";
					match res {
						Ok(()) => (),
						Err(Error::InherentDataExists(id)) =>
							log::debug!(
								target: LOG_TARGET,
								"Some error already reported for inherent {:?}, new non fatal \
								error is ignored",
								id
							),
						Err(Error::FatalErrorReported) =>
							log::error!(
								target: LOG_TARGET,
								"Fatal error already reported, unexpected considering there is \
								only one fatal error",
							),
						Err(_) =>
							log::error!(
								target: LOG_TARGET,
								"Unexpected error from `put_error` operation",
							),
					}
				}

				for xt in block.extrinsics() {
					// Inherents are before any other extrinsics.
					// And signed extrinsics are not inherents.
					if #scrate::sp_runtime::traits::Extrinsic::is_signed(xt).unwrap_or(false) {
						break
					}

					let mut is_inherent = false;

					#(
						#pallet_attrs
						{
							let call = <#unchecked_extrinsic as ExtrinsicCall>::call(xt);
							if let Some(call) = IsSubType::<_>::is_sub_type(call) {
								if #pallet_names::is_inherent(call) {
									is_inherent = true;
									if let Err(e) = #pallet_names::check_inherent(call, self) {
										handle_put_error_result(result.put_error(
											#pallet_names::INHERENT_IDENTIFIER, &e
										));
										if e.is_fatal_error() {
											return result;
										}
									}
								}
							}
						}
					)*

					// Inherents are before any other extrinsics.
					// No module marked it as inherent thus it is not.
					if !is_inherent {
						break
					}
				}

				#(
					#pallet_attrs
					match #pallet_names::is_inherent_required(self) {
						Ok(Some(e)) => {
							let found = block.extrinsics().iter().any(|xt| {
								let is_signed = #scrate::sp_runtime::traits::Extrinsic::is_signed(xt)
									.unwrap_or(false);

								if !is_signed {
									let call = <
										#unchecked_extrinsic as ExtrinsicCall
									>::call(xt);
									if let Some(call) = IsSubType::<_>::is_sub_type(call) {
										#pallet_names::is_inherent(&call)
									} else {
										false
									}
								} else {
									// Signed extrinsics are not inherents.
									false
								}
							});

							if !found {
								handle_put_error_result(result.put_error(
									#pallet_names::INHERENT_IDENTIFIER, &e
								));
								if e.is_fatal_error() {
									return result;
								}
							}
						},
						Ok(None) => (),
						Err(e) => {
							handle_put_error_result(result.put_error(
								#pallet_names::INHERENT_IDENTIFIER, &e
							));
							if e.is_fatal_error() {
								return result;
							}
						},
					}
				)*

				result
			}
		}

		impl #scrate::traits::EnsureInherentsAreFirst<#block> for #runtime {
			fn ensure_inherents_are_first(block: &#block) -> Result<(), u32> {
				use #scrate::inherent::ProvideInherent;
				use #scrate::traits::{IsSubType, ExtrinsicCall};
				use #scrate::sp_runtime::traits::Block as _;

				let mut first_signed_observed = false;

				for (i, xt) in block.extrinsics().iter().enumerate() {
					let is_signed = #scrate::sp_runtime::traits::Extrinsic::is_signed(xt)
						.unwrap_or(false);

					let is_inherent = if is_signed {
						// Signed extrinsics are not inherents.
						false
					} else {
						let mut is_inherent = false;
						#(
							#pallet_attrs
							{
								let call = <#unchecked_extrinsic as ExtrinsicCall>::call(xt);
								if let Some(call) = IsSubType::<_>::is_sub_type(call) {
									if #pallet_names::is_inherent(&call) {
										is_inherent = true;
									}
								}
							}
						)*
						is_inherent
					};

					if !is_inherent {
						first_signed_observed = true;
					}

					if first_signed_observed && is_inherent {
						return Err(i as u32)
					}
				}

				Ok(())
			}
		}
	}
}
