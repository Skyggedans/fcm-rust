use serde_json::Value;

#[cfg(test)]
mod tests;

#[derive(Serialize, Debug, PartialEq)]
pub struct WebNotification<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct WebpushFcmOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<&'a str>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct WebpushConfig<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<WebNotification<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<WebpushFcmOptions<'a>>,
}

pub struct WebpushConfigBuilder<'a> {
    data: Option<Value>,
    headers: Option<Value>,
    notification: Option<WebNotification<'a>>,
    fcm_options: Option<WebpushFcmOptions<'a>>,
}

impl<'a> WebpushConfigBuilder<'a> {
    /// Get a new `ApnsBuilder` instance, with a title.
    pub fn new() -> WebpushConfigBuilder<'a> {
        WebpushConfigBuilder {
            data: None,
            headers: None,
            notification: None,
            fcm_options: None,
        }
    }

    pub fn data(&mut self, data: Value) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn headers(&mut self, headers: Value) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn notification(&mut self, notification: WebNotification<'a>) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    pub fn fcm_options(&mut self, fcm_options: WebpushFcmOptions<'a>) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }

    /// Complete the build and get a `ApnConfig` instance
    pub fn finalize(self) -> WebpushConfig<'a> {
        WebpushConfig {
            data: self.data,
            headers: self.headers,
            notification: self.notification,
            fcm_options: self.fcm_options,
        }
    }
}
