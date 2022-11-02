// errors5.rs

// This program uses a completed version of the code from errors4.
// It won't compile right now! Why?
// Execute `rustlings hint errors5` for hints!


use std::error;
use std::fmt;
use std::num::ParseIntError;

// 返回两种错误之一，有五种策略
// * [处理多种错误类型 - 通过例子学 Rust 中文版](https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types.html)
// TODO: update the return type of `main()` to make this compile.
// 创建一个新Error包裹旧error的处理策略如下：
// * [rust语言基础学习: 使用From和TryFrom trait进行类型之间的转换 - 架构小白|青蛙小白|关注程序开发、互联网技术、云原生](https://blog.frognew.com/2020/07/rust-from-tryfrom-trait.html)
// 第一版
// #[derive(PartialEq, Debug)]
// enum ParsePosNonzeroError {
//     Creation(CreationError),
//     ParseInt(ParseIntError),
// }

// impl FromStr for PositiveNonzeroInteger {
    // 理解这里是“和类型”
//     type Err = ParsePosNonzeroError;
//  fn from_str                                              熟悉Self::Err语法
//     fn from_str(s: &str) -> Result<PositiveNonzeroInteger, Self::Err> {
//         let x = s.parse();
//         if x.is_err() {
//             return Err(ParsePosNonzeroError::ParseInt(x.err().unwrap()));
//         }
//         let r = PositiveNonzeroInteger::new(x.unwrap());
//         match r {
//             Ok(i) => Ok(i),
//             Err(e) => Err(ParsePosNonzeroError::Creation(e)),
//         }
        
//     }
// }


// 第二版：From trait简化错误处理，就是在进行错误传播当上下游错误类型不一致时，通过实现From trait，使用?操作符自动实现下游到上游的错误转换。
// #[derive(PartialEq, Debug)]
// enum ParsePosNonzeroError {
//     Creation(CreationError),
//     ParseInt(ParseIntError),
// }

// impl From<CreationError> for ParsePosNonzeroError {
//     fn from(e: CreationError) -> Self {
//         ParsePosNonzeroError::Creation(e)
//     }
// }

// impl From<ParseIntError> for ParsePosNonzeroError {
//     fn from(e: ParseIntError) -> Self {
//         ParsePosNonzeroError::ParseInt(e)
//     }
// }

// impl FromStr for PositiveNonzeroInteger {
//     type Err = ParsePosNonzeroError;
//     fn from_str(s: &str) -> Result<PositiveNonzeroInteger, Self::Err> {
//         let x = s.parse()?;
//         Ok(PositiveNonzeroInteger::new(x)?)
//     }
// }

// fn main() -> Result<(), ParseIntError> {
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    // let output = PositiveNonzeroInteger::new(x)?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    // println!("output={:?}", output);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
