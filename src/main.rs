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

mod lesson_06 {
    pub mod arithmetic;
    pub mod cac;    //casting & conversion
}

fn main() {
    // lesson_03::constants::constants();
    // lesson_04::compound::compound();
    // lesson_05::input::input();
    lesson_06::cac::cac()
}
