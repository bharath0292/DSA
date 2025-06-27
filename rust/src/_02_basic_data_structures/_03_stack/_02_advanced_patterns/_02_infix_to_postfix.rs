use std::char;

use crate::_02_basic_data_structures::_03_stack::_01_stack::Stack;

fn _precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

pub fn _infix_to_postfix(expr: &str) -> String{
    let mut postfix = String::new();
    let mut stack: Stack<char> = Stack::new();

    for char in expr.chars(){
        match char {
            '(' => stack.push(char),
            ')' => {
                while let Some(&top) = stack.peek(){
                    if top == '(' {
                        stack.pop();
                        break;
                    }
                    postfix.push(char);
                }
            }
            '+' | '-' | '*' | '/' => {
                while !stack.is_empty() &&
                      _precedence(*stack.peek().unwrap()) >= _precedence(char) {
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(char);
            }
            _ => {}
        }
    }

    while !stack.is_empty() {
        postfix.push(stack.pop().unwrap());
    }

    postfix
}