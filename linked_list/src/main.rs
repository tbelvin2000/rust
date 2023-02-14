//mod linked_list;
struct Thing {
    opt: Option<Box<u32>>
}
fn main() {
    let thing = Thing{opt:Some(Box::new(5))};
    let pointer = &thing;
    match pointer.opt {
        None => println!("There is nothing"),
        Some(thing) => println!("There is a thing {}", thing)
    }
}
