use std::sync::Mutex;

#[derive(Debug)]
struct Employee {
    id: i32,
}

#[derive(Debug)]
struct Detail {
    emp_id: i32,
    name: String,
}

impl Employee {
    fn get_details(&self) -> Detail {
        Detail {
            emp_id: self.id,
            name: String::from("sean"),
        }
    }
}

impl Detail {
    fn get_name(&self) -> &String {
        &self.name
    }
}


struct Hello;

impl Hello {
    fn get_value(&self) -> Result<i32, String>  {
        Ok(5)
    }
}


fn mutex_1() {
    let m = Mutex::new(5);
    let guard = m.lock().unwrap();
    println!("the value of guard {}", guard);

    println!("try to aquire lock");
    let w = m.lock().unwrap();
    println!("lock acquired");
}

fn mutex_2() {
    let m = Mutex::new(Hello);
    match m.lock().unwrap().get_value() {
        Ok(v) => {
            //println!("try to aquire lock {}", v);
            //m.lock().unwrap();
            //println!("lock acquired");
        },
        Err(e) => {}
    };
    println!("try to aquire lock out match arm");
    m.lock().unwrap();
    println!("lock acquired out match arm");
}

fn mutex_3() {
    let m = Mutex::new(Hello);
    let h = m.lock().unwrap().get_value();
    println!("try to aquire lock");
    m.lock().unwrap();
    println!("lock acquired");
}


fn main() {
    let emp = Employee{id: 123};
    let n = emp.get_details().get_name().len();
    println!("{}", n);
    mutex_2();
}
