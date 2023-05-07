use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Entrypoint into the program
///
/// # This is a comment
/// Here is how you do comments and comment sections
///
/// # Other
/// I'll clean up this program as I learn more concepts to do so in Rust.
fn main() {
    print_choose_path();

    let user_input: i32 = get_user_number_input();

    if user_input < 0 {
        println!("Why would you think there are negative numbers?")
    }

    match user_input {
        1 => guessing_game(),
        2 => is_divisible_by_4_3_2(),
        3 => breaking_nested_loops(),
        4 => strings(),
        5 => scope_demonstration(),
        6 => slice_demo(),
        _ => println!("Input does not equal to any value"),
    }
}

fn print_choose_path() {
    println!("Please choose program you want to run.");
    println!("---------");
    println!("1) Guessing game.");
    println!("2) Is divisible by 4, 3 or 2.");
    println!("3) Breaking in nested loops.");
    println!("4) All about Strings");
    println!("5) Scope demonstration");
    println!("6) Slice demonstration");
    println!("---------");
}

fn get_user_number_input() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("The value you entered is not a number!");
            0
        }
    };

    input
}

fn get_user_string_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: String = match input.trim().parse() {
        Ok(str) => str,
        Err(_) => {
            println!("The value you entered is not a number!");
            String::from("")
        }
    };

    input
}

/// Starts the guessing game
///
/// # Function
/// The user will choose a number and the program will tell you if its higher or lower then the
/// generated number. You'll win when the value is equal!
fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The value you entered is not a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn is_divisible_by_4_3_2() {
    println!("Choose a number!");

    let number = get_user_number_input();

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn breaking_nested_loops() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn strings() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

fn scope_demonstration() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn slice_demo() {
    println!("Enter a text:");

    let mut user_input: String = get_user_string_input();
    let user_input_clone: String = user_input.clone();

    let length = first_word_end_len(&user_input);
    let first_word_str = first_word(&user_input_clone);

    // length isn’t connected to the state of user_input at all
    user_input.clear();
    // but this would result in an error
    // user_input_clone.clear();

    println!(
        "Your sentence is (cleared to demontrate that length isn't connected with the input): {}",
        user_input
    );
    println!("Length of first word is: {}", length);
    println!("The first word is: {}", first_word_str);
}

fn first_word_end_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn struct_intro() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}