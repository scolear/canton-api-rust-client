# JsAssignedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | **String** | The ID of the source synchronizer. Must be a valid synchronizer id.  Required | 
**target** | **String** | The ID of the target synchronizer. Must be a valid synchronizer id.  Required | 
**reassignment_id** | **String** | The ID from the unassigned event. For correlation capabilities. Must be a valid LedgerString (as described in ``value.proto``).  Required | 
**submitter** | Option<**String**> | Party on whose behalf the assign command was executed. Empty if the assignment happened offline via the repair service. Must be a valid PartyIdString (as described in ``value.proto``).  Optional | [optional]
**reassignment_counter** | **i64** | The reassignment counter associated with this assignment on the target synchronizer.  Reassignment counters strictly increase with each unassign command for the same contract. Creation of the contract corresponds to reassignment_counter equal to zero.  Required | 
**created_event** | [**models::CreatedEvent**](CreatedEvent.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


