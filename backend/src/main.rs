use sailfish::TemplateOnce;

fn main() {
    println!("Hello, world!");
}

#[derive(TemplateOnce)]
#[template(path = "emails/reset-password.html")]
struct ResetPassword {
    /// Website name
    website: String,
    /// Password reset url
    url: String,
    /// Sender email
    from: String,
    /// Copyright year
    year: String,
}

#[tokio::test]
fn test() {
    let email = ResetPassword {
        website: "Example Website".to_string(),
        url: "https://example.com/account/reset/aA612312yh6da2131".to_string(),
        from: "no-reply@example.com".to_string(),
        year: "2023".to_string(),
    };

    let template = email.render_once()
    .unwrap();
    let mailbox = Mailbox
}
