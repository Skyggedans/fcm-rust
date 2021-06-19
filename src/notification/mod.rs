use std::borrow::Cow;

#[cfg(test)]
mod tests;

/// This struct represents a FCM notification. Use the
/// corresponding `NotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
pub struct Notification<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
}

/// A builder to get a `Notification` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::NotificationBuilder;
///
/// let mut builder = NotificationBuilder::new();
//  builder.title("Australia vs New Zealand");
/// builder.body("3 runs to win in 1 ball");
/// let notification = builder.finalize();
/// ```
pub struct NotificationBuilder<'a> {
    title: Option<&'a str>,
    body: Option<&'a str>,
    image: Option<&'a str>,
}

impl<'a> NotificationBuilder<'a> {
    /// Get a new `NotificationBuilder` instance, with a title.
    pub fn new() -> NotificationBuilder<'a> {
        NotificationBuilder {
            title: None,
            body: None,
            image: None,
        }
    }

    // Set the title of the notification
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    /// Set the body of the notification
    pub fn body(&mut self, body: &'a str) -> &mut Self {
        self.body = Some(body);
        self
    }

    /// Set the notification image.
    pub fn image(&mut self, icon: &'a str) -> &mut Self {
        self.image = Some(icon);
        self
    }

    /// Complete the build and get a `Notification` instance
    pub fn finalize(self) -> Notification<'a> {
        Notification {
            title: self.title,
            body: self.body,
            image: self.icon,
        }
    }
}
