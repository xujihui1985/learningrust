
fn main() {
  // let mut qf = QuickFind::<20>::new();
  // qf.union(1, 10);
  // qf.union(2, 8);
  // qf.union(1, 2);

  // println!("{:?}", qf);
  // println!("{:?}",qf.connected(1, 8));


  let mut qu = QuickUnion::new(20);
  qu.union(1, 10);
  qu.union(2, 1);
  println!("{:?}", qu);

  println!("{:?}", qu.connected(2, 10));

  let b: &Vec<u32>;
  {
    let c = vec![1_u32];
    b = extend(&c);
  }
  println!("{:?}", b);

}

#[derive(Debug)]
struct QuickFind<const N:usize> {
  id: [u32; N],
}

impl<const N:usize> QuickFind<N> {
  fn new() -> QuickFind<N> {
    let mut arr = [0_u32; N];
    for (i, v) in arr.iter_mut().enumerate() {
      *v = i as u32;
    }
    QuickFind {
      id: arr,
    }
  }

  fn connected(&self, p: usize, q: usize) -> bool {
    self.id[p] == self.id[q]
  }

  fn union(&mut self, p: usize, q: usize) {
    let pid = self.id[p];
    let qid = self.id[q];
    for i in 0..N {
      if self.id[i] == pid {
        self.id[i] = qid;
      }
    }
  }
}

#[derive(Debug)]
struct QuickUnion {
  id: Vec<u32>,
  sz: Vec<u32>,
}

impl QuickUnion {
  fn new(n: usize) -> Self {
    let v = (0..n as u32).collect();
    let sz = vec![1; n]; // default size of each tree is 1
    Self {
      id: v,
      sz: sz,
    }
  }

  fn root(&self, i: u32) -> u32 {
    let mut i = i;
    while i != self.id[i as usize] {
      i = self.id[i as usize];
    }
    return i;
  }

  fn connected(&self, p: u32, q: u32) -> bool {
    self.root(p) == self.root(q)
  }

  fn union(&mut self, p: u32, q: u32) {
    let i = self.root(p);
    let j = self.root(q);
    println!("i: {} j: {}", i, j);
    self.id[i as usize] = j;
  }

  fn union_improved(&mut self, p: u32, q: u32) {
    let i = self.root_improve(p);
    let j = self.root_improve(q);
    if self.sz[i as usize] < self.sz[j as usize] {
      self.id[i as usize] = j;
      self.sz[j as usize] = self.sz[j as usize] + self.sz[i as usize];
    } else {
      self.id[j as usize] = i;
      self.sz[i as usize] = self.sz[i as usize] + self.sz[j as usize];
    }
  }


  fn root_improve(&mut self, i: u32) -> u32 {
    let mut i = i as usize;
    while i != self.id[i] as usize {
      self.id[i] = self.id[self.id[i] as usize]; // path compression
      i = self.id[i] as usize;
    }
    return i as u32;
  }
}


// 'a must be at least as long as 'b aka outlive 'b
fn weird<'a, 'b, T>(_w: &'b &'a (), borrow: &'a T) -> &'b T {
  borrow
}

const FOREVER: &'static &'static () = &&();

fn extend<'a, 'b, T>(borrow: &'a T)  -> &'b T {
  //let weird_function: fn(&'b &'a (), &'a T) -> &'b T = weird;
  let weird_function = weird;
  weird_function(FOREVER, borrow)
}