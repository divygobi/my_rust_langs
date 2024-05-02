use std::{env,  vec};
use std::fs::File;
//use std::hash::Hash;
use std::io::{prelude::*, stdin, stdout};

use sexp::Atom::*;
use sexp::*;

//use im::{hashmap, HashMap, HashSet};

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

impl Reg{
    fn to_string(&self) -> String {
        match self {
            Reg::RAX => "rax".to_string(),
            Reg::RSP => "rsp".to_string(),
        }
    }
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
                [Sexp::Atom(S(op)), Sexp::List(bindings), e] if op == "let" => {
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
               [Sexp::Atom(S(s))] => panic!("Unbound variable identifier {}", s),
                _ => panic!("parse error"),
            }
        },
        _ => panic!("parse error"),
    }
    
}



type Stack = im::HashMap<String, i32>;

fn compile_to_instrs(e: &Expr, stack: &Stack, mut stck_ptr: i32) -> Vec<Instr> {
    let mut instrs: Vec<Instr>  = vec![];
    match e {
        Expr::Number(n) => {
            instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n)));
        },
        Expr::Id(var) => {
            if !stack.contains_key(var){
                panic!("Unbound variable identifier {}", var)
            }
            instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, *stack.get(var).unwrap())));
        },

        Expr::Let(bindings, e) => {

           //recursively do the expressions for each thing, at the end of each bindings, they will compile down to
           //some sort of number in RAX
           //that value in RAX will be put inside the stack, and itll have an assigned thing based on let

           let mut scope_bindings = im::HashSet::new();

           let mut new_stack: Stack = stack.clone();
           for binding in bindings{
                if scope_bindings.contains(&binding.0) {
                    panic!("Duplicate binding in the same scope");
                }
                let mut binding_code = compile_to_instrs(&binding.1, &stack, stck_ptr);
                let bindings_pos = stck_ptr + 1;
                instrs.append(&mut binding_code);
                //the size var here is being force casted into a i32s
                let save_binding = Instr::IMov(Val::RegOffset(Reg::RSP, bindings_pos), Val::Reg(Reg::RAX));
                instrs.push(save_binding);
                new_stack = new_stack.update(binding.0.clone(), bindings_pos);
                scope_bindings.insert(&binding.0);
                //update stck ptr
                stck_ptr += 1;
           } 
           instrs.append(&mut compile_to_instrs(e, &new_stack, stck_ptr));
            
        },
        Expr::UnOp(op, exp) if matches!(op, Op1::Add1) => {
            let mut expr_code = compile_to_instrs(&exp, &stack, stck_ptr);
            instrs.append(&mut expr_code);
            instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1)));
        },
        Expr::UnOp(op, exp) if matches!(op, Op1::Sub1) => {
            let mut expr_code = compile_to_instrs(&exp, &stack, stck_ptr);
            instrs.append(&mut expr_code);
            instrs.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(1)));
        },
        Expr::BinOp(op, exp1, exp2) if matches!(op, Op2::Plus) => {
           
            //compile into assembly fo the first expression in the bin op
            let mut exp1_code = compile_to_instrs(&exp1, &stack.clone(), stck_ptr);
            //reserve a space in the space at stack pointer + 1
            let exp1_pos = stck_ptr + 1;
            instrs.append(&mut exp1_code);

            //put the resulting RAX from exp one into stack pointer + 1
            let save_exp1: Instr = Instr::IMov(Val::RegOffset(Reg::RSP, exp1_pos), Val::Reg(Reg::RAX));

            instrs.push(save_exp1);

                        //compile into assembly fo the first expression in the second op
            let mut exp2_code = compile_to_instrs(&exp2, &stack.clone(), stck_ptr);
            instrs.append(&mut exp2_code);

            let add_exps: Instr = Instr::IAdd(Val::Reg(Reg::RAX),Val::RegOffset(Reg::RSP, exp1_pos));
            instrs.push(add_exps);
        },
        Expr::BinOp(op, exp1, exp2) if matches!(op, Op2::Minus) => {
            //compile into assembly fo the first expression in the bin op
            let mut exp1_code = compile_to_instrs(&exp1, &stack.clone(), stck_ptr);
            //reserve a space in the space at stack pointer + 1
            let exp1_pos = stck_ptr + 1;
            instrs.append(&mut exp1_code);

            //put the resulting RAX from exp one into stack pointer + 1
            let save_exp1: Instr = Instr::IMov(Val::RegOffset(Reg::RSP, exp1_pos), Val::Reg(Reg::RAX));

            instrs.push(save_exp1);

                        //compile into assembly fo the first expression in the second op
            let mut exp2_code = compile_to_instrs(&exp2, &stack.clone(), stck_ptr);
            instrs.append(&mut exp2_code);

            let add_exps: Instr = Instr::ISub(Val::RegOffset(Reg::RSP, exp1_pos), Val::Reg(Reg::RAX));
            instrs.push(add_exps);
        },
        Expr::BinOp(op, exp1, exp2) if matches!(op, Op2::Times) => {
             //compile into assembly fo the first expression in the bin op
            let mut exp1_code = compile_to_instrs(&exp1, &stack.clone(), stck_ptr);
            //reserve a space in the space at stack pointer + 1
            let exp1_pos = stck_ptr + 1;
            instrs.append(&mut exp1_code);

            //put the resulting RAX from exp one into stack pointer + 1
            let save_exp1: Instr = Instr::IMov(Val::RegOffset(Reg::RSP, exp1_pos), Val::Reg(Reg::RAX));

            instrs.push(save_exp1);

                        //compile into assembly fo the first expression in the second op
            let mut exp2_code = compile_to_instrs(&exp2, &stack.clone(), stck_ptr);
            instrs.append(&mut exp2_code);

            let add_exps: Instr = Instr::IMul(Val::Reg(Reg::RAX),Val::RegOffset(Reg::RSP, exp1_pos));
            instrs.push(add_exps);
            },
            _ => {panic!("compile_error")},
        };
     
     return instrs;
}
    


fn instr_to_str(i: &Instr) -> String {
    match i { 
        Instr::IAdd(v1, v2) => {
            format!("add {}, {}",val_to_str(v1),val_to_str(v2)).to_string()
        }
        Instr::IMov(v1, v2) => {
            format!("mov {}, {}",val_to_str(v1),val_to_str(v2)).to_string()
        },
        Instr::ISub(v1, v2) => {
            format!("sub {}, {}",val_to_str(v1),val_to_str(v2)).to_string()
        },
        Instr::IMul(v1, v2) => {
            format!("mul {}, {}",val_to_str(v1),val_to_str(v2)).to_string()
        },
    }
}

fn val_to_str(v: &Val) -> String {
    match v {
        Val::Imm(i) => {
            i.to_string()
        },
        Val::Reg(reg) => {
            reg.to_string()    
        },
        Val::RegOffset(reg, offset)  => {
           format!("[{} - 8*{}]", reg.to_string(), *offset).to_string()
        }

    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "-i" {
        println!("You are in interactive mode!");   
        stdout().flush()?; // Flush stdout to ensure the message is displayed immediately

        loop {
            print!(">");
            stdout().flush()?; // Flush stdout to ensure the message is displayed immediately
            let mut buffer = String::new();
            stdin().read_line(&mut buffer)?;
            let trimmed_buffer = buffer.trim();
            if trimmed_buffer == "exit" {
                return Ok(());
            }
            let expr = parse_expr(&sexp::parse(&trimmed_buffer).unwrap());
            let stack_offset    = 0;
            let stack = im::HashMap::new();
            let instrs = compile_to_instrs(&expr, &stack,stack_offset);        
            let mut instr_string = "".to_owned();
            for instr in instrs{
                instr_string.push_str(instr_to_str(&instr).as_str());
                instr_string.push_str("\n");
            }


            print!("{}",instr_string);
        }
      
    }

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file: File = File::open(in_name).expect("Something is wrong with your source file");
    let mut in_contents =  String::new();
    in_file.read_to_string(&mut in_contents).expect("Error reading in something from the source");

    let expr = parse_expr(&sexp::parse(&in_contents).unwrap());
    let stack_offset    = 0;
    let stack = im::HashMap::new();
    let instrs = compile_to_instrs(&expr, &stack,stack_offset);
    

    // You will make result hold the result of actually compiling
    let mut result = "".to_owned();
    for instr in instrs{
        result.push_str(instr_to_str(&instr).as_str());
        result.push_str("\n");
    }
    

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
