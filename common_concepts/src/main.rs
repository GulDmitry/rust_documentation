fn main() {
    // Variables and Mutability
    let x = 5; // Immutable by default.
    eprintln!("The value of x is: {}", x);
    let mut x = 5; // Immutable by default.
    x = 6;
    eprintln!("The value of x is: {}", x);

    const MAX_PINTS: u32 = 100_000;

    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "   ";
//    spaces = spaces.len(); // Impossible to mutate a variable's type.

    // Data Types
    // i32 by default - works faster.
    // f64 by default - works the same as f32.

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;

    // Array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // How functions Work
    another_function(5);

//    let x = (let y = 6); // Error, let statement doesn't return a value.
    let y = {
        let x = 3;
        x + 1 // No ending semicolon. return = no semicolon
    };
    println!("The value of y is: {}", y);

    let x = five();

    // Control flow
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else if number > 5 {
        println!("condition");
    } else {
        println!("condition was false");
    }

    let bool_var = false;
    if bool_var { // No type casting.
        println!("Never reached.");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6 // "six" causes an error because of type.
    };

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // reverse the range.
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // Fibonacci
    let mut prev = 0;
    let mut next = 0;
    let number_of_signs = 9;
    for step in 0..number_of_signs { // 0 ... 8
        println!("{}", next);
        if next == 0 {
            next = 1;
            continue;
        }

        let mut tmp = prev;
        prev = next;
        next = tmp + next;
    }

    // Ownership
    let s1 = String::from("hello");
    let s2 = s1;
//    println!("{}, world!", s1); // Error s1 is out of scope here.
    let s2 = s1.clone(); // Is fine, put s2 to heap, not just a pointer.

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // Is fine because of fixed size.
    // The same for bool, float, char, tuple of copied types.
}

fn five() -> i32 {
    5 // return
}

fn another_function(x: i32) {
    println!("Another function x is {}.", x);
}
