use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("This is the main.rs of the cargo there won't be really any code here except to show how to import stuff");
    println!("\n\nThis main.rs is considered the root crate\n\n\n\n");

    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);

}
