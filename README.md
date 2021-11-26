# smtpclient
Rust SMTP client library, implements builder variant and standard initialisation.

Basic usage is a builder pattern with you initalising with the required fields then specifying other's as required.
Below example creates an Smtp struct then adds the data required to send a txt file and log into an smtp server that requires Username and Password
```rust
    let smtp_client_builder = SmtpBuilder::new(
        "host".to_string(),  
        "port".to_string(), 
        "sender".to_string(), 
        "recipient".to_string(), 
        "domain".to_string()
        );

        smtp_client_builder
        .subject("Email Subject".to_string())
        .body("This is a body - Generated by builder\nCan I have 한글? 안녕하세요~~".to_string())
        .display_name("Who I am".to_string())
        .login("username".o_string(), "password".to_string())
        .attach("test.txt".to_string(), ContentType::PlainText).unwrap()
        .send().unwrap(); 
```

### TODO:
- ~~Add builder~~
- ~~Unit Tests~~
- ~~Convert to lib~~
- ~~TLS~~
- ~~Attachments~~
- Compare to other libs
- HTML Emails
- ~~Env Variables~~
- ~~한글~~

