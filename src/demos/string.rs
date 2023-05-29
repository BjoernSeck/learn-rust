pub fn string_demo() {
    string_format();
    substrings();
}

fn string_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{}", s);
}

fn substrings() {
    let hello = "Здравствуйте";
    // in bytes. Unicode scalar values (graphemes) may be made up of more than 1 byte.
    let sub_s = &hello[0..4];

    println!("4 byte substring of {hello}: {sub_s}");

    for c in sub_s.chars() {
        println!("{c}");
    }

    for b in sub_s.bytes() {
        println!("{b}");
    }
}