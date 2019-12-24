use std::mem;

fn main() {
    // different integer types
    let unsigned_byte:u8 = 65;
    println!("A byte is indicated by u8/i8 and takes {} byte", mem::size_of_val(&unsigned_byte));
    let signed_two:i16 = 65;
    println!("A 16bit integer indicated by u16/i16 takes {} bytes", mem::size_of_val(&signed_two));
    let unsigned_four:u32 = 65;
    println!("A 32bit integer indicated by u32/i32 takes {} bytes", mem::size_of_val(&unsigned_four));
    let signed_eight:i64 = 65;
    println!("A 64bit integer indicated by u64/i64 takes {} bytes", mem::size_of_val(&signed_eight));
    let unsigned_word:usize = 65;
    println!("A word is indicated by usize/isize and takes {} bytes", mem::size_of_val(&unsigned_word));
}
