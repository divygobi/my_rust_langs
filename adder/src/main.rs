use std::f32::consts::E;
use std::fmt::format;
use std::env;
use std::fs::File;
use std::io::prelude::*;

enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>)
}

fn eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add1(e1) => eval(&e1)+1,
        Expr::Sub1(e1) => eval(&e1)-1,
        Expr::Negate(e1) => eval(&e1)*-1,

    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_num() {
//       let expr1 = Expr::Num(10);
//       let result = eval(&expr1);
//       assert_eq!(result, 10);
//     }

//     #[test]
//     fn test_add1(){
//         let expr1 = Expr::Num(10);
//         let add1_to_expr1 = Expr::Add1(Box::new(expr1));
//         let result = eval(&add1_to_expr1);
//         assert_eq!(result, 1);
//     }


// }
fn main() -> std::io::Result<()>{
    //Raw text of the program to be compiled
    
    //gets all the args 
    let args: Vec<String> = env::args().collect();
    // if(args.len() != 3){
    //     Err(("You need to put in a source and target file properly"));
    // }

    //source file
    let in_name = &args[1];
    //target file
    let out_name = &args[2];

    
    let mut in_file = File::open(in_name).expect("Something is wrong with your source file");
    let mut in_contents =  String::new();
    in_file.read_to_string(&mut in_contents).expect("Error reading in something from the source");

    //Compiles the program
    let result = compile(in_contents);

    //
    let asm_program = format!("
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
", result);

    let mut out_file = File::create(out_name).expect("Something is wrong with your output file");
    out_file.write_all(asm_program.as_bytes()).expect("Error writing to the source file");
    Ok(())

}


fn compile(program: String) -> String{
    let num: i32 = program.trim().parse::<i32>().unwrap();
    return format!("mov rax, {}", num);

}

