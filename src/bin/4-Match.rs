// fn main(){
//     let some_bool=true;
//     let my_name="sarthak";

//     match some_bool {
//         true=> println!("Its True"),
//         false=> println!("It's false"),
//     }

//     match my_name{
//         "Aditya" => println!("Not my name he is my friend"),
//         "yash" => println!("Hello Yash"),
//         "sarthak"=>println!("My name"),
//         _=>println!("Other cases")
//     }


// }


// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

// fn main() {
//     let my_bool = false;

//     match my_bool {
//         true => println!("it's true"),
//         false => println!("it's false"),
//     }
// }


// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let my_number = 2;

    match my_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}
