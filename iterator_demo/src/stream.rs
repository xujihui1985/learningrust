use futures_util::{StreamExt, stream};


#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![1,2,3,4,5]).map(|i| async move {i}).buffered(2).fuse();
    let mut stream2 = stream::iter(vec![1,2,3]).map(|i| async move {i}).fuse();

    while let Some(v) = stream.next().await {
        println!("{}", v);
    }

    // loop {
    //     tokio::select! {
    //         msg = stream.next().fuse() => match msg {
    //             Some(i) => println!("stream: {}", i.await),
    //             None => println!("stream: {}", "end"),
    //         },
    //         // msg = stream2.next() => match msg {
    //         //     Some(i) => println!("stream2: {}", i.await),
    //         //     None => println!("stream2: {}", "end"),
    //         // },
    //         // else => {
    //         //     println!("end");
    //         //     break;
    //         // }
    //     }
    // }   
}