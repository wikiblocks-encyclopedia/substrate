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
// limitations under the License.

use crate::pallet::Def;
use syn::spanned::Spanned;

/// If attribute `#[pallet::generate_store(..)]` is defined then:
/// * generate Store trait with all storages,
/// * implement Store trait for Pallet.
pub fn expand_store_trait(def: &mut Def) -> proc_macro2::TokenStream {
	let (trait_vis, trait_store, attribute_span) =
		if let Some(store) = &def.pallet_struct.store { store } else { return Default::default() };

	let type_impl_gen = &def.type_impl_generics(trait_store.span());
	let type_use_gen = &def.type_use_generics(trait_store.span());
	let pallet_ident = &def.pallet_struct.pallet;

	let mut where_clauses = vec![&def.config.where_clause];
	where_clauses.extend(def.storages.iter().map(|storage| &storage.where_clause));
	let completed_where_clause = super::merge_where_clauses(&where_clauses);

	let storage_names = &def.storages.iter().map(|storage| &storage.ident).collect::<Vec<_>>();
	let storage_cfg_attrs =
		&def.storages.iter().map(|storage| &storage.cfg_attrs).collect::<Vec<_>>();
	let warnig_struct_name = syn::Ident::new("Store", *attribute_span);
	let warning: syn::ItemStruct = syn::parse_quote!(
		#[deprecated(note = r"
		Use of `#[pallet::generate_store(pub(super) trait Store)]` will be removed after July 2023.
		Check https://github.com/paritytech/substrate/pull/13535 for more details.")]
		struct #warnig_struct_name;
	);

	quote::quote_spanned!(trait_store.span() =>
		const _:() = {
			#warning
			const _: Option<#warnig_struct_name> = None;
		};
		#trait_vis trait #trait_store {
			#(
				#(#storage_cfg_attrs)*
				type #storage_names;
			)*
		}
		impl<#type_impl_gen> #trait_store for #pallet_ident<#type_use_gen>
			#completed_where_clause
		{
			#(
				#(#storage_cfg_attrs)*
				type #storage_names = #storage_names<#type_use_gen>;
			)*
		}
	)
}
