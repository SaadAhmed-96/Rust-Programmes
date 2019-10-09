fn main () {
    let a: u8 = 10; // a has a value of 10
    let b = &a // b point to a 
    let c = &b // c points to b
    println!("a:{}, b:{}, c:{}",a,b,c);
    println!("The address of a is {:p}",b);
    println!("The address of a is {:p}",c);

    //  *************************************************

    let s = String::from('Pakistan);
    let result = length(&s);
    println!("the length of the word {} is {}",s,result);
}

fn length(x: &String) -> usize {
    x.len()
}