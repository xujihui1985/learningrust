fn main() {
    let x = vec![1,2,3];
    let equal_to_x = move |z: Vec<u32>| z == x;
    let y = vec![1,2,3];
    equal_to_x(y);

    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
// Calling the map method to create a new iterator and then calling the collect method to consume the new iterator and create a vector
    println!("{:?}", v2);
    let shoes = vec![
        Shoe {size: 10, style: String::from("aaa")},
        Shoe {size: 13, style: String::from("bbb")},
    ];
    let res = shoes_in_my_size(shoes, 10);
    println!("res {:?}", res);

}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}