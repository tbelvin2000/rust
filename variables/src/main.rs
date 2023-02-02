fn main() {
    let tup: (u32, i32, f64);
    tup = (16, 24, 18.5);
    let (x, _, z) = tup;
    println!("The tuple is: {:?}", tup);
    println!("x is: {:?}, y is: {:?}, z is: {:?}", x, tup.1, z);
}
