#[derive(Debug)]
struct Book {
    name:&str,
    author:&str,
    price: u16,
    availability:bool,
}

fn main() {

    let book1 = Book {
        name: "Book A",
        author:"Author A",
        price:500,
        availability:true,
    };
}