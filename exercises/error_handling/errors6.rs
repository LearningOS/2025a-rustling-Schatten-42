// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    /* let x = s.parse();
   
    match x {
      Err(e) => Err(ParsePosNonzeroError::from_parseint(e)),
      Ok(i) => PositiveNonzeroInteger::new(i).map_err(ParsePosNonzeroError::from_creation)
    } // Ok() 会忽略 map_err， Err() 会忽略 map */

    // 更简洁 
    s.parse().map_err(ParsePosNonzeroError::from_parseint)
             .and_then(|v| PositiveNonzeroInteger::new(v).map_err(ParsePosNonzeroError::from_creation))
      // and_then
      // 如果前一个返回的 Result 是 Ok(v) --> Ok() 解包 v 传入闭包，继续链式调用(继续new成结构体) 【✅这是我们的目的】
      // 如果前一个是 Err() ,直接提前将Err返回（和map对Err的处理完全一样）
      // 
      // and_then 和map的区别：
      // map 传入Ok会自动解包，操作后自动Ok装包回去
      // `map遇到Ok回Ok遇到Err回Err`
      // and_then会解包Ok, 但它不自动包Ok 所以必要时需要自己包上Ok

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
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
