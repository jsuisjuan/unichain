mod utils;
use utils::generate_id;
mod model;

fn main() {
    println!("Welcome to your UniChain!");
    let _owner: (i64, String, String) = (
        generate_id(),
        String::from("Juan Carvalho Silva de Lima"),
        String::from("juanc.s.delima@gmail.com"),
    );
}
