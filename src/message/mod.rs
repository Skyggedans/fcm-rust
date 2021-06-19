use std::borrow::Cow;

use erased_serde::Serialize;
use serde_json::{self, Value};

use crate::notification::Notification;
use crate::android::AndroidConfig;

#[cfg(test)]
mod tests;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Normal,
    High,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPriority {
    Unspecified,
    Min,
    Low,
    Default,
    High,
    Max,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct FcmOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<&'a str>,
}


#[derive(Serialize, Debug, PartialEq)]
pub struct MessageBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<AndroidConfig<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    dry_run: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<FcmOptions<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Notification<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,

    //webpush
    //apns
}

/// Represents a FCM message. Construct the FCM message
/// using various utility methods and finally send it.
/// # Examples:
/// ```rust
/// use fcm::MessageBuilder;
///
/// let mut builder = MessageBuilder::new("<FCM access token>", "<device token>");
/// builder.dry_run(true);
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct Message<'a> {
    pub access_token: &'a str,
    pub body: MessageBody<'a>,
}

///
/// A builder to get a `Message` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::MessageBuilder;
///
/// let mut builder = MessageBuilder::new("<FCM access token>", "<device token>");
/// builder.dry_run(true);
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct MessageBuilder<'a> {
    access_token: &'a str,
    android: Option<AndroidConfig<'a>>,
    condition: Option<&'a str>,
    data: Option<Value>,
    dry_run: Option<bool>,
    fcm_options: Option<FcmOptions<'a>>,
    name: Option<&'a str>,
    notification: Option<Notification<'a>>,
    token: Option<&'a str>,
    topic: Option<&'a str>,
}

impl<'a> MessageBuilder<'a> {
    /// Get a new instance of Message. You need to supply to.
    pub fn new(access_token: &'a str, token: &'a str) -> Self {
        MessageBuilder {
            access_token,
            android: None,
            condition: None,
            data: None,
            dry_run: None,
            fcm_options: None,
            name: None,
            notification: None,
            token: Some(token),
            topic: None,
        }
    }

    /// Android specific options for messages sent through FCM connection server.
    pub fn android(&mut self, android: AndroidConfig<'a>) -> &mut Self
    {
        self.android = Some(android);
        self
    }

    /// Condition to send a message to, e.g. "'foo' in topics && 'bar' in topics"..
    pub fn condition(&mut self, condition: &'a str) -> &mut Self {
        self.condition = Some(condition);
        self
    }

    /// Use this to add custom key-value pairs to the message. This data
    /// must be handled appropriately on the client end. The data can be
    /// anything that Serde can serialize to JSON.
    ///
    /// # Examples:
    /// ```rust
    /// use fcm::MessageBuilder;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("message", "Howdy!");
    ///
    /// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
    /// builder.data(&map);
    /// let message = builder.finalize();
    /// ```
    pub fn data(&mut self, data: &dyn Serialize) -> Result<&mut Self, serde_json::Error> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    /// When set to `true`, allows you to test FCM without actually sending the message.
    pub fn dry_run(&mut self, dry_run: bool) -> &mut Self {
        self.dry_run = Some(dry_run);
        self
    }

    pub fn fcm_options(&mut self, fcm_options: FcmOptions<'a>) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }

    /// The identifier of the message sent, in the format of projects/*/messages/{message_id}.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    /// Use this to set a `Notification` for the message.
    /// # Examples:
    /// ```rust
    /// use fcm::{MessageBuilder, NotificationBuilder};
    ///
    /// let mut builder = NotificationBuilder::new();
    /// builder.title("Hey!");
    /// builder.body("Do you want to catch up later?");
    /// let notification = builder.finalize();
    ///
    /// let mut builder = MessageBuilder::new("<FCM API access token>", "<registration id>");
    /// builder.notification(notification);
    /// let message = builder.finalize();
    /// ```
    pub fn notification(&mut self, notification: Notification<'a>) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    pub fn token(&mut self, token: &'a str) -> &mut Self {
        self.token = Some(token);
        self
    }

    pub fn topic(&mut self, topic: &'a str) -> &mut Self {
        self.topic = Some(topic);
        self
    }

    /// Complete the build and get a `Message` instance
    pub fn finalize(self) -> Message<'a> {
        Message {
            access_token: self.access_token,
            body: MessageBody {
                android: self.android,
                condition: self.condition,
                data: self.data.clone(),
                dry_run: self.dry_run,
                fcm_options: self.fcm_options,
                name: self.name,
                notification: self.notification,
                token: self.token,
                topic: self.topic,
            },
        }
    }
}
