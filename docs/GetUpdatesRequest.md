# GetUpdatesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**begin_exclusive** | Option<**i64**> | Beginning of the requested ledger section (non-negative integer). The response will only contain transactions whose offset is strictly greater than this. If not populated or set to zero, the stream will start from the beginning of the ledger. If positive, the streaming will start after this absolute offset. If the ledger has been pruned, this parameter must be specified and be greater than the pruning offset.  Optional | [optional]
**end_inclusive** | Option<**i64**> | End of the requested ledger section. The response will only contain transactions whose offset is less than or equal to this. If empty, the stream will not terminate. If specified, the stream will terminate after this absolute offset (positive integer) is reached.  Optional | [optional]
**filter** | Option<[**models::TransactionFilter**](TransactionFilter.md)> |  | [optional]
**verbose** | Option<**bool**> | Provided for backwards compatibility, it will be removed in the Canton version 3.5.0. If enabled, values served over the API will contain more information than strictly necessary to interpret the data. In particular, setting the verbose flag to true triggers the ledger to include labels, record and variant type ids for record fields. Optional for backwards compatibility, if defined update_format must be unset | [optional]
**update_format** | [**models::UpdateFormat**](UpdateFormat.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


