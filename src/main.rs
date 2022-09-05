use std::cell::{Ref, RefCell};

#[derive(Debug)]
enum LogicTypes {
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
struct LogicNode<'a> {
    l_type: LogicTypes,
    inputs: Vec<RefCell<LogicNode<'a>>>,
    outputs: Vec<RefCell<LogicNode<'a>>>,
}

impl LogicNode<'_>{
    fn new() -> LogicNode<'static>{

    }
}

fn main() {
    println!("Hello, world!");

    let node0 = 
}
