use tokio_cron_async::JobSchedule;
use chrono::Local;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("start {}", Local::now().time());
    let schedule: JobSchedule = JobSchedule::new();

    schedule.add(
        "1/10 * * * * *",
        Box::new(|_i| {
            Box::pin(async move {
                println!("1/10 {}", Local::now().time());
                sleep(Duration::from_secs(10)).await;
            })
        }),
    )
    .await
    .expect("Added");

    schedule.add(
        "1/20 * * * * *",
        Box::new(|_i| {
            Box::pin(async move {
                println!("1/20 {}", Local::now().time());
                sleep(Duration::from_secs(20)).await;
            })
        }),
    )
    .await
    .expect("Added");

    tokio::spawn(schedule.run()).await.ok();
}