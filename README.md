# ifttt_webhook_rust

[![crate.io](https://img.shields.io/crates/v/ifttt_webhook_rust)](https://crates.io/crates/ifttt_webhook_rust)

a simple lib binding to the [ifttt](https://ifttt.com/home)'s [webhook](https://ifttt.com/maker_webhooks) api.
there is a async interface can be activate in feature `non-blocking`.
and a time delay trigger function for delay the trigger in feature `delay`.

the blocking interface use [ureq](https://crates.io/crates/ureq),
and the non-blocking interface use [reqwest](https://crates.io/crates/reqwest) internally.

about ifttt webhook usage:
For example,You can call a url (supplied by ifttt) then receive a notification (could include data* you supplied) on you phone.

*sometimes you can set some json data (up to three fields in it) on the request, which is depends on the service webhook connected with.

# Installation

- find in [crates.io](https://crates.io/crates/ifttt_webhook_rust)

- use [cargo-edit](https://crates.io/crates/cargo-edit)
```sh
cargo add ifttt_webhook_rust
```

# Usage
### blocking api
*code*
```rust
    extern crate ifttt_webhook_rust
    extern crate dotenv
    use ifttt_webhook_rust::*

    dotenv::dotenv().unwrap();
    let event_name = dotenv::var("EVENT").unwrap();
    let api_key = dotenv::var("KEY").unwrap();
    let client = BlockingIftttWebHookClient::new(&api_key);
    let res = client.trigger(&event_name, None);
    assert!(res.is_ok())
```
### non-blocking api
*Cargo.toml*
```toml
ifttt_webhook_rust={version=*,default-features= false,features=["non-blocking"]}
```
*code*
```rust
    extern crate ifttt_webhook_rust
    extern crate dotenv
    use ifttt_webhook_rust::*

    dotenv::dotenv().unwrap();
    let event_name = dotenv::var("EVENT").unwrap();
    let api_key = dotenv::var("KEY").unwrap();
    let data = WebHookData::new(Some("test_blocking1"), Some("test2"), None);
    let client = NonBlockingIftttWebHookClient::new(&api_key);
    let res = client.trigger(&event_name, data).await;
    assert!(res.is_ok())
```
### non-blocking api with time delay
*Cargo.toml*
```toml
ifttt_webhook_rust={version=*,default-features= false,features=["delay"]}
```
*code*
```rust
    extern crate ifttt_webhook_rust
    extern crate dotenv
    use ifttt_webhook_rust::*
    
    dotenv::dotenv().unwrap();
    let event_name = dotenv::var("EVENT").unwrap();
    let api_key = dotenv::var("KEY").unwrap();
    let client = NonBlockingIftttWebHookClient::new(&api_key);    
    let res_handler: DelayResultHandler =
        client.trigger_with_delay(&event_name, None, std::time::Duration::from_secs(5));
    ///do something else
    let res = res_handler.await;
    assert!(res.is_ok())
```