use crate::domain::notification::port::EmailSender;
use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub struct MailhogEmailSender {
    mailer: SmtpTransport,
    from: String,
}

impl MailhogEmailSender {
    pub fn new() -> Self {
        let host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".into());
        let port: u16 = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "1025".into())
            .parse()
            .expect("Invalid SMTP_PORT");

        let from = env::var("SMTP_FROM")
            .unwrap_or_else(|_| "no-reply@paybridge.local".into());

        let mailer = SmtpTransport::builder_dangerous(host)
            .port(port)
            .build();

        Self { mailer, from }
    }
}

impl EmailSender for MailhogEmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_string())?;

        self.mailer.send(&email)?;
        Ok(())
    }
}
