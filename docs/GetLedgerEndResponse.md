# GetLedgerEndResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**offset** | Option<**i64**> | It will always be a non-negative integer. If zero, the participant view of the ledger is empty. If positive, the absolute offset of the ledger as viewed by the participant.  Optional | [optional]
**synchronizer_times** | Option<[**Vec<models::SynchronizerTime>**](SynchronizerTime.md)> | Final record times observed by this participant for each of the requested synchronizers. If there is no connection to a sequencer, corresponding record time can be arbitrarily far back in the past.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


