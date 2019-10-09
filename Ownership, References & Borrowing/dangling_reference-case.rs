fn main() {
    let result = dangle();
    print!("{}", result);
} //result goes out of the scope and is dropped




// **************** Dangling Reference Error ********************

fn dangle() -> &String {
    let s = String::from("Hello"); // s comes in to scope
    &s // address is moving out of the function to the calling function


    // ************************************************************

    // **********************Correction************************

    fn dangle() -> String {
        let s = String::from("hello");// s comes in to scope
        s // address is moving out of the function to the calling function
    }
} // s is deployed