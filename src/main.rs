use core::{fmt, num};
use std::{fmt::Alignment::Left, fs, process::exit, vec};

//  Token Vector Display trait

// struct TokenVector {
//     vec: Vec<Token>,
// }

// impl fmt::Display for TokenVector {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for token in &self.vec {
//             write!(f, " `{}` ", token)?;
//         }
//         Ok(())
//     }
// }

// ################
// Tokens and stuff
// ################
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    INT,
    FLOAT,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
}

#[derive(Debug, Clone)]
struct Token {
    type_: TokenType,
    val_: Option<String>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match &self.val_ {
            Option::Some(some) => writeln!(f, "TYPE: '{:?}' VALUE: '{}'", self.type_, some),
            Option::None => writeln!(f, "TYPE: '{:?}' VALUE:NULL", self.type_),
        };
        val
    }
}

// #############
//     LEXER
// #############

fn lexer(code: &str) -> Vec<Token> {
    let code = code.to_string();
    let mut count = 0;

    let code_chars: Vec<char> = code.chars().collect();
    let code_len = code_chars.len();

    let mut lexed_code: Vec<Token> = vec![];

    while count < code_len {
        if code_chars[count] == ' ' || code_chars[count] == '\t' {
            count += 1;
            continue;
        }

        if code_chars[count].is_ascii_digit() {
            if code_len > count + 1 && code_chars[count].is_ascii_digit() {
                let mut is_float = false;

                let mut number: String = code_chars[count].to_string();

                if code_chars[count + 1] == '.' || code_chars[count + 1].is_ascii_digit() {
                    number = number + &code_chars[count + 1].to_string();
                    count += 1;
                }
                while code_len > count + 1 && code_chars[count + 1].is_ascii_digit() {
                    count += 1;
                    number = number + &code_chars[count].to_string();
                }
                if code_len > count + 1 && code_chars[count + 1] == '.' {
                    count += 1;
                    number = number + &code_chars[count].to_string();
                    is_float = true;

                    while code_len > count + 1 && code_chars[count + 1].is_ascii_digit() {
                        count += 1;
                        number = number + &code_chars[count].to_string();
                    }
                }
                if is_float {
                    lexed_code.push(Token {
                        type_: TokenType::FLOAT,
                        val_: Some(number),
                    });
                    count += 1;
                    continue;
                }

                lexed_code.push(Token {
                    type_: TokenType::INT,
                    val_: Some(number),
                });
                count += 1;
                continue;
            }

            lexed_code.push(Token {
                type_: TokenType::INT,
                val_: Some(code_chars[count].to_string()),
            });

            count += 1;
            continue;
        } else if code_chars[count] == '+' {
            lexed_code.push(Token {
                type_: TokenType::PLUS,
                val_: Option::None,
            });
            count += 1;
            continue;
        } else if code_chars[count] == '-' {
            lexed_code.push(Token {
                type_: TokenType::MINUS,
                val_: Option::None,
            });
            count += 1;
            continue;
        } else if code_chars[count] == '*' {
            lexed_code.push(Token {
                type_: TokenType::MUL,
                val_: Option::None,
            });
            count += 1;
            continue;
        } else if code_chars[count] == '/' {
            lexed_code.push(Token {
                type_: TokenType::DIV,
                val_: Option::None,
            });
            count += 1;
            continue;
        } else if code_chars[count] == '(' {
            lexed_code.push(Token {
                type_: TokenType::LPAREN,
                val_: Option::None,
            });
            count += 1;
            continue;
        } else if code_chars[count] == ')' {
            lexed_code.push(Token {
                type_: TokenType::RPAREN,
                val_: Option::None,
            });
            count += 1;
            continue;
        } else {
            println!(
                "Error: Invalid Token: {} index: {}",
                code_chars[count], count
            );
            exit(1);
        }
    }

    lexed_code
}

// ##########
//   NODES
// ##########

#[derive(Debug, Clone)]
enum ASTNode {
    Number(NumberNode),
    BinaryOp(BinOpNode),
    UnaryOp(UnaryOpNode),
}

#[derive(Clone, Debug)]
struct NumberNode {
    token: Token,
}
impl fmt::Display for NumberNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Type: 'NumberNode' Number: '{}'", self.token)
    }
}

#[derive(Debug, Clone)]
struct BinOpNode {
    left_node: Box<ASTNode>,
    op: TokenType,
    right_node: Box<ASTNode>,
}
impl fmt::Display for BinOpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "left_node: {:?} operator: {:?} right_node: {:?} ",
            self.left_node, self.op, self.right_node
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct UnaryOpNode {
    operand: Box<ASTNode>,
    op: TokenType,
}
impl fmt::Display for UnaryOpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Op: {:?}  Token: {:?}", self.op, self.operand)?;
        Ok(())
    }
}

// ###########
//   PARSER
// ###########
struct Parser {
    tokens: Vec<Token>,
    index: usize,
    current_token: Option<Token>,
}

impl Parser {
    fn parse(&mut self) -> ASTNode {
        let result = self.expression();
        result.unwrap()
    }

    fn advance(&mut self) -> Option<Token> {
        self.index += 1;

        if self.index < self.tokens.len() {
            self.current_token = Some(self.tokens[self.index].clone());
            return self.current_token.clone();
        }
        self.current_token = Option::None;
        return self.current_token.clone();
    }

    fn factor(&mut self) -> Option<ASTNode> {
        let token = self.current_token.as_ref().unwrap().clone();

        if matches!(token.type_, TokenType::PLUS) {
            self.advance();
            let operand = self.factor();
            return Option::Some(ASTNode::UnaryOp(UnaryOpNode {
                operand: Box::new(operand.unwrap()),
                op: TokenType::PLUS,
            }));
        } else if matches!(token.type_, TokenType::MINUS) {
            self.advance();
            let operand = self.factor();
            return Option::Some(ASTNode::UnaryOp(UnaryOpNode {
                operand: Box::new(operand.unwrap()),
                op: TokenType::MINUS,
            }));
        } else if matches!(token.type_, TokenType::INT | TokenType::FLOAT) {
            self.advance();
            return Option::Some(ASTNode::Number(NumberNode { token: token }));
        } else if matches!(token.type_, TokenType::LPAREN) {
            self.advance();
            let exp = self.expression();

            if self.current_token.is_some()
                && self.current_token.as_ref().unwrap().type_ == TokenType::RPAREN
            {
                self.advance();
                if exp.is_some() {
                    return exp;
                } else {
                    println!("Error: expression is NONE");
                    exit(1);
                }
            }
            println!("Error: cannot find Right parenthesis");
            exit(1);
        }

        Option::None
    }

    fn term(&mut self) -> Option<ASTNode> {
        let left = self.factor();

        let mut trm: BinOpNode;

        if self.current_token.as_ref().is_some()
            && matches!(
                self.current_token.as_ref().unwrap().type_,
                TokenType::DIV | TokenType::MUL
            )
        {
            let op_token = self.current_token.clone();
            self.advance();
            let right = self.factor();

            trm = BinOpNode {
                left_node: Box::new(left.unwrap()),
                right_node: Box::new(right.unwrap()),
                op: op_token.unwrap().type_,
            };

            while self.current_token.as_ref().is_some()
                && matches!(
                    self.current_token.as_ref().unwrap().type_,
                    TokenType::MUL | TokenType::DIV
                )
            {
                let left_node = trm.clone();
                let op = self.current_token.as_ref().unwrap().clone();
                self.advance();
                let right_node = self.factor();

                trm = BinOpNode {
                    left_node: Box::new(ASTNode::BinaryOp(left_node)),
                    op: op.type_,
                    right_node: Box::new(right_node.unwrap()),
                };
            }

            return Option::Some(ASTNode::BinaryOp(trm));
        }

        if left.is_some() {
            return Option::Some(left.unwrap());
        }

        Option::None
    }

    fn expression(&mut self) -> Option<ASTNode> {
        let mut left: Option<ASTNode> = self.term();
        if left.is_none() {
            left = Some(self.factor().unwrap());
        }

        let mut trm: BinOpNode;

        if self.current_token.as_ref().is_some()
            && matches!(
                self.current_token.as_ref().unwrap().type_,
                TokenType::PLUS | TokenType::MINUS
            )
        {
            let op_token = self.current_token.clone();
            self.advance();
            let right = self.term();

            trm = BinOpNode {
                left_node: Box::new(left.unwrap()),
                right_node: Box::new(right.unwrap()),
                op: op_token.unwrap().type_,
            };

            while self.current_token.as_ref().is_some()
                && matches!(
                    self.current_token.as_ref().unwrap().type_,
                    TokenType::MINUS | TokenType::PLUS
                )
            {
                let left_node = trm.clone();
                let op = self.current_token.as_ref().unwrap().clone();
                self.advance();
                let right_node = self.term();

                trm = BinOpNode {
                    left_node: Box::new(ASTNode::BinaryOp(left_node)),
                    op: op.type_,
                    right_node: Box::new(right_node.unwrap()),
                };
            }

            return Option::Some(ASTNode::BinaryOp(trm));
        }

        if left.is_some() {
            return Option::Some(left.unwrap());
        }
        Option::None
    }
}

// ############
// Interpreter
// ############

struct Interpreter {}

impl Interpreter {
    fn visit(&self, node: ASTNode) -> f64 {
        match node {
            ASTNode::Number(number) => {
                let num_str = number.token.val_.unwrap();
                let num: f64 = num_str.parse().unwrap();
                return num;
            }
            ASTNode::UnaryOp(u_node) => {
                let op = u_node.op;
                if op == TokenType::PLUS {
                    let mut value = Self::visit(self, *u_node.operand);
                    value = 1.0 * (value);

                    return value;
                } else if op == TokenType::MINUS {
                    let mut value = Self::visit(self, *u_node.operand);
                    value = -1.0 * (value);

                    return value;
                } else {
                    println!("Error: Unkown operator used in unary operation");
                    exit(1);
                }
            }
            ASTNode::BinaryOp(b_node) => {
                let mut op = b_node.op;
                let left = Self::visit(self, *b_node.left_node);
                let right = Self::visit(self, *b_node.right_node);

                match op {
                    TokenType::PLUS => {
                        return left + right;
                    }
                    TokenType::MINUS => {
                        return left - right;
                    }
                    TokenType::MUL => {
                        return left * right;
                    }
                    TokenType::DIV => {
                        return left / right;
                    }
                    _ => {
                        println!("Error: Unkown Operator used in binary operation.");
                        exit(1);
                    }
                }
            }
            _ => {
                println!("Error: Unkown node");
                exit(1);
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("file.ayl");

    let code = match content {
        Ok(code) => code,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let lexed_code = lexer(&code);

    let current_token = Some(lexed_code[0].clone());

    let mut parser = Parser {
        tokens: lexed_code,
        index: 0,
        current_token: current_token,
    };

    let parsed_code = parser.parse();

    let interpreter = Interpreter {};
    let result = interpreter.visit(parsed_code);
    println!("{}", result);
}
