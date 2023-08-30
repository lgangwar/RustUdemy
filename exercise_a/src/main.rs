//exercise # 1

const STARTING_MISSILES : i32 = 8;
const READY_AMOUNT : i32 = 2;

fn do_stuff(qty : f64, oz : f64) -> f64{
    //return qty * oz;
    qty * oz //tail expression
}

fn main() {
    let mut missiles : i32 = STARTING_MISSILES;
    let ready : i32 = READY_AMOUNT;

    println!("Firing {} of my {} missiles....", ready, missiles);
    
    missiles = missiles - ready; 
    println!("{} missiles left", missiles);

    //warning and error cases
    //println!("{} missiles left", missiles - ready);
    //let some : i32 ;
    //READY_AMOUNT = 23;

    println!("{}", do_stuff(34.0, 56.0));
    

}
