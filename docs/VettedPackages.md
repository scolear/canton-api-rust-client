# VettedPackages

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**packages** | [**Vec<models::VettedPackage>**](VettedPackage.md) | Sorted by package_name and package_version where known, and package_id as a last resort.  Required: must be non-empty | 
**participant_id** | **String** | Participant on which these packages are vetted.  Required | 
**synchronizer_id** | **String** | Synchronizer on which these packages are vetted.  Required | 
**topology_serial** | **i32** | Serial of last ``VettedPackages`` topology transaction of this participant and on this synchronizer.  Required | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


