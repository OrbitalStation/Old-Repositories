use std::fmt::{Debug, Formatter};

/// Offset in a file that is expected to be in bytes
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ByteOffset(pub usize);

impl Debug for ByteOffset {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		Debug::fmt(&self.0, f)
	}
}
