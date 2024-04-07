use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
struct GroundStation{
    freq: f64,
}

pub fn borrow_mut() {
    let base = Rc::new(RefCell::new(GroundStation{
        freq: 123.12,
    }));

    println!("base: {:?}", base);

    {

        let mut mutb = base.borrow_mut();
        mutb.freq -= 10.0;
        println!("mutb {:?}", mutb);
    }

    println!("{:?}", base);

}
