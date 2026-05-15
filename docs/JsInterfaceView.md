# JsInterfaceView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interface_id** | **String** | The interface implemented by the matched event. The identifier uses the package-id reference format.  Required | 
**view_status** | [**models::JsStatus**](JsStatus.md) |  | 
**view_value** | Option<**serde_json::Value**> | The value of the interface's view method on this event. Set if it was requested in the ``InterfaceFilter`` and it could be successfully computed.  Optional | [optional]
**implementation_package_id** | Option<**String**> | The package defining the interface implementation used to compute the view. Can be different from the package that was used to create the contract itself, as the contract arguments can be upgraded or downgraded using smart-contract upgrading as part of computing the interface view. Populated if the view computation is successful, otherwise empty.  Optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


