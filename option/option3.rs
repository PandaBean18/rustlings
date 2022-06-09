// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    if let Some(ref p) = y {
        println!("Co-ordinates are {},{} ", p.x, p.y);
    } else {
        println!("No matches");
    }
    y; // Fix without deleting this line.
}
