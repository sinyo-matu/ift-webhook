#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn non_blocking_trigger_without_data() {
        dotenv::dotenv().unwrap();
        let event_name = dotenv::var("EVENT").unwrap();
        let api_key = dotenv::var("KEY").unwrap();
        let client = NonBlockingIftttWebHookClient::new(&api_key);
        let res = client.trigger(&event_name, None).await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn non_blocking_trigger_with_data() {
        dotenv::dotenv().unwrap();
        let event_name = dotenv::var("EVENT").unwrap();
        let api_key = dotenv::var("KEY").unwrap();
        let data = WebHookData::new(Some("test_blocking1"), Some("test2"), None);
        let client = NonBlockingIftttWebHookClient::new(&api_key);
        let res = client.trigger(&event_name, data).await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn delay_trigger_without_data() {
        dotenv::dotenv().unwrap();
        let event_name = dotenv::var("EVENT").unwrap();
        let api_key = dotenv::var("KEY").unwrap();
        let client = NonBlockingIftttWebHookClient::new(&api_key);
        let res_handler: DelayResultHandler =
            client.trigger_with_delay(&event_name, None, std::time::Duration::from_secs(5));
        println!("yo");
        let res = res_handler.await;
        assert!(res.is_ok())
    }

    #[test]
    fn blocking_trigger_without_data() {
        dotenv::dotenv().unwrap();
        let event_name = dotenv::var("EVENT").unwrap();
        let api_key = dotenv::var("KEY").unwrap();
        let client = BlockingIftttWebHookClient::new(&api_key);
        let res = client.trigger(&event_name, None);
        assert!(res.is_ok())
    }

    #[test]
    fn blocking_trigger_with_data() {
        dotenv::dotenv().unwrap();
        let event_name = dotenv::var("EVENT").unwrap();
        let api_key = dotenv::var("KEY").unwrap();
        let client = BlockingIftttWebHookClient::new(&api_key);
        let data = WebHookData::new(Some("test1"), Some("test2"), Some("test3"));
        let res = client.trigger(&event_name, data);
        assert!(res.is_ok())
    }
}
