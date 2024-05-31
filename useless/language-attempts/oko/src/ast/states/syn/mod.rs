modules!(modifiers primitive complex parser);

use crate::*;

/// Builds a primary syntax tree
///
/// Only parses the code, no validity checks are performed
#[derive(Debug, Clone, Visitable)]
pub struct SynASTState {
	pub stmts: Vec <Stmt>
}

impl ASTState for SynASTState {
	type PreviousState = RawCodeASTStage;

	type DataToAdvance = ();

	type AdvancementError = ParseError;

	fn from_previous(ast: AST <Self::PreviousState>, _: &Self::DataToAdvance) -> Result <AST <Self>, ParseError> {
		let mut parser = Parser::new(&ast.state.code);
		let result = parser.parse::<Surrounded <ParseRepeatedly <_, NoSep, True, 0>, Optional <Newline>>>()?;
		parser.assert_is_empty(*result.value.last_error);
		Ok(AST {
			state: Self {
				stmts: result.value.vec
			}
		})
	}
}
