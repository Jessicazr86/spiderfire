/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub use search_params::URLSearchParams;

#[js_class]
mod search_params {
	use std::cell::RefCell;
	use std::rc::Rc;

	use url::Url;

	#[ion(no_constructor, into_value)]
	pub struct URLSearchParams {
		url: Rc<RefCell<Url>>,
	}

	// TODO: Allow URLSearchParams to be formed with just a string of query pairs
	// TODO: Implement URLSearchParams.prototype.set, URLSearchParams.prototype.delete, and URLSearchParams.prototype.sort
	// TODO: Implement [Symbol.iterator] for URLSearchParams
	impl URLSearchParams {
		pub(crate) fn from_url(url: Rc<RefCell<Url>>) -> URLSearchParams {
			URLSearchParams { url }
		}

		pub fn append(&mut self, name: String, value: String) {
			self.url.borrow_mut().query_pairs_mut().append_pair(&name, &value);
		}

		pub fn get(&self, key: String) -> Option<String> {
			self.url.borrow().query_pairs().into_owned().find(|(k, _)| k == &key).map(|(_, v)| v)
		}

		pub fn getAll(&self, key: String) -> Vec<String> {
			self.url
				.borrow()
				.query_pairs()
				.into_owned()
				.filter(|(k, _)| k == &key)
				.map(|(_, v)| v)
				.collect()
		}

		pub fn has(&self, key: String, value: Option<String>) -> bool {
			if let Some(value) = value {
				self.url.borrow().query_pairs().into_owned().any(|(k, v)| k == key && v == value)
			} else {
				self.url.borrow().query_pairs().into_owned().any(|(k, _)| k == key)
			}
		}

		pub fn size(&self) -> i32 {
			self.url.borrow().query_pairs().count() as i32
		}

		pub fn toString(&self) -> String {
			String::from(self.url.borrow().query().unwrap_or(""))
		}
	}
}