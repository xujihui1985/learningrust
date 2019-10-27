#[derive(Debug)]
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

fn main() {
    println!("Hello, world!");
    let s = [1,2,3,4,5,6,7];
    let e = find_extrema(&s);
    println!("{}", e.greatest);
    println!("{}", e.least);
}
