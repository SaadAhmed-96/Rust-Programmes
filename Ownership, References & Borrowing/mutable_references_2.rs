fn main() {
    let mut s = String::from("Hello");

    {
        let a =&mut s;
        println!("{}",a);
    }

    {
        let b = &mut s;
        println!("{}",b);
    }
}