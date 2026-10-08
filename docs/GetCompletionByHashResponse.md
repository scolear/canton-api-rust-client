# GetCompletionByHashResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_completion** | Option<[**models::Completion1**](Completion1.md)> |  | [optional]
**last_rejected_completions** | Option<[**Vec<models::Completion1>**](Completion1.md)> | Recent rejected completions for the given transaction hash, in descending offset order. The server may truncate this list if there are too many entries. Use ``submission_id`` to correlate with a specific command submission. Only completions beyond the current pruning offset are visible.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


