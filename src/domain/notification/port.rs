use anyhow::Result;
// 'pub trait' defines a public interface named EmailSender.
// ': Send + Sync' are "Marker Traits". 
// - Send: This object can be safely moved to a different thread.
// - Sync: This object can be safely shared between multiple threads.
pub trait EmailSender: Send + Sync {

    /// Sends an email.
    ///
    /// # Arguments
    /// * &self    - An instance method that borrows the struct implementing this trait.
    /// * to       - A string slice representing the recipient's address.
    /// * subject  - A string slice for the email's subject line.
    /// * body     - A string slice for the actual content of the email.
    ///
    /// # Returns
    /// * anyhow::Result<()> - Returns Ok(()) if successful, or an error if it fails.
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<()>;
}