use std::char;

use crate::_02_basic_data_structures::_03_stack::_01_stack::Stack;

pub fn _is_balanced(expr: &str) -> bool {
    let mut stack = Stack::<char>::new();

    for char in expr.chars() {
        match char {
            '(' | '{' | '[' => stack.push(char),
            _ => {
                if stack.is_empty() {
                    return false;
                }
                match stack.pop() {
                    Some(last) if _is_matching(last, char) => {}
                    _ => return false,
                }
            }
        }
    }

    stack.is_empty()
}

fn _is_matching(open: char, close: char) -> bool {
    matches!((open, close), ('(', ')') | ('{', '}') | ('[', ']'))
}
