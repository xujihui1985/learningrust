
fn main() {

}

fn first_elements(s: &[u8]) -> Vec<u8> {
    let mut iter = s.iter();
    let first: Vec<u8>= iter.by_ref().take(3).copied().collect();
    println!("first: {:?}", first);
    let next = iter.take(3).copied().collect();
    next
}

#[derive(Clone, Debug)]
struct Thing{
    id: u32,
    name: String
}

fn first_elements_clone(s: &[Thing]) -> Vec<Thing> {
    let mut iter = s.iter();
    let first: Vec<_> = iter.by_ref()
        .take(3)
        .cloned()
        .collect();

    println!("first: {:?}", first);
    let next = iter.take(3).cloned().collect();
    next
}

fn first_elements2(s: &[u8]) -> Vec<u8> {
    let mut iter = s.iter();
    let first: Vec<u8>= iter.by_ref().take(3).copied().collect();
    println!("first: {:?}", first);
    let next = iter.take(3).copied().collect();
    next
}