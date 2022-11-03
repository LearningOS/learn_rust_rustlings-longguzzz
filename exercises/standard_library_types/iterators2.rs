// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        // Some(first) => {
        //     c.nth(0).make_ascii_uppercase();
        // },
        // * [Why is capitalizing the first letter of a string so convoluted in Rust? - Stack Overflow](https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust)
        // 要找到正确的概念来表达，非常困难……
        // 比如，是要用c.nth(0)还是说c[0]，还是说某种函数式编程的方式……
        // 有很多可能思路，但是能通过编译的很难找

        // 所以Rust里除了语法和相应的各种概念，还要积累各种 接口 的知识
        // 包括说，接口自身的定义、相互间一般怎么联调……
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
    // c.to_string()
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // let iterators = words.iter_mut();
    // let iterators = words.iter();
    // let mut iterators = words.iter();
    // let iterators = words.iter().collect();
    
    // let mut res = Vec::from(words);
    // let mut res = Vec::<String>::from(words);

    // let mut res = Vec::from(words);
    // let iterators = res.iter_mut();




    // let res = Vec::from(words);
    // let iterators = res.iter();
    // while let Some(word) = iterators.next() {
    //     capitalize_first(word);
    // }

    // // Vec<String>::from(res)
    // // Vec::<String>::from(res)
    // let res : Vec<String> = res.iter().collect();
    // res


    // let words = words.iter().map(capitalize_first).collect::<Vec<String>>();


    // * [Rustlings iterators2: Introducing Iterators by manually calling .next and using .iter() | egghead.io](https://egghead.io/lessons/rust-rustlings-iterators2-introducing-iterators-by-manually-calling-next-and-using-iter)
    // 会写，比能读的要求要高得多
    // 但是也得尽量积累“各接口是怎么定义的”、“能怎么写”……这些用法
    words.iter().map(|w| capitalize_first(w)).collect::<Vec<String>>()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // String::new()
    // words.iter().map(capitalize_first).collect::<String>()
    //                  expected signature of `fn(&&str) -> _`
    //              required by a bound introduced by this call
    // trait bounds were not satisfied
    words.iter().map(|w| capitalize_first(w)).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
