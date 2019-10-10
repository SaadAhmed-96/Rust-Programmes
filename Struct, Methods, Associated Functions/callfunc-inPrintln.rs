fn main() {
    let height = 100;
    let width = 50;

    println!("The Area of the square is {}",area(height,width));
}

fn area(height:u32,width:u32) -> u32 {
    height* width
}