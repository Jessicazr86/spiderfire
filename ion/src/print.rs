/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::io::Write;

use mozjs::jsapi::Value;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::IonContext;
use crate::types::{array::is_array, string::to_string};

pub const INDENT: &str = "  ";
pub const NEWLINE: &str = "\n";

/**
 * Prints a [Value] with the appropriate colour and indentation, to stdout or stderr.
 */
pub fn print_value(cx: IonContext, val: Value, indents: usize, is_stderr: bool) {
	let mut out = if !is_stderr {
		StandardStream::stdout(ColorChoice::Auto)
	} else {
		StandardStream::stderr(ColorChoice::Auto)
	};

	if val.is_number() {
		out.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
	} else if val.is_boolean() {
		out.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
	} else if val.is_string() {
		out.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
	} else if is_array(cx, val) {
		out.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 127, 63)))).unwrap();
	} else if val.is_object() {
		out.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(240, 240, 240)))).unwrap();
	} else {
		out.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(118, 118, 118)))).unwrap();
	}

	write!(out, "{}", indent(&to_string(cx, val), indents, false)).unwrap();
	out.reset().unwrap();
}

pub fn indent(string: &str, indents: usize, initial: bool) -> String {
	if string.contains(NEWLINE) {
		let indent = INDENT.repeat(indents);
		if initial {
			(indent.clone() + string).replace(NEWLINE, &(String::from(NEWLINE) + &indent))
		} else {
			string.replace(NEWLINE, &(String::from(NEWLINE) + &indent))
		}
	} else {
		string.to_string()
	}
}