
fn main() {
    let mut floats = vec![1.1,1.2,3.2];
    sort_floats(&mut floats);
    println!("{:?}", floats);

    let a = floats.iter();
}

fn sort_floats<T>(data: &mut [T]) where T: PartialOrd {
    use std::cmp::Ordering::Less;
    data.sort_by(|a,b| {
        a.partial_cmp(b).unwrap_or(Less)
    });
}
