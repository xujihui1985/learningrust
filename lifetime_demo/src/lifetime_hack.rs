
const FOREVER: &'static &'static () = &&();

fn main() {
  println!("hello world");
  let hello = "hello";
  // hac(FOREVER, &hello);

  let b: &Vec<u32>;
  {
    let c = vec![1_u32];
    b = extend(&c);
    // b = weird(FOREVER,&c);
  }
  println!("{:?}", b);
}

fn borrow_check() {
  let borrow: &Vec<u32>;
  {
    let c = vec![1_u32];
    borrow = extend(&c);
  }
  println!("{:?}", borrow);
}

// 'a must be at least as long as 'b aka outlive 'b
fn hack<'a, 'b, T>(_w: &'b &'a (), borrow: &'a T) -> &'b T {
  borrow
}


fn extend<'a, 'b, T>(borrow: &'a T)  -> &'b T {
  let hack_func: fn(&'static &'static (), &'a T) -> &'b T = hack;
  // weird_function(FOREVER, borrow)
  let res = hack_func(FOREVER, borrow);
  res
}


// variance
// function return are covariant
// function arguments are contravariant

// fn return_static<'a>(_v: &'a u32) -> &'static u32 {
//   &42
// }

// fn test_a<F>(f: F)
//   where F:for<'a> Fn(&'a u32) -> &'a u32
// {
// }

// fn return_tika<'a>(v: &'a u32) -> &'a u32 {
//   v 
// }

// fn use_testa() {
//   test_a(return_static); 
// }

// Higher rank trait bounds (HRTB)

fn extend2<'a, 'b, T>(borrow: &'a T)  -> &'b T {
  let step1: for<'x, 'y> fn(&'y &'x (), &'x T) -> &'y T = hack;
  let step2: for<'x, 'y> fn(&'y &'static (), &'x T) -> &'y T = step1;
  let step3: fn(&'b &'static (), &'a T) -> &'b T = step2;
    
  let res = step3(FOREVER, borrow);
  res
}