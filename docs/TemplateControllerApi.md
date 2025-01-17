# \TemplateControllerApi

All URIs are relative to *https://api.mailslurp.com*

| Method                                                           | HTTP request                       | Description                           |
| ---------------------------------------------------------------- | ---------------------------------- | ------------------------------------- |
| [**create_template**](TemplateControllerApi#create_template)     | **post** /templates                | Create a Template                     |
| [**delete_template**](TemplateControllerApi#delete_template)     | **delete** /templates/{templateId} | Delete Template                       |
| [**get_all_templates**](TemplateControllerApi#get_all_templates) | **get** /templates/paginated       | Get all Templates in paginated format |
| [**get_template**](TemplateControllerApi#get_template)           | **get** /templates/{templateId}    | Get Template                          |
| [**get_templates**](TemplateControllerApi#get_templates)         | **get** /templates                 | Get all Templates                     |
| [**update_template**](TemplateControllerApi#update_template)     | **put** /templates/{templateId}    | Update a Template                     |

## create_template

> crate::models::TemplateDto create_template(create_template_options)
> Create a Template

### Parameters

| Name                        | Type                                               | Description           | Required   | Notes |
| --------------------------- | -------------------------------------------------- | --------------------- | ---------- | ----- |
| **create_template_options** | [**CreateTemplateOptions**](CreateTemplateOptions) | createTemplateOptions | [required] |

### Return type

[**crate::models::TemplateDto**](TemplateDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## delete_template

> delete_template(template_id)
> Delete Template

### Parameters

| Name            | Type           | Description | Required   | Notes |
| --------------- | -------------- | ----------- | ---------- | ----- |
| **template_id** | [**String**]() | templateId  | [required] |

### Return type

(empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_all_templates

> crate::models::PageTemplateProjection get_all_templates(before, page, since, size, sort)
> Get all Templates in paginated format

### Parameters

| Name       | Type               | Description                                     | Required | Notes            |
| ---------- | ------------------ | ----------------------------------------------- | -------- | ---------------- |
| **before** | Option<**String**> | Filter by created at before the given timestamp |          |
| **page**   | Option<**i32**>    | Optional page index in list pagination          |          | [default to 0]   |
| **since**  | Option<**String**> | Filter by created at after the given timestamp  |          |
| **size**   | Option<**i32**>    | Optional page size in list pagination           |          | [default to 20]  |
| **sort**   | Option<**String**> | Optional createdAt sort direction ASC or DESC   |          | [default to ASC] |

### Return type

[**crate::models::PageTemplateProjection**](PageTemplateProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_template

> crate::models::TemplateDto get_template(template_id)
> Get Template

### Parameters

| Name            | Type           | Description | Required   | Notes |
| --------------- | -------------- | ----------- | ---------- | ----- |
| **template_id** | [**String**]() | templateId  | [required] |

### Return type

[**crate::models::TemplateDto**](TemplateDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## get_templates

> Vec<crate::models::TemplateProjection> get_templates()
> Get all Templates

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::TemplateProjection>**](TemplateProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

## update_template

> crate::models::TemplateDto update_template(template_id, create_template_options)
> Update a Template

### Parameters

| Name                        | Type                                               | Description           | Required   | Notes |
| --------------------------- | -------------------------------------------------- | --------------------- | ---------- | ----- |
| **template_id**             | [**String**]()                                     | templateId            | [required] |
| **create_template_options** | [**CreateTemplateOptions**](CreateTemplateOptions) | createTemplateOptions | [required] |

### Return type

[**crate::models::TemplateDto**](TemplateDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)
