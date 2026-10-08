# JsExecuteSubmissionAndWaitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prepared_transaction** | **String** | the prepared transaction Typically this is the value of the `prepared_transaction` field in `PrepareSubmissionResponse` obtained from calling `prepareSubmission`.  Required | 
**party_signatures** | [**models::PartySignatures**](PartySignatures.md) |  | 
**deduplication_period** | Option<[**models::DeduplicationPeriod2**](DeduplicationPeriod2.md)> |  | [optional]
**submission_id** | **String** | A unique identifier to distinguish completions for different submissions with the same change ID. Typically a random UUID. Applications are expected to use a different UUID for each retry of a submission with the same change ID. Must be a valid LedgerString (as described in ``value.proto``).  Required | 
**user_id** | Option<**String**> | See [PrepareSubmissionRequest.user_id]  Optional | [optional]
**hashing_scheme_version** | **HashingSchemeVersion** | The hashing scheme version used when building the hash. HASHING_SCHEME_VERSION_V4 requires a synchronizer running protocol version 36 or later and is rejected on synchronizers running older protocol versions.  Required (enum: HASHING_SCHEME_VERSION_UNSPECIFIED, HASHING_SCHEME_VERSION_V2, HASHING_SCHEME_VERSION_V3, HASHING_SCHEME_VERSION_V4) | 
**min_ledger_time** | Option<[**models::MinLedgerTime**](MinLedgerTime.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


