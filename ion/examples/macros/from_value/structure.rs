use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use ion::{Context, FromValue, Object, Result, Value};
use ion::conversions::{ConversionBehavior, FromValue};

#[derive(FromValue)]
pub struct Complex<'cx> {
	#[ion(inherit)]
	pub raw: Object<'cx>,
	pub truth: bool,
	#[ion(convert = ConversionBehavior::EnforceRange, strict)]
	pub mode: u32,
	#[ion(default = String::from("string"))]
	pub text: String,
	#[ion(strict, default = true)]
	pub other: bool,
	#[ion(default, convert = ConversionBehavior::Clamp)]
	pub optional: Option<i32>,
	#[ion(default = Arc::new(AtomicU64::new(16)), parser = |v| parse_as_atomic_arc(cx, v))]
	pub parsed: Arc<AtomicU64>,
}

fn parse_as_atomic_arc<'cx: 'v, 'v>(cx: &'cx Context, value: Value<'v>) -> Result<Arc<AtomicU64>> {
	u64::from_value(cx, &value, true, ConversionBehavior::Default).map(|num| Arc::new(AtomicU64::new(num)))
}
