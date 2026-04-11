# GenerateExternalPartyTopologyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**party_id** | **String** | The generated party id  Required | 
**public_key_fingerprint** | **String** | The fingerprint of the supplied public key  Required | 
**topology_transactions** | **Vec<String>** | The serialized topology transactions which need to be signed and submitted as part of the allocate party process Note that the serialization includes the versioning information. Therefore, the transaction here is serialized as an `UntypedVersionedMessage` which in turn contains the serialized `TopologyTransaction` in the version supported by the synchronizer.  Required: must be non-empty | 
**multi_hash** | **String** | the multi-hash which may be signed instead of each individual transaction  Required: must be non-empty | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


