extern crate cc;

fn main() {
    cc::Build::new()
        .file("cityhash_1/city.cc")
        .include("cityhash_1")
        .cpp(true)
        .compile("cityhash_1");

    cc::Build::new()
        .file("cityhash_1_1_1/city.cc")
        .include("cityhash_1_1_1")
        .cpp(true)
        .compile("cityhash_1_1_1");
}
