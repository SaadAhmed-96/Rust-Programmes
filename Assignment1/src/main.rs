fn main() {

// Q#1:
// Write a Rust program, in which you need to declare/initialize variables for
// Product_Name, Product_manufacturer and Price. Print these information on your
// screen/terminal.

    println!("QUESTION NO.1");
    let product_Name = "Aurous motherboard";
    let product_manufacturer = "Gigabyte";
    let price = 26000;

    println!("Produce name: {}",Product_Name);
    println!("Product manufacturer: {}",Product_manufacturer);
    println!("Price: Rs.{}",Price);


// Q#2:
// Take any integer value in a variable then add ‘5’ to that then multiply it with ‘4’, and
// print on your screen/terminal. You are not allowed to change the variable name throughout the
// program i.e you have to do shadowing.

    println!("QUESTION NO.2");
    let mut variable1 = 128;
    variable1 = variable1 + 5;
    variable1 = variable1 * 4;
    println!("The varaible value after addition, multiplication through shadowing is {}", variable1);


// Q#3:
// Write a Rust program to perform mathematical operations between two numbers.
// Declare two float variables and assign some hard-coded values to them. Then print the result of
// addition, subtraction, division, and multiplication in between these two variables.

    println!("QUESTION NO.3");

    let float1:f64 = 1.28; 
    let float2:f64 = 1.01;
    let addition = float1+float2;
    let subtraction = float1-float2;
    let division = float1/float2;
    let multiplication = float1*float2;

    println!("The Result of Addition is: {}", addition);
    println!("The Result of Subtraction is: {}", subtraction);
    println!("The Result of Division is: {}", division );
    println!("The Result of Multiplication is: {}", multiplication);


// Q#4:
// Write a Rust program and declare a tuple for data “Fruit_Name, Weight and Price”.
// Destructure the tuple in different variables and print them on your screen/terminal.

    println!("QUESTION NO.4");
    let tuple = ("Apple",5.05,100);
    let (p,q,r) = tuple;
    println!("Fruit_Name: {}", p);
    println!("Weight: {}",q);
    println!("Price: {}",r);


// Q#5: 
// Write a Rust program by initializing an array of Cars, and another array with their prices.
// Print the data as below:
// Output be like:
// Car Name: Passo - Price: 800000
// Car Name: Vitz - Price: 900000
// Car Name: Swift - Price: 950000
// (Note: For the sake of simplicity, limit only 5 cars.)

    println!("QUESTION NO.5");
    let cars = ["Passo", "Vitz" , "Swift"];
    let prices = [1500, 2500 , 3000 ];
    println!("Car Name: {} - Price:{}",cars[0], prices[0]);
    println!("Car Name: {} - Price:{}",cars[1], prices[1]);
    println!("Car Name: {} - Price:{}",cars[2], prices[2]);

}