type Point = (f32, f32);

#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat
}

enum Action {
    Drive,
    Turn(Direction),
    Stop
}
enum Direction {
    Left,
    Right,
}


enum RelationShip {
    Father,
    Mother,
    Daughter,
    Son,
    VarA(Fuz),
    Other(u32)
}

#[derive(Debug)]
struct Foo {
    baz: String,
    quzx: i32,
    z: Fuz,
}

#[derive(Debug)]
struct Fuz {
    zed: i32,
}

fn print_action(a: Action) {
    match a {
        Action::Drive => println!("Drive"),
        Action::Turn(d) => match d {
            Direction::Left => println!("Turn left"),
            Direction::Right => println!("Turn Right"),
        },
        Action::Stop => println!("Stop"),
    }
}


fn main() {
    let a = Foo {
        quzx: 10,
        baz: String::from("hello"),
        z: Fuz {
            zed: 123,
        }
    };
    println!("{:?}", a);
    
    let p: Point  = (1.0, 2.0);
    println!("{}", p.0);

    let pet = Animal::Dog;
    let pet2 = Animal::Cat;

//    assert!(pet == pet2);

    let program = vec![Action::Drive, Action::Turn(Direction::Left), Action::Stop];

    for action in program {
        print_action(action);
    }
    

}
