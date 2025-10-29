use module_1;
use module_2;
use module_3;

fn main() {
    println!("Hello, world!");

    module_1::module_name();
    module_2::module_name();
    module_3::module_name();

    println!("{}", module_2::get_module_name());
}
