//mod multiple_impl;
//mod borrow;
mod wrapper;

fn main() {
//    let a = multiple_impl::Result<T, String>::Err(String::from("aaaaa"));
 //   a.unwrap();
    let w = wrapper::LaterBoundWrapper::new(String::from("aaa"));
    w.inspect();
    wrapper::take_later_bound_wrapper(w);
}
