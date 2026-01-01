use std::sync::Arc;
use anyhow::Result;
use crate::domain::notification::port::EmailSender;

pub struct NotificationService {
    email_sender: Arc<dyn EmailSender>,
}

impl NotificationService {
    pub fn new(email_sender: Arc<dyn EmailSender>) -> Self {
        Self { email_sender }
    }

    pub fn send_test_email(&self, to: &str) -> Result<()> {
        self.email_sender.send(
            to,
            "Paybridge Notification Test",
            "This is a dummy MailHog test email",
        )
    }
}
