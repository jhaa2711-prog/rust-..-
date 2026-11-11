use std::{i128, string};

fn main() {
    let x: i32 = -42;//signed integer can be negative or positive
    let y: u64 = 100;//unsigned only positive nums
    //also i32 and i64 are not just 2x but are 2 exponential to the power of 32 and 64 respectively
    //e.g i32 = 2^32 and i64 = 2^64 
    //most devices (except for debian based servers from 1999 which are probably older than us) use 64bit archetecture than 32bit
    //but for normal programs i32 is enough (saves approximately 6^-90917981728701545466132541 resources of universe)
    println!("signed integer: {}", x);
    println!("usigned : {}", y);
    //i32 (32 bits) , i64(64 bits)
    //range: i32 - 2147483647
    // i64 - 9223372036854775807
    let e: i32 = 2147483647;
    let yy: i64 = 9223372036854775807;
    println!("max val of i32 is :{}", e);
    println!("max val of i64 is: {}", yy);

    //=======================
    // floats[decimal]
    // f32,f64
    let pi: f64 = 3.14;
    println!("pi is {pi}");
    //now reason there is a bracket is cuz bracket means blank and allows u to get the variable e.g pi
    let snowing: bool = true;
    println!("is it snowing? : {snowing}");//println! is print line and the ! means it is a macro not a function , also its different from print! cuz print! does not add a new line after printing

    //char type
    let letter: char = 'a';
    println!("first alphabet: {letter}");
    compunds();}


fn compunds() {
    //compound data types
    // arrays,tuples,slices ,and stringsszzzsszzzssszzssssssssss[snake sound]
    //arrays ⇓⇓⇓⇓⇓
    let num: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];//if ya are cming from python (or any other hll), you might think why is there 7 instead of 6 cuz arrays start from 0 and end at n-1 but in rust the number is the length of the array not the last index.
    println!("numbers till 7 {num:?}");
    println!("{:?}", num);
    let frui: [&str; 4] = ["apple", "banana", "hi", "kkkk"];
    println!("fruits are {}", frui[1]);//if you just want whole you can do it without these square bracks 
    println!("fruits are {:?}", &frui[0..3]);//this is a slice of the array frui from index 0 to 2 (3-1)
    println!("fruits = {:?}", frui);//this is the whole array frui)
    //tuples
    let hum: (String, i64, bool) = ("alice".to_string(), 30, true);
    println!("hum tuple:   {:?}", hum);
    let mixxx: (&str, i64, bool, [i32; 5]) = ("steve", 222, true, [1, 3, 4, 5, 6]);
    println!("mix is {mixxx:?}");
    //sliceesss
    let numm: &[i32] = &[1, 2, 3, 4, 5];
    println!("{numm:?}");
}
