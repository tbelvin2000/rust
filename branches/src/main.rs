fn main() {
    let n = 5;
    println!("Fact of {n} is {:?}", fact(n));
}

fn fact(n: u32) -> u32 {
    fn fact_tr(n: u32, f: u32) -> u32 {
        if n == 0 {f} else {fact_tr(n-1, n*f)}
    }
    fact_tr(n, 1)
}