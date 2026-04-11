# GetContractRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract. Must be a valid LedgerString (as described in ``value.proto``).  Required | 
**querying_parties** | Option<**Vec<String>**> | The list of querying parties The stakeholders of the referenced contract must have an intersection with any of these parties to return the result. If no querying_parties specified, all possible contracts could be returned.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


