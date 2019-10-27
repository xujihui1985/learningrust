static mut STASH: &i32 = &123;

struct Extrema {
    greatest: i32,
    least: i32,
}

struct ExtremaWithLifetime<'a> {
    greatest: &'a i32,
    least: &'a i32,
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
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
    Extrema{greatest :*greatest, least: *least}
}

// 'a lifetime could be omitted
fn find_extrema_with_lifetime<'a>(slice: &'a [i32]) -> ExtremaWithLifetime<'a> {
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
    ExtremaWithLifetime{greatest, least}
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        f(&321);
        unsafe {
            assert_eq!(STASH, &321);
        }
    }

    #[test]
    fn test_loop() {
        'out:
        loop {
            println!("run in loop");
            thread::sleep(Duration::from_secs(1));
            break 'out;
        }
    }

    #[test]
    fn test_extrema() {
        let a = [1,2,3,4,5];
        let e = find_extrema(&a);
        assert_eq!(e.least, 1);
        assert_eq!(e.greatest, 5);
    }

    #[test]
    fn test_extrema_with_lifetime() {
        let a = [1,2,3,4,5];
        let e = find_extrema_with_lifetime(&a);
        assert_eq!(*e.least, 1);
        assert_eq!(*e.greatest, 5);
    }


}
