// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let mut a: Vec<i32> = Vec::new();
    let mut i = 0; 

    while i < 100 {
        a.push(i);
        i += 1;
    }

    let arr = ['a'; 10]; // Will create ['a', 'a', 'a'..]

    println!("{:?}", arr);


    println!("{:?}", a);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
