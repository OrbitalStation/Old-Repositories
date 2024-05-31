extern crate llvm_sys as llvm;

use oko::*;
use std::ffi::CStr;
use std::process::Command;

fn main() {
    create_llvm_context();
    create_llvm_module("oko");
    create_llvm_builder();

    let code = std::fs::read_to_string("code").unwrap() + "\n";

    let code = code.replace("    ", "\t");

    let code = remove_comments(code);

    Type::initialize_statics();

    let mut stmts = parse_raw_oko_code(&code).unwrap();

    bake_types();

    check_each_function_is_unique(&mut stmts);

    parse_body_in_each_function(&mut stmts);

    transpile_statements_into_llvm(&mut stmts);

    unsafe { llvm::analysis::LLVMVerifyModule(llvm_module(), llvm::analysis::LLVMVerifierFailureAction::LLVMAbortProcessAction, core::ptr::null_mut()); }

	let res = unsafe { llvm::core::LLVMPrintModuleToString(llvm_module()) };
	let res = unsafe { CStr::from_ptr(res as _) };
	let res = res.to_str().unwrap();

    std::fs::write("result.ll", res).unwrap();

    Command::new("clang").arg("-Wno-override-module").arg("result.ll").arg("-o").arg("result").status().unwrap();
    let status = Command::new("./result").status().unwrap();

    println!("Resulting code:\n{res}");
    println!("The result of the program: {}", status.code().unwrap());

    drop_llvm_builder();
    drop_llvm_module();
    drop_llvm_context();
}
