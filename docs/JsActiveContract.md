# JsActiveContract

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_event** | [**models::CreatedEvent**](CreatedEvent.md) |  | 
**synchronizer_id** | **String** | A valid synchronizer id  Required | 
**reassignment_counter** | **i64** | The reassignment counter of this event.  Reassignment counters strictly increase with each unassign command for the same contract. Creation of the contract corresponds to a reassignment_counter equal to zero. This field will be the reassignment_counter of the latest observable activation event on this synchronizer, which is before the active_at_offset.  Required | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


