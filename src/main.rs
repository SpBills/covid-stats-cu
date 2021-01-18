use actix_web::{App, HttpServer};
use cronjob::CronJob;

mod tasks;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut cron = CronJob::new("Fetch COVID data.", tasks::polling::CovidData::poll);
    cron.minutes("5");
    cron.start_job();

    HttpServer::new(|| App::new())
        .bind("localhost:9090")?
        .run()
        .await
}
