struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point{y:-32, x:0};
    println!("Point is {}, {}", p.x, p.y);
    let x = -32;
    let y = 0;
    let p = Point{y, x};
    println!("Point is {}, {}", p.x, p.y);
}
