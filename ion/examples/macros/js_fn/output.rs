use ion::{Array, Context, js_fn, Object, Result, ResultExc};

#[js_fn]
fn output_empty() {}

#[js_fn]
fn output_regular() -> i8 {
	0
}

#[js_fn]
fn output_object<'cx>(cx: &'cx Context) -> Object<'cx> {
	Object::new(cx)
}

#[js_fn]
fn output_result_empty() -> Result<()> {
	Ok(())
}

#[js_fn]
fn output_result_regular() -> Result<f64> {
	Ok(f64::EPSILON)
}

#[js_fn]
fn output_result_array<'cx>(cx: &'cx Context) -> Result<Array<'cx>> {
	Ok(Array::new_with_length(cx, 8))
}

#[js_fn]
fn output_result_exception() -> ResultExc<()> {
	Ok(())
}
