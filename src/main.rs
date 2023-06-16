use kriegdercreator::configuration::get_configuration;
use kriegdercreator::startup::Application;
use kriegdercreator::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //NOTE: Initialize telemetry (cf. Telemetry.rs)
    let subscriber = get_subscriber(std::io::stdout, "kriegdercreator".into(), "info".into());
    init_subscriber(subscriber);

    //NOTE: Load configurations, connect to database and start the server
    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
