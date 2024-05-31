use std::fmt::{Debug, Formatter};
use std::ops::Range;
use crate::*;

#[derive(Clone)]
pub struct Span {
	pub range: Range <ByteOffset>
}

impl Span {
	pub fn new(start: ByteOffset, end: ByteOffset) -> Self {
		Self {
			range: Range {
				start,
				end
			}
		}
	}

	pub fn len(&self) -> usize {
		self.range.end.0 - self.range.start.0
	}

	pub fn add(&mut self, offset: usize) {
		self.range.start.0 += offset;
		self.range.end.0 += offset
	}
}

impl Debug for Span {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		self.range.fmt(f)
	}
}
