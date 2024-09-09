// // build.rs
// cc::Build::new()
//     .file("foo.c")
//     .file("bar.c")
//     .compile("foo");

fn main() {
    cc::Build::new()
        .cpp(true)  // This tells `cc` to compile C++ code
        .file("foo.cc")
        .include(".")
        .compile("my_cpp_code");
}