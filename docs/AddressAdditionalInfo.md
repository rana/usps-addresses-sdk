# AddressAdditionalInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery_point** | Option<**String**> | A specific set of digits between 00 and 99 assigned to every address that is combined with the ZIP + 4&#174; Code to provide a unique identifier for every delivery address.  A street address does not necessarily represent a single delivery point because a street address such as one for an apartment building may have several delivery points. | [optional]
**carrier_route** | Option<**String**> | This is the carrier route code (values unspecified). | [optional]
**dpv_confirmation** | Option<**String**> | The DPV Confirmation Indicator is the primary method used by the USPS&#174; to determine whether an address was considered deliverable or undeliverable. | [optional]
**dpvcmra** | Option<**String**> | Indicates if the location is a [Commercial Mail Receiving Agency (CMRA)](https://faq.usps.com/s/article/Mail-Services-at-Non-Postal-Sites-CMRA) | [optional]
**business** | Option<**String**> | Indicates whether this is a business address. | [optional]
**central_delivery_point** | Option<**String**> | Central Delivery is for all business office buildings, office complexes, and/or industrial/professional parks. This may include call windows, horizontal locked mail receptacles, cluster box units. | [optional]
**vacant** | Option<**String**> | Indicates whether the location designated by the address is occupied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


