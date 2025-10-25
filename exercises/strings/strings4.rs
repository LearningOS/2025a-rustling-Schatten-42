// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
  string_slice("blue");
  string("red".to_string());
  string(String::from("hi"));
  string("rust is fun!".to_owned());
  string_slice("nice weather".into());
  string(format!("Interpolation {}", "Station")); // format!()返回 String
  string_slice(&String::from("abc")[0..1]);
  string_slice("  hello there ".trim());
  string("Happy Monday!".to_string().replace("Mon", "Tues"));
  string("mY sHiFt KeY iS sTiCkY".to_lowercase()); 
  // to_xxcase() 返回 String:
//   - 大小写转换 可能改变长度（如 ß → ss），
//   - 因此必须 分配新缓冲区 → 返回 String
}
