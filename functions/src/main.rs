fn main() {
    println!("Hello, world!");
    another_fun("Yeet Yeet", -5);
    println!("add results {:?}", add(5, 6));
    //println!("The type of () is {}", <()>);
}

fn another_fun(str: &str, num: i32) -> () {
    println!("I print the input here: {:?}, {:?}", str, num)
}

fn add(num1: i32, num2: i32) -> i32 {
    /* Block commment xd */
    num1 + num2
}
/*
slkdjf
as;ldfkjas;
sdfj;;lkskdjv */
