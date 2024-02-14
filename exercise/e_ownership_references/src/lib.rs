pub fn inspect(arg: &String) {
  println!(
    "{} is a {} word",
    arg,
    if arg.ends_with("s") {
      "plural"
    } else {
      "singular"
    }
  );
}

pub fn change(arg: &mut String) {
  if !arg.ends_with("s") {
    arg.push_str("s");
  }
}

pub fn eat(arg: String) -> bool {
  arg.starts_with("b") && arg.contains("a")
}

pub fn bedazzle(s: &mut String) {
  *s = String::from("sparkly");
}
