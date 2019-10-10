fn main () {
    let rec1 = (100,50);
    println!("The Area of sqaure is : {}", area(rec1) );

}

fn area (dimension: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}