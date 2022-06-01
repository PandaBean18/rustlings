// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)


fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // below is a block that has a scope of its own and hence can have another variable named number
    {
        let number = 3;
        println!("Number plus two is : {}", number + 2);
    }
    
    // the varible `number` is still a string in this scope 
    number = "F-I-V-E"; 
    println!("This is spelt as: {}", number);
}
