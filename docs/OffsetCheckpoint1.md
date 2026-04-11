# OffsetCheckpoint1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**offset** | **i64** | The participant's offset, the details of the offset field are described in ``community/ledger-api/README.md``. Must be a valid absolute offset (positive integer).  Required | 
**synchronizer_times** | Option<[**Vec<models::SynchronizerTime>**](SynchronizerTime.md)> | The times associated with each synchronizer at this offset.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


