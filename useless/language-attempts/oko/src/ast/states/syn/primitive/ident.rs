use crate::*;

define_string_seq_alias!(Ident, IdentStringSeqValidator(pub is_not_first: bool) fn call_mut(&mut self, args: (char,)) -> bool {
	// First letter alphabetic, other alphanumeric
	if self.is_not_first {
		args.0.is_alphanumeric()
	} else {
		self.is_not_first = true;
		args.0.is_alphabetic()
	}
});
