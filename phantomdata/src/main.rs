use libc;
fn main() {
    //    let foo_auth = Authenticator::<Foo>::new();
    //   let res  = foo_auth.auth(1);
    //  println!("res {}", res);
    let mut my_child = None;
    {
        let root = StatRoot::create().unwrap();
        my_child = root.child();
    }

    if let Some(child) = my_child {
        println!("got my child");
        let c = child.child().unwrap();
        
    }
}

trait GetInstance {
    type Output;
    fn get_instance(id: i64) -> Option<Self::Output>;
}

struct Foo;

impl GetInstance for Foo {
    type Output = Self;

    fn get_instance(id: i64) -> Option<Self::Output> {
        if id == 1 {
            Some(Foo)
        } else {
            None
        }
    }
}

struct Authenticator<T: GetInstance> {
    _marker: std::marker::PhantomData<*const T>,
}

impl<T: GetInstance> Authenticator<T> {
    fn new() -> Authenticator<T> {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    fn auth(&self, id: i64) -> bool {
        T::get_instance(id).is_some()
    }
}

struct x_stat {
    id: i32,
}

fn x_stat_get(res: *mut *mut x_stat) {
    let a = Box::into_raw(Box::new(x_stat{id: 1}));
    unsafe {
        *res = a;
    }
}

fn x_stat_child(node: *mut x_stat) -> *mut x_stat {
    node
    //let mut child = Box::new(x_stat { id: 2 });
    //child.as_mut() as *mut x_stat
}

fn x_stat_free(node: *mut x_stat) {
    unsafe {
        if !node.is_null() {
            libc::free(node as *mut libc::c_void);
        }
    }
}

struct StatRoot {
    node: *mut x_stat,
}

impl StatRoot {
    pub fn create() -> Option<StatRoot> {
        let mut node: *mut x_stat = std::ptr::null_mut();
        x_stat_get(&mut node);
        if node.is_null() {
            unsafe {
                println!("root node is null {}", (*node).id);
            }
        }
        Some(StatRoot { node })
    }

    pub fn child(&self) -> Option<StatChild> {
        let node = x_stat_child(self.node);
        StatChild::new(node)
    }
}

impl Drop for StatRoot {
    fn drop(&mut self) {
        unsafe {
            println!("drop root");
            x_stat_free(self.node);
            println!("root dropped");
        }
    }
}

struct StatChild {
    node: *mut x_stat,
}

impl StatChild {
    pub fn new(node: *mut x_stat) -> Option<StatChild> {
        if node.is_null() {
            None
        } else {
            Some(StatChild { node })
        }
    }

    pub fn child(&self) -> Option<StatChild> {
        unsafe {println!("{}", (*self.node).id);}
        let node = x_stat_child(self.node);
        StatChild::new(node)
    }
}
