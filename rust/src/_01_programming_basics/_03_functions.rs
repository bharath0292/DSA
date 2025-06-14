// Basic function
fn _add(x: i32, y: i32) -> i32 {
    x + y
}

// Multiple retrun values using tuple
fn _divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        return Err(String::from("division by zero"));
    }

    Ok(x / y)
}

// Generic functions
fn _print_type<T: std::fmt::Display>(item: T) {
    println!("{}", item)
}

// Closures
fn _create_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// Function with lifetime parameter
fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Implementing methods for struct
#[allow(dead_code)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[allow(dead_code)]
impl Rectangle {
    // Associated function
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Method that  modifies self
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// Traits
#[allow(dead_code)]
trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Higher order function
fn _apply<F>(f: F, x: i32, y: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(x, y)
}
