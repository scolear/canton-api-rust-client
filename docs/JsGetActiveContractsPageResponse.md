# JsGetActiveContractsPageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_contracts** | [**Vec<models::JsGetActiveContractsResponse>**](JsGetActiveContractsResponse.md) | The collection of active contracts for this page response.  Required: must be non-empty | 
**active_at_offset** | **i64** | The active_at_offset which was specified in the request, or the calculated active_at_offset from the actual ledger end from at the evaluation of the request.  Required | 
**next_page_token** | Option<**String**> | If not present this is the last page. If present, this token must be used to get the next page.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


