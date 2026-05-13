# AllocateExternalPartyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**synchronizer** | **String** | TODO(#27670) support synchronizer aliases Synchronizer ID on which to onboard the party  Required | 
**onboarding_transactions** | [**Vec<models::SignedTransaction>**](SignedTransaction.md) | TopologyTransactions to onboard the external party Can contain: - A namespace for the party. This can be either a single NamespaceDelegation, or DecentralizedNamespaceDefinition along with its authorized namespace owners in the form of NamespaceDelegations. May be provided, if so it must be fully authorized by the signatures in this request combined with the existing topology state. - A PartyToParticipant to register the hosting relationship of the party, and the party's signing keys and threshold. Must be provided.  Required: must be non-empty | 
**multi_hash_signatures** | Option<[**Vec<models::Signature>**](Signature.md)> | Optional signatures of the combined hash of all onboarding_transactions This may be used instead of providing signatures on each individual transaction  Optional: can be empty | [optional]
**identity_provider_id** | Option<**String**> | The id of the ``Identity Provider`` If not set, assume the party is managed by the default identity provider.  Optional | [optional]
**wait_for_allocation** | Option<**bool**> | When true, this RPC will attempt to wait for the party to be allocated on the synchronizer before returning. When false, the allocation will happen asynchronously. This is a best effort only as this synchronization is only possible for non decentralized parties (single hosting node). For decentralized parties, this flag is ignored. Defaults to true.  Optional | [optional]
**user_id** | Option<**String**> | The user who will get the act_as rights to the newly allocated party. If set to an empty string (the default), no user will get rights to the party.  Optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


