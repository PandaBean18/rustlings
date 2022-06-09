// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    match maybe_number {
        Some(i) => {
            println!("printing: {}", i);
        }
        None => {
            println!("Not a number!");
        }
    }
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: Vec<Option<u16>> = vec![];
    for iter in 0..5 {
        let number_to_add: Option<u16> = {
            Some(((iter * 1235) + 2) / (4 * 16))
        };

        numbers.push(number_to_add)
    }
    // println!("{:?}", numbers);
}
