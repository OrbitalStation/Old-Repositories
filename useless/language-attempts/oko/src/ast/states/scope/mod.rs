modules!(ty ty_def storage_chain handle);

use crate::*;

/// Modifies the AST by introducing scopes
///
/// Performs according checks
#[derive(Debug, Clone, Visitable)]
pub struct ScopeASTState {
	pub stmts: Vec <Stmt>,
	#[novisit]
	pub type_list: Vec <ScopeItem>
}

impl ASTState for ScopeASTState {
	type PreviousState = SynASTState;
	type DataToAdvance = ();
	type AdvancementError = ParseError;

	fn from_previous(mut ast: AST <Self::PreviousState>, _: &Self::DataToAdvance) -> Result <AST <Self>, Self::AdvancementError> {
		let mut error = None;

		let mut type_list = vec![];

		let mut offset = 0;

		scope_do_handle(&mut ast, &mut || Box::new(std::iter::empty()), &mut type_list, &mut error, &mut offset);

		if let Some(err) = error {
			return Err(err)
		}

		Ok(AST {
			state: Self {
				stmts: ast.state.stmts,
				type_list
			}
		})
	}
}
