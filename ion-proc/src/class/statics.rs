/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;
use std::ffi::CString;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use syn::{ItemImpl, ItemStatic, LitStr, parse2};

use crate::attribute::class::Name;
use crate::class::accessor::Accessor;
use crate::class::method::Method;
use crate::class::property::Property;

pub(crate) fn class_spec(ion: &TokenStream, class: &Ident, literal: &LitStr) -> ItemStatic {
	let name = String::from_utf8(CString::new(literal.value()).unwrap().into_bytes_with_nul()).unwrap();
	parse_quote!(
		static CLASS: ::mozjs::jsapi::JSClass = ::mozjs::jsapi::JSClass {
			name: #name.as_ptr().cast(),
			flags: #ion::objects::class_reserved_slots(<#class as #ion::ClassDefinition>::PARENT_PROTOTYPE_CHAIN_LENGTH + 1) | ::mozjs::jsapi::JSCLASS_BACKGROUND_FINALIZE,
			cOps: OPERATIONS as *const _ as *mut _,
			spec: ::std::ptr::null_mut(),
			ext: ::std::ptr::null_mut(),
			oOps: ::std::ptr::null_mut(),
		};
	)
}

pub(crate) fn methods_to_specs(ion: &TokenStream, methods: &[Method], stat: bool) -> ItemStatic {
	let ident = if stat { quote!(STATIC_FUNCTIONS) } else { quote!(FUNCTIONS) };
	let mut specs: Vec<_> = methods
		.iter()
		.flat_map(|method| {
			let ident = method.method.sig.ident.clone();
			let nargs = method.nargs as u16;
			(*method.names)
				.iter()
				.map(|name| match name {
					Name::String(literal) => {
						let mut name = literal.value();
						if name.is_case(Case::Snake) {
							name = name.to_case(Case::Camel)
						}
						quote!(#ion::function_spec!(#ident, #name, #nargs, #ion::flags::PropertyFlags::CONSTANT_ENUMERATED))
					}
					Name::Symbol(symbol) => {
						quote!(#ion::function_spec_symbol!(#ident, #symbol, #nargs, #ion::flags::PropertyFlags::CONSTANT))
					}
				})
				.collect::<Vec<_>>()
		})
		.collect();
	specs.push(quote!(::mozjs::jsapi::JSFunctionSpec::ZERO));

	parse_quote!(
		static #ident: &[::mozjs::jsapi::JSFunctionSpec] = &[
			#(#specs),*
		];
	)
}

pub(crate) fn properties_to_specs(
	ion: &TokenStream, properties: &[Property], accessors: &HashMap<String, Accessor>, class: &Ident, stat: bool,
) -> ItemStatic {
	let ident: Ident = if stat {
		parse_quote!(STATIC_PROPERTIES)
	} else {
		parse_quote!(PROPERTIES)
	};

	let mut specs: Vec<_> = properties.iter().flat_map(|property| property.to_specs(ion, class)).collect();
	accessors.iter().for_each(|(_, accessor)| specs.extend(accessor.to_specs(ion)));

	specs.push(quote!(::mozjs::jsapi::JSPropertySpec::ZERO));

	parse_quote!(
		static #ident: &[::mozjs::jsapi::JSPropertySpec] = &[
			#(#specs),*
		];
	)
}

pub(crate) fn class_definition(
	ion: &TokenStream, class_ident: &Ident, class_name: &LitStr, constructor_ident: &Ident, constructor_nargs: u32,
) -> ItemImpl {
	parse2(quote!(
		impl #ion::ClassDefinition for #class_ident {
			const NAME: &'static str = #class_name;

			fn class() -> &'static ::mozjs::jsapi::JSClass {
				&CLASS
			}

			fn constructor() -> (#ion::functions::NativeFunction, ::core::primitive::u32) {
				(#constructor_ident, #constructor_nargs)
			}

			fn functions() -> &'static [::mozjs::jsapi::JSFunctionSpec] {
				&FUNCTIONS
			}

			fn properties() -> &'static [::mozjs::jsapi::JSPropertySpec] {
				&PROPERTIES
			}

			fn static_functions() -> &'static [::mozjs::jsapi::JSFunctionSpec] {
				&STATIC_FUNCTIONS
			}

			fn static_properties() -> &'static [::mozjs::jsapi::JSPropertySpec] {
				&STATIC_PROPERTIES
			}
		}
	))
	.unwrap()
}
