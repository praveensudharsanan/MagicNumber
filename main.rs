use rand::Rng;
use magic_num::magic_number;

fn main() {
    let num = magic_num::magic_number();
    if num == 0 {
        let random_number: u8 = rand::thread_rng().gen_range(0,250);
        println!("Your random number is  {}", random_number);


    }else{
        let random_number: u8 = rand::thread_rng().gen_range(0,231);
        println!("Your random number is  {}", random_number);


    }


}
