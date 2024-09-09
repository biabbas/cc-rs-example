extern "C" {
    fn foo_function();
    fn bar_function(x: i32) -> i32;
}
pub fn call() {
    unsafe {
        foo_function();
        bar_function(42);
        foo_factorial(bar_function(5));
    }
}

fn main() {
    call();
}