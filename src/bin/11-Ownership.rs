// Rust use Ownership model to manage memory


// enum Light{
//     Bright,
//     Dull
// }

// fn display_light(light:&Light){
//     match  light {
//         Light::Bright=>println!("Bright"),
//         Light::Dull=>println!("Dull"),
//     }
// }
// fn main(){
//     let dull= Light::Dull;
//     display_light(&dull);
//     display_light(&dull);
// }



// struct Book{
//     pages:i32,
//     rating:i32,
// }

// fn display_page(book:&Book){
//     println!("{}",book.pages)
// }
// fn display_rating(book:&Book){
//     println!("{}",book.rating)
// }
// fn main(){
//     let book:Book=Book { pages: 12, rating: 3 };
//     display_page(&book);
//     display_rating(&book);
// }


// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item:&GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

fn display_id(item:&GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let my_item:GroceryItem = GroceryItem {
        quantity: 3,
        id: 99,
    };
    display_quantity(&my_item);
    display_id(&my_item);
}
