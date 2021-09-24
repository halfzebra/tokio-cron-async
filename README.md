# tokio-cron-async

Simple CRON scheduler using Tokio for async jobs.

```rs
use tokio_cron_async::JobSchedule;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let schedule: JobSchedule = JobSchedule::new();

    schedule.add("1/10 * * * * *", Box::new(|_uuid| {
        Box::pin(async move {
            println!("10 seconds have passed");
            sleep(Duration::from_secs(10)).await;
        })
    }))
    .await
    .ok();

    tokio::spawn(schedule.run()).await.ok();
}
```

## Motivation

> Why?

I wanted something like [mvniekerk/tokio-cron-scheduler][1], but with async jobs.

> Why weren't this contributed to the mentioned repo?

Because I'm not confident in this implementation and the project I've built this for can tolerate potential risks.
It might be contributed there at later stages.

## Inspiration

- [mvniekerk/tokio-cron-scheduler][1]

[1]: <https://github.com/mvniekerk/tokio-cron-scheduler> "mvniekerk/tokio-cron-scheduler repository on GitHub"
