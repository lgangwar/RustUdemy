
use hello::greet;
use rand::Rng;

//standard data structure
//use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut bunnies = 2;

    println!("how many bunnies are -> {}", bunnies);
    bunnies = 34;
    println!("how many bunnies are -> {}", bunnies);

    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    //println!("{}, {}", x, y); error


    let enigma : i32 ;
    //before this enigma use give error
    enigma = 42;
    println!("enigma: {}", enigma);
    
    
    //
    greet();

    let x = rand::thread_rng().gen_range(0, 100);
    println!("randomeness coe: {}", x);

    let info = (1, 3.14, 999);
    println!("tuple is: {:#?}", info);

}
