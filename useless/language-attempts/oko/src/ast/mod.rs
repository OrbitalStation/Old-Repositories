modules!(states state interstate visitable);

use crate::*;

#[derive(Debug, Clone, Visitable)]
pub struct AST <State: ASTState> {
	pub state: State
}

impl AST <InitialState> {
	pub const fn new() -> Self {
		Self { state: InitialState }
	}
}

impl <State: ASTState> AST <State> {
	#[inline]
	pub fn advance_with <Next: ASTState <PreviousState = State>> (self, data: &Next::DataToAdvance) -> Result <AST <Next>, Next::AdvancementError> {
		Next::from_previous(self, data)
	}

	#[inline]
	pub fn advance <Next: ASTState <PreviousState = State, DataToAdvance = ()>> (self) -> Result <AST <Next>, Next::AdvancementError> {
		self.advance_with(&())
	}
}
