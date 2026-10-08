# ListVettedPackagesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vetted_packages** | Option<[**Vec<models::VettedPackages>**](VettedPackages.md)> | All ``VettedPackages`` that contain at least one ``VettedPackage`` matching both a ``PackageMetadataFilter`` and a ``TopologyStateFilter``. Sorted by synchronizer_id then participant_id.  Optional: can be empty | [optional]
**next_page_token** | Option<**String**> | Pagination token to retrieve the next page. Empty string once there are no further results.  ``PackageMetadataFilter`` is applied after ``TopologyStateFilter`` has already produced a page, so that page can end up empty even when it has a next token. Keep following the token until there's no token left.  Optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


