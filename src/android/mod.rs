use std::borrow::Cow;
use crate::{NotificationPriority, Notification};
use serde_json::Value;

#[cfg(test)]
mod tests;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AndroidMessagePriority {
    Normal,
    High,
}

/// This struct represents a FCM Android notification. Use the
/// corresponding `AndroidNotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
pub struct AndroidFcmOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<&'a str>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Color {
    #[serde(skip_serializing_if = "Option::is_none")]
    red: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    green: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    blue: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    alpha: Option<f32>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct LightSettings<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    light_on_duration: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    light_off_duration: Option<&'a str>,
}

/// This struct represents a FCM Android notification. Use the
/// corresponding `AndroidNotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
pub struct AndroidConfig<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    direct_boot_ok: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<AndroidFcmOptions<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<AndroidNotification<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<AndroidMessagePriority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<&'a str>,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Unspecified,
    Public,
    Private,
    Secret,
}

/// This struct represents a FCM Android notification. Use the
/// corresponding `AndroidNotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
pub struct AndroidNotification<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    badge: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,1

    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_args: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_sound: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_vibrate_timings: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_light_settings: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    click_action: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_time: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<&'a str>,1

    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,1

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    light_settings: Option<LightSettings<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification_priority: Option<NotificationPriority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ticker: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,1

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_args: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vibrate_timings: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,
}

/// A builder to get an `AndroidNotification` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::AndroidNotificationBuilder;
///
/// let mut builder = AndroidNotificationBuilder::new();
//  builder.title("Australia vs New Zealand");
/// builder.body("3 runs to win in 1 ball");
/// let notification = builder.finalize();
/// ```
pub struct AndroidNotificationBuilder<'a> {
    title: Option<&'a str>,
    body: Option<&'a str>,
    icon: Option<&'a str>,
    sound: Option<&'a str>,
    badge: Option<&'a str>,
    tag: Option<&'a str>,
    color: Option<&'a str>,
    click_action: Option<&'a str>,
    body_loc_key: Option<&'a str>,
    body_loc_args: Option<Vec<Cow<'a, str>>>,
    title_loc_key: Option<&'a str>,
    title_loc_args: Option<Vec<Cow<'a, str>>>,
}

impl<'a> AndroidNotificationBuilder<'a> {
    /// Get a new `AndroidNotificationBuilder` instance, with a title.
    pub fn new() -> AndroidNotificationBuilder<'a> {
        AndroidNotificationBuilder {
            title: None,
            body: None,
            icon: None,
            sound: None,
            badge: None,
            tag: None,
            color: None,
            click_action: None,
            body_loc_key: None,
            body_loc_args: None,
            title_loc_key: None,
            title_loc_args: None,
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

    /// Set the notification icon.
    pub fn icon(&mut self, icon: &'a str) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    /// Set the sound to be played
    pub fn sound(&mut self, sound: &'a str) -> &mut Self {
        self.sound = Some(sound);
        self
    }

    /// Set the badge for iOS notifications
    pub fn badge(&mut self, badge: &'a str) -> &mut Self {
        self.badge = Some(badge);
        self
    }

    /// Tagging a notification allows you to replace existing notifications
    /// with the same tag with this new notification
    pub fn tag(&mut self, tag: &'a str) -> &mut Self {
        self.tag = Some(tag);
        self
    }

    /// The color of the icon, in #rrggbb format
    pub fn color(&mut self, color: &'a str) -> &mut Self {
        self.color = Some(color);
        self
    }

    /// What happens when the user clicks on the notification. Refer to
    /// https://developers.google.com/cloud-messaging/http-server-ref#table2 for
    /// details.
    pub fn click_action(&mut self, click_action: &'a str) -> &mut Self {
        self.click_action = Some(click_action);
        self
    }

    /// Set the body key string for localization
    pub fn body_loc_key(&mut self, body_loc_key: &'a str) -> &mut Self {
        self.body_loc_key = Some(body_loc_key);
        self
    }

    /// String value to replace format specifiers in the body string.
    pub fn body_loc_args<S>(&mut self, body_loc_args: &'a [S]) -> &mut Self
        where
            S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = body_loc_args.iter().map(|a| a.as_ref().into()).collect();

        self.body_loc_args = Some(converted);
        self
    }

    /// Set the title key string for localization
    pub fn title_loc_key(&mut self, title_loc_key: &'a str) -> &mut Self {
        self.title_loc_key = Some(title_loc_key);
        self
    }

    /// String value to replace format specifiers in the title string.
    pub fn title_loc_args<S>(&mut self, title_loc_args: &'a [S]) -> &mut Self
        where
            S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = title_loc_args.iter().map(|a| a.as_ref().into()).collect();

        self.title_loc_args = Some(converted);
        self
    }

    /// Complete the build and get a `AndroidNotification` instance
    pub fn finalize(self) -> AndroidNotification<'a> {
        AndroidNotification {
            title: self.title,
            body: self.body,
            image: self.image,
        }
    }
}
