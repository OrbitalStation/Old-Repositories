use oko::*;

// TODO: Decrease code duplication in functions process_enum and process_struct in oko_proc_macro::derive_visitable
// TODO: Add `ParseDebug`

// TODO: check issue #81138 (something with ranges)

// FIXME: Somehow newlines in blocks do not affect span

fn main() {
    let ast = AST::new();
    let ast = ast.advance_with::<RawCodeASTStage>("code").unwrap();
    let ast = ast.advance::<SynASTState>().unwrap();
    let ast = ast.advance::<ScopeASTState>().unwrap();

    println!("\n{ast:#?}")
}
