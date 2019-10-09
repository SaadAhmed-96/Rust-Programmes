fn main() {
    let result = gives_ownership() // function call
    println!("{}",result);
    let s1 = String::from("Pakistan"); // s1 comes into Scope
    let result1 = takes_and_back(s1); //s1 is moved into takes and gives back
    println!("{}", result1);
}

fn gives_ownership() -> String {
    let s = String::from("Hello World");
    s
}

fn takes_and_back(x: String) -> {
    x // x is moved out to the calling function
}