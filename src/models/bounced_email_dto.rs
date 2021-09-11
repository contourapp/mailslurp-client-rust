/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BouncedEmailDto : Bounced email



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BouncedEmailDto {
    #[serde(rename = "bounceMta", skip_serializing_if = "Option::is_none")]
    pub bounce_mta: Option<String>,
    #[serde(rename = "bounceRecipients", skip_serializing_if = "Option::is_none")]
    pub bounce_recipients: Option<Vec<String>>,
    #[serde(rename = "bounceSubType", skip_serializing_if = "Option::is_none")]
    pub bounce_sub_type: Option<String>,
    #[serde(rename = "bounceType", skip_serializing_if = "Option::is_none")]
    pub bounce_type: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "notificationType")]
    pub notification_type: String,
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "sentToRecipients", skip_serializing_if = "Option::is_none")]
    pub sent_to_recipients: Option<Vec<String>>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl BouncedEmailDto {
    /// Bounced email
    pub fn new(created_at: String, notification_type: String, sender: String, user_id: String) -> BouncedEmailDto {
        BouncedEmailDto {
            bounce_mta: None,
            bounce_recipients: None,
            bounce_sub_type: None,
            bounce_type: None,
            created_at,
            id: None,
            notification_type,
            sender,
            sent_to_recipients: None,
            user_id,
        }
    }
}


