use std::mem;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn value_types () { // this function returns an empty tuple, ie. -> ()
    // different integer types
    // all integers can be either unsigned (starts with a u) or signed (starts with an i)
    let unsigned_byte:u8 = 65;
    println!("A (8 bit) byte is indicated by u8/i8 and takes {} byte", mem::size_of_val(&unsigned_byte));
    let signed_two:i16 = 65;
    println!("A 16bit integer indicated by u16/i16 takes {} bytes", mem::size_of_val(&signed_two));
    let unsigned_four:u32 = 65;
    println!("A 32bit integer indicated by u32/i32 takes {} bytes", mem::size_of_val(&unsigned_four));
    let signed_eight:i64 = 65;
    println!("A 64bit integer indicated by u64/i64 takes {} bytes", mem::size_of_val(&signed_eight));
    let signed_sixteen:i128 = 65;
    println!("A 128bit integer indicated by u128/i128 takes {} bytes", mem::size_of_val(&signed_sixteen));
    let unsigned_word:usize = 65;
    println!("A word is indicated by usize/isize and takes {} bytes", mem::size_of_val(&unsigned_word));

    /*
    ** You can enter literals as
    ** Decimal	        dec = 98_222 // _ is beimg used as a separated, ie 98,222
    ** Hex              hex = 0xff
    ** Octal	        oct = 0o77
    ** Binary           bin = 0b1111_0000 // again _ is an optional separator
    ** Byte (u8 only)   byt = b'A'
    */

    // floating point number can either be single or double precision
    let single_precision:f32 = 65.0;
    println!("A single precision float (32 bit) is indicated by f32 and takes {} bytes", mem::size_of_val(&single_precision));
    let double_precision:f64 = 65.0;
    println!("A double precision float (64 bit) is indicated by f64 and takes {} bytes", mem::size_of_val(&double_precision));

    // boolean
    let logical:bool = true;
    println!("A boolean value is indicated by bool and takes {} byte", mem::size_of_val(&logical));

    // character
    let mychar:char = 'r';
    println!("A single character is indicated by char and takes {} bytes", mem::size_of_val(&mychar));

    // array (making this mutable)
    let mut myarr:[i32; 5] = [1,2,3,4,5]; // zero shortcut [0;5]
    println!("An array is indicated by [type; num elements] - {} elements takes {} bytes", myarr.len(), mem::size_of_val(&myarr));
    myarr[0] = 0; // arrays are 0 based
    println!("To print use the debug flag :? giving {:?}", myarr);
    let slice = &myarr[0..2]; // inclusive 0 (first) element, exclusive 2 (third) element; so first and second elements
    println!("Also use debug to print a slice {:?}", slice);

    // tuple
    let mytuple:(i32, f64, char, [i32; 5]) = (65, 65.0, 'r', myarr);
    println!("A tuple indicated by (type, ...) with any type include arrays and tuples");
    println!("{:#?}", mytuple);
    let (first, second, _, _) = mytuple; // destructing discaring 3rd and 4th
    println!("The first three values are {}, {} & {}", first, second, mytuple.2); // tuples are 0 based

    // string slice (see std_ref_types for String type)
    let strslice = "My string"; // type &str
    println!("A str slice (reference) denoted by &str - {}", strslice);
}

struct Size {
    width: u32,
    height: u32
}

fn fn_area (obj: &Size) -> u32 {
    obj.width * obj.height // return
}

fn std_ref_types() {
    // String
    let h = "Hello ".to_string();
    let w = String::from("world!");
    let mystr = h + &w; // concat string (note the reference on the second)
    println!("Concatenated string {}", mystr);

    // moving ownership - same thing happens if you move the ownership to a function
    let moved_string = mystr;
    println!("Ownership of String moved {}", moved_string);
    // cannot now use mystr

    // Vec (vector)
    let mut v = Vec::new();
    v.push(1);
    println!("First value of vector {}", v[0]); // vector like array are 0 based

    // Structure
    let mysize = Size { width: 20, height: 10};
    println!("Area of {} x {} is {}", mysize.width, mysize.height, fn_area(&mysize));
    // impl (object) will be covered later
}

fn guessing_game() {
    const MIN_GUESS_VALUE: u32 = 1;
    const MAX_GUESS_VALUE: u32 = 100;

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(MIN_GUESS_VALUE, MAX_GUESS_VALUE + 1);

    loop {
        println!("Please input your guess between {} and {}.", MIN_GUESS_VALUE, MAX_GUESS_VALUE);
        let mut guess_in = String::new();

        io::stdin()
            .read_line(&mut guess_in)
            .expect("Failed to read line");

        println!("You guessed: {}", guess_in);

        let guess: u32 = match guess_in.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("That was not a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {
    value_types();
    std_ref_types();
    guessing_game();
}