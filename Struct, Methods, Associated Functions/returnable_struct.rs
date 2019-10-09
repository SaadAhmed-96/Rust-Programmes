#[derive(Debug)]

struct Book {
    name:String,
    author: String,
    price: u16,
    availability: bool,
}

fn main() {
    let book_1 = build (let name:String::from("Book A"),let author:String::from("Author A"));
    println!("{:#?}",book_1);

    fn build (name:String,author:String) -> Book {
        Book {
            name: name,
            author:author,
            price:500,
            availability: true,
        }
    }
}