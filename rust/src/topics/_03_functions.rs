// Basic function
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Multiple retrun values using tuple
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        return Err(String::from("division by zero"));
    }

    Ok(x / y)
}

// Generic functions
fn print_type<T: std::fmt::Display>(item: T) {
    println!("{}", item)
}

// Closures
fn create_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// Function with lifetime parameter
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Implementing methods for struct
struct Rectangle {
    width: f64,
    height: f64,
}

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
trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Higher order function
fn apply<F>(f: F, x: i32, y: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(x, y)
}

pub fn main() {
    println!("add: {}", add(1, 2));
    println!("divide: {:?}", divide(5.0, 3.0));
    println!("print_type: {:?}", print_type("String"));

    let mut counter = create_counter();
    println!("create_counter: {:?}", counter());

    println!("longest: {:?}", longest("Hello", "World"));

    let mut rectangle = Rectangle::new(100.0, 200.0);
    rectangle.area();
    rectangle.scale(10.0);

    apply(add, 2, 2);
}
