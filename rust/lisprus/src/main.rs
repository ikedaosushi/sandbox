use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::f64::consts;

#[derive(Debug,Clone)]
enum Val {
    List(Vec<Val>),
    Number(f64),
    Symbol(String),
}

type Env = HashMap<String, Val>;

fn main() {
    let mut global_env = standard_env();
    loop {
        print!("lisp.rs> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let mut input_ = input.trim();
        println!("{}", input_);
        read_eval_print(input_, &mut global_env);
    }
}

fn standard_env() -> Env {
    let mut env: Env = HashMap::new();
    env.insert("pi".to_string(), Val::Number(consts::PI));
    env
}

fn read_eval_print(program: &str, mut env: &mut Env) {
    let parsed_program = parse(program);
    eval(parsed_program, &mut env);
    print!("=> ");
    println!("");
}

fn parse(program: &str) -> Val {
    let mut tokens = tokenize(program);
    let val = read_from_tokens(&mut tokens);
    val
}

fn tokenize(chars: &str) -> Vec<String> {
    chars.replace("("," ( ").replace(")", " ) ").split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Val {
    if tokens.len() == 0 {
        panic!("unexpected EOF while reading");
    }

    let token = tokens.remove(0);
    println!("reading token: '{}'", token);

    if "(".to_string() == token {
        let mut list: Vec<Val> = Vec::new();
        while tokens[0] != ")".to_string() {
            list.push(read_from_tokens(tokens));
        }
        tokens.remove(0);
        Val::List(list)
    } else if ")".to_string() == token {
        panic!("unexpected ')' while reading");
    } else {
        atom(token)
    }
}

fn atom(token: String) -> Val {
    let number = token.parse::<f64>();
    match number {
        Ok(x) => Val::Number(x),
        Err(_) => Val::Symbol(token),
    }
}

fn eval(val: Val, mut env: &mut Env) -> Val {
    println!("eval {:?}", &val);
    let result = match val {
        Val::Symbol(x) => access_variable(x, &mut env),
        Val::Number(_) => val,
        Val::List(list) => {
            let mut args = list;
            let proc_name = args.remove(0);
            match proc_name {
                Val::Symbol(symbol) => {
                    match symbol.trim() {
                        "quote" => args.remove(0),
                        "if" => {
                            let test = args.remove(0);
                            let conseq = args.remove(0);
                            let alt = args.remove(0);
                            let test_result = eval(test, &mut env);
                            let exp = if !is_false(test_result) { conseq } else { alt };
                            eval(exp, &mut env)
                        },
                        // "defun" => {
                        //     let var = args.remove(0);
                        //     let exp = args.remove(0);
                        //     let var_name = match var {
                        //         Val::Symbol(x) => x,
                        //         _ => panic!("First arg to define must be a symbol"),
                        //     };
                        //     let exp_result = eval(exp, &mut env);
                        //     env.insert(var_name, exp_result);
                        //     symbol_false()
                        // },
                        _ => {
                            let evaluated_args: Vec<Val> = args.iter().map(|arg| eval(arg.clone(), &mut env)).collect();
                            call_proc(&symbol, evaluated_args)
                        },
                    }
                },
                _ => panic!("unknown list form"),
            }
        }
    };
    println!("eval result {:?}", &result);
    result
}


fn access_variable(var_name: String, env: &Env) -> Val {
    match env.get(&var_name) {
        Some(x) => x.clone(),
        None => panic!("undefined variable {}", var_name),
    }
}

fn apply_arithmetic<F: Fn(f64, f64) -> f64>(args: Vec<Val>, operator: F) -> Val {
    let mut accumulated: f64 = 0f64;
    for i in 0..args.len() {
        accumulated = match args[i] {
            Val::Number(operand) => {
                if i == 0 { operand } else { operator(accumulated, operand) }
            },
            _ => panic!("args to arithmetic functions must be Numbers"),
        };
    };
    Val::Number(accumulated)
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn sub(a: f64, b: f64) -> f64 {
    a - b
}

fn mul(a: f64, b: f64) -> f64 {
    a * b
}

fn div(a: f64, b: f64) -> f64 {
    a / b
}

fn call_proc(proc_name: &String, mut args: Vec<Val>) -> Val {
    match proc_name.trim() {
        "+" => apply_arithmetic(args, add),
        "-" => apply_arithmetic(args, sub),
        "*" => apply_arithmetic(args, mul),
        "/" => apply_arithmetic(args, div),
        ">" => apply_compare(args, gt),
        "<" => apply_compare(args, lt),
        ">=" => apply_compare(args, gte),
        "<=" => apply_compare(args, lte),
        "=" => apply_compare(args, eq),
        "not" => apply_not(args, not),
        "list" => Val::List(args),
        "begin" => {
            match args.pop() {
                Some(x) => x,
                None => symbol_false(),
            }
        }
        _ => panic!("unknown proc"),
    }
}

fn apply_not<F: Fn(Val) -> Val>(args: Vec<Val>, func: F) -> Val {
    if args.len() != 1 {
        panic!("Wrong number of args. expected 1, got {}", args.len());
    } else {
        func(args[0].clone())
    }
}

fn not(val: Val) -> Val {
    let boolean = is_false(val);
    bool_to_symbol(boolean)
}

fn is_false(val: Val) -> bool {
    match val {
        Val::Symbol(x) => x == "#f",
        _ => false,
    }
}

fn apply_compare<F: Fn(Val, Val) -> Val>(args: Vec<Val>, func: F) -> Val {
    if args.len() != 2 {
        panic!("Wrong number of args, expected 2, got {}", args.len());
    } else {
        func(args[0].clone(), args[1].clone())
    }

}

fn gt(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) > extract_number(b))
}

fn lt(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) < extract_number(b))
}

fn gte(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) >= extract_number(b))
}

fn lte(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) <= extract_number(b))
}

fn eq(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) == extract_number(b))
}

fn extract_number(val: Val) -> f64 {
    match val {
        Val::Number(x) => x,
        _ => panic!("expected a Number, got (TODO)")
    }
}

fn bool_to_symbol(x: bool) -> Val {
    if x { symbol_true() } else { symbol_false() }
}

fn symbol_true() -> Val {
    Val::Symbol("#t".to_string())
}

fn symbol_false() -> Val {
    Val::Symbol("#f".to_string())
}




