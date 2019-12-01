fn main() {
    cc::Build::new()
        .file("doubler.c")
        .compile("doubler");
}