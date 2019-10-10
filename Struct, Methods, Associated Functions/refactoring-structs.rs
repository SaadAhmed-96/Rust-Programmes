##[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
fn main () {
    let rec_1 = Rectangle {
        height: 100,
        width: 50,
    };
    println!("The Area of rectangle is {}",area(rec_1));
}

fn area(rec: Rectangle) ->u32 {
    rec.height* rec.width
}