/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use syn::{Abi, Error, FnArg, Generics, ItemFn, Result};
use syn::punctuated::Punctuated;

use crate::attribute::krate::Crates;
use crate::function::wrapper::impl_wrapper_fn;

pub(crate) mod inner;
pub(crate) mod parameters;
pub(crate) mod wrapper;

// TODO: Partially Remove Error Handling in Infallible Functions
pub(crate) fn impl_js_fn(mut function: ItemFn) -> Result<ItemFn> {
	let crates = Crates::from_attributes(&mut function.attrs)?;

	let (wrapper, _, _) = impl_wrapper_fn(&crates, function.clone(), None, true, false)?;

	let ion = &crates.ion;

	check_abi(&mut function)?;
	set_signature(&mut function)?;
	function.attrs.clear();
	function.attrs.push(parse_quote!(#[allow(non_snake_case)]));

	let body = parse_quote!({
		let cx = &#ion::Context::new_unchecked(cx);
		let args = &mut #ion::Arguments::new(cx, argc, vp);

		#wrapper
		let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| wrapper(cx, args)));
		#ion::functions::__handle_native_function_result(cx, result, args.rval())
	});
	function.block = Box::new(body);

	Ok(function)
}

pub(crate) fn check_abi(function: &mut ItemFn) -> Result<()> {
	match &function.sig.abi {
		None => function.sig.abi = Some(parse_quote!(extern "C")),
		Some(Abi { name: None, .. }) => {}
		Some(Abi { name: Some(abi), .. }) if abi.value() == "C" => {}
		Some(Abi { name: Some(non_c_abi), .. }) => return Err(Error::new_spanned(non_c_abi, "Expected C ABI")),
	}
	Ok(())
}

pub(crate) fn set_signature(function: &mut ItemFn) -> Result<()> {
	function.sig.asyncness = None;
	function.sig.unsafety = Some(<Token![unsafe]>::default());
	let params: [FnArg; 3] = [
		parse_quote!(cx: *mut ::mozjs::jsapi::JSContext),
		parse_quote!(argc: ::core::primitive::u32),
		parse_quote!(vp: *mut ::mozjs::jsval::JSVal),
	];
	function.sig.generics = Generics::default();
	function.sig.inputs = Punctuated::<_, _>::from_iter(params);
	function.sig.output = parse_quote!(-> ::core::primitive::bool);
	Ok(())
}
