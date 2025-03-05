/*
    Infix Notation: Operators are written between the operands they operate on, e.g. 3 + 4.
    Prefix Notation: Operators are written before the operands, e.g + 3 4
    Postfix Notation: Operators are written after operands.

Infix Expressions are harder for Computers to evaluate because of the additional 
work needed to decide precedence. Infix notation is how expressions are written 
and recognized by humans and, generally, input to programs. Given that they are harder 
to evaluate, they are generally converted to one of the two remaining forms. 
A very well known algorithm for converting an infix notation to a postfix notation 
is Shunting Yard Algorithm by Edgar Dijkstra. 

This algorithm takes as input an Infix Expression and produces a queue that 
has this expression converted to postfix notation. The same algorithm can be 
modified so that it outputs the result of the evaluation of expression instead of a queue. 
The trick is using two stacks instead of one, one for operands, and one for operators. 
 */
/*
1) INFIX -> RPN
    -> loop through each char in input string
    ->if digit ,it can be number keep adding to current token
    -> if its +, - or / or * save previous number and store operator too
now i have a tokens vector
2) BUILD Synatx Tree

 */

use std::io;

 #[derive(Debug)]
enum Expression {
    Val(i32), //ma opresc cand am valoare numar
    Add(Box<Expression>, Box<Expression>), //o operatie se face intre 2 chestii 
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> i32 {
        match self {
            Expression::Val(number) => *number,
            Expression::Add(left, right) => left.eval() + right.eval(),
            Expression::Sub(left, right) => left.eval() - right.eval(),
            Expression::Mul(left,right) => left.eval() * right.eval(),
            Expression::Div(left, right) => left.eval() /right.eval(),
        }
    }
}
#[derive(Debug)]
struct Parser {
    token_vector: Vec<String>,
    index: usize,
}

impl Parser {
    fn new(expression: &str) -> Parser {
        //separ tokenurile
        let mut toks = Vec::new();
        let mut num = String::new();
        
        for ch in expression.chars()  {
            if ch.is_digit(10) {
                num.push(ch);
            } else {
                if !num.is_empty() {
                    toks.push(num.clone()); //atentie la drop si context
                    num.clear();
                }
                if ch.is_whitespace() {
                    continue;
                }
                if "+-/*".contains(ch) {
                    toks.push(ch.to_string());
                } else {
                    //return Err(format!("Invalid character: '{}'", ch));
                    println!("ups");
                    continue;
                }
            }
        }

        if !num.is_empty() {
            toks.push(num);
        }
    
        Parser {
        token_vector: toks,
        index: 0,
    }
}
fn current_token(&self) -> Option<&String> {
    self.token_vector.get(self.index)
}

// Advance to the next token
fn advance(&mut self) {
    if self.index < self.token_vector.len() {
        self.index += 1;
    }
}

fn parse_exp(&mut self) -> Expression {
    self.parse_term()
}

fn parse_term(&mut self) -> Expression {
    let mut expr = self.parse_factor();

        while let Some(op) = self.current_token().cloned() {
            if op == "+" || op == "-" {
                self.advance();
                let right = self.parse_factor();
                expr = if op == "+" {
                    Expression::Add(Box::new(expr), Box::new(right))
                } else {
                    Expression::Sub(Box::new(expr), Box::new(right))
                };
            } else {
                break;
            }
        }
        expr
}

fn parse_factor(&mut self) -> Expression {
    let mut expr = self.parse_number();

    while let Some(op) = self.current_token().cloned() {
        //trebuie sa clonez pt ca nu pot folosi referinat mutabila si imutabila at the same time
        if op == "*" || op == "/" {
            self.advance();
            let right = self.parse_number();
            expr = if op == "*" {
                Expression::Mul(Box::new(expr), Box::new(right))
            } else {
                Expression::Div(Box::new(expr), Box::new(right))
            };
        } else {
            break;
        }
    }
    expr
}

fn parse_number(&mut self) -> Expression {
    if let Some(num) = self.current_token() {
        if let Ok(value) = num.parse::<i32>() {
            self.advance();
            return Expression::Val(value);
        }
    }
    panic!("Expected a number!");
}

}
fn main() {

    println!("input: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failure input");
    let input = input.trim(); //shadowing

    let mut parser = Parser::new(&input);
    println!("{:#?}", parser);
    //let expression = 
    let expression = parser.parse_exp();
    let result = expression.eval();

    println!("Result: {}", result);

}