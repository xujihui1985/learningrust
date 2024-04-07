### tokio runtime

#### io eventloop
#### scheduler
scheduler take futures that is not finished, call the poll method on that future, and the poll method return
the status of the future, Ready or Pending, when return Pending, the future was put back to the runtime to execute again
at some later point of time

multi thread scheduler

multi thread scheduler create native thread for each cpu core of your workload

each one has it's local queue of tasks it knows about, if there is no task in the local queue, it will look global queue, if there is 
no task in global either, then it will steal task from local queue of other core so future must be send, must be allowed to move between threads


current thread scheduler
the use case is for the task that you don't want to move between different cores

does not start any threads, it startup tokio runtime on the current thread

##### Blocking

when running future that block, like reading from stdio with sync manner, it will block system level thread
you not just blocking the future, you actually blocking entire worker thread, there is no other way to execute other tasks
because the os level thread is blocked

`spawn_blocking` tokio has 2 thread pools, one of them is for async tasks, the other is blocking thread pool which is expect to block
they are just doing something that we know will block the current thread, it's quite same as native thread pool
`block_in_place` run the clousure on the current thread, so the future doesn't need to be `send`

the diffence between spawn_blocking and block_in_place is with spawn_blocking you spawn a new thread and block on the new thread, while block_in_place, you move the task
on current worker to new worker then turn the current work to become a blocking thread

use block_in_place when you need to do some blocking stuff that can no be send

`shutdown_background` gracefully shutdown 

why Waker is not a trait is because Clone is not object safe

`localset` run future that are not sent, once you are running on the thread, you are not allow to move them on other thread, LocalSet is a set of task that all
executed on the same thread, **localset can only be used on top level task**




#### spawn
put the future you give on to the queue of the tasks, and it doesn't do anything other then just put it on the queue
and returns a joinhandle to the task

** when joinhandle drop, it doesn't do anything, it will not stop the task **

every future that has been spawned on the runtime becomes a **task**, that doesn't mean every future on the runtime is a task
future can contain other futures

only the future you paas to spawn, become the top level task, tokio scheduler only knows about the top level task, it can't see the inner future

#### timer


### io resource

when you found you put io resource like tcpsteam inside a mutex and share between two tasks, it's better to use a dedicated thread to process tcpstream
and communicate with other thread by channel


### util

oneshot channel is useful for bridging the sync and async world

watch channel similar as boardcast channel, but only keeps most recent value, the sender side update the latest value instead of push the value in the queue
the receive side can subscribe the change of the value, for example, the value update 10 times, but will only get the latest value, useful in config change case
when the notification is not imporant, but the latest value is imporant

notify

semaphore

cancelation safe

```
#[tokio::main]
async fn main() {

    let mut tcp = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    loop {
        tokio::select! {
            v = read_string_of_json(&mut tcp) => {
                println!("{}", v);
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                println!("wait");
            }

        }
    }


}
 
async fn read_string_of_json(tcp: &mut TcpStream) -> String {
    let mut s = String::new();
    loop {
        let mut buf = [0u8; 1024];
        let n = tcp.read(&mut buf).await.unwrap();
        s.push_str(std::str::from_utf8(&buf[..n]).unwrap());
        if s.len() > 100 {
            return s
        }
    }
}
```
imaging read_string_of_json read 50 bytes and yield, then time sleep resolved, what will happened is the read_string_of_json future will just dropped, this is not 
cancelation safe


```
#[tokio::main]
async fn main() {

    let mut tcp = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut fut = std::pin::pin!(read_string_of_json(&mut tcp));

    loop {
        tokio::select! {
            v = &mut fut => {
                println!("{}", v);
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                println!("wait");
            }

        }
    }
}
 
```
the fix is to move the read_string_of_json out of loop, and select the reference of future, and remember to pin it to make compiler happy, this way the future will
not be dropped

#### take away

- when the joinhandle of spawn drop, it didn't cancel the future, if you really want to cancel it need call abort explicatly
- every time do a spawn, asign the join handle to variable, and decide what to do with that variable
