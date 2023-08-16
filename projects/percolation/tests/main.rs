use percolation::SquareSite;

#[test]
fn ready() {
    println!("it works!")
}



#[test]
fn test() {
    let mut ss = SquareSite::uniform(10, 2);
    ss.scan();
    println!("{:#?}", ss);
}