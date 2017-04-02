use std::io::{self, Write};

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulus,
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

fn get_variable(name: &str) -> Option<f64> {
    match name {
        "pi"  => Some(3.1415926535),
        "tau" => Some(3.1415926535*2.0),
        "e"   => Some(2.7182818284),
        _ => None
    }
}

fn params(name: &str, stack: &mut Vec<f64>, args: usize) -> Result<Vec<f64>, String> {
    if stack.len() >= args {
        let index = stack.len() - args;
        Ok(stack.split_off(index))
    } 
    else { Err(format!("Not enough parameters for function '{}'", name)) }
}

fn run_function(name: &str, p: &mut Vec<f64>) -> Result<f64, String> {
    match name {
        "sin"  => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].sin()),
            Err(msg) => Err(msg)
        },
        "cos" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].cos()),
            Err(msg) => Err(msg)
        },
        "tan" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].tan()),
            Err(msg) => Err(msg)
        },
        "abs" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].abs()),
            Err(msg) => Err(msg)
        },
        "sign" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].signum()),
            Err(msg) => Err(msg)
        },
        "frac" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].fract()),
            Err(msg) => Err(msg)
        },
        "min" => match params(&name, p, 2) {
            Ok(v) => Ok(f64::min(v[0], v[1])),
            Err(msg) => Err(msg)
        },
        "max" => match params(&name, p, 2) {
            Ok(v) => Ok(f64::max(v[0], v[1])),
            Err(msg) => Err(msg)
        },
        "pow" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].powf(v[1])),
            Err(msg) => Err(msg)
        },
        "sqrt" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].sqrt()),
            Err(msg) => Err(msg)
        },
        "rad" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].to_radians()),
            Err(msg) => Err(msg)
        },
        "deg" => match params(&name, p, 1) {
            Ok(v) => Ok(v[0].to_degrees()),
            Err(msg) => Err(msg)
        },
        "add2" => match params(&name, p, 2) {
            Ok(v) => Ok(v[0] + v[1]),
            Err(msg) => Err(msg)
        },
        "add3" => match params(&name, p, 3) {
            Ok(v) => Ok(v[0] + v[1] + v[2]),
            Err(msg) => Err(msg)
        },
        "add4" => match params(&name, p, 4) {
            Ok(v) => Ok(v[0] + v[1] + v[2] + v[3]),
            Err(msg) => Err(msg)
        },
        _ => Err(format!("Unknown function '{}'", name))
    }
}

fn push_token(token: Token, stack: &mut Vec<Token>, output: &mut Vec<Token>) -> Result<Token, String> {
    let ret = token.clone();
    match token {
        Token::LeftParen | Token::Function(_) => stack.push(token),
        Token::RightParen => {
            while stack.len() > 0 {
                let t = stack.pop().unwrap();
                match t {
                    Token::LeftParen => return Ok(ret),
                    _ => output.push(t)
                }
            }
            return Err(String::from("Mismatched parentheses"))
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
                    Token::LeftParen => return Ok(ret),
                    _ => output.push(stack.pop().unwrap())
                }
            }
            return Err(String::from("Mismatched comma or mismatched parentheses"))
        },
        Token::Number(_) => output.push(token),
        _ => {}
    }
    Ok(ret)
}

fn is_unary(last_token: &Token) -> bool {
    match *last_token {
        Token::LeftParen | Token::Comma | Token::Operator{..} | Token::None => true,
        _ => false
    }
}

fn parse(input: &str) -> Result<Vec<Token>, String> {
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
                    if c.is_digit(10) || c == '.' {
                        current_str.push(c);
                        if i+1 == input.len() {
                            match push_token(Token::Number(current_str.parse().unwrap()), &mut stack, &mut output) {
                                Ok(token) => last_token = token,
                                Err(msg) => return Err(msg)
                            }
                            current_str.clear();
                        }
                        break;
                    }
                    else {
                        read_state = ReadState::None; 
                        match push_token(Token::Number(current_str.parse().unwrap()), &mut stack, &mut output) {
                            Ok(token) => last_token = token,
                            Err(msg) => return Err(msg)
                        }
                        current_str.clear();
                    }
                },
                ReadState::Name => {
                    if c.is_alphabetic() || c.is_digit(10) {
                        current_str.push(c);
                        if i+1 == input.len() {
                            if let Some(val) = get_variable(&current_str) {
                                match push_token(Token::Number(val), &mut stack, &mut output) {
                                    Ok(token) => last_token = token,
                                    Err(msg) => return Err(msg)
                                }
                            }
                            else { return Err(format!("Unknown variable '{}'", current_str)) }
                            current_str.clear();
                        }
                        break;
                    }
                    else if c == '(' {
                        read_state = ReadState::None;
                        match push_token(Token::Function(current_str.clone()), &mut stack, &mut output) {
                            Ok(token) => last_token = token,
                            Err(msg) => return Err(msg)
                        }
                        current_str.clear();
                    }
                    else {
                        read_state = ReadState::None; 
                        if let Some(val) = get_variable(&current_str) {
                            match push_token(Token::Number(val), &mut stack, &mut output) {
                                Ok(token) => last_token = token,
                                Err(msg) => return Err(msg)
                            }
                        }
                        else { return Err(format!("Unknown variable '{}'", current_str)) }
                        current_str.clear();
                    }
                },
                ReadState::None => {
                    if c.is_digit(10) || c == '.' {
                        read_state = ReadState::Number;
                        continue;
                    }
                    else if c.is_alphabetic() {
                        read_state = ReadState::Name;
                        continue;
                    }
                    else if c.is_whitespace() { break }
                    else {
                        if let Some(token) = match c {
                            '(' => Some(Token::LeftParen),
                            ')' => Some(Token::RightParen),
                            '+' => if !is_unary(&last_token) { Some(Token::Operator{ operator: Operator::Add, unary: false, precedence: 1 }) }
                                else { Some(Token::None) },
                            '-' => if is_unary(&last_token) { Some(Token::Operator{ operator: Operator::Negate, unary: true, precedence: 3 }) }
                                else { Some(Token::Operator{ operator: Operator::Subtract, unary: false, precedence: 1 }) },
                            '/' => Some(Token::Operator{ operator: Operator::Divide, unary: false, precedence: 2 }),
                            '*' => Some(Token::Operator{ operator: Operator::Multiply, unary: false, precedence: 2 }),
                            '%' => Some(Token::Operator{ operator: Operator::Modulus, unary: false, precedence: 2 }),
                            '^' => Some(Token::Operator{ operator: Operator::Power, unary: false, precedence: 4 }),
                            ',' => Some(Token::Comma),
                            _ => None
                        } {
                            match push_token(token, &mut stack, &mut output) {
                                Ok(token) => last_token = token,
                                Err(msg) => return Err(msg)
                            }
                        }
                        else { return Err(format!("Invalid character '{}' in expression", c)) }
                    }
                    break;
                }
            }
        }
    }
    while stack.len() > 0 {
        match *stack.last().unwrap() {
            Token::LeftParen | Token::RightParen => return Err(String::from("Mismatched parentheses")),
            _ => output.push(stack.pop().unwrap())
        }
    }
    Ok(output)
}

fn solve(tokens: Vec<Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    for token in &tokens {
        let result: f64;
        match *token {
            Token::Number(n) => result = n,
            Token::Operator{ operator, .. } => match operator {
                Operator::Add => {
                    if stack.len() >= 2 { result = stack.pop().unwrap() + stack.pop().unwrap() }
                    else { return Err(String::from("Add operator requires 2 operands")) }
                },
                Operator::Subtract => {
                    if stack.len() >= 2 {
                        let n = stack.pop().unwrap();
                        result = stack.pop().unwrap() - n;
                    }
                    else { return Err(String::from("Minus operator requires 2 operands")) }
                },
                Operator::Multiply => {
                    if stack.len() >= 2 { result = stack.pop().unwrap() * stack.pop().unwrap() }
                    else { return Err(String::from("Multiply operator requires 2 operands")) }
                },
                Operator::Divide => {
                    if stack.len() >= 2 {
                        let n = stack.pop().unwrap();
                        result = stack.pop().unwrap() / n;
                    }
                    else { return Err(String::from("Divide operator requires 2 operands")) }
                }
                Operator::Modulus => {
                    if stack.len() >= 2 {
                        let n = stack.pop().unwrap();
                        result = stack.pop().unwrap() % n;
                    }
                    else { return Err(String::from("Modulus operator requires 2 operands")) }
                }
                Operator::Negate =>  {
                    if stack.len() >= 1 { result = -stack.pop().unwrap() }
                    else { return Err(String::from("Negate operator has no operand")) }
                },
                Operator::Power => {
                    if stack.len() >= 2 {
                        let n = stack.pop().unwrap();
                        result = stack.pop().unwrap().powf(n);
                    }
                    else { return Err(String::from("Exponent operator requires 2 operands")) }
                }
            },
            Token::Function(ref name) => {
                match run_function(name, &mut stack) {
                    Ok(value) => result = value,
                    Err(msg) => return Err(msg)
                }
            },
            _ => return Err(String::from("Invalid expression"))
        };
        stack.push(result);
    }
    if stack.len() != 1 { return Err(String::from("Invalid expression")) }
    Ok(*stack.last().unwrap())
}

fn solve_expression(input: &str) -> Result<f64, String> {
    match parse(input) {
        Ok(tokens) => solve(tokens),
        Err(msg) => Err(msg)
    }
}

fn main() {
    let mut input = String::new();
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        input = args[1].clone();
    }
    else {
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.pop();
    }
    match solve_expression(&input) {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("Error: {}", msg)
    }
}

#[test]
fn long_expression() {
    assert_eq!(Ok(100.0), solve_expression(&String::from("
        ((abs(cos(((((--(abs((((1+1+(1+1)+1+1+(4*1))+1+(10-11))/10 * 10) % 9 - 10)^2-80+9))
        *10/10+(2*2 + 6)-5-2-(2+1))*2.00000000-5.6-4.400000000000000000000)-9)*pi))*10.0*
        sign(max(12.44343234, 11.84934))*add4(1.0, 2.0, -1.0, 1.0)/3)^2)+(2*2+2^2*2-2)-10")));
}
