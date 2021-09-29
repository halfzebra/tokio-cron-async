# tokio-cron-async

CRON scheduler using Tokio for async jobs.

```rs
use tokio_cron_async::JobSchedule;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schedule: JobSchedule = JobSchedule::new();

    schedule.add("1/10 * * * * *", Box::new(|_uuid| {
        Box::pin(async move {
            println!("10 seconds have passed");
            sleep(Duration::from_secs(10)).await;
        })
    }))
    .await?;

    tokio::spawn(schedule.run()).await?;

    Ok(())
}
```

## Motivation

> Why?

I wanted something like [mvniekerk/tokio-cron-scheduler][1], but with async jobs.

> Why weren't this contributed to the mentioned repo?

It's a bit unclear wether the API from this repo is suitable for different use-cases and project I've built this for can tolerate potential risk.
It might be contributed there at later stages.

## Inspiration

- [mvniekerk/tokio-cron-scheduler][1]

[1]: <https://github.com/mvniekerk/tokio-cron-scheduler> "mvniekerk/tokio-cron-scheduler repository on GitHub"
