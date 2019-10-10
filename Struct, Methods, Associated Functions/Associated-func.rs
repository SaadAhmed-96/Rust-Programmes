struct Rectangle {
    height: u32,
    width : u32
}

impl Rectangle {
    // add code here
    fn area (&self) -> u32 { //now compiler is here
        self.width*self.height
    }
}

fn main() {
    let rec1 = Rectangle{height: 100, width:50}; //height, width
    let result = rec_1.area(); //call
    println!("The area of Rectangle is: {}", result);
}