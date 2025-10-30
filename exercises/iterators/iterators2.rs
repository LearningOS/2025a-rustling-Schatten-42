// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        // Some(first) => first.to_uppercase().to_string() + &input[1..],
        // Some(first) => first.to_uppercase().to_string() + c.as_str(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        // char 的 to_uppercase() 会返回一个ToUppercase 迭代器，collect变String
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
  // [lowB 做法]
  // let mut ss = Vec::<String>::new();
  // for s in words.iter() {
  //   ss.push(capitalize_first(s)); // 这里自动把 s:&&str 解引用成 &str 了
  // }
  // ss
  // [高级做法]
  words.iter().map(|w| capitalize_first(w)).collect()
  // map 从iter中获取到的w 也是 &&str [传入什么，就传出什么] 回的仍是迭代器
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {

  // 第二题需要： String 拼成Vec<String>
  // 第三题需要：多个String直接拼String
  // 使用 collect 的 “目标导向类型推导”（turbofish）可以一行代码实现两个功能
    words.iter().map(|w| capitalize_first(w)).collect()
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
