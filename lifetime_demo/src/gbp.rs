#[derive(Debug, PartialEq)]
pub struct GBP(i32);

pub fn on_money(a: i32, b: i32) -> GBP {
    let mut g = GBP(a);
    let r;
    {
        r = &g;
        g.0 += 2;
    }
    let res = GBP(g.0 + b);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = on_money(1,2);
        assert_eq!(g, GBP(3));
    }
}
