// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

mod sausage_factory {
    // ALL functions and methods in rust are private by default, use `pub` keyword to make them public

    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
