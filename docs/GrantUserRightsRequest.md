# GrantUserRightsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The user to whom to grant rights.  Required | 
**rights** | Option<[**Vec<models::Right>**](Right.md)> | The rights to grant.  Optional: can be empty | [optional]
**identity_provider_id** | Option<**String**> | The id of the ``Identity Provider`` If not set, assume the user is managed by the default identity provider.  Optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


