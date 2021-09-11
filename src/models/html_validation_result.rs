/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HtmlValidationResult : HTML Validation Results



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HtmlValidationResult {
    /// Optional errors resulting from HTML validation
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::ValidationMessage>>,
    /// Is HTML validation result valid
    #[serde(rename = "isValid", skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    /// Optional warnings resulting from HTML validation
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::ValidationMessage>>,
}

impl HtmlValidationResult {
    /// HTML Validation Results
    pub fn new() -> HtmlValidationResult {
        HtmlValidationResult {
            errors: None,
            is_valid: None,
            warnings: None,
        }
    }
}

