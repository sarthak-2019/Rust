

// fn main(){
//     let numbers=(1,2,3);
//     println!("{}",numbers.0);
//     println!("{}",numbers.1);
//     println!("{}",numbers.2);

//     let (name,age)=("Emma",20)

//     let favs=("red",1,"New York","Sky");


// }

// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coordinate() -> (i32, i32) {
    return (1, 7)
}

fn main() {
    let (x, y) = coordinate();

    if y > 5 {
        println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
