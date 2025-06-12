// Two commonly used strings
// String - owned
// &str- borrowed String slice

// When passing to function use &str
// Use an owned string to store in struct

// fn print_it(data: &str){
//     println!("{:?}",data);
// }
// fn main(){
//     print_it("Hello World");
//     let owned_string="owned string".to_owned();
//     let another_ownded= String::from("another");
//     print_it(&owned_string);
//     print_it(&another_ownded);
// }



// struct Employee{

//     // Cannot do this because it is borrowed
//     // name:&str

//     // Do this instead
//     name:String,
// }

// fn main(){
//     let emp_name = "Hello".to_owned();
//     let emp=Employee{
//         name:emp_name
//     };
//     println!("{:?}",emp.name)
// }



// struct  LineItem{
//     name:String,
//     count:i32,
// }
// fn print_name(data: &str){
//     println!("name is {:?}",data)
// }
// fn main(){
//     let receipt=vec![
//         LineItem{
//             name:"cereal".to_owned(),
//             count:1
//         },
//         LineItem{
//             name:String::from("fruit"),
//             count:2
//         }
//     ];
//     for item in receipt{
//         print_name(&item.name);
//         println!("count is {:?}",item.count)
//     }
// }



// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::from("green"),
            age: 7,
        },
        Person {
            name: String::from("Anna"),
            fav_color: String::from("purple"),
            age: 9,
        },
        Person {
            name: String::from("Katie"),
            fav_color: String::from("blue"),
            age: 14,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}
