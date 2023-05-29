//mod.rs is the old style. Use domos.rs instead without this folder. This is only intended to show that it could be usecould be used.
pub mod vector;
pub mod string;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn option_demo() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_char);
    println!("{:?}", absent_number);
    let x: i8 = 6;
    let y: Option<i8> = Some(5);
    //
    // This does not work yet:
    // let sum = x + y;
    let y = y.unwrap_or_default();

    // Now it does
    let sum = x + y;

    println!("{sum}")
}

pub fn get_user_number_input() -> i32 {
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

pub fn get_user_string_input() -> String {
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
pub fn guessing_game() {
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

pub fn is_divisible_by_4_3_2() {
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

pub fn breaking_nested_loops() {
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

pub fn strings() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

pub fn scope_demonstration() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

pub fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

pub fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

pub fn slice_demo() {
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

pub fn first_word_end_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn struct_intro() {
    let mut user1 = build_user(String::from("anotheremail"), String::from("anotheremail"));

    user1.email = String::from("anotheremail@example.com");

    println!(
        "User1: {}, {}, {}, {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    // Reminder: To ensure memory safety, after the line let user2 = ...,
    // Rust considers user1 as no longer valid. Its values are **moved**
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // This will not move user3. email and username do not implement the Copy Trait
    // but the other values do.
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user2
    };

    println!(
        "User2: {}, {}, {}, {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    println!(
        "User3: {}, {}, {}, {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );
}

fn build_user(email: String, username: String) -> User {
    //Init shorthand
    //Because the email field and the email parameter have the same name,
    // we only need to write email rather than email: email.
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn tuple_struct_intro() {
    let black = Color(0, 0, 0);
    let origin = Point(1, 1, 1);

    println!("{}", black.0);
    println!("{}", origin.0);

    //Unit-Like Structs Without Any Fields
    //let subject = AlwaysEqual;
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
//struct AlwaysEqual;

pub fn enum_demo() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let message_quit = Message::Quit;
    // let message_move = Message::Move {x: 1, y: 2};
    let message_write = Message::Write(String::from("test"));
    let message_change_color = Message::ChangeColor(1, 2, 3);
    println!("{:?}", message_quit);
    // println!("{:?}", message_move);
    println!("{:?}", message_write);
    println!("{:?}", message_change_color);
    message_write.call();
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    // Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Called Message");
    }
}
