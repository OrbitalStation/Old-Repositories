mod comments;

use crate::*;

/// Accepts the code and transforms it into a form suitable for the next stage
#[derive(Debug, Clone)]
pub struct RawCodeASTStage {
	pub code: String
}

impl Visitable for RawCodeASTStage {
	fn visit_mut <T> (&mut self, _: &mut impl VisitCallback <T>, _: &VisitProperties) {}
}

impl ASTState for RawCodeASTStage {
	type PreviousState = InitialState;
	type DataToAdvance = str;
	type AdvancementError = std::io::Error;

	fn from_previous(_: AST <Self::PreviousState>, filename: &Self::DataToAdvance) -> Result <AST <Self>, Self::AdvancementError> {
		let mut code = std::fs::read_to_string(filename)?;
		code.push('\n');
		let code = code.replace("    ", "\t");
		let code = comments::remove_comments(code);
		Ok(AST {
			state: Self {
				code
			}
		})
	}
}
