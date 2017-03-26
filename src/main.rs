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

#[derive(Copy, Clone)]
enum Token {
    LeftDelimiter,
    RightDelimiter,
    Operator(Operator, u32),
    Number(f64),
    None
}

fn is_numeric(c: char) -> bool {
    if c.is_digit(10) || c == '.' { true } else { false }
}

fn is_unary(last_token: &Token) -> bool {
    match *last_token {
        Token::LeftDelimiter => true,
        Token::Operator(_, _) => true,
        _ => false
    }
}

fn push_token(token: Token, stack: &mut Vec<Token>, output: &mut Vec<Token>) {
    match token {
        Token::LeftDelimiter => stack.push(token),
        Token::RightDelimiter => {
            while stack.len() > 0 {
                let t = stack.pop().unwrap();
                match t {
                    Token::LeftDelimiter => return,
                    _ => output.push(t)
                }
            }
            panic!("Mismatched parentheses");
        },
        Token::Operator(_, op_precedence) => {
            while stack.len() > 0 {
                match *stack.last().unwrap() {
                    Token::LeftDelimiter | Token::RightDelimiter => break,
                    Token::Operator(_, precedence) => if precedence < op_precedence { break },
                    _ => {}
                }
                output.push(stack.pop().unwrap());
            }
            stack.push(token);
        },
        Token::Number(_) => output.push(token),
        _ => {}
    }
}

fn parse(input: String) -> Vec<Token> {
    let mut is_reading_number = false;
    let mut current_str = String::new();
    let mut stack: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    let mut last_token: Token = Token::None;

    for(i, c) in input.chars().enumerate() {
        if is_numeric(c) {
            is_reading_number = true;
            current_str.push(c);
            if i+1 == input.len() {
                output.push(Token::Number(current_str.parse().unwrap()));
                last_token = *output.last().unwrap();
            }
        }
        else {
            if is_reading_number {
                is_reading_number = false;
                output.push(Token::Number(current_str.parse().unwrap()));
                last_token = *output.last().unwrap();
                current_str.clear();
            }
            if c.is_whitespace() { continue }
            let token = match c {
                '(' => Token::LeftDelimiter,
                ')' => Token::RightDelimiter,
                '+' => if !is_unary(&last_token) { Token::Operator(Operator::Add, 1) }
                       else { Token::None },
                '-' => if is_unary(&last_token) { Token::Operator(Operator::Negate, 3) }
                       else { Token::Operator(Operator::Subtract, 1) },
                '/' => Token::Operator(Operator::Divide, 2),
                '*' => Token::Operator(Operator::Multiply, 2),
                '%' => Token::Operator(Operator::Mod, 2),
                '^' => Token::Operator(Operator::Power, 4),
                _ => panic!("Invalid character in expression")
            };
            last_token = token;
            push_token(token, &mut stack, &mut output);
        }
    }

    while stack.len() > 0 {
        match *stack.last().unwrap() {
            Token::LeftDelimiter | Token::RightDelimiter => panic!("Mismatched parentheses!"),
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
            Token::Operator(operator, _) => match operator {
                Operator::Add => get_param(&mut stack) + get_param(&mut stack),
                Operator::Subtract => get_param(&mut stack) - get_param(&mut stack),
                Operator::Multiply => get_param(&mut stack) * get_param(&mut stack),
                Operator::Divide => get_param(&mut stack) / get_param(&mut stack),
                Operator::Mod => ((get_param(&mut stack) as u64) % (get_param(&mut stack) as u64)) as f64,
                Operator::Negate => -get_param(&mut stack),
                Operator::Power => get_param(&mut stack).powf(get_param(&mut stack))
            },
            _ => 0.0
        };
        stack.push(result);
    }

    if stack.len() != 1 {
        panic!("Invalid expression");
    }

    *stack.last().unwrap()
}

fn main() {
    print!(">>> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.pop();
    println!("{}", solve(parse(input)));
}
