use crate::configuration::{DatabaseSettings, Settings};
//use crate::email_client::EmailClient;
use crate::routes::{health_check,fetch_userdata};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(configuration.database.with_db());

        //NOTE: Build an Email Client
        // let sender_email = configuration
        //     .email_client
        //     .sender()
        //     .expect("Invalid sender email address");
        // let timeout = configuration.email_client.timeout();
        // let email_client = EmailClient::new(
        //     configuration.email_client.smtp_host,
        //     sender_email,
        //     configuration.email_client.sender_password,
        //     timeout,
        // );
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener: TcpListener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let base_url = configuration.application.base_url;
        let server = run(listener, connection_pool, base_url.clone())?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

pub fn run(tcp: TcpListener, db_pool: PgPool, base_url: String) -> Result<Server, std::io::Error> {
    let db_pool: web::Data<PgPool> = web::Data::new(db_pool);
    //let email_client: web::Data<EmailClient> = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/fetch_data", web::get().to(fetch_userdata))
            .app_data(web::PayloadConfig::new(1 << 25))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
    })
    .listen(tcp)?
    .run();
    Ok(server)
}
