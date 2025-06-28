use core::f64;

use crate::_02_basic_data_structures::_03_stack::_01_stack::Stack;

#[allow(dead_code)]
struct Calculator {
    stack: Stack<f64>,
}

fn _is_operator(token: &str) -> bool {
    matches!(token, "+" | "-" | "*" | "/")
}

fn _apply_operator(a: Option<f64>, b: Option<f64>, operator: &str) -> f64 {
    match (a, b) {
        (Some(x), Some(y)) => match operator {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => {
                if y != 0.0 {
                    x / y
                } else {
                    f64::NAN
                }
            }
            _ => f64::NAN,
        },
        _ => f64::NAN,
    }
}

#[allow(dead_code)]
impl Calculator {
    pub fn new() -> Self {
        Calculator {
            stack: Stack::new(),
        }
    }

    pub fn evaluate(&mut self, post_fix: &str) -> f64 {
        for char in post_fix.split_whitespace() {
            if _is_operator(char) {
                let b = self.stack.pop();
                let a = self.stack.pop();

                let result = _apply_operator(a, b, char);
                self.stack.push(result);
            } else {
                match char.parse::<f64>() {
                    Ok(num) => self.stack.push(num),
                    Err(_) => {}
                }
            }
        }

        self.stack.pop().unwrap_or(f64::NAN)
    }
}
