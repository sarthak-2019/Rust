// struct  Temperature{
//     degress_f:f64,
// }

// impl  Temperature {
//     fn show_temp(&self){
//         println!("{}",self.degress_f)
//     }
//     fn freezing()->Self{
//         return  Self{
//             degress_f:32.0
//         };
//     }
//     fn boiling()->Self{
//         return  Self{
//             degress_f:212.0
//         };
//     }
// }
// fn main(){
//     let hot=Temperature{degress_f:99.9};
//     hot.show_temp();

//     let cold=Temperature::freezing();
//     cold.show_temp();

//     let boil=Temperature::boiling();
//     boil.show_temp();
// }





// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}
struct ShippingBox{
    dimensions:Dimensions,
    weight:f64,
    color:Color,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }

}
fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print();
}
