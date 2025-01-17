# WebhookNewEmailPayload

## Properties

| Name                      | Type                                                                     | Description                                                                                                | Notes      |
| ------------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- | ---------- |
| **attachment_meta_datas** | Option<[**Vec<crate::models::AttachmentMetaData>**](AttachmentMetaData)> | List of attachment meta data objects if attachments present                                                | [optional] |
| **bcc**                   | Option<**Vec<String>**>                                                  | List of `BCC` recipients email addresses that the email was addressed to. See recipients object for names. | [optional] |
| **cc**                    | Option<**Vec<String>**>                                                  | List of `CC` recipients email addresses that the email was addressed to. See recipients object for names.  | [optional] |
| **created_at**            | Option<**String**>                                                       | Date time of event creation                                                                                | [optional] |
| **email_id**              | Option<**String**>                                                       | ID of the email that was received. Use this ID for fetching the email with the `EmailController`.          | [optional] |
| **event_name**            | Option<**String**>                                                       | Name of the event type webhook is being triggered for.                                                     | [optional] |
| **from**                  | Option<**String**>                                                       | Who the email was sent from. An email address - see fromName for the sender name.                          | [optional] |
| **inbox_id**              | Option<**String**>                                                       | Id of the inbox that received an email                                                                     | [optional] |
| **message_id**            | Option<**String**>                                                       | Idempotent message ID. Store this ID locally or in a database to prevent message duplication.              | [optional] |
| **subject**               | Option<**String**>                                                       | The subject line of the email message as specified by SMTP subject header                                  | [optional] |
| **to**                    | Option<**Vec<String>**>                                                  | List of `To` recipient email addresses that the email was addressed to. See recipients object for names.   | [optional] |
| **webhook_id**            | Option<**String**>                                                       | ID of webhook entity being triggered                                                                       | [optional] |
| **webhook_name**          | Option<**String**>                                                       | Name of the webhook being triggered                                                                        | [optional] |

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)
