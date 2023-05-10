// mod lesson_03 {
//     pub mod variables;
//     pub mod scope; 
//     pub mod constants;
// }

// mod lesson_04 {
//     // primitive data types
//     pub mod scalar;  
//     pub mod compound;  
// }

// mod lesson_05 {
//     pub mod input;
// }

// mod lesson_06 {
//     pub mod arithmetic;
//     pub mod cac;    //casting & conversion
// }

// mod lesson_07 {
//     pub mod conditions;
//     pub mod fizzbuzz;
// }

mod lesson_08 {
    pub mod math;
}

fn main() {
    // lesson_03::constants::constants();
    // lesson_04::compound::compound();
    // lesson_05::input::input();
    // lesson_06::cac::cac();
    // lesson_07::fizzbuzz::fizzbuzz();
    println!("{}", lesson_08::math::math(12.2, 19.1, "div"))

}
