# GetActiveContractsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | Option<[**models::TransactionFilter**](TransactionFilter.md)> |  | [optional]
**verbose** | Option<**bool**> | Deprecated: disabled unless `canton.participants.<participant>.features.deprecated.enable-deprecated-parameters-34 = true`; it will be removed in the Canton version 3.7.0. If enabled, values served over the API will contain more information than strictly necessary to interpret the data. In particular, setting the verbose flag to true triggers the ledger to include labels for record fields. Optional, if specified event_format must be unset. | [optional]
**active_at_offset** | **i64** | The offset at which the snapshot of the active contracts will be computed. Must be no greater than the current ledger end offset. Must be greater than or equal to the last pruning offset. Must be a valid absolute offset (positive integer) or ledger begin offset (zero). If zero, the empty set will be returned.  Required | 
**event_format** | Option<[**models::EventFormat**](EventFormat.md)> |  | [optional]
**stream_continuation_token** | Option<**String**> | Opaque representation of a continuation token defining a position in the active contracts snapshot. The prefix of the active contracts snapshot will be omitted up to and including the element from which the continuation token was read. To reuse the continuation token from a `GetActiveContractsPageResponse`:  - subsequent request must be executed on the same participant with the same version of canton, - subsequent request must have the same active_at_offset, - subsequent request must have the same event_format - and the participant must not have been pruned after the active_at_offset.  If not specified, the whole active contracts snapshot will be returned.  Optional: can be empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


