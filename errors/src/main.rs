fn main() {
    // let v = vec![0, 1, 2, 3];
    // println!("{:?}", v[6]);
    //
    // let fruits = vec!["banana", "apple", "coconut", "orange"];
    // // Pick some fruit
    // let first = fruits.get(0);
    // println!("{:?}", first);
    // // Pick some more fruit
    // let third = fruits.get(2);
    // println!("{:?}", third);
    // // Out of index fruit
    // let out_fruit = fruits.get(99);
    // println!("{:?}", out_fruit);
    //
    let fruits = vec!["banana", "apple", "coconout", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconout") => println!("Coconouts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
         }
    }
    //
    let a_number: Option<u8> = Some(9);
    match a_number {
        Some(9) => println!("That's my lucky number!"),
        _ => {},
    }
    // if let operator: compares a pattern with an expression. If the expression matches the pattern,
    // the if block is executed.
    let n_number: Option<u8> = Some(9);
    if let Some(9) = n_number {
        println!("That's my lucky number!!!");
    }
    // unwrap and expect
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");
    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy");
}
