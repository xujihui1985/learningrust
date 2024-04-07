use metatdata::MetaData;

#[derive(MetaData)]
#[metadata(author = "hello", serial_version = 4)]
struct Foo {
    #[metadata(author = "sean")]
    a: u32,
}

fn main() {
    let foo = Foo{a: 1};
    println!("author: {}", foo.author());
    println!("version: {}", foo.serial_version());
    println!("fields: {:?}", foo.field_authors());
}
