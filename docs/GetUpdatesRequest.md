# GetUpdatesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**begin_exclusive** | **i64** | Exclusive lower bound offset of the requested ledger section (non-negative integer). The response will only contain transactions whose offset is strictly greater than this. If set to zero, the lower bound is set to the beginning of the ledger. If the participant has been pruned, this parameter must be greater or equal than the pruning offset. Required | 
**end_inclusive** | Option<**i64**> | Inclusive higher bound offset of the requested ledger section. If specified the response will only contain transactions whose offset is less than or equal to this. If not specified,  - the descending_order must not be selected, - the stream will not terminate.  Optional | [optional]
**filter** | Option<[**models::TransactionFilter**](TransactionFilter.md)> |  | [optional]
**verbose** | Option<**bool**> | Deprecated: disabled unless `canton.participants.<participant>.features.deprecated.enable-deprecated-parameters-34 = true`; it will be removed in the Canton version 3.7.0. If enabled, values served over the API will contain more information than strictly necessary to interpret the data. In particular, setting the verbose flag to true triggers the ledger to include labels, record and variant type ids for record fields. Optional for backwards compatibility, if defined update_format must be unset | [optional]
**update_format** | Option<[**models::UpdateFormat**](UpdateFormat.md)> |  | [optional]
**descending_order** | Option<**bool**> | If set, the stream will populate the elements in descending order.  Optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


