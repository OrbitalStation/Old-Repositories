use std::fmt::Debug;
use crate::*;

pub trait ASTState : Visitable where Self: Sized {
	type PreviousState: ASTState;

	type DataToAdvance: ?Sized;

	type AdvancementError: Debug;

	fn from_previous(ast: AST <Self::PreviousState>, data: &Self::DataToAdvance) -> Result <AST <Self>, Self::AdvancementError>;
}
