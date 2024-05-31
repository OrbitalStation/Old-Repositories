use crate::*;

pub struct ScopeStorageChain <'a> {
	pub prev: Option <Box <TypeIndexIterRaw <'a>>>,
	pub next: Box <TypeIndexIterRaw <'a>>
}

impl <'a> Iterator for ScopeStorageChain <'a> {
	type Item = (usize, &'a ScopeItem);

	fn next(&mut self) -> Option <Self::Item> {
		if let Some(prev) = self.prev.as_mut() {
			match prev.next() {
				Some(x) => return Some(x),
				None => self.prev = None
			}
		}
		self.next.next()
	}
}
