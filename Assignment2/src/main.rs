fn main () {



    // Q#1:
    // ***********************************
    
// println!("QUESTION NO.01");
// let num:i32 = 8;
// check_num(num);

// ***********************************



    // Q#2:
    // ***********************************
    
// println!("QUESTION NO.01");
// let int:i32 = 8;
// let float:f32 = 6.5;
// let boolean:bool = true;
// let string:&str = "Fighting !!!!!!"
// data_types(int,float,boolean,string);

// ***********************************


// Q#3:
    // ***********************************

// println!("QUESTION NO.3");
// let number=2;
// println!("The input number is = {}", number);
// let (value1,value2)=square(number);
// println!("The number you gave is {} and its square is {} ", value1,value2);
// ***********************************


// Q#4:
    // ***********************************
// println!("QUESTION NO.4");
// let sub1=80.0;
// let sub2=85.0;
// let value=result(sub1,sub2);
// println!("************REPORT*************");
// println!("You have successfully passed with a percentage={}%",value);
// ***********************************
}






// Q#1:
// Write a rust program and define a user defined function that receives one argument of
// any suitable data type and print whether the number is positive, negative or equal to zero.
// (hint: if/else)

// fn check_num(num:i32) {
//     if num > 0 {
//         println!("The Number is Positive");
//     }
//     else if num < 0 {
//         println!("The Nnmber is Negative");
//     }
//     else {
//         println!("The number is equal to zero");
//     }

//  *******************************************


// Q#2:
// Write a rust program and define a user defined function that receives 4 arguments of
// different data types (integer, float, boolean, string) and print them on the console.


// fn data_types(int:i32,float:f32,boolean:bool,string:&str) {

//     println!("This {} is of integer data type", int );
//     println!("This {} is of float data type", float );
//     println!("This {} is of boolean data type", boolean );
//     println!("This {} is of string data type", string );
// }
//  *******************************************

// Q#3:
// Write a rust program and define a user defined function that receives a number and
// return the number itself and its square by using tuple.


// fn square(number:u32) -> (u32,u32)
// {
//     let number_square=number*number;
//     (number,number_square)
   
// }
//  *******************************************


// Q#4:
// Write a program to know the result of the test (Pass/Fail) by using a user defined
// function, and perform the following operations: (Note: Consider marks of only 2 subjects for
// the sake of simplicity. Maximum marks are 100 for each subject.)

// a. Add marks and print the total.
// b. Calculate the percentage and print it, return percentage to main function
// c. If percentage >= 70, Print Pass, Else, print Fail.

// fn result(sub1:f32,sub2:f32)->f32
// {
//     println!("Marks in subject 1= {}",sub1);
//     println!("Marks in subject 2= {}",sub2);
//     let sum=sub1+sub2;
//     println!("The sum of marks in both subjects= {}",sum);
//     let aggregate=sum/200.0;
//     println!("The aggregate= {}",aggregate);
//     let percentage=aggregate*100.0;
//     println!("The percentage is={}%",percentage);
//     if percentage>=70.0
//     {
//         println!("You are Pass!");
//         println!("*******CONGRATULATIONS*********");
//     }
    
//     else
//     {
//         println!("You are Fail");
//         println!("*******PLEASE TRY AGAIN*********");
//     }
    
//     percentage
    
// }

// ***********************************