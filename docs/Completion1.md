# Completion1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**command_id** | **String** | The ID of the succeeded or failed command. Must be a valid LedgerString (as described in ``value.proto``).  Required | 
**status** | Option<[**models::JsStatus**](JsStatus.md)> |  | [optional]
**update_id** | Option<**String**> | The update_id of the transaction or reassignment that resulted from the command with command_id.  Only set for successfully executed commands. Must be a valid LedgerString (as described in ``value.proto``).  Optional | [optional]
**user_id** | **String** | The user-id that was used for the submission, as described in ``commands.proto``. Must be a valid UserIdString (as described in ``value.proto``).  Required | 
**act_as** | **Vec<String>** | The set of parties on whose behalf the commands were executed. Contains the ``act_as`` parties from ``commands.proto`` filtered to the requesting parties in CompletionStreamRequest. The order of the parties need not be the same as in the submission. Each element must be a valid PartyIdString (as described in ``value.proto``).  Required: must be non-empty | 
**submission_id** | Option<**String**> | The submission ID this completion refers to, as described in ``commands.proto``. Must be a valid LedgerString (as described in ``value.proto``).  Optional | [optional]
**deduplication_period** | Option<[**models::DeduplicationPeriod1**](DeduplicationPeriod1.md)> |  | [optional]
**trace_context** | Option<[**models::TraceContext**](TraceContext.md)> |  | [optional]
**offset** | **i64** | May be used in a subsequent CompletionStreamRequest to resume the consumption of this stream at a later time. Must be a valid absolute offset (positive integer).  Required | 
**synchronizer_time** | [**models::SynchronizerTime**](SynchronizerTime.md) |  | 
**paid_traffic_cost** | Option<**i64**> | The traffic cost paid by this participant node for the confirmation request for the submitted command.  Commands whose execution is rejected before their corresponding confirmation request is ordered by the synchronizer will report a paid traffic cost of zero. If a confirmation request is ordered for a command, but the request fails (e.g., due to contention with a concurrent contract archival), the traffic cost is paid and reported on the failed completion for the request.  If you want to correlate the traffic cost of a successful completion with the transaction that resulted from the command, you can use the ``offset`` field to retrieve the transaction using ``UpdateService.GetUpdateByOffset`` on the same participant node; or alternatively use the ``update_id`` field to retrieve the transaction using ``UpdateService.GetUpdateById`` on any participant node that sees the transaction.  Note: for completions processed before the participant started serving traffic cost on the Ledger API, this field will be set to zero. Additionally, the total cost incurred by the submitting node for the submission of the transaction may be greater than the reported cost, for example if retries were issued due to failed submissions to the synchronizer. The cost reported here is the one paid for ordering the confirmation request.  Optional | [optional]
**transaction_hash** | Option<**String**> | The transaction hash signed by the external party for prepared transactions that were submitted using `InteractiveSubmissionService.ExecuteSubmission` or one of its variants.  Use this to correlate prepared transactions with their execution result on the *same participant* that executed the transaction.  Not available for:  - transactions executed on Canton versions prior to 3.5 - transactions of internal parties submitted using `CommandSubmissionService.Submit` and its variants  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


