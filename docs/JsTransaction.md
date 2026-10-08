# JsTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**update_id** | **String** | Assigned by the server. Useful for correlating logs. Must be a valid LedgerString (as described in ``value.proto``).  Required | 
**command_id** | Option<**String**> | The ID of the command which resulted in this transaction. Missing for everyone except the submitting party. Must be a valid LedgerString (as described in ``value.proto``).  Optional | [optional]
**workflow_id** | Option<**String**> | The workflow ID used in command submission. Must be a valid LedgerString (as described in ``value.proto``).  Optional | [optional]
**effective_at** | **String** | Ledger effective time.  Required | 
**events** | [**Vec<models::Event>**](Event.md) | The collection of events. Contains:  - ``CreatedEvent`` or ``ArchivedEvent`` in case of ACS_DELTA transaction shape - ``CreatedEvent`` or ``ExercisedEvent`` in case of LEDGER_EFFECTS transaction shape  Required: must be non-empty | 
**offset** | **i64** | The absolute offset. The details of this field are described in ``community/ledger-api/README.md``. It is a valid absolute offset (positive integer).  Required | 
**synchronizer_id** | **String** | A valid synchronizer id. Identifies the synchronizer that synchronized the transaction.  Required | 
**trace_context** | Option<[**models::TraceContext**](TraceContext.md)> |  | [optional]
**record_time** | **String** | The time at which the transaction was recorded. The record time refers to the synchronizer which synchronized the transaction.  Required | 
**external_transaction_hash** | Option<**String**> | Deprecated: please use ``transaction_hash`` which has the same value. For transaction externally signed, contains the external transaction hash signed by the external party. Can be used to correlate an external submission with a committed transaction.  Optional: can be empty | [optional]
**paid_traffic_cost** | Option<**i64**> | The traffic cost that this participant node paid for the confirmation request for this transaction.  Not set for transactions that were - initiated by another participant - initiated offline via the repair service - processed before the participant started serving traffic cost on the Ledger API - returned as part of a query filtering for a non submitting party  Optional | [optional]
**transaction_hash** | Option<**String**> | The transaction hash signed by the external party for prepared transactions that were submitted using `InteractiveSubmissionService.ExecuteSubmission` or one of its variants.  Use this to correlate prepared transactions with their execution result on the *same participant* that executed the transaction.  Not available for:  - transactions executed on Canton versions prior to 3.6 - transactions of internal parties submitted using `CommandSubmissionService.Submit` and its variants  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


