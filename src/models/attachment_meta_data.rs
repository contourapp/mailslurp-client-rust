/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttachmentMetaData : Meta data associated with an attachment. Attachments are stored as byte blobs so the meta data is stored separately.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentMetaData {
    /// Size of attachment in bytes
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// Content type of attachment such as `image/png`
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// ID of attachment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of attachment if given
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AttachmentMetaData {
    /// Meta data associated with an attachment. Attachments are stored as byte blobs so the meta data is stored separately.
    pub fn new() -> AttachmentMetaData {
        AttachmentMetaData {
            content_length: None,
            content_type: None,
            id: None,
            name: None,
        }
    }
}

