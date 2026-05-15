# SignedTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction** | **String** | The serialized TopologyTransaction  Required: must be non-empty | 
**signatures** | Option<[**Vec<models::Signature>**](Signature.md)> | Additional signatures for this transaction specifically Use for transactions that require additional signatures beyond the namespace key signatures e.g: PartyToParticipant must be signed by all registered keys  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


