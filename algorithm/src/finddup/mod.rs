use std::collections::{HashMap, HashSet};

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    if nums.len() == 0 {
        return res;
    }
    nums.sort();
    for i in 0..(nums.len() - 1) {
        if nums[i] == nums[i + 1] {
            if !res.contains(&nums[i]) {
                res.push(nums[i]);
            }
        }
    }
    res
}

pub fn find_duplicates_set(nums: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    let mut res = HashSet::<i32>::new();
    if nums.len() == 0 {
        return Vec::new();
    }
    for i in nums {
        let v = map.entry(i).or_insert(0);
        *v += 1;
        if *v > 1 {
            res.insert(i);
        }
    }
    res.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::finddup::*;

    #[test]
    fn it_works() {
        let v = vec![1,2,3,4,4,5,6];
        let res = find_duplicates(v);
        assert_eq!(1, res.len());
        let v = vec![1,2,3,4,4,5,6];
        let res = find_duplicates_set(v);
        assert_eq!(1, res.len());
    }

    #[test]
    fn it_works2() {
        let mut v = vec![1,2,3,4,4,5,6];
        let i = &mut v[0];
        *i += 1;
        println!("{:?}", v)
    }
}
