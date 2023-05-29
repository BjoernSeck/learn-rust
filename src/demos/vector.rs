pub fn vector_demo() {
    // Can hold any type. `vec!` is the macro for this.
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    // Same as let v = vec![1, 2, 3];
    println!("Full Vector: {:?}", v);
    println!("Third element: {}", &v[2]);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}