/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use ::std::cell::RefCell;
use ::std::collections::hash_map::{Entry, HashMap};
use ::std::fs;
use ::std::path::Path;
use ::std::ptr;

use mozjs::conversions::jsstr_to_string;
use mozjs::jsapi::*;
use mozjs::rust::{CompileOptionsWrapper, transform_u16_to_source_text};

thread_local! {
	static MODULE_REGISTRY: RefCell<HashMap<String, *mut JSObject>> = RefCell::new(HashMap::new());
}

pub(crate) fn compile_module(cx: *mut JSContext, filename: &String, script: &String) -> *mut JSObject {
	let options = unsafe { CompileOptionsWrapper::new(cx, filename, 1) };
	let script_text: Vec<u16> = script.encode_utf16().collect();
	let mut source = transform_u16_to_source_text(script_text.as_slice());

	unsafe {
		return CompileModule(cx, options.ptr as *const ReadOnlyCompileOptions, &mut source);
	}
}

pub(crate) unsafe extern "C" fn resolve_module(cx: *mut JSContext, _mod_private: Handle<Value>, name: Handle<*mut JSString>) -> *mut JSObject {
	let name = jsstr_to_string(cx, name.get());
	let mut ret: *mut JSObject = ptr::null_mut();

	MODULE_REGISTRY.with(|registry| {
		let mut registry = registry.borrow_mut();
		let mut to_return = false;
		match (*registry).entry(name.clone()) {
			Entry::Vacant(_) => (),
			Entry::Occupied(o) => {
				ret = *o.get();
				to_return = true;
			}
		}
		if to_return {
			return;
		}

		rooted!(in(cx) let mut module = JS_NewPlainObject(cx));
		let path = Path::new(&name);
		let script = fs::read_to_string(path);

		if let Ok(script) = script {
			module.handle_mut().set(compile_module(cx, &name, &script));
			match (*registry).entry(name) {
				Entry::Vacant(v) => {
					v.insert(module.handle().get());
					ret = module.handle().get();
				}
				Entry::Occupied(_) => (),
			}
		}
	});

	return ret;
}