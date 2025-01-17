# \SentEmailsControllerApi

All URIs are relative to *https://api.mailslurp.com*

| Method                                                                                       | HTTP request                       | Description                                                |
| -------------------------------------------------------------------------------------------- | ---------------------------------- | ---------------------------------------------------------- |
| [**get_all_sent_tracking_pixels**](SentEmailsControllerApi#get_all_sent_tracking_pixels)     | **get** /sent/tracking-pixels      | Get all sent email tracking pixels in paginated form       |
| [**get_sent_email**](SentEmailsControllerApi#get_sent_email)                                 | **get** /sent/{id}                 | Get sent email receipt                                     |
| [**get_sent_email_html_content**](SentEmailsControllerApi#get_sent_email_html_content)       | **get** /sent/{id}/html            | Get sent email HTML content                                |
| [**get_sent_email_tracking_pixels**](SentEmailsControllerApi#get_sent_email_tracking_pixels) | **get** /sent/{id}/tracking-pixels | Get all tracking pixels for a sent email in paginated form |
| [**get_sent_emails**](SentEmailsControllerApi#get_sent_emails)                               | **get** /sent                      | Get all sent emails in paginated form                      |
| [**get_sent_organization_emails**](SentEmailsControllerApi#get_sent_organization_emails)     | **get** /sent/organization         | Get all sent organization emails in paginated form         |

## get_all_sent_tracking_pixels

> crate::models::PageTrackingPixelProjection get_all_sent_tracking_pixels(before, page, search_filter, since, size, sort)
> Get all sent email tracking pixels in paginated form

### Parameters

| Name              | Type               | Description                                                      | Required | Notes            |
| ----------------- | ------------------ | ---------------------------------------------------------------- | -------- | ---------------- |
| **before**        | Option<**String**> | Filter by created at before the given timestamp                  |          |
| **page**          | Option<**i32**>    | Optional page index in sent email tracking pixel list pagination |          | [default to 0]   |
| **search_filter** | Option<**String**> | Optional search filter                                           |          |
| **since**         | Option<**String**> | Filter by created at after the given timestamp                   |          |
| **size**          | Option<**i32**>    | Optional page size in sent email tracking pixel list pagination  |          | [default to 20]  |
| **sort**          | Option<**String**> | Optional createdAt sort direction ASC or DESC                    |          | [default to ASC] |

### Return type

[**crate::models::PageTrackingPixelProjection**](PageTrackingPixelProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_sent_email

> crate::models::SentEmailDto get_sent_email(id)
> Get sent email receipt

### Parameters

| Name   | Type           | Description | Required   | Notes |
| ------ | -------------- | ----------- | ---------- | ----- |
| **id** | [**String**]() | id          | [required] |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_sent_email_html_content

> String get_sent_email_html_content(id)
> Get sent email HTML content

### Parameters

| Name   | Type           | Description | Required   | Notes |
| ------ | -------------- | ----------- | ---------- | ----- |
| **id** | [**String**]() | id          | [required] |

### Return type

**String**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_sent_email_tracking_pixels

> crate::models::PageTrackingPixelProjection get_sent_email_tracking_pixels(id, before, page, search_filter, since, size, sort)
> Get all tracking pixels for a sent email in paginated form

### Parameters

| Name              | Type               | Description                                                      | Required   | Notes            |
| ----------------- | ------------------ | ---------------------------------------------------------------- | ---------- | ---------------- |
| **id**            | [**String**]()     | id                                                               | [required] |
| **before**        | Option<**String**> | Filter by created at before the given timestamp                  |            |
| **page**          | Option<**i32**>    | Optional page index in sent email tracking pixel list pagination |            | [default to 0]   |
| **search_filter** | Option<**String**> | Optional search filter                                           |            |
| **since**         | Option<**String**> | Filter by created at after the given timestamp                   |            |
| **size**          | Option<**i32**>    | Optional page size in sent email tracking pixel list pagination  |            | [default to 20]  |
| **sort**          | Option<**String**> | Optional createdAt sort direction ASC or DESC                    |            | [default to ASC] |

### Return type

[**crate::models::PageTrackingPixelProjection**](PageTrackingPixelProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_sent_emails

> crate::models::PageSentEmailProjection get_sent_emails(before, inbox_id, page, search_filter, since, size, sort)
> Get all sent emails in paginated form

### Parameters

| Name              | Type                   | Description                                             | Required | Notes            |
| ----------------- | ---------------------- | ------------------------------------------------------- | -------- | ---------------- |
| **before**        | Option<**String**>     | Filter by created at before the given timestamp         |          |
| **inbox_id**      | Option<[**String**]()> | Optional inboxId to filter sender of sent emails by     |          |
| **page**          | Option<**i32**>        | Optional page index in inbox sent email list pagination |          | [default to 0]   |
| **search_filter** | Option<**String**>     | Optional search filter                                  |          |
| **since**         | Option<**String**>     | Filter by created at after the given timestamp          |          |
| **size**          | Option<**i32**>        | Optional page size in inbox sent email list pagination  |          | [default to 20]  |
| **sort**          | Option<**String**>     | Optional createdAt sort direction ASC or DESC           |          | [default to ASC] |

### Return type

[**crate::models::PageSentEmailProjection**](PageSentEmailProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_sent_organization_emails

> crate::models::PageSentEmailProjection get_sent_organization_emails(before, inbox_id, page, search_filter, since, size, sort)
> Get all sent organization emails in paginated form

### Parameters

| Name              | Type                   | Description                                         | Required | Notes            |
| ----------------- | ---------------------- | --------------------------------------------------- | -------- | ---------------- |
| **before**        | Option<**String**>     | Filter by created at before the given timestamp     |          |
| **inbox_id**      | Option<[**String**]()> | Optional inboxId to filter sender of sent emails by |          |
| **page**          | Option<**i32**>        | Optional page index in sent email list pagination   |          | [default to 0]   |
| **search_filter** | Option<**String**>     | Optional search filter                              |          |
| **since**         | Option<**String**>     | Filter by created at after the given timestamp      |          |
| **size**          | Option<**i32**>        | Optional page size in sent email list pagination    |          | [default to 20]  |
| **sort**          | Option<**String**>     | Optional createdAt sort direction ASC or DESC       |          | [default to ASC] |

### Return type

[**crate::models::PageSentEmailProjection**](PageSentEmailProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)
