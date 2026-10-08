# UpdateAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | The account ID whose account to update.  NOTE: Currently, the account ID is tied to and identified by a party ID. This constraint is expected to be removed in a future release, allowing for user-defined account IDs that are not necessarily tied to a specific party.  Required | 
**balance_delta** | Option<**i64**> | Balance DELTA to apply to the current balance. Negative values will decrease the balance, positive values will increase the balance. If unset, the balance will not be updated  Note that setting a negative value may result in a negative balance if the delta is larger than the current balance. Optional | [optional]
**deduplication_id** | **String** | Identifier of this logical update, used to de-duplicate. The caller MUST use a distinct id for every distinct update, and MUST reuse the same id when retrying an update that failed after a retryable error. An already processed id is ignored, while a fresh id applies the delta again.  Required | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


