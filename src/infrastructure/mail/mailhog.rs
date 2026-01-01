use crate::domain::notification::port::EmailSender;
pub struct mailhogEmailSender;
impl EmailSender for mailhogEmailSender {
    fn send(&self, to: &str, subject: &str ,body: &str) -> anyhow::Result<()> {
        println!("==== MAILHOG DUMMY ====");
        println!("to: {}", to);
        println!("subject: {}", subject);
        println!("body: {}", body);
        println!("======================");
        Ok(())
    }
}