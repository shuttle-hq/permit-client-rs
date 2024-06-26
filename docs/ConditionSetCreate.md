# ConditionSetCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A unique id by which Permit will identify the condition set. The key will be used as the generated rego rule name. | 
**r#type** | Option<[**models::ConditionSetType**](ConditionSetType.md)> | the type of the set: UserSet or ResourceSet | [optional][default to Userset]
**autogenerated** | Option<**bool**> | whether the set was autogenerated by the system. | [optional][default to false]
**resource_id** | Option<[**models::ResourceId**](Resource_Id.md)> |  | [optional]
**name** | **String** | A descriptive name for the set, i.e: 'US based employees' or 'Users behind VPN' | 
**description** | Option<**String**> | an optional longer description of the set | [optional]
**conditions** | Option<[**serde_json::Value**](.md)> | a boolean expression that consists of multiple conditions, with and/or logic. | [optional][default to {}]
**parent_id** | Option<[**models::ParentId**](Parent_Id.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


