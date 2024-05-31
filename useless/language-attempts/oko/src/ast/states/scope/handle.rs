use crate::*;

pub fn scope_do_handle <'a, L: Visitable> (
	// The type representing the current scope we need to handle
	launcher: &mut L,
	// The previous(parent) scope
	prev_scope: *mut dyn FnMut() -> Box <TypeIndexIterRaw <'a>>,
	// The storage to put types from this scope to
	new_types: *mut Vec <ScopeItem>,
	// Set to `Some(...)` if type resolution fails
	error: *mut Option <ParseError>,
	// Offset used to assign correct indexes to types
	offset: *mut usize
) {
	unsafe {
		// Collect all the types defined in the current scope
		launcher.visit_mut(&mut get_scope_types(&mut *new_types), &VisitProperties::SCOPE_LIMITED);

		let offs = *offset;

		let mut next = || Box::new(ScopeStorageChain {
			prev: Some(core::mem::transmute((*prev_scope)())),
			next: Box::new((offs..).zip((&*new_types).iter()))
		}) as Box<(dyn Iterator <Item = (usize, &ScopeItem)>)>;

		// Resolute all `Type`s
		launcher.visit_mut(&mut resolute_ty(&mut *next(), error), &VisitProperties::SCOPE_LIMITED);

		if (*error).is_some() { return }

		*offset += (*new_types).len();

		// Handle nested blocks
		let mut newer_types = vec![];
		launcher.visit_mut(&mut handle_block(core::mem::transmute(&mut next as &mut (dyn FnMut() -> Box <dyn Iterator <Item = (usize, &'static ScopeItem)>>)), &mut newer_types as *mut _, error, offset), &VisitProperties::SCOPE_LIMITED);

		if (*error).is_some() { return }

		(*new_types).append(&mut newer_types)
	}
}

fn handle_block <'a> (prev_scope: *mut dyn FnMut() -> Box <TypeIndexIterRaw <'a>>, new_types: *mut Vec <ScopeItem>, error: *mut Option <ParseError>, offset: *mut usize) -> impl VisitCallback <Block <FunAct>> + 'a {
	move |block| scope_do_handle(&mut block.0, prev_scope, new_types, error, offset)
}

fn get_scope_types <'a> (list: &'a mut Vec <ScopeItem>) -> impl VisitCallback <TyStmt> + 'a {
	|ty| list.push(ScopeItem::from_ty_stmt(ty.clone()))
}

fn resolute_ty <'a> (scope: *mut TypeIndexIterRaw <'a>, error: *mut Option <ParseError>) -> impl VisitCallback <Type> + 'a {
	move |ty| if let Some(x) = ty.inner.resolute(unsafe { core::mem::transmute(scope) }) {
		*ty = x
	} else {
		unsafe { *error = Some(ParseError::new(ty.inner.span())) }
	}
}
