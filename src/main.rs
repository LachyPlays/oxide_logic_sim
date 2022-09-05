enum logic_types {
    or(),
    nor(),
    and(),
    nand(),
    xor(),
    xnor(),
    buffer(),
    not()
}

#[derive(Debug)]
struct logic_node {
    l_type: logic_types,
    
}

fn main() {
    println!("Hello, world!");
}
