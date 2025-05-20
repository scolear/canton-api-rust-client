# ExerciseCommand

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_id** | **String** | The template of contract the client wants to exercise. Both package-name and package-id reference identifier formats for the template-id are supported. Note: The package-id reference identifier format is deprecated. We plan to end support for this format in version 3.4.  Required | 
**contract_id** | **String** | The ID of the contract the client wants to exercise upon. Must be a valid LedgerString (as described in ``value.proto``). Required | 
**choice** | **String** | The name of the choice the client wants to exercise. Must be a valid NameString (as described in ``value.proto``) Required | 
**choice_argument** | Option<[**serde_json::Value**](.md)> | The argument for this choice. Required | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


