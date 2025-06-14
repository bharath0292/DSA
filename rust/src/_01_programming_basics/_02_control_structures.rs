// If else statement
fn _check_numbers(x: i32) {
    if x > 0 {
        println!("Positive");
    } else if x < 0 {
        println!("Positive");
    } else {
        println!("Positive");
    }

    // If as an expression
    let _result = if x > 0 { "positve" } else { "non-positive" };
}

fn _loop_examples() {
    // loop infinite
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
    }

    // While loop
    let mut number = 0;
    while number <= 5 {
        number += 1
    }

    // For loop with range
    for i in 0..5 {
        println!("{}", i);
    }

    // For loop with iterator
    let number = vec![1, 2, 3, 4, 5];
    for (index, value) in number.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value)
    }

    // Match expression
    let day = 4;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        _ => println!("Other day"),
    }
}
