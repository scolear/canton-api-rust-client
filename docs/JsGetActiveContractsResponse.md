# JsGetActiveContractsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_id** | Option<**String**> | The workflow ID used in command submission which corresponds to the contract_entry. Only set if the ``workflow_id`` for the command was set. Must be a valid LedgerString (as described in ``value.proto``).  Optional | [optional]
**contract_entry** | Option<[**models::JsContractEntry**](JsContractEntry.md)> |  | [optional]
**stream_continuation_token** | Option<**String**> | Opaque representation of a continuation token which can be used in the request to bypass the already processed part of the active contracts snapshot. Only populated for the streaming ``GetActiveContracts`` rpc call.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


