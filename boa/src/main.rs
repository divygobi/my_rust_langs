use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

use sexp::Atom::*;
use sexp::*;

use im::HashMap;

#[derive(Debug)]
enum Val {
    Reg(Reg),
    Imm(i32),
    RegOffset(Reg, i32),
}

#[derive(Debug)]
enum Reg {
    RAX,
    RSP,
}

#[derive(Debug)]
enum Instr {
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
}

#[derive(Debug)]
enum Op1 {
    Add1,
    Sub1,
}

#[derive(Debug)]
enum Op2 {
    Plus,
    Minus,
    Times,
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
}


fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Number(i32::try_from(*n).unwrap()),
        Sexp::Atom(S(s)) => Expr::Id(s.to_string()),
        Sexp::List(vec) => {
            match &vec[..] {
               [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1,Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1,Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)),Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)),Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)),Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), ..] if op == "Let" => {
                    let bindings = &vec[1..vec.len() - 1];
                    let e = vec.last().unwrap();
                    let mut parsed_bindings = vec![];
                    for binding in bindings{
                        parsed_bindings.push(parse_bind(binding))
                    }
                    Expr::Let(parsed_bindings, Box::new(parse_expr(e)))
                },
                _ => panic!("parse error"),
            }
        },
        _ => panic!("parse error"),
    }
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => {
            match &vec[..] {
               [Sexp::Atom(S(s)), e] => (s.to_string(), parse_expr(e)),
                _ => panic!("parse error"),
            }
        },
        _ => panic!("parse error"),
    }
    
}



fn compile_to_instrs(e: &Expr, stack_offset: i32) -> Vec<Instr> {
    let mut instrs: Vec<Instr>  = vec![];
    match e {
        Expr::Number(n) => todo!(),
        Expr::UnOp(op, exp) if matches!(op, Op1::Add1) => todo!(),
        Expr::UnOp(op, exp) if matches!(op, Op1::Sub1) => todo!(),
        Expr::BinOp(op, exp1, exp2) if matches!(op, Op2::Plus) => todo!(),
        Expr::BinOp(op, exp1, exp2) if matches!(op, Op2::Minus) => todo!(),
        Expr::BinOp(op, exp1, exp2) if matches!(op, Op2::Times) => todo!(),
        Expr::Id(_) => todo!(),
        Expr::Let(_, _) => todo!(),
        _ => panic!("compile_error"),
    }
    return  instrs;
}

fn instr_to_str(i: &Instr) -> String {
    todo!("instr_to_str");
}

fn val_to_str(v: &Val) -> String {
    todo!("val_to_str");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file: File = File::open(in_name).expect("Something is wrong with your source file");
    let mut in_contents =  String::new();
    in_file.read_to_string(&mut in_contents).expect("Error reading in something from the source");

    let expr = parse_expr(&sexp::parse(&in_contents).unwrap());
    let stack_offset    = 0;
    let instrs = compile_to_instrs(&expr, stack_offset);
    

    // You will make result hold the result of actually compiling
    let result = "mov rax, 131";
    

    let asm_program = format!(
        "
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
