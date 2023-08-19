use percolation::{Cell, SquareSite};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut ss = SquareSite::uniform(10, 2);
    ss.scan(10);
    println!("{:#?}", ss);
}
#[test]
fn test2() {
    let mut ss = Cell::new(1);
    ss.set_id(6);
    ss.replace_id(6, 5);
    println!("{:#?}", ss);
}
