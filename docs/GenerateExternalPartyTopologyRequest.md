# GenerateExternalPartyTopologyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**synchronizer** | **String** | Synchronizer-id for which we are building this request. TODO(#27670) support synchronizer aliases  Required | 
**party_hint** | **String** | The actual party id will be constructed from this hint and a fingerprint of the public key  Required | 
**public_key** | [**models::SigningPublicKey**](SigningPublicKey.md) |  | 
**local_participant_observation_only** | Option<**bool**> | If true, then the local participant will only be observing, not confirming. Default false.  Optional | [optional]
**other_confirming_participant_uids** | Option<**Vec<String>**> | Other participant ids which should be confirming for this party  Optional: can be empty | [optional]
**confirmation_threshold** | Option<**i32**> | Confirmation threshold >= 1 for the party. Defaults to all available confirmers (or if set to 0).  Optional | [optional]
**observing_participant_uids** | Option<**Vec<String>**> | Other observing participant ids for this party  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


