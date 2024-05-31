use crate::*;

/// The very first state of the AST; just a stub
#[derive(Debug, Clone)]
pub struct InitialState;

impl Visitable for InitialState {
	fn visit_mut <T> (&mut self, _: &mut impl VisitCallback <T>, _: &VisitProperties) {}
}

impl ASTState for InitialState {
	type PreviousState = InitialState;
	type DataToAdvance = ();
	type AdvancementError = ();

	fn from_previous(_: AST <Self::PreviousState>, _: &Self::DataToAdvance) -> Result <AST <Self>, Self::AdvancementError> {
		Ok(AST::new())
	}
}
