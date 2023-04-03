#[derive(Debug)]
struct MySlice<'a, T> {
   // mut_slice: &'a mut [T],
   slice: &'a mut [T],
}

impl<'a, T> MySlice<'a, T> {

    // fn get_mut<'b:'a>(&'b mut self) -> Option<&'a mut T> {
    //     let res:&'a mut T = self.slice.get_mut(0)?;
    //     Some(res)
    // }

    fn get<'b>(self: &'b MySlice<'a, T>) -> &'a T where 'b:'static{
        // let slice:&'a [T] = self.slice;
        let res = self.slice.get(0).unwrap();
        res
        // let res:&'a T = self.slice.get(0).unwrap();
        // res
        // todo!()
    }

    // fn set<'b>(self: &'b mut MySlice<'a, T>, c: &'a mut [T]) {
    //     self.slice = c;
    // }
}

fn test_lifetime<'a:'b, 'b>(x: &'b str, y: &'a str) -> &'a str {
    y
}

fn smallest_number<'a>(n: &'a [i32]) -> &'a i32 {
    todo!()
}

#[derive(Debug)]
struct Foo<'a> {
    x_mut: &'a mut u32,
    owned: u32,
}

impl <'a> Foo<'a> {
    fn get_x_mut<'foo>(self: &'foo mut Foo<'a>) -> &'a mut u32 where 'foo:'a{
        self.x_mut = &mut self.owned;
        // self.x_mut = v;
        self.x_mut
    }

    fn set_x_mut<'foo>(self: &'foo mut Foo<'a>, v: &'foo mut u32) where 'foo: 'a{
        self.x_mut = v;
    }
}

static mut NUM: u32 = 5;
// static ref xx: &u32 = &5;

unsafe fn run() {
    let x_ref: &'static mut u32 = &mut NUM;
    let y;
    {
        let mut foo = Foo {
            x_mut: x_ref,
            owned: 123,
        };
        let y_ref = &mut 6;
        y = foo.get_x_mut();
        // foo.set_x_mut(y_ref);
        // println!("{}", y);
    }

    // println!("{}", y);
    // // let x;
    // // {
    // //     let y = &mut 5;
    // //     let mut f = Foo {
    // //         x_mut: y,
    // //     };
    // //     x = &mut f;
    // //     let z = x.get();
    // //     println!("{:?}", z);
    // }
    // x.hello();
    // println!("{:?}", x);
    // let s;
    //
    // {
    //     let n = [1];
    //     let n_ref = &n;
    //     s = smallest_number(n_ref);
    // }
    // println!("{}", s);

    // let aaaa = String::from("aaa");
    // let result;
    // {
    //     let bbbbb = String::from("bbb");
    //     result = test_lifetime(bbbbb.as_str(), aaaa.as_str());
    // }
    // println!("The longest string is {}", result);
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r {}", r);



    let mut s = vec![1];
    let mut s3 = vec![2];
    let s2 = &mut s;
    // let mut ms = MySlice{
    //     slice: &mut s3,
    // };
    {
        // let mut v = vec![123];
        // ms.slice = &mut v;
        // let ms_res = &ms;
        // ms_res.get();
    }

    // println!("{:?}", ms);
}


// generic over lifetime 'a
// tie the lifetime of my MyiterWrapper, how long that struct can live for,
// to whatever the lifetime of the inner slices
struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    // the element that MyIter yield also tie to the life time of MyIter
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // if self.slice.is_empty() {
        //     return None;
        // }
        let (first, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(first)
    }
}

struct MyMutableIterator<'a, T> {
    slice: &'a mut [T],
}

impl <'a, T> Iterator for MyMutableIterator<'a, T> {
    type Item = &'a mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice: &'next mut &'a mut[T] = &mut self.slice;
        // let slice: &'a mut[T] = self.slice;
        let slice:&'a mut [T] = std::mem::replace(slice, &mut []);
        // let (first, rest) = slice.split_first_mut()?;
        // self.slice = rest;
        // Some(first)
        let element = slice.get_mut(0);
        element
        // todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {run();}
    }

}