#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u16;
    availability: bool,
}//template


fn main() {
    let book_1 = Book {
        name:String::from("Book A"),
        author:String::from("Author A"),
        price: 500,
        availability: true,
    } ;

    let book_2 = Book {
        name:String::from("Book B"),
        author:String::from("Author B"),
        price: 600,
        availability: true,
    } ;

    let book_3 = Book {
        name:String::from("Book C"),
        author:String::from("Author C"),
        ..book_1
    } ;

    println!("{:#?}",book_1);
    println!("{:#?}",book_2);
    println!("{:#?}",book_3);
}