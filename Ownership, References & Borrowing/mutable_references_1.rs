fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
}

fn change (x: &mut String) {
    x.push_str("World");
    println!("{}",x )
}