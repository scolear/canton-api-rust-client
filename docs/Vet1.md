# Vet1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**packages** | [**Vec<models::VettedPackagesRef>**](VettedPackagesRef.md) | Packages to be vetted.  If a reference in this list matches more than one package, the change is considered ambiguous and the entire update request is rejected. In other words, every reference must match exactly one package.  Required: must be non-empty | 
**new_valid_from_inclusive** | Option<**String**> | The time from which these packages should be vetted, prior lower bounds are overwritten. Optional | [optional]
**new_valid_until_exclusive** | Option<**String**> | The time until which these packages should be vetted, prior upper bounds are overwritten. Optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


