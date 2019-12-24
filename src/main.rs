use std::mem;

fn main() {
    let mynumber:u8 = 65;
    println!("mynumber is {} which takes {} bytes", mynumber,  mem::size_of_val(&mynumber));
}
