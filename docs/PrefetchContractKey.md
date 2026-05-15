# PrefetchContractKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_id** | **String** | The template of contract the client wants to prefetch. Both package-name and package-id reference identifier formats for the template-id are supported. Note: The package-id reference identifier format is deprecated. We plan to end support for this format in version 3.4.  Required | 
**contract_key** | Option<**serde_json::Value**> | The key of the contract the client wants to prefetch.  Required | 
**limit** | Option<**i32**> | The number of contracts to prefetch for this key, if available. This is in addition to disclosed contracts. - for backward compatibility reason, absence is interpreted as 1 - 0 is forbidden - capped at 2^31 - 1. The system may impose further limits.  Optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


