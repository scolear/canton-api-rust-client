# JsGetUpdatesPageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updates** | Option<[**Vec<models::JsGetUpdateResponse>**](JsGetUpdateResponse.md)> | The first max_page_size updates that match the filter in the request. In case descending_order was selected, the order of the updates is in reversed offset order.  Optional: can be empty | [optional]
**lowest_page_offset_exclusive** | **i64** | Represents the lower bound of this page.  Required | 
**highest_page_offset_inclusive** | **i64** | Represents the upper bound of the page.  Required | 
**next_page_token** | Option<**String**> | If the value is not populated, this is the last page. If the value is populated, this token can be used to get the next page. If the original ``GetFirstUpdatePageRequest`` end_offset_inclusive was not specified and the request uses ascending order, then this token will always be populated, so you can use it to \"tail\" the ledger by repeatedly polling with the new page token returned.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


