/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::cell::{Cell, RefCell};
use std::collections::vec_deque::VecDeque;
use std::ffi::c_void;
use std::rc::Rc;

use mozjs::glue::{CreateJobQueue, JobQueueTraps};
use mozjs::jsapi::{CurrentGlobalOrNull, Handle, JobQueueIsEmpty, JobQueueMayNotBeEmpty, JSContext, JSFunction, JSObject, SetJobQueue};

use ion::{Context, ErrorReport, Function, Object};

#[derive(Clone, Debug)]
pub enum Microtask {
	Promise(*mut JSObject),
	User(*mut JSFunction),
	None,
}

#[derive(Clone, Debug, Default)]
pub struct MicrotaskQueue {
	queue: RefCell<VecDeque<Microtask>>,
	draining: Cell<bool>,
}

impl Microtask {
	pub fn run(&self, cx: &Context) -> Result<(), Option<ErrorReport>> {
		match self {
			Microtask::Promise(job) => {
				let object = cx.root_object(*job);
				let function = Function::from_object(cx, &object).unwrap();

				function.call(cx, &Object::null(cx), &[]).map(|_| ())
			}
			Microtask::User(callback) => {
				let callback = Function::from(cx.root_function(*callback));
				callback.call(cx, &Object::global(cx), &[])?;
				Ok(())
			}
			Microtask::None => Ok(()),
		}
	}
}

impl MicrotaskQueue {
	pub fn enqueue(&self, cx: *mut JSContext, microtask: Microtask) {
		{
			self.queue.borrow_mut().push_back(microtask);
		}
		unsafe { JobQueueMayNotBeEmpty(cx) }
	}

	pub fn run_jobs(&self, cx: &Context) -> Result<(), Option<ErrorReport>> {
		if self.draining.get() {
			return Ok(());
		}

		self.draining.set(true);

		while let Some(microtask) = self.front() {
			microtask.run(cx)?;
		}

		self.draining.set(false);
		unsafe { JobQueueIsEmpty(cx.as_ptr()) };

		Ok(())
	}

	fn front(&self) -> Option<Microtask> {
		self.queue.borrow_mut().pop_front()
	}

	pub fn is_empty(&self) -> bool {
		self.queue.borrow().is_empty()
	}
}

unsafe extern "C" fn get_incumbent_global(_extra: *const c_void, cx: *mut JSContext) -> *mut JSObject {
	unsafe { CurrentGlobalOrNull(cx) }
}

unsafe extern "C" fn enqueue_promise_job(
	extra: *const c_void, cx: *mut JSContext, _promise: Handle<*mut JSObject>, job: Handle<*mut JSObject>, _: Handle<*mut JSObject>,
	_: Handle<*mut JSObject>,
) -> bool {
	let queue: &MicrotaskQueue = unsafe { &*extra.cast() };
	if !job.is_null() {
		queue.enqueue(cx, Microtask::Promise(job.get()))
	} else {
		queue.enqueue(cx, Microtask::None)
	};
	true
}

unsafe extern "C" fn empty(extra: *const c_void) -> bool {
	let queue: &MicrotaskQueue = unsafe { &*extra.cast() };
	queue.queue.borrow().is_empty()
}

static JOB_QUEUE_TRAPS: JobQueueTraps = JobQueueTraps {
	getIncumbentGlobal: Some(get_incumbent_global),
	enqueuePromiseJob: Some(enqueue_promise_job),
	empty: Some(empty),
};

pub(crate) fn init_microtask_queue(cx: &Context) -> Rc<MicrotaskQueue> {
	let microtask_queue = Rc::new(MicrotaskQueue::default());
	unsafe {
		let queue = Rc::into_raw(Rc::clone(&microtask_queue));
		let queue = CreateJobQueue(&JOB_QUEUE_TRAPS, queue.cast());
		SetJobQueue(cx.as_ptr(), queue);
	}
	microtask_queue
}
