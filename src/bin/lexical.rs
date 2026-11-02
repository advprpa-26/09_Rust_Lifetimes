fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    print!("{}", r1);

    s.push_str(" advprpa!");
}

