#![allow(dead_code)]

use std::io::{self, Write};

#[derive(Debug, Copy, Clone)]
enum Precedence {
	Assignment,
	LogOr,
	LogXor,
	LogAnd,
	Equality,
	Relational,
	Additive,
	Multiplicative,
	Power,
	Unary,
	Postfix,
}

#[derive(Debug, Copy, Clone)]
enum Associativity {
    None,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    // unary operators
    Negation,
    Identity,
    Not,

    // binary operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Assignment,
    Equality,
    Inequality,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Nand,
    Or,
    Nor,
    Power,
    Factorial,

    // single-arg functions
    Sin,
    Cos,
    Tan,
    Abs,
    Sqrt,
    Radians,
    Degrees,
    Ln,
    Log2,
    Log10,
    Exp,
    Sign,
    Result,

    // multi-arg functions
    Min,
    Max,

    Count
}

#[derive(Debug, Copy, Clone)]
enum Token {
    LeftParentheses,
    RightParentheses,
    ArgumentSeparator,
    Operator{ parameters: u32, precedence: Precedence, associativity: Associativity, operation: Operation },
    Function{ parameters: u32, operation: Operation },
    Operand(f64),
}

struct Calculator {
    results: Vec<f64>
}

fn operator(parameters: u32, precedence: Precedence, associativity: Associativity, operation: Operation) -> Result<Token, String> {
    Ok(Token::Operator{ parameters, precedence, associativity, operation })
}

impl Calculator {

    fn get_number(&self, pos: &mut usize, chars: &Vec<char>) -> Token {
        let mut found_dot = false;
        let mut digits = chars[*pos].to_string();

        loop {
            *pos += 1;
            if *pos == chars.len() {
                break;
            }

            let ch = chars[*pos];

            if ch.is_digit(10) {
                digits.push(ch);
            }
            else {
                if ch == '.' {
                    if !found_dot {
                        found_dot = true;
                        digits.push(ch);
                        continue;
                    }
                }
                break;
            }
        }

        Token::Operand(digits.parse().unwrap())
    }

    fn get_identifier(&self, pos: &mut usize, chars: &Vec<char>) -> Result<Token, String> {
        let mut ident = chars[*pos].to_string();

        loop {
            *pos += 1;
            if *pos == chars.len() {
                break;
            }
            let ch = chars[*pos];
            if ch.is_alphabetic() || ch.is_digit(10) {
                ident.push(ch);
                continue;
            }
            break;
        }

        match ident.to_lowercase().as_ref() {
            "pi"    => Ok(Token::Operand(std::f64::consts::PI)),
            "tau"   => Ok(Token::Operand(std::f64::consts::PI * 2.0)),
            "e"     => Ok(Token::Operand(std::f64::consts::E)),
            "true"  => Ok(Token::Operand(1.0)),
            "false" => Ok(Token::Operand(0.0)),

            "not"  => operator(1, Precedence::Unary,  Associativity::None, Operation::Not),
            "mod"  => operator(2, Precedence::Multiplicative, Associativity::Left, Operation::Modulus),
            "and"  => operator(2, Precedence::LogAnd, Associativity::Left, Operation::And),
            "nand" => operator(2, Precedence::LogAnd, Associativity::Left, Operation::Nand),
            "or"   => operator(2, Precedence::LogOr, Associativity::Left, Operation::Or),
            "nor"  => operator(2, Precedence::LogOr, Associativity::Left, Operation::Nor),

            "sin"     => Ok(Token::Function{ parameters: 1, operation: Operation::Sin }),
            "cos"     => Ok(Token::Function{ parameters: 1, operation: Operation::Cos }),
            "tan"     => Ok(Token::Function{ parameters: 1, operation: Operation::Tan }),
            "abs"     => Ok(Token::Function{ parameters: 1, operation: Operation::Abs }),
            "sqrt"    => Ok(Token::Function{ parameters: 1, operation: Operation::Sqrt }),
            "radians" => Ok(Token::Function{ parameters: 1, operation: Operation::Radians }),
            "degrees" => Ok(Token::Function{ parameters: 1, operation: Operation::Degrees }),
            "result"  => Ok(Token::Function{ parameters: 1, operation: Operation::Result }),
            "ln"      => Ok(Token::Function{ parameters: 1, operation: Operation::Ln }),
            "log2"    => Ok(Token::Function{ parameters: 1, operation: Operation::Log2 }),
            "log10"   => Ok(Token::Function{ parameters: 1, operation: Operation::Log10 }),
            "exp"     => Ok(Token::Function{ parameters: 1, operation: Operation::Exp }),
            "sign"    => Ok(Token::Function{ parameters: 1, operation: Operation::Sign }),

            "min"  => Ok(Token::Function{ parameters: 2, operation: Operation::Min }),
            "max"  => Ok(Token::Function{ parameters: 2, operation: Operation::Max }),
            "pow"  => Ok(Token::Function{ parameters: 2, operation: Operation::Power }),

            _ => Err(format!("Unknown identifier '{}'", ident))
        }
    }

    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        if expression.len() == 0 {
            return Err(String::from("Expression is empty"));
        }

        let chars: Vec<char> = expression.chars().collect();
        let mut output: Vec<Token> = Vec::new();
        let mut pos: usize = 0;

        loop {
            // ignore whitespace
            while pos != expression.len() && chars[pos].is_whitespace() {
                pos += 1;
            }

            if pos == expression.len() {
                break;
            }
            let ch = chars[pos];

            // handle characters that have no ambiquity
            if let Some(token) = match chars[pos] {
                '(' => Some(Token::LeftParentheses),
                ')' => Some(Token::RightParentheses),
                ',' => Some(Token::ArgumentSeparator),
                '/' => Some(Token::Operator{ parameters: 2, precedence: Precedence::Multiplicative, associativity: Associativity::Left, operation: Operation::Division }),
                '%' => Some(Token::Operator{ parameters: 2, precedence: Precedence::Multiplicative, associativity: Associativity::Left, operation: Operation::Modulus }),
                '^' => Some(Token::Operator{ parameters: 2, precedence: Precedence::Power, associativity: Associativity::Right, operation: Operation::Power }),
                _ => None
            } {
                output.push(token);
                pos += 1;
            }

            // numbers
            else if ch.is_digit(10) {
                output.push(self.get_number(&mut pos, &chars));
            }

            // identifiers
            else if ch.is_alphabetic() {
                let token = self.get_identifier(&mut pos, &chars)?;
                output.push(token);
            }

            else if ch == '*' {
                pos += 1;
                if pos == chars.len() ||
                    (pos != chars.len() && chars[pos] != '*') {
                    output.push(operator(2, Precedence::Multiplicative, Associativity::Left, Operation::Multiplication).unwrap());
                }
                else {
                    output.push(operator(2, Precedence::Power, Associativity::Right, Operation::Power).unwrap());
                    pos += 1;
                }
            }

            else if ch == '+' {
                match output.last() {
                    Some(&Token::Operand(..)) |
                    Some(&Token::RightParentheses) |
                    Some(&Token::Operator{ precedence: Precedence::Postfix, .. }) => {
                        output.push(operator(2, Precedence::Additive, Associativity::Left, Operation::Addition).unwrap());
                    },
                    _ => {
                        output.push(operator(1, Precedence::Unary, Associativity::None, Operation::Identity).unwrap());
                    }
                }
                pos += 1;
            }

            else if ch == '-' {
                match output.last() {
                    Some(&Token::Operand(..)) |
                    Some(&Token::RightParentheses) |
                    Some(&Token::Operator{ precedence: Precedence::Postfix, .. }) => {
                        output.push(operator(2, Precedence::Additive, Associativity::Left, Operation::Subtraction).unwrap());
                    },
                    _ => {
                        output.push(operator(1, Precedence::Unary, Associativity::None, Operation::Negation).unwrap());
                    }
                }
                pos += 1;
            }

            else if ch == '!' {
                pos += 1;
                if pos == chars.len() || (pos != chars.len() && chars[pos] != '=') {
                    output.push(operator(1, Precedence::Postfix, Associativity::None, Operation::Factorial).unwrap());
                }
                else {
                    output.push(operator(2, Precedence::Equality, Associativity::Left, Operation::Inequality).unwrap());
                    pos +=1;
                }
            }

            else if ch == '>' {
                pos += 1;
                if pos == chars.len() || (pos != chars.len() && chars[pos] != '=') {
                    output.push(operator(2, Precedence::Relational, Associativity::Left, Operation::Greater).unwrap());
                }
                else {
                    output.push(operator(2, Precedence::Relational, Associativity::Left, Operation::GreaterEqual).unwrap());
                    pos +=1;
                }
            }

            else if ch == '<' {
                pos += 1;
                if pos == chars.len() || (pos != chars.len() && chars[pos] != '=') {
                    output.push(operator(2, Precedence::Relational, Associativity::Left, Operation::Less).unwrap());
                }
                else {
                    output.push(operator(2, Precedence::Relational, Associativity::Left, Operation::LessEqual).unwrap());
                    pos +=1;
                }
            }

            else if ch == '=' {
                pos += 1;
                if pos == chars.len() || (pos != chars.len() && chars[pos] != '=') {
                    output.push(operator(2, Precedence::Assignment, Associativity::Left, Operation::Assignment).unwrap());
                }
                else {
                    output.push(operator(2, Precedence::Equality, Associativity::Left, Operation::Equality).unwrap());
                    pos +=1;
                }
            }

		    else if ch == '&' && pos + 1 != chars.len() && chars[pos + 1] == '&' {
                output.push(operator(2, Precedence::LogAnd, Associativity::Left, Operation::And).unwrap());
		        pos += 2;
		    }

		    else if ch == '|' && pos + 1 != chars.len() && chars[pos + 1] == '|' {
                output.push(operator(2, Precedence::LogOr, Associativity::Left, Operation::Or).unwrap());
		        pos += 2;
		    }

            else {
                return Err(format!("Invalid character '{}'", ch));
            }
        }

        Ok(output)
    }

    fn parse(&self, tokens: Vec<Token>) -> Result<Vec<Token>, String> {

        let mut stack: Vec<Token> = Vec::new();
        let mut output: Vec<Token> = Vec::new();

        for token in tokens {
            match token {
                Token::Operand{ .. } => output.push(token),

                Token::ArgumentSeparator => {
                    while let Some(t) = stack.last().cloned() {
                        if let Token::LeftParentheses = t { break; }
                        else { output.push(stack.pop().unwrap()); }
                    }
                    if stack.is_empty() {
                        return Err(String::from("Missing left parentheses"));
                    }
                },

                Token::Function{ .. } | Token::LeftParentheses => stack.push(token),

                Token::RightParentheses => {
                    while let Some(t) = stack.last().cloned() {
                        if let Token::LeftParentheses = t { break; }
                        else { output.push(stack.pop().unwrap()); }
                    }
                    if stack.is_empty() {
                        return Err(String::from("Missing left parentheses"));
                    }

                    // pop the left parentheses
                    stack.pop();

                    // check if parentheses followed a function
                    if let Some(last) = stack.last().cloned() {
                        if let Token::Function{ .. } = last {
                            stack.pop();
                            output.push(last);
                        }
                    }
                },

                Token::Operator{ precedence: prec1, associativity, .. } => {
                    while let Some(Token::Operator{ precedence: prec2, .. }) = stack.last().cloned() {
                        match associativity {
                            Associativity::None => break,
                            Associativity::Left if prec1 as u32 > prec2 as u32 => break,
                            Associativity::Right if prec1 as u32 >= prec2 as u32 => break,
                            _ => output.push(stack.pop().unwrap())
                        }
                    }
                    stack.push(token);
                }
            }
        }

        while let Some(stack_token) = stack.last().cloned() {
            if let Token::RightParentheses = stack_token {
                return Err(String::from("Missing right parentheses"));
            }
            output.push(stack_token);
            stack.pop();
        }

        Ok(output)
    }

    fn evaluate_rpn(&self, tokens: Vec<Token>) -> Result<f64, String> {

        let mut params : Vec<f64> = Vec::new();
        let mut operands : Vec<f64> = Vec::new();

        for token in tokens {
            match token {
                Token::Operand(value) => operands.push(value),
                Token::Function{ parameters, operation } |
                Token::Operator{ parameters, operation, .. } => {

                    if operands.len() < parameters as usize {
                        return Err(String::from("Insufficient operands"));
                    }

                    params.clear();
                    for _ in 0..parameters {
                        params.insert(0, operands.pop().unwrap());
                    }

                    match operation {
                        Operation::Identity => operands.push(params[0]),
                        Operation::Negation => operands.push(params[0] * -1.0),
                        Operation::Not => operands.push(if params[0] == 0.0 { 1.0 } else { 0.0 }),
                        Operation::Addition => operands.push(params[0] + params[1]),
                        Operation::Subtraction => operands.push(params[0] - params[1]),
                        Operation::Multiplication => operands.push(params[0] * params[1]),
                        Operation::Division => operands.push(params[0] / params[1]),
                        Operation::Modulus => operands.push(params[0] % params[1]),
                        Operation::Equality => operands.push(if params[0] == params[1] { 1.0 } else { 0.0 }),
                        Operation::Inequality => operands.push(if params[0] != params[1] { 1.0 } else { 0.0 }),
                        Operation::Less => operands.push(if params[0] < params[1] { 1.0 } else { 0.0 }),
                        Operation::LessEqual => operands.push(if params[0] <= params[1] { 1.0 } else { 0.0 }),
                        Operation::Greater => operands.push(if params[0] > params[1] { 1.0 } else { 0.0 }),
                        Operation::GreaterEqual => operands.push(if params[0] >= params[1] { 1.0 } else { 0.0 }),
                        Operation::And => operands.push(if params[0] != 0.0 && params[1] != 0.0 { 1.0 } else { 0.0 }),
                        Operation::Nand => operands.push(if params[0] != 0.0 && params[1] != 0.0 { 0.0 } else { 1.0 }),
                        Operation::Or => operands.push(if params[0] != 0.0 || params[1] != 0.0 { 1.0 } else { 0.0 }),
                        Operation::Nor => operands.push(if params[0] != 0.0 || params[1] != 0.0 { 0.0 } else { 1.0 }),
                        Operation::Power => operands.push(params[0].powf(params[1])),
                        Operation::Sin => operands.push(params[0].sin()),
                        Operation::Cos => operands.push(params[0].cos()),
                        Operation::Tan => operands.push(params[0].tan()),
                        Operation::Abs => operands.push(params[0].abs()),
                        Operation::Sqrt => operands.push(params[0].sqrt()),
                        Operation::Radians => operands.push(params[0].to_radians()),
                        Operation::Degrees => operands.push(params[0].to_degrees()),
                        Operation::Min => operands.push(f64::min(params[0], params[1])),
                        Operation::Max => operands.push(f64::max(params[0], params[1])),
                        Operation::Result => {
                            let index = params[0] as usize;
                            if index < 1 || index > self.results.len() {
                                return Err(String::from("Index out of range"));
                            }
                            operands.push(self.results[index - 1]);
                        }
                        Operation::Ln => operands.push(params[0].ln()),
                        Operation::Log2 => operands.push(params[0].log2()),
                        Operation::Log10 => operands.push(params[0].log10()),
                        Operation::Exp => operands.push(params[0].exp()),
                        Operation::Sign => operands.push(params[0].signum()),
                        Operation::Factorial => {
                            let mut x = params[0] as i64;
                            let mut y = x;
                            for i in 1..y {
                                x *= i;
                            }
                            operands.push(x as f64);
                        },
                        _ => return Err(String::from("Operation not implemented"))
                    }
                },
                _ => { }
            }
        }

        if operands.len() > 1 {
            return Err(String::from("Too many operands"))
        }

        match operands.first() {
            Some(value) => Ok(*value),
            _ => Err(String::from("Insufficient operands"))
        }
    }

    fn evaluate_expression(&mut self, input: &str) -> Result<f64, String> {
        let result = self.evaluate_rpn(self.parse(self.tokenize(input)?)?)?;
        self.results.push(result);
        Ok(result)
    }

    fn new() -> Calculator {
        let results = Vec::new();
        Calculator{ results }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut calculator = Calculator::new();

    // command line usage
    if args.len() > 1 {
        let input = args[1].clone();
        match calculator.evaluate_expression(&input) {
            Ok(result) => println!("{}", result),
            Err(msg) => println!("Error: {}", msg)
        }
    }

    // console usage
    else {
        loop {
            print!(">>> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.len() > 1 {
                match calculator.evaluate_expression(&input) {
                    Ok(result) => println!("[{}] = {}", calculator.results.len(), result),
                    Err(msg) => println!("Error: {}", msg)
                }
            }
        }
    }
}

#[test]
fn test_boolean() {
    assert_eq!(Ok(1.0), Calculator::new().evaluate_expression(&String::from("
		not false and true or false and true nand false or true and (true or false and (false nor (true and false)))"
	)));
}

#[test]
fn test_relational() {
    assert_eq!(Ok(1.0), Calculator::new().evaluate_expression(&String::from("
        8.0 > 7.9999 and 8 >= 8.0 and 8 < 9 and 1 <= 1"
    )));
}

#[test]
fn test_result() {
    let mut calculator = Calculator::new();
    calculator.evaluate_expression("1+1").unwrap();
    calculator.evaluate_expression("1+2").unwrap();
    calculator.evaluate_expression("1+3").unwrap();
    calculator.evaluate_expression("1+4").unwrap();
    assert!(calculator.evaluate_expression(&String::from("result(0)")).is_err());
    assert_eq!(Ok(2.0), calculator.evaluate_expression(&String::from("result(1)")));
    assert_eq!(Ok(3.0), calculator.evaluate_expression(&String::from("result(2)")));
    assert_eq!(Ok(4.0), calculator.evaluate_expression(&String::from("result(3)")));
    assert_eq!(Ok(5.0), calculator.evaluate_expression(&String::from("result(4)")));
}

#[test]
fn test_mixed() {
    assert_eq!(Ok(100.0), Calculator::new().evaluate_expression(&String::from("
        (abs(cos(((((--(abs((((1+1+(1+1)+1+1+(4*1))+1+(10-11))/10 * 10) % 9 - 10)^2-80+9))
        *10/10+(2*2 + 6)-5-2-(2+1))*2.00000000-5.6-4.400000000000000000000)-9)*pi))*10.0*
        sign(max(12.44343234, 11.84934)))*(1+2+3+4)+3!!-720+2**-2-0.25"
    )));
}
