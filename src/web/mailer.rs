// All Rights Reserved — The Cochran Block, LLC
//! Outbound notification email. Gmail SMTP via lettre.
//!
//! Required env (via approuter/.env or process env):
//!   SMTP_HOST       = smtp.gmail.com   (default)
//!   SMTP_PORT       = 465              (default — implicit TLS)
//!   SMTP_USER       = mcochran@cochranblock.org      (full gmail-hosted address)
//!   SMTP_PASS       = <16-char gmail app password>
//!   BOOKING_NOTIFY_TO = mcochran@cochranblock.org    (who to notify on /book submit)

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct BookingEmail {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub date_iso: String,
    pub time_label: String,
    pub topic: String,
    pub ip: String,
}

/// Send the booking notification. Runs synchronously on the caller's thread,
/// so call from tokio::spawn_blocking or tokio::spawn. Returns Err on any
/// missing-config or transport failure; caller decides whether to fail the
/// submit or just log.
pub fn send_booking(b: &BookingEmail) -> Result<(), String> {
    let host = std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".into());
    let port: u16 = std::env::var("SMTP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(465);
    let user = std::env::var("SMTP_USER").map_err(|_| "SMTP_USER unset".to_string())?;
    let pass = std::env::var("SMTP_PASS").map_err(|_| "SMTP_PASS unset".to_string())?;
    let to = std::env::var("BOOKING_NOTIFY_TO").unwrap_or_else(|_| user.clone());

    let subject = format!(
        "[book] {} · {} at {} · {}",
        b.name, b.date_iso, b.time_label, b.topic
    );
    let body = format!(
        "New discovery-call request from cochranblock.org/book\n\
         \n\
         ID:    {}\n\
         Name:  {}\n\
         Email: {}\n\
         Phone: {}\n\
         Date:  {} ({})\n\
         Topic: {}\n\
         IP:    {}\n\
         \n\
         Reply directly to this email to contact the requester — the From header\n\
         is set to their address.\n",
        b.id, b.name, b.email, b.phone, b.date_iso, b.time_label, b.topic, b.ip
    );

    let from_mbox = format!("cochranblock booking <{}>", user);
    let reply_to_mbox = format!("{} <{}>", b.name, b.email);

    let msg = Message::builder()
        .from(from_mbox.parse().map_err(|e| format!("from parse: {}", e))?)
        .reply_to(
            reply_to_mbox
                .parse()
                .map_err(|e| format!("reply-to parse: {}", e))?,
        )
        .to(to.parse().map_err(|e| format!("to parse: {}", e))?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|e| format!("message build: {}", e))?;

    let creds = Credentials::new(user, pass);
    let builder = if port == 465 {
        SmtpTransport::relay(&host).map_err(|e| format!("relay config: {}", e))?
    } else {
        SmtpTransport::starttls_relay(&host).map_err(|e| format!("starttls config: {}", e))?
    };
    let mailer = builder.port(port).credentials(creds).build();

    mailer
        .send(&msg)
        .map(|_| ())
        .map_err(|e| format!("smtp send: {}", e))
}
