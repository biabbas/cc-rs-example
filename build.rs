// // build.rs
// cc::Build::new()
//     .file("foo.c")
//     .file("bar.c")
//     .compile("foo");

// build.rs
fn main() {
// build.rs
cc::Build::new()
    .file("foo.c")
    .file("bar.c")
    .compile("foo");
cc::Build::new()
    .file("foo.cc")
    .cpp(true)
    .compile("foocpp");
}