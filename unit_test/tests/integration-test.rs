use mybuck::Bucketizer;

#[test]
fn it_works() {
    let b = Bucketizer::new()
        .bucket(Some(0.0), Some(1.0), 0.5);
    assert_eq!(b.bucketizer(0.1), Some(0.5));
}

