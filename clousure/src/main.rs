mod capture;

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

    let res = f2()(12);
    println!("res is {}", res);

    fn calc(x: i32, y: i32, func: Box<dyn Fn(i32, i32) -> i32 + '_>) -> i32 {
        let res = func(x, y);
        res
    }

    let c = 1;
    let a = calc(1,2, Box::new(|x,y| x+y+c));
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

fn f2() -> fn(i32) -> i32 {
    let f = |x| {x + 1};
    f
}

struct Hello {

}

impl Hello {
    fn display(&self) -> usize {
        let mut aaa: usize = 1;
        self.walk(|a: u32| { aaa += a as usize;});
        aaa
    }

    fn walk<T>(&self, mut f: T) where T:FnMut(u32) {
        let mut a:Vec<u32> = vec![1,2,3];
        for i in (0..10){
            let b = a.get_mut(i).unwrap();
            f(*b)
        }
    }
}

fn add(a: &mut Vec<u32>) {

}

struct Rec {
    num: i32,
}

fn modify(data: &mut Vec<Rec>) {
    let mut ret = Vec::<&mut Rec>::new();
    for j in 0 .. 10 { 
        let mut rec = data.get_mut(j).unwrap(); //<- error is here
        ret.push(rec);
    }
    //ret
}
