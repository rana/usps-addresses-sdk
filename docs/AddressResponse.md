# AddressResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**firm** | Option<**String**> | This is the business or firm name at the address. | [optional]
**address** | Option<[**models::DomesticAddress**](DomesticAddress.md)> |  | [optional]
**additional_info** | Option<[**models::AddressAdditionalInfo**](AddressAdditionalInfo.md)> |  | [optional]
**corrections** | Option<[**Vec<models::AddressCorrectionsInner>**](AddressCorrections_inner.md)> | The corrections made to the requested address. | [optional]
**matches** | Option<[**Vec<models::AddressMatchesInner>**](AddressMatches_inner.md)> | Matches made to the requested address. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


