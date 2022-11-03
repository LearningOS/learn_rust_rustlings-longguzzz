// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


// fn main() {
//     let optional_word = Some(String::from("rustlings"));
//     // TODO: Make this an if let statement whose value is "Some" type
//     if let word = optional_word {
//         println!("The word is: {:?}", word);
//     } else {
//         println!("The optional word doesn't contain anything");
//     }

//     let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
//     for x in 1..10 {
//         optional_integers_vec.push(Some(x));
//     }

//     // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
//     // You can stack `Option<T>`'s into while let and if let
//     //    不知道为什么，这样会卡死
//     while let integer = optional_integers_vec.pop() {
//         println!("current value: {:?}", integer);
//     }
// }

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let word = optional_word {
        println!("The word is: {:?}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    // * [Rustlings option2: Using if let and while let to process Optional values | egghead.io](https://egghead.io/lessons/rust-rustlings-option2-using-if-let-and-while-let-to-process-optional-values)
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {}", integer);
    }
}
