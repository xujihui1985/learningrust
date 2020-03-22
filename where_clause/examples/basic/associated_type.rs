fn debug_iter<I>(iter: I)
    where I: Iterator,
          I::Item: Debug {
    for item in iter {
        println!("{:?}", item);
    }
}
