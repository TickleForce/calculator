use std::io::{self, Write};

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Mod,
    Power,
    Negate
}

#[derive(Clone)]
enum Token {
    LeftParen,
    RightParen,
    Comma,
    Operator{ operator: Operator, unary: bool, precedence: u32 },
    Function(String),
    Number(f64),
    None
}

fn get_variable(name: &str) -> f64 {
    match name {
        "pi"  => 3.1415926535,
        "tau" => 3.1415926535*2.0,
        "e"   => 2.7182818284,
        _ => 0.0
    }
}

fn run_function(name: &str, stack: &mut Vec<f64>) -> f64 {
    match name {
        "sin"  => get_param(stack).sin(),
        "cos"  => get_param(stack).cos(),
        "tan"  => get_param(stack).tan(),
        "abs"  => get_param(stack).abs(),
        "sign" => get_param(stack).signum(),
        "frac" => get_param(stack).fract(),
        "min"  => f64::min(get_param(stack), get_param(stack)),
        "max"  => f64::max(get_param(stack), get_param(stack)),
        "pow"  => get_param(stack).powf(get_param(stack)),
        "sqrt" => get_param(stack).sqrt(),
        "rad"  => get_param(stack).to_radians(),
        "deg"  => get_param(stack).to_degrees(),
        "add2" => get_param(stack) + get_param(stack),
        "add3" => get_param(stack) + get_param(stack) + get_param(stack),
        "add4" => get_param(stack) + get_param(stack) + get_param(stack) + get_param(stack),
        _ => 0.0
    }
}

fn is_numeric(c: char) -> bool {
    if c.is_digit(10) || c == '.' { true } else { false }
}

fn is_unary(last_token: &Token) -> bool {
    match *last_token {
        Token::LeftParen | Token::Comma | Token::Operator{..} | Token::None => true,
        _ => false
    }
}

fn push_token(token: Token, stack: &mut Vec<Token>, output: &mut Vec<Token>) -> Token {
    let ret = token.clone();
    match token {
        Token::LeftParen | Token::Function(_) => stack.push(token),
        Token::RightParen => {
            while stack.len() > 0 {
                let t = stack.pop().unwrap();
                match t {
                    Token::LeftParen => return ret,
                    _ => output.push(t)
                }
            }
            panic!("Mismatched parentheses");
        },
        Token::Operator{ unary, precedence: op_precedence, .. } => {
            if unary { stack.push(token) }
            else {
                while stack.len() > 0 {
                    match *stack.last().unwrap() {
                        Token::LeftParen | Token::RightParen => break,
                        Token::Operator{ precedence, .. } => if precedence < op_precedence { break },
                        _ => {}
                    }
                    output.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
        },
        Token::Comma => {
            while stack.len() > 0 {
                match *stack.last().unwrap() {
                    Token::LeftParen => return ret,
                    _ => output.push(stack.pop().unwrap())
                }
            }
            panic!("Misplaced comma or mismatched parentheses");
        },
        Token::Number(_) => output.push(token),
        _ => {}
    }
    ret
}

fn parse(input: &str) -> Vec<Token> {
    enum ReadState {
        Number,
        Name,
        None
    }

    let mut read_state = ReadState::None;
    let mut current_str = String::new();
    let mut stack: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    let mut last_token: Token = Token::None;

    for(i, c) in input.chars().enumerate() {
        loop {
            match read_state {
                ReadState::Number => {
                    if is_numeric(c) {
                        current_str.push(c);
                        if i+1 == input.len() {
                            last_token = push_token(Token::Number(current_str.parse().unwrap()), &mut stack, &mut output);
                        }
                        break;
                    }
                    else {
                        read_state = ReadState::None; 
                        last_token = push_token(Token::Number(current_str.parse().unwrap()), &mut stack, &mut output);
                        current_str.clear();
                    }
                },
                ReadState::Name => {
                    if c.is_alphabetic() || is_numeric(c) {
                        current_str.push(c);
                        if i+1 == input.len() {
                            last_token = push_token(Token::Number(get_variable(&current_str)), &mut stack, &mut output);
                        }
                        break;
                    }
                    else if c == '(' {
                        read_state = ReadState::None;
                        last_token = push_token(Token::Function(current_str.clone()), &mut stack, &mut output);
                        current_str.clear();
                    }
                    else {
                        read_state = ReadState::None; 
                        last_token = push_token(Token::Number(get_variable(&current_str)), &mut stack, &mut output);
                        current_str.clear();
                    }
                },
                ReadState::None => {
                    if is_numeric(c) {
                        read_state = ReadState::Number;
                        continue;
                    }
                    else if c.is_alphabetic() {
                        read_state = ReadState::Name;
                        continue;
                    }
                    else if c.is_whitespace() { break }
                    else {
                        let token = match c {
                            '(' => Token::LeftParen,
                            ')' => Token::RightParen,
                            '+' => if !is_unary(&last_token) { Token::Operator{ operator: Operator::Add, unary: false, precedence: 1 } }
                                else { Token::None },
                            '-' => if is_unary(&last_token) { Token::Operator{ operator: Operator::Negate, unary: true, precedence: 3 } }
                                else { Token::Operator{ operator: Operator::Subtract, unary: false, precedence: 1 } },
                            '/' => Token::Operator{ operator: Operator::Divide, unary: false, precedence: 2 },
                            '*' => Token::Operator{ operator: Operator::Multiply, unary: false, precedence: 2 },
                            '%' => Token::Operator{ operator: Operator::Mod, unary: false, precedence: 2 },
                            '^' => Token::Operator{ operator: Operator::Power, unary: false, precedence: 4 },
                            ',' => Token::Comma,
                            _ => panic!("Invalid character in expression")
                        };
                        last_token = push_token(token, &mut stack, &mut output);
                    }
                    break;
                }
            }
        }
    }
    while stack.len() > 0 {
        match *stack.last().unwrap() {
            Token::LeftParen | Token::RightParen => panic!("Mismatched parentheses!"),
            _ => output.push(stack.pop().unwrap())
        }
    }
    output
}

fn get_param(stack: &mut Vec<f64>) -> f64 {
    stack.pop().expect("Missing operand")
}

fn solve(tokens: Vec<Token>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    for token in &tokens {
        let result: f64 = match *token {
            Token::Number(n) => n,
            Token::Operator{ operator, .. } => match operator {
                Operator::Add => get_param(&mut stack) + get_param(&mut stack),
                Operator::Subtract => {
                    let n = get_param(&mut stack);
                    get_param(&mut stack) - n
                }
                Operator::Multiply => get_param(&mut stack) * get_param(&mut stack),
                Operator::Divide => {
                    let n = get_param(&mut stack);
                    get_param(&mut stack) / n
                }
                Operator::Mod => {
                    let n = get_param(&mut stack) as u64;
                    ((get_param(&mut stack) as u64) % n) as f64
                }
                Operator::Negate => -get_param(&mut stack),
                Operator::Power => {
                    let n = get_param(&mut stack);
                    get_param(&mut stack).powf(n)
                }
            },
            Token::Function(ref name) => run_function(name, &mut stack),
            _ => 0.0
        };
        stack.push(result);
    }
    if stack.len() != 1 {
        panic!("Invalid expression");
    }
    *stack.last().unwrap()
}

fn solve_expression(input: &str) -> f64 {
    solve(parse(input))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        println!("{}", solve_expression(&args[1]));
    }
    else {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.pop();
        println!("{}", solve_expression(&input));
    }
}

#[test]
fn long_expression() {
    assert_eq!(100.0, solve_expression(&String::from("
        ((abs(cos(((((--(abs((((1+1+(1+1)+1+1+(4*1))+1+(10-11))/10 * 10) % 9 - 10)^2-80+9))
        *10/10+(2*2 + 6)-5-2-(2+1))*2.00000000-5.6-4.400000000000000000000)-9)*pi))*10.0*
        sign(max(12.44343234, 11.84934))*add4(1.0, 2.0, -1.0, 1.0)/3)^2)+(2*2+2^2*2-2)-10")));
}
