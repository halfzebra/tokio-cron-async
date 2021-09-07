# Notes

## First itreration

Some time was needed to figure out how the type for async callback would look.

Unlike the example from [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html), using `std::sync::Mutex` is problematic for shared mutable state in async code.
Tokio documentation on [Shared state](https://tokio.rs/tokio/tutorial/shared-state) mentions `tokio::sync::Mutex`, which is a bit easier to integrate.

The bare bone idea of the runner is using the following code as at it's core:

```rust
use tokio::time::{sleep, Duration};
use tokio::sync::{Mutex};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc};

pub type Callback = Pin<Box<dyn Fn(usize) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> + Send + Sync>>;

#[tokio::main]
async fn main() {
    let futures: Arc<Mutex<Vec<Callback>>> = Arc::new(Mutex::new(vec![]));

    futures.lock().await.push(Box::pin(|i| Box::pin(async move {
        sleep(Duration::from_secs(1)).await;
        println!("One second passed! {}", i);
    })));

    futures.lock().await.push(Box::pin(|i| Box::pin(async move {
        sleep(Duration::from_secs(2)).await;
        println!("Two second passed! {}", i);
    })));

    tokio::spawn(async move {
        let lock = futures.lock().await;

        for (i, fut) in (*lock).iter().enumerate() {
            fut(i).await;
        }
    }).await.ok();
}
```

## Second iteration

After experiencing issues with [reqwest](https://github.com/seanmonstar/reqwest), I've got a [suggestion for simplification](https://users.rust-lang.org/t/sharing-futures-across-threads-insidie-async-block-callbacks-shared-across-threads/65064/4):

```rs
pub type Callback = Box<dyn Fn(usize) -> Pin<Box<dyn Future<Output=()> + Send>> + Send + Sync>;
```

## Third iteration

It became apparent that futures would be executed sequentially if they are awaited in a loop.
Instead of queueing Futures, it's necessary to await every future in a separate `tokio::spawn` task.
This complicates things quite a bit, but this is the best working POC so far.

```rs
use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type Callback = Box<dyn Fn(usize) -> Pin<Box<dyn Future<Output=()> + Send>> + Send + Sync>;

#[tokio::main]
async fn main() {
    let futures: Arc<Mutex<Vec<Arc<Mutex<Callback>>>>> = Arc::new(Mutex::new(vec![]));

    futures.lock().await.push(Arc::new(Mutex::new(Box::new(|i| Box::pin(async move {
        sleep(Duration::from_secs(1)).await;
        println!("One second passed! {}", i);
    })))));

    futures.lock().await.push(Arc::new(Mutex::new(Box::new(|i| Box::pin(async move {
        sleep(Duration::from_secs(2)).await;
        println!("Two second passed! {}", i);
    })))));

    tokio::spawn(async move {
        let lock = futures.lock().await;

        loop {
            for (i, fut) in (*lock).iter().enumerate() {
                let clone = Arc::clone(fut);

                tokio::spawn(async move {
                    (clone.lock().await)(i).await;
                });
            }
        }

    }).await.ok();
}
```

## Links

Closures, Futures and async-await primer:

- [Demystifying Closures, Futures and async-await in Rust–Part 1: Closures](https://medium.com/swlh/demystifying-closures-futures-and-async-await-in-rust-part-1-closures-97e531e4dc50) 
- [Demystifying Closures, Futures, and async-await in Rust Part 2: Futures](https://levelup.gitconnected.com/demystifying-closures-futures-and-async-await-in-rust-part-2-futures-abe95ab332a2)
- [Demystifying Closures, Futures and async-await in Rust–Part 3: Async & Await](https://medium.com/@alistairisrael/demystifying-closures-futures-and-async-await-in-rust-part-3-async-await-9ed20eede7a4)