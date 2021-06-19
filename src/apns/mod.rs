use std::borrow::Cow;
use serde_json::Value;

#[cfg(test)]
mod tests;

#[derive(Serialize, Debug, PartialEq)]
pub struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct ApnsFcmOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<&'a str>,
}

/// This struct represents a FCM Android notification. Use the
/// corresponding `AndroidNotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
pub struct ApnsConfig<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Payload<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<ApnsFcmOptions<'a>>,
}

pub struct ApnsConfigBuilder<'a> {
    headers: Option<Value>,
    payload: Option<Payload<'a>>,
    fcm_options: Option<ApnsFcmOptions<'a>>,
}

impl<'a> ApnsConfigBuilder<'a> {
    /// Get a new `ApnsBuilder` instance, with a title.
    pub fn new() -> ApnsConfigBuilder<'a> {
        ApnsConfigBuilder {
            headers: None,
            payload: None,
            fcm_options: None,
        }
    }

    pub fn headers(&mut self, headers: Value) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn payload(&mut self, payload: Payload<'a>) -> &mut Self {
        self.payload = Some(payload);
        self
    }

    pub fn fcm_options(&mut self, fcm_options: ApnsFcmOptions<'a>) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }

    /// Complete the build and get a `ApnsConfig` instance
    pub fn finalize(self) -> ApnsConfig<'a> {
        ApnsConfig {
            headers: self.headers,
            payload: self.payload,
            fcm_options: self.fcm_options,
        }
    }
}
