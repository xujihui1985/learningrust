enum Message {
    Hello {id: i32}
}

fn main() {
    let msg = Message::Hello {id: 3};

    match msg {
        Message::Hello {id: id_variable@3..=7} => { // 
            println!("found id in range {}", id_variable) //
        },
        Message::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        _ => {
            println!("default")
        },
    }

}