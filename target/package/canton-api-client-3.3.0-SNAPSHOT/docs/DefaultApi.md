# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_v2_idps_idp_id**](DefaultApi.md#delete_v2_idps_idp_id) | **DELETE** /v2/idps/{idp_id} | 
[**delete_v2_users_user_id**](DefaultApi.md#delete_v2_users_user_id) | **DELETE** /v2/users/{user_id} | 
[**get_v2_idps**](DefaultApi.md#get_v2_idps) | **GET** /v2/idps | 
[**get_v2_idps_idp_id**](DefaultApi.md#get_v2_idps_idp_id) | **GET** /v2/idps/{idp_id} | 
[**get_v2_interactive_submission_preferred_package_version**](DefaultApi.md#get_v2_interactive_submission_preferred_package_version) | **GET** /v2/interactive-submission/preferred-package-version | 
[**get_v2_packages**](DefaultApi.md#get_v2_packages) | **GET** /v2/packages | 
[**get_v2_packages_package_id**](DefaultApi.md#get_v2_packages_package_id) | **GET** /v2/packages/{package_id} | 
[**get_v2_packages_package_id_status**](DefaultApi.md#get_v2_packages_package_id_status) | **GET** /v2/packages/{package_id}/status | 
[**get_v2_parties**](DefaultApi.md#get_v2_parties) | **GET** /v2/parties | 
[**get_v2_parties_participant_id**](DefaultApi.md#get_v2_parties_participant_id) | **GET** /v2/parties/participant-id | 
[**get_v2_parties_party**](DefaultApi.md#get_v2_parties_party) | **GET** /v2/parties/{party} | 
[**get_v2_state_connected_synchronizers**](DefaultApi.md#get_v2_state_connected_synchronizers) | **GET** /v2/state/connected-synchronizers | 
[**get_v2_state_latest_pruned_offsets**](DefaultApi.md#get_v2_state_latest_pruned_offsets) | **GET** /v2/state/latest-pruned-offsets | 
[**get_v2_state_ledger_end**](DefaultApi.md#get_v2_state_ledger_end) | **GET** /v2/state/ledger-end | 
[**get_v2_updates_transaction_tree_by_id_update_id**](DefaultApi.md#get_v2_updates_transaction_tree_by_id_update_id) | **GET** /v2/updates/transaction-tree-by-id/{update_id} | 
[**get_v2_updates_transaction_tree_by_offset_offset**](DefaultApi.md#get_v2_updates_transaction_tree_by_offset_offset) | **GET** /v2/updates/transaction-tree-by-offset/{offset} | 
[**get_v2_users**](DefaultApi.md#get_v2_users) | **GET** /v2/users | 
[**get_v2_users_user_id**](DefaultApi.md#get_v2_users_user_id) | **GET** /v2/users/{user_id} | 
[**get_v2_users_user_id_rights**](DefaultApi.md#get_v2_users_user_id_rights) | **GET** /v2/users/{user_id}/rights | 
[**get_v2_version**](DefaultApi.md#get_v2_version) | **GET** /v2/version | 
[**patch_v2_idps_idp_id**](DefaultApi.md#patch_v2_idps_idp_id) | **PATCH** /v2/idps/{idp_id} | 
[**patch_v2_parties_party**](DefaultApi.md#patch_v2_parties_party) | **PATCH** /v2/parties/{party} | 
[**patch_v2_users_user_id**](DefaultApi.md#patch_v2_users_user_id) | **PATCH** /v2/users/{user_id} | 
[**patch_v2_users_user_id_identity_provider_id**](DefaultApi.md#patch_v2_users_user_id_identity_provider_id) | **PATCH** /v2/users/{user_id}/identity-provider-id | 
[**patch_v2_users_user_id_rights**](DefaultApi.md#patch_v2_users_user_id_rights) | **PATCH** /v2/users/{user_id}/rights | 
[**post_v2_commands_async_submit**](DefaultApi.md#post_v2_commands_async_submit) | **POST** /v2/commands/async/submit | 
[**post_v2_commands_async_submit_reassignment**](DefaultApi.md#post_v2_commands_async_submit_reassignment) | **POST** /v2/commands/async/submit-reassignment | 
[**post_v2_commands_completions**](DefaultApi.md#post_v2_commands_completions) | **POST** /v2/commands/completions | 
[**post_v2_commands_submit_and_wait**](DefaultApi.md#post_v2_commands_submit_and_wait) | **POST** /v2/commands/submit-and-wait | 
[**post_v2_commands_submit_and_wait_for_reassignment**](DefaultApi.md#post_v2_commands_submit_and_wait_for_reassignment) | **POST** /v2/commands/submit-and-wait-for-reassignment | 
[**post_v2_commands_submit_and_wait_for_transaction**](DefaultApi.md#post_v2_commands_submit_and_wait_for_transaction) | **POST** /v2/commands/submit-and-wait-for-transaction | 
[**post_v2_commands_submit_and_wait_for_transaction_tree**](DefaultApi.md#post_v2_commands_submit_and_wait_for_transaction_tree) | **POST** /v2/commands/submit-and-wait-for-transaction-tree | 
[**post_v2_events_events_by_contract_id**](DefaultApi.md#post_v2_events_events_by_contract_id) | **POST** /v2/events/events-by-contract-id | 
[**post_v2_idps**](DefaultApi.md#post_v2_idps) | **POST** /v2/idps | 
[**post_v2_interactive_submission_execute**](DefaultApi.md#post_v2_interactive_submission_execute) | **POST** /v2/interactive-submission/execute | 
[**post_v2_interactive_submission_prepare**](DefaultApi.md#post_v2_interactive_submission_prepare) | **POST** /v2/interactive-submission/prepare | 
[**post_v2_packages**](DefaultApi.md#post_v2_packages) | **POST** /v2/packages | 
[**post_v2_parties**](DefaultApi.md#post_v2_parties) | **POST** /v2/parties | 
[**post_v2_state_active_contracts**](DefaultApi.md#post_v2_state_active_contracts) | **POST** /v2/state/active-contracts | 
[**post_v2_updates_flats**](DefaultApi.md#post_v2_updates_flats) | **POST** /v2/updates/flats | 
[**post_v2_updates_transaction_by_id**](DefaultApi.md#post_v2_updates_transaction_by_id) | **POST** /v2/updates/transaction-by-id | 
[**post_v2_updates_transaction_by_offset**](DefaultApi.md#post_v2_updates_transaction_by_offset) | **POST** /v2/updates/transaction-by-offset | 
[**post_v2_updates_trees**](DefaultApi.md#post_v2_updates_trees) | **POST** /v2/updates/trees | 
[**post_v2_updates_update_by_id**](DefaultApi.md#post_v2_updates_update_by_id) | **POST** /v2/updates/update-by-id | 
[**post_v2_updates_update_by_offset**](DefaultApi.md#post_v2_updates_update_by_offset) | **POST** /v2/updates/update-by-offset | 
[**post_v2_users**](DefaultApi.md#post_v2_users) | **POST** /v2/users | 
[**post_v2_users_user_id_rights**](DefaultApi.md#post_v2_users_user_id_rights) | **POST** /v2/users/{user_id}/rights | 



## delete_v2_idps_idp_id

> serde_json::Value delete_v2_idps_idp_id(idp_id)


Delete identity provider config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idp_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_v2_users_user_id

> serde_json::Value delete_v2_users_user_id(user_id)


Delete user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_idps

> models::ListIdentityProviderConfigsResponse get_v2_idps()


List all identity provider configs

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListIdentityProviderConfigsResponse**](ListIdentityProviderConfigsResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_idps_idp_id

> models::GetIdentityProviderConfigResponse get_v2_idps_idp_id(idp_id)


Get identity provider config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idp_id** | **String** |  | [required] |

### Return type

[**models::GetIdentityProviderConfigResponse**](GetIdentityProviderConfigResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_interactive_submission_preferred_package_version

> models::GetPreferredPackageVersionResponse get_v2_interactive_submission_preferred_package_version(package_name, parties, vetting_valid_at, synchronizer_id)


Get the preferred package version for constructing a command submission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_name** | **String** |  | [required] |
**parties** | Option<[**Vec<String>**](String.md)> |  |  |
**vetting_valid_at** | Option<**String**> |  |  |
**synchronizer_id** | Option<**String**> |  |  |

### Return type

[**models::GetPreferredPackageVersionResponse**](GetPreferredPackageVersionResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_packages

> models::ListPackagesResponse get_v2_packages()


List all packages uploaded on the participant node

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListPackagesResponse**](ListPackagesResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_packages_package_id

> std::path::PathBuf get_v2_packages_package_id(package_id)


Download the package for the requested package-id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_packages_package_id_status

> models::GetPackageStatusResponse get_v2_packages_package_id_status(package_id)


Get package status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |

### Return type

[**models::GetPackageStatusResponse**](GetPackageStatusResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_parties

> models::ListKnownPartiesResponse get_v2_parties(page_size, page_token)


List all known parties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | maximum number of elements in a returned page |  |
**page_token** | Option<**String**> | token - to continue results from a given page, leave empty to start from the beginning of the list, obtain token from the result of previous page |  |

### Return type

[**models::ListKnownPartiesResponse**](ListKnownPartiesResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_parties_participant_id

> models::GetParticipantIdResponse get_v2_parties_participant_id()


Get participant id

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetParticipantIdResponse**](GetParticipantIdResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_parties_party

> models::GetPartiesResponse get_v2_parties_party(party, identity_provider_id, parties)


Get party details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party** | **String** |  | [required] |
**identity_provider_id** | Option<**String**> |  |  |
**parties** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::GetPartiesResponse**](GetPartiesResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_state_connected_synchronizers

> models::GetConnectedSynchronizersResponse get_v2_state_connected_synchronizers(party, participant_id)


Get connected synchronizers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party** | **String** |  | [required] |
**participant_id** | Option<**String**> |  |  |

### Return type

[**models::GetConnectedSynchronizersResponse**](GetConnectedSynchronizersResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_state_latest_pruned_offsets

> models::GetLatestPrunedOffsetsResponse get_v2_state_latest_pruned_offsets()


Get latest pruned offsets

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetLatestPrunedOffsetsResponse**](GetLatestPrunedOffsetsResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_state_ledger_end

> models::GetLedgerEndResponse get_v2_state_ledger_end()


Get ledger end

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetLedgerEndResponse**](GetLedgerEndResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_updates_transaction_tree_by_id_update_id

> models::JsGetTransactionTreeResponse get_v2_updates_transaction_tree_by_id_update_id(update_id, parties)


Get transaction tree by  id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_id** | **String** |  | [required] |
**parties** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::JsGetTransactionTreeResponse**](JsGetTransactionTreeResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_updates_transaction_tree_by_offset_offset

> models::JsGetTransactionTreeResponse get_v2_updates_transaction_tree_by_offset_offset(offset, parties)


Get transaction tree by offset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | **i64** |  | [required] |
**parties** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::JsGetTransactionTreeResponse**](JsGetTransactionTreeResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_users

> models::ListUsersResponse get_v2_users(page_size, page_token)


List all users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | maximum number of elements in a returned page |  |
**page_token** | Option<**String**> | token - to continue results from a given page, leave empty to start from the beginning of the list, obtain token from the result of previous page |  |

### Return type

[**models::ListUsersResponse**](ListUsersResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_users_user_id

> models::GetUserResponse get_v2_users_user_id(user_id)


Get user details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**models::GetUserResponse**](GetUserResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_users_user_id_rights

> models::ListUserRightsResponse get_v2_users_user_id_rights(user_id)


List user rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**models::ListUserRightsResponse**](ListUserRightsResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_version

> models::GetLedgerApiVersionResponse get_v2_version()


Get the version details of the participant node

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetLedgerApiVersionResponse**](GetLedgerApiVersionResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_v2_idps_idp_id

> models::UpdateIdentityProviderConfigResponse patch_v2_idps_idp_id(idp_id, update_identity_provider_config_request)


Update identity provider config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idp_id** | **String** |  | [required] |
**update_identity_provider_config_request** | [**UpdateIdentityProviderConfigRequest**](UpdateIdentityProviderConfigRequest.md) |  | [required] |

### Return type

[**models::UpdateIdentityProviderConfigResponse**](UpdateIdentityProviderConfigResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_v2_parties_party

> models::UpdatePartyDetailsResponse patch_v2_parties_party(party, update_party_details_request)


Allocate a new party to the participant node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party** | **String** |  | [required] |
**update_party_details_request** | [**UpdatePartyDetailsRequest**](UpdatePartyDetailsRequest.md) |  | [required] |

### Return type

[**models::UpdatePartyDetailsResponse**](UpdatePartyDetailsResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_v2_users_user_id

> models::UpdateUserResponse patch_v2_users_user_id(user_id, update_user_request)


Update  user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**models::UpdateUserResponse**](UpdateUserResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_v2_users_user_id_identity_provider_id

> serde_json::Value patch_v2_users_user_id_identity_provider_id(user_id, update_user_identity_provider_id_request)


Update user identity provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**update_user_identity_provider_id_request** | [**UpdateUserIdentityProviderIdRequest**](UpdateUserIdentityProviderIdRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_v2_users_user_id_rights

> models::RevokeUserRightsResponse patch_v2_users_user_id_rights(user_id, revoke_user_rights_request)


Revoke user rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**revoke_user_rights_request** | [**RevokeUserRightsRequest**](RevokeUserRightsRequest.md) |  | [required] |

### Return type

[**models::RevokeUserRightsResponse**](RevokeUserRightsResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_commands_async_submit

> serde_json::Value post_v2_commands_async_submit(js_commands)


Submit a command asynchronously

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**js_commands** | [**JsCommands**](JsCommands.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_commands_async_submit_reassignment

> serde_json::Value post_v2_commands_async_submit_reassignment(submit_reassignment_request)


Submit reassignment command asynchronously

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_reassignment_request** | [**SubmitReassignmentRequest**](SubmitReassignmentRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_commands_completions

> Vec<models::CompletionStreamResponse> post_v2_commands_completions(completion_stream_request, limit, stream_idle_timeout_ms)


Query completions list (blocking call)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_stream_request** | [**CompletionStreamRequest**](CompletionStreamRequest.md) |  | [required] |
**limit** | Option<**i64**> | maximum number of elements to return, this param is ignored if is bigger than server setting |  |
**stream_idle_timeout_ms** | Option<**i64**> | timeout to complete and send result if no new elements are received (for open ended streams) |  |

### Return type

[**Vec<models::CompletionStreamResponse>**](CompletionStreamResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_commands_submit_and_wait

> models::SubmitAndWaitResponse post_v2_commands_submit_and_wait(js_commands)


Submit a batch of commands and wait for the completion details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**js_commands** | [**JsCommands**](JsCommands.md) |  | [required] |

### Return type

[**models::SubmitAndWaitResponse**](SubmitAndWaitResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_commands_submit_and_wait_for_reassignment

> models::JsSubmitAndWaitForReassignmentResponse post_v2_commands_submit_and_wait_for_reassignment(submit_and_wait_for_reassignment_request)


Submit a batch of reassignment commands and wait for the reassignment response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_and_wait_for_reassignment_request** | [**SubmitAndWaitForReassignmentRequest**](SubmitAndWaitForReassignmentRequest.md) |  | [required] |

### Return type

[**models::JsSubmitAndWaitForReassignmentResponse**](JsSubmitAndWaitForReassignmentResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_commands_submit_and_wait_for_transaction

> models::JsSubmitAndWaitForTransactionResponse post_v2_commands_submit_and_wait_for_transaction(js_submit_and_wait_for_transaction_request)


Submit a batch of commands and wait for the transaction response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**js_submit_and_wait_for_transaction_request** | [**JsSubmitAndWaitForTransactionRequest**](JsSubmitAndWaitForTransactionRequest.md) |  | [required] |

### Return type

[**models::JsSubmitAndWaitForTransactionResponse**](JsSubmitAndWaitForTransactionResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_commands_submit_and_wait_for_transaction_tree

> models::JsSubmitAndWaitForTransactionTreeResponse post_v2_commands_submit_and_wait_for_transaction_tree(js_commands)


Submit a batch of commands and wait for the transaction trees response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**js_commands** | [**JsCommands**](JsCommands.md) |  | [required] |

### Return type

[**models::JsSubmitAndWaitForTransactionTreeResponse**](JsSubmitAndWaitForTransactionTreeResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_events_events_by_contract_id

> models::JsGetEventsByContractIdResponse post_v2_events_events_by_contract_id(get_events_by_contract_id_request)


Get events by contract Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_events_by_contract_id_request** | [**GetEventsByContractIdRequest**](GetEventsByContractIdRequest.md) |  | [required] |

### Return type

[**models::JsGetEventsByContractIdResponse**](JsGetEventsByContractIdResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_idps

> models::CreateIdentityProviderConfigResponse post_v2_idps(create_identity_provider_config_request)


Create identity provider configs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_identity_provider_config_request** | [**CreateIdentityProviderConfigRequest**](CreateIdentityProviderConfigRequest.md) |  | [required] |

### Return type

[**models::CreateIdentityProviderConfigResponse**](CreateIdentityProviderConfigResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_interactive_submission_execute

> serde_json::Value post_v2_interactive_submission_execute(js_execute_submission_request)


Execute a signed transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**js_execute_submission_request** | [**JsExecuteSubmissionRequest**](JsExecuteSubmissionRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_interactive_submission_prepare

> models::JsPrepareSubmissionResponse post_v2_interactive_submission_prepare(js_prepare_submission_request)


Prepare commands for signing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**js_prepare_submission_request** | [**JsPrepareSubmissionRequest**](JsPrepareSubmissionRequest.md) |  | [required] |

### Return type

[**models::JsPrepareSubmissionResponse**](JsPrepareSubmissionResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_packages

> serde_json::Value post_v2_packages(body)


Upload a DAR to the participant node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **std::path::PathBuf** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_parties

> models::AllocatePartyResponse post_v2_parties(allocate_party_request)


Allocate a new party to the participant node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allocate_party_request** | [**AllocatePartyRequest**](AllocatePartyRequest.md) |  | [required] |

### Return type

[**models::AllocatePartyResponse**](AllocatePartyResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_state_active_contracts

> Vec<models::JsGetActiveContractsResponse> post_v2_state_active_contracts(get_active_contracts_request, limit, stream_idle_timeout_ms)


Query active contracts list (blocking call)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_active_contracts_request** | [**GetActiveContractsRequest**](GetActiveContractsRequest.md) |  | [required] |
**limit** | Option<**i64**> | maximum number of elements to return, this param is ignored if is bigger than server setting |  |
**stream_idle_timeout_ms** | Option<**i64**> | timeout to complete and send result if no new elements are received (for open ended streams) |  |

### Return type

[**Vec<models::JsGetActiveContractsResponse>**](JsGetActiveContractsResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_updates_flats

> Vec<models::JsGetUpdatesResponse> post_v2_updates_flats(get_updates_request, limit, stream_idle_timeout_ms)


Query flat transactions update list (blocking call)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_updates_request** | [**GetUpdatesRequest**](GetUpdatesRequest.md) |  | [required] |
**limit** | Option<**i64**> | maximum number of elements to return, this param is ignored if is bigger than server setting |  |
**stream_idle_timeout_ms** | Option<**i64**> | timeout to complete and send result if no new elements are received (for open ended streams) |  |

### Return type

[**Vec<models::JsGetUpdatesResponse>**](JsGetUpdatesResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_updates_transaction_by_id

> models::JsGetTransactionResponse post_v2_updates_transaction_by_id(get_transaction_by_id_request)


Get transaction by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_transaction_by_id_request** | [**GetTransactionByIdRequest**](GetTransactionByIdRequest.md) |  | [required] |

### Return type

[**models::JsGetTransactionResponse**](JsGetTransactionResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_updates_transaction_by_offset

> models::JsGetTransactionResponse post_v2_updates_transaction_by_offset(get_transaction_by_offset_request)


Get transaction by offset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_transaction_by_offset_request** | [**GetTransactionByOffsetRequest**](GetTransactionByOffsetRequest.md) |  | [required] |

### Return type

[**models::JsGetTransactionResponse**](JsGetTransactionResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_updates_trees

> Vec<models::JsGetUpdateTreesResponse> post_v2_updates_trees(get_updates_request, limit, stream_idle_timeout_ms)


Query update transactions tree list (blocking call)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_updates_request** | [**GetUpdatesRequest**](GetUpdatesRequest.md) |  | [required] |
**limit** | Option<**i64**> | maximum number of elements to return, this param is ignored if is bigger than server setting |  |
**stream_idle_timeout_ms** | Option<**i64**> | timeout to complete and send result if no new elements are received (for open ended streams) |  |

### Return type

[**Vec<models::JsGetUpdateTreesResponse>**](JsGetUpdateTreesResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_updates_update_by_id

> models::JsGetUpdateResponse post_v2_updates_update_by_id(get_update_by_id_request)


Get update by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_update_by_id_request** | [**GetUpdateByIdRequest**](GetUpdateByIdRequest.md) |  | [required] |

### Return type

[**models::JsGetUpdateResponse**](JsGetUpdateResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_updates_update_by_offset

> models::JsGetUpdateResponse post_v2_updates_update_by_offset(get_update_by_offset_request)


Get update by offset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_update_by_offset_request** | [**GetUpdateByOffsetRequest**](GetUpdateByOffsetRequest.md) |  | [required] |

### Return type

[**models::JsGetUpdateResponse**](JsGetUpdateResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_users

> models::CreateUserResponse post_v2_users(create_user_request)


Create user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**models::CreateUserResponse**](CreateUserResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v2_users_user_id_rights

> models::GrantUserRightsResponse post_v2_users_user_id_rights(user_id, grant_user_rights_request)


Grant user rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**grant_user_rights_request** | [**GrantUserRightsRequest**](GrantUserRightsRequest.md) |  | [required] |

### Return type

[**models::GrantUserRightsResponse**](GrantUserRightsResponse.md)

### Authorization

[httpAuth](../README.md#httpAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

