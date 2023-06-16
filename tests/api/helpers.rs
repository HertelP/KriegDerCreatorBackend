use kriegdercreator::configuration::{get_configuration, DatabaseSettings};
use kriegdercreator::startup::{get_connection_pool, Application};
use kriegdercreator::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

//NOTE: Init Tracing
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(std::io::stdout, subscriber_name, default_filter_level);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(std::io::sink, subscriber_name, default_filter_level);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
}
impl TestApp {
    // pub async fn post_user_create(&self, body: String) -> reqwest::Response {
    //     reqwest::Client::new()
    //         .post(&format! {"{}/TODO",&self.address})
    //         .header("Content-Type", "application/x-www-form-urlencoded")
    //         .body(body)
    //         .send()
    //         .await
    //         .expect("Failed to execute request.")
    // }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configs");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        db_pool: get_connection_pool(&configuration.database),
        port: application_port,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");
    //Migrate Database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create Database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the Database.");

    connection_pool
}
