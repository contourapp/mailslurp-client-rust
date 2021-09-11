/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GravatarUrl {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl GravatarUrl {
    pub fn new(hash: String, url: String) -> GravatarUrl {
        GravatarUrl {
            hash,
            url,
        }
    }
}

