fn main () {
    let s = String::from("Pakistan");
    let (result,result1) = length(s);
    println!("The length of the word {} is {}",result, result1 );
}

fn length(name: String) -> (String,usize) {
    (name,name.len())
}