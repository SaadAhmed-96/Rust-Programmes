##[derive(Debug)]

struct Colour (i32,i32,i32);
struct Points (i32,i32,i32);

fn main () {
    let black = Colour(6,9,0);
    println!("{:#?}",black );
}

fn another_function (x: Colour) {
    println!("{:#?}",x );
}