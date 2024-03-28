fn main() {
    let n = 5;
    let f = fib(n);
    println!("The {n}th fibonacci number is: {f}");
}

fn fib(n: u32) -> i32 {
    if n == 0 {
	1
    } else if n == 1 {
	1
    } else {
	fib(n-1) + fib(n-2)
    }
}
