extern "C" {
    fn foo_factorial(x: i32);
}
pub fn call() {
    unsafe {
        foo_factorial(5);
    }
}

fn main() {
    call();
}