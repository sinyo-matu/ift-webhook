# ifttt_webhook_rust

[![crate.io](https://img.shields.io/crates/v/ifttt_webhook_rust)](https://crates.io/crates/ifttt_webhook_rust)

binding to the ifttt webhook api.
there is a async interface can be activate in feature `non-blocking`.
and a time delay trigger function for delay the trigger in feature `delay`.

the blocking interface use [ureq](https://crates.io/crates/ureq),
and the non-blocking interface use [reqwest](https://crates.io/crates/reqwest) internally.

# Installation

- find in [crates.io](https://crates.io/crates/ifttt_webhook_rust)

- use [cargo-edit](https://crates.io/crates/cargo-edit)
```sh
cargo add ifttt_webhook_rust
```

# Usage
- *blocking api*
```rust
    extern crate ifttt_webhook_rust
    extern crate dotenv
    use ifttt_webhook_rust::*

    dotenv::dotenv().unwrap();
    let event_name = dotenv::var("EVENT").unwrap();
    let api_key = dotenv::var("KEY").unwrap();
    let blocking_client = BlockingIftttWebHookClient::new(&event_name, &api_key);
    let res = client.trigger(None);
    assert!(res.is_ok())
```
- *non-blocking api*
```rust
    extern crate ifttt_webhook_rust
    extern crate dotenv
    use ifttt_webhook_rust::*

    dotenv::dotenv().unwrap();
    let event_name = dotenv::var("EVENT").unwrap();
    let api_key = dotenv::var("KEY").unwrap();
    let data = WebHookData::new(Some("foo"), Some("bar"), None);
    let client = NonBlockingIftttWebHookClient::new(&event_name, &api_key);
    let res = client.trigger(data).await;
    assert!(res.is_ok())
```
- *non-blocking api with time delay*
```rust
    extern crate ifttt_webhook_rust
    extern crate dotenv
    use ifttt_webhook_rust::*
    
    dotenv::dotenv().unwrap();
    let event_name = dotenv::var("EVENT").unwrap();
    let api_key = dotenv::var("KEY").unwrap();
    let client = NonBlockingIftttWebHookClient::new(&event_name, api_key);
    let res_handler: DelayResultHandler =
        client.trigger_with_delay(None,std::time::Duration::from_secs(5));
    ///do something else
    let res = res_handler.await;
    assert!(res.is_ok())
```