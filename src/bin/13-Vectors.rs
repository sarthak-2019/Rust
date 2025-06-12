// fn main(){
//     let val=vec![1,2,3];
//     let mut my_numbers=Vec::new();
//     my_numbers.push(1);
//     my_numbers.push(2);
//     my_numbers.push(3);
//     my_numbers.pop();
//     let size=my_numbers.len();
//     println!("{}",size);


//     for num in my_numbers{
//         println!("{:?}",num);
//     }
// }

// struct Test{
//     score:i32,
// }

// fn main(){
//     let my_scores = vec![
//         Test{score:1},
//         Test{score:2},
//         Test{score:3},
//         Test{score:4},
//     ];
//     for test in my_scores{
//         println!("{:?}",test.score)
//     }
// }


// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let my_numbers = vec![10, 20, 30, 40];
    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }

    println!("number of elements = {:?}", my_numbers.len());
}


