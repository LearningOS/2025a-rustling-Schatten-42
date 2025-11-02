// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


/*
第 1 步：底层结构

  要理解 Cow，首先要看它的定义。Cow 本质上是一个枚举，它有两个变体（variants）：

   1 pub enum Cow<'a, T: ?Sized + ToOwned> {
   2     Borrowed(&'a T),
   3     Owned(<T as ToOwned>::Owned),
   4 }

  让我们来分解这个定义：

   1. `enum Cow<'a, T>`:
       * 'a 是一个生命周期参数。它告诉编译器 Cow 中借用的数据能活多久。
       * T 是一个泛型参数，代表 Cow 所封装的数据类型。

   2. `T: ?Sized + ToOwned`:
       * ?Sized：这个约束表示 T 类型的大小在编译时可以不是固定的。这非常重要，因为它允许我们使用 Cow<'a, str>
          和 Cow<'a, [i32]> 这样的类型，因为 str 和 [i32] 都是动态大小类型（DSTs）。
       * ToOwned：这是 Cow 的魔法核心。ToOwned 是一个 trait，它定义了如何从一个借用的引用（如
         &T）创建一个拥有的副本。
           * 对于 str 类型，它的 Owned 版本就是 String。
           * 对于 [i32] 切片，它的 Owned 版本就是 Vec<i32>。

   3. 变体 `Borrowed(&'a T)`:
       * 这个变体持有一个对类型 T 的不可变引用。
       * 当 Cow 是 Borrowed
         状态时，它不拥有数据，只是“看”着别人的数据。这非常轻量，只占用一个指针的内存空间，没有堆内存分配。

   4. 变体 `Owned(<T as ToOwned>::Owned)`:
       * 这个变体持有一个 T 类型对应的拥有所有权的版本。
       * 例如，对于 Cow<'a, str>，这个变体就是 Owned(String)。
       * 当 Cow 是 Owned 状态时，它拥有自己的数据，通常这些数据存储在堆上。

  所以，一个 Cow<'a, str> 变量，它要么是一个 Borrowed(&'a str)，要么是一个 Owned(String)。

第 2 步：使用方法

1. 创建 Cow

  你可以使用 Cow::Borrowed 或 Cow::Owned 来创建它，也可以用 From trait 隐式转换。

  -  use std::borrow::Cow;
  -  
  -  // 创建一个借用的 Cow
  -  let borrowed_str: Cow<'_, str> = Cow::Borrowed("这是一个不可变的静态字符串");
  -  
  -  // 从 &str 隐式转换
  -  let borrowed_from: Cow<'_, str> = "这也是借用的".into();
  -  
  -  // 创建一个拥有的 Cow
  -  let owned_string: Cow<'_, str> = Cow::Owned(String::from(
  -  "这是一个堆上分配的字符串"));
  -  
  -  // 从 String 隐式转换
  -  let owned_from: Cow<'_, str> = String::from("这也是拥有的").into();

  2. 读取数据

  Cow 实现了 Deref trait，这意味着你可以像直接使用 &T 一样调用它的方法，而无需关心它是
  Borrowed 还是 Owned。

  - use std::borrow::Cow;
  - 
  - fn print_length(s: Cow<'_, str>) {
  -     // 直接调用 &str 的方法，因为 Cow<str> Deref 到 &str
  -     println!("'{}' 的长度是: {}", s, s.len());
  - }
  - 
  - print_length(Cow::Borrowed("你好")); // 输出: '你好' 的长度是: 6
  - print_length(Cow::Owned(String::from("Hello"))); // 输出: 'Hello' 的长度是: 5

  3. 修改数据 (写时复制的核心)

  当你需要修改数据时，Cow 的真正威力就显现出来了。你需要使用 to_mut() 方法。

   * 如果 Cow 是 Owned 状态，to_mut() 会返回一个指向内部拥有数据的可变引用。没有额外开销。
   * 如果 Cow 是 Borrowed 状态，to_mut() 会首先克隆借用的数据，将其变成 Owned
     状态，然后返回一个指向新克隆出来的数据的可变引用。这就是“写时复制”。

    1 use std::borrow::Cow;
    2 
    3 // 1. 从一个借用开始
    4 let mut cow: Cow<'_, str> = "hello".into();
    5 println!("修改前: cow 指向一个借用的 &str");
    6 
    7 // 此时 cow 是 Borrowed(&'static str)
    8 // 我们想在后面加上 ", world"
    9 // to_mut() 会检查到它是 Borrowed，于是：
   10 // 1. 调用 "hello".to_owned() -> String::from("hello")
   11 // 2. 将 cow 内部状态变为 Owned(String::from("hello"))
   12 // 3. 返回 &mut String
   13 let mut_str = cow.to_mut();
   14 mut_str.push_str(", world");
   15 
   16 println!("修改后: {}", cow); // 输出: 修改后: hello, world
   17 
   18 // 此时 cow 已经是 Owned(String) 了
   19 // 再次调用 to_mut() 不会再发生克隆
   20 println!("再次修改: cow 已经拥有数据，直接修改");
   21 cow.to_mut().push_str("!");
   22 
   23 println!("最终结果: {}", cow); // 输出: 最终结果: hello, world!

  4. 获取所有权

  如果你希望无论如何都得到一个拥有的值，可以使用 into_owned()。

   * 如果 Cow 是 Owned，它会直接返回拥有的值（零成本）。
   * 如果 Cow 是 Borrowed，它会克隆数据并返回新的拥有值。

    1 use std::borrow::Cow;
    2 
    3 let borrowed: Cow<'_, str> = "borrowed".into();
    4 let owned: Cow<'_, str> = String::from("owned").into();
    5 
    6 // borrowed 是 Borrowed, .into_owned() 会克隆
    7 let new_string_1: String = borrowed.into_owned();
    8 
    9 // owned 是 Owned, .into_owned() 只是移动所有权
   10 let new_string_2: String = owned.into_owned();

*/

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            // 如果出现了负数,立马调用to_mut,往后的 input 就是 Owned 状态的了
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);// 这里直接声明了一个 Owned 状态的 input 了
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // 返回的 input 就是原来的样子 (Owned 字段持有所有权)
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice); // 这里直接声明了一个 Owned 状态的 input 了
        match abs_all(&mut input) {      // 因为Owned本来就拥有所有权, 他自己可以随便改input
            // TODO                     // to_mut() 返回原来的所有权的可变引用, 但最终返回值(是input)仍然是被修改后的原所有权对应的 Cow
            Cow::Owned(_) => Ok(()), // 返回的 input 仍然是 Owned, 只不过可变引用修改了一下内部元素 (Owned 字段持有所有权)
            _ => Err("Expected owned value"),

        }
    }
}
// 自己总结: cow 用于不确定是否要修改某个数据的情况
// 1. 以默认只读开始, Cow::Borrowed("xxx") 可以创建一个不可变的静态字符串
// 2. 在Owned / Borrowed 状态下都可以直接读取(默认实现了解引用)
// 3. 需要修改时,再使用to_mut, 这个时候, 会返回一个新的可变引用(Owned数据不变, Borrowed数据被`写时复制`)
// 4. 如果需要所有权,使用 .into_owned(): Borrowed数据被复制一份,返回副本的所有权 / Owned 数据用直接返回其所有权(零成本)

/* 
第 3 步：应用场景

  Cow 主要用在那些“大部分情况下读取，偶尔需要修改”的场景中。

  场景 1：作为函数参数，提高 API 灵活性

  假设你要写一个函数，它接收一个字符串，如果字符串符合某个规则，就原样返回；如果不符合，就修改它再返回。

  不使用 `Cow` 的笨办法：
  函数总是返回 String。这意味着即使输入是 &str 且无需修改，你也必须克隆它，造成浪费。

   1 // 浪费性能的实现
   2 fn process_message_bad(msg: &str) -> String {
   3     if msg.contains("error") {
   4         msg.replace("error", "ok") // 创建新的 String
   5     } else {
   6         msg.to_owned() // 不想修改，但为了匹配返回类型，被迫克隆
   7     }
   8 }

  使用 `Cow` 的优雅实现：

    1 use std::borrow::Cow;
    2 
    3 fn process_message_good(msg: &str) -> Cow<'_, str> {
    4     if msg.contains("error") {
    5         // 需要修改，创建 String 并包装成 Cow::Owned
    6         Cow::Owned(msg.replace("error", "ok"))
    7     } else {
    8         // 无需修改，直接借用输入的 &str，包装成 Cow::Borrowed
    9         Cow::Borrowed(msg)
   10     }
   11 }
   12 
   13 let msg1 = "这是一个 error";
   14 let processed1 = process_message_good(msg1); // 返回 Cow::Owned(String)
   15 
   16 let msg2 = "一切正常";
   17 let processed2 = process_message_good(msg2); // 返回 Cow::Borrowed(&str)，零复制！
   18 
   19 println!("处理前: '{}', 处理后: '{}'", msg1, processed1);
   20 println!("处理前: '{}', 处理后: '{}'", msg2, processed2);



  场景 2：数据结构中的字段

  假设你有一个配置结构体，其中一些字段有静态的默认值，但用户也可以提供自定义值。

    1 use std::borrow::Cow;
    2 
    3 struct Config {
    4     prompt: Cow<'static, str>,
    5 }
    6 
    7 impl Default for Config {
    8     fn default() -> Self {
    9         Config {
   10             // 默认值是一个静态字符串，使用 Borrowed 非常高效
   11             prompt: Cow::Borrowed(">>> "),
   12         }
   13     }
   14 }
   15 
   16 // 默认配置
   17 let default_config = Config::default();
   18 println!("默认提示符: {}", default_config.prompt);
   19 
   20 // 用户自定义配置
   21 let user_config = Config {
   22     // 用户输入一个 String，我们用 Owned 存储
   23     prompt: Cow::Owned(String::from("请输入命令: ")),
   24 };
   25 println!("用户自定义提示符: {}", user_config.prompt);
  在这个例子中，如果用户不自定义 prompt，程序就绝不会为它分配内存，极大地提高了效率。
*/