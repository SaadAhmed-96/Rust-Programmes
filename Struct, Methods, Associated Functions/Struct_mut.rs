#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u16;
    availability: bool,
}//template


fn main() {
    let mut book_1 = Book {
        name:String::from("Book A"),
        author:String::from("Author A"),
        price: 500,
        availability: true,
    } ;
book_1.name = String::from("Book AA");

    println!("{:#?}",book_1);
}