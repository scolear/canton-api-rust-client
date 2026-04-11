# JsReassignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**update_id** | **String** | Assigned by the server. Useful for correlating logs. Must be a valid LedgerString (as described in ``value.proto``).  Required | 
**command_id** | Option<**String**> | The ID of the command which resulted in this reassignment. Missing for everyone except the submitting party on the submitting participant. Must be a valid LedgerString (as described in ``value.proto``).  Optional | [optional]
**workflow_id** | Option<**String**> | The workflow ID used in reassignment command submission. Only set if the ``workflow_id`` for the command was set. Must be a valid LedgerString (as described in ``value.proto``).  Optional | [optional]
**offset** | **i64** | The participant's offset. The details of this field are described in ``community/ledger-api/README.md``. Must be a valid absolute offset (positive integer).  Required | 
**events** | [**Vec<models::JsReassignmentEvent>**](JsReassignmentEvent.md) | The collection of reassignment events.  Required: must be non-empty | 
**trace_context** | Option<[**models::TraceContext**](TraceContext.md)> |  | [optional]
**record_time** | **String** | The time at which the reassignment was recorded. The record time refers to the source/target synchronizer for an unassign/assign event respectively.  Required | 
**synchronizer_id** | **String** | A valid synchronizer id. Identifies the synchronizer that synchronized this Reassignment.  Required | 
**paid_traffic_cost** | Option<**i64**> | The traffic cost that this participant node paid for the corresponding (un)assignment request.  Not set for transactions that were - initiated by another participant - initiated offline via the repair service - processed before the participant started serving traffic cost on the Ledger API - returned as part of a query filtering for a non submitting party  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


