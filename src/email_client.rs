use crate::domain::UserEmail;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use secrecy::{ExposeSecret, Secret};

pub struct EmailClient {
    sender: UserEmail,
    mailer: SmtpTransport,
}
//TODO: remove unwraps()
//TODO: Handle click on confirmation link twice scenario
impl EmailClient {
    pub async fn send_email(
        &self,
        recipient: UserEmail,
        subject: &str,
        _html_body: &str,
        text_body: &str,
    ) -> Result<(), anyhow::Error> {
        let email = Message::builder()
            .from(self.sender.as_ref().parse().unwrap())
            .to(recipient.as_ref().parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(text_body))
            .unwrap();

        match self.mailer.send(&email) {
            Ok(_) => println!("Email sent"),
            Err(_) => println!("Email not sent"),
        };
        Ok(())
    }
    pub fn new(
        smtp_connection: String,
        sender: UserEmail,
        password: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let creds = Credentials::new(
            sender.as_ref().to_owned(),
            password.expose_secret().to_owned(),
        );
        let mailer = SmtpTransport::relay(&smtp_connection)
            .unwrap()
            .credentials(creds)
            .timeout(Some(timeout))
            .build();
        Self { mailer, sender }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

#[cfg(test)]
mod tests {
    use crate::configuration::get_configuration;
    use crate::email_client::EmailClient;
    fn email_client() -> EmailClient {
        let c = get_configuration()
            .expect("Failed to read configs")
            .email_client;
        let sender_email = c.sender().expect("Invalid sender email address");

        EmailClient::new(
            c.smtp_host,
            sender_email,
            c.sender_password,
            std::time::Duration::from_millis(200),
        )
    }

    // #[tokio::test]
    // async fn smtp_connection_successful() {
    //     assert_eq!(email_client().mailer.test_connection().unwrap(), true);
    // }
}
