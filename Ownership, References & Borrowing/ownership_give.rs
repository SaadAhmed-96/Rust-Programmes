fn main() {
    let s =String::from("Hello"); //s comes into scope
    take_ownership(s);// s is moved into take_ownership
println!("{}",s )//s is dropped now

let num = 5; // num comes into the scope
makes_copy(5) // num is moved in to the function makes_copy
println!("{}", num );
}



fn take_ownership(x: String) { // x comes into the scope
    println!("{}",x);
}// x is out of the scope and will be dropped

fn makes_copy(x:i32) {
    println!("{}",x);
}