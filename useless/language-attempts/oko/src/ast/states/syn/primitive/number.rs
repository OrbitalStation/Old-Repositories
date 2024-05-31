use crate::*;

define_string_seq_alias!(Number, NumberStringSeqValidator() fn call_mut(&mut self, args: (char,)) -> bool {
	args.0.is_numeric()
});
