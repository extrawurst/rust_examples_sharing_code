mod shared;

pub fn test1_hello() {
    shared::foo();
    println!("test1");
}

fn main() {
    test1_hello();
}
