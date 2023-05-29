use crate::demos::get_user_number_input;

pub mod if_let;
pub mod demos;

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
        1 => demos::guessing_game(),
        2 => demos::is_divisible_by_4_3_2(),
        3 => demos::breaking_nested_loops(),
        4 => demos::strings(),
        5 => demos::scope_demonstration(),
        6 => demos::slice_demo(),
        7 => demos::struct_intro(),
        8 => demos::tuple_struct_intro(),
        9 => demos::enum_demo(),
        10 => demos::option_demo(),
        11 => if_let::if_let_demo(),
        12 => demos::vector::vector_demo(),
        13 => demos::string::string_demo(),
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
    println!("7) Struct intro");
    println!("8) Tuple struct intro");
    println!("9) Enum demo");
    println!("10) Option demo");
    println!("11) If let demo");
    println!("12) Vector demo");
    println!("13) String demo");
    println!("---------");
}
