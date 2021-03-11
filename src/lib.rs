mod test;

#[cfg(feature = "non-blocking")]
use std::collections::HashMap;
#[cfg(feature = "delay")]
use tokio::task::JoinHandle;
#[cfg(feature = "delay")]
use tokio::time::sleep;
#[cfg(feature = "blocking")]
use ureq::{SerdeMap, SerdeValue};

#[cfg(feature = "delay")]
pub type DelayResultHandler = tokio::task::JoinHandle<Result<(), Error>>;

///
#[cfg(feature = "blocking")]
#[derive(Debug, Clone)]
pub struct IftWHClient {
    client: ureq::Agent,
    api_key: String,
}

pub struct WebHookData {
    value1: Option<String>,
    value2: Option<String>,
    value3: Option<String>,
}

impl WebHookData {
    pub fn new(value1: Option<&str>, value2: Option<&str>, value3: Option<&str>) -> Option<Self> {
        Some(Self {
            value1: value1.map(|s| s.to_string()),
            value2: value2.map(|s| s.to_string()),
            value3: value3.map(|s| s.to_string()),
        })
    }
}

///Blocking api client
#[cfg(feature = "blocking")]
impl IftWHClient {
    pub fn new(api_key: &str) -> Self {
        let client = ureq::Agent::new();
        Self {
            client,
            api_key: api_key.to_string(),
        }
    }

    pub fn trigger(&self, event_name: &str, data: Option<WebHookData>) -> Result<(), Error> {
        let url = format!(
            "https://maker.ifttt.com/trigger/{event}/with/key/{key}",
            event = event_name,
            key = self.api_key
        );
        match data {
            Some(data) => {
                let value = make_serde_value(data);
                let res = self
                    .client
                    .post(&url)
                    .set("Content-Type", "application/json")
                    .send_json(value)?;
                if res.status() != 200 {
                    return Err(Error::IftttResponseError);
                }
                return Ok(());
            }
            None => {
                let res = self.client.post(&url).call()?;
                if res.status() != 200 {
                    return Err(Error::IftttResponseError);
                }
                return Ok(());
            }
        }
    }
}

/// async api client
#[cfg(feature = "non-blocking")]
#[derive(Debug, Clone)]
pub struct AsyncIftWHClient {
    client: reqwest::Client,
    api_key: String,
}

#[cfg(feature = "non-blocking")]
impl AsyncIftWHClient {
    pub fn new(api_key: &str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            api_key: api_key.to_string(),
        }
    }

    pub async fn trigger(&self, event_name: &str, data: Option<WebHookData>) -> Result<(), Error> {
        let url = format!(
            "https://maker.ifttt.com/trigger/{event}/with/key/{key}",
            event = event_name,
            key = self.api_key
        );
        match data {
            Some(data) => {
                let map = nonblocking_make_serde_value(data);
                let res = self.client.post(&url).json(&map).send().await?;

                if res.status() != reqwest::StatusCode::OK {
                    return Err(Error::IftttResponseError);
                }
                return Ok(());
            }
            None => {
                let res = self.client.post(&url).send().await?;
                if res.status() != reqwest::StatusCode::OK {
                    return Err(Error::IftttResponseError);
                }
                return Ok(());
            }
        }
    }

    ///this delay function will take ownership of the client.
    #[cfg(feature = "delay")]
    pub fn trigger_with_delay(
        self,
        event_name: &str,
        data: Option<WebHookData>,
        delay_time: std::time::Duration,
    ) -> DelayResultHandler {
        let url = format!(
            "https://maker.ifttt.com/trigger/{event}/with/key/{key}",
            event = event_name,
            key = self.api_key
        );
        match data {
            Some(data) => {
                let map = nonblocking_make_serde_value(data);
                let handler: JoinHandle<Result<(), Error>> = tokio::spawn(async move {
                    sleep(delay_time).await;
                    let res = self.client.post(&url).json(&map).send().await?;
                    if res.status() != reqwest::StatusCode::OK {
                        return Err(Error::IftttResponseError);
                    };
                    Ok(())
                });
                return handler;
            }
            None => {
                let handler: JoinHandle<Result<(), Error>> = tokio::spawn(async move {
                    sleep(delay_time).await;
                    let res = self.client.post(&url).send().await?;
                    if res.status() != reqwest::StatusCode::OK {
                        return Err(Error::IftttResponseError);
                    }
                    Ok(())
                });
                return handler;
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "blocking")]
    BlockingRequestError(ureq::Error),
    #[cfg(feature = "non-blocking")]
    NonBlockingRequestError(reqwest::Error),
    IftttResponseError,
}
#[cfg(feature = "blocking")]
impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Self::BlockingRequestError(e)
    }
}
#[cfg(feature = "non-blocking")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::NonBlockingRequestError(e)
    }
}

#[inline]
#[cfg(feature = "blocking")]
fn make_serde_value(data: WebHookData) -> SerdeValue {
    let mut map = SerdeMap::new();
    if let Some(value1) = data.value1 {
        map.insert("value1".to_string(), SerdeValue::String(value1));
    }
    if let Some(value2) = data.value2 {
        map.insert("value2".to_string(), SerdeValue::String(value2));
    }
    if let Some(value3) = data.value3 {
        map.insert("value3".to_string(), SerdeValue::String(value3));
    }
    let value = SerdeValue::Object(map);
    value
}

#[inline]
#[cfg(feature = "non-blocking")]
fn nonblocking_make_serde_value(data: WebHookData) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    if let Some(value1) = data.value1 {
        map.insert("value1".to_string(), value1);
    }
    if let Some(value2) = data.value2 {
        map.insert("value2".to_string(), value2);
    }
    if let Some(value3) = data.value3 {
        map.insert("value3".to_string(), value3);
    }
    map
}
