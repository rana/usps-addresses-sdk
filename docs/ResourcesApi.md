# \ResourcesApi

All URIs are relative to *https://api.usps.com/addresses/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_address**](ResourcesApi.md#get_address) | **GET** /address | Returns the best standardized address for a given address.
[**get_city_state**](ResourcesApi.md#get_city_state) | **GET** /city-state | Returns the city and state for a given ZIP Code
[**get_zip_code**](ResourcesApi.md#get_zip_code) | **GET** /zipcode | Returns the ZIP Code for a given address.



## get_address

> models::AddressResponse get_address(street_address, state, secondary_address, city, urbanization, zip_code, zip_plus4)
Returns the best standardized address for a given address.

Standardizes street addresses including city and street abbreviations, provides missing information such as ZIP Code&#8482; and ZIP + 4&#174;.  Must specify a street address, a state and either a city or a ZIP Code&#8482;.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**street_address** | **String** | The number of a building along with the name of the road or street on which it is located. | [required] |
**state** | **String** | This is two-character state code of the address. | [required] |
**secondary_address** | Option<**String**> | The secondary unit designator, such as apartment(APT) or suite(STE) number, defining the exact location of the address within a building.  For more information please see [Postal Explorer](https://pe.usps.com/text/pub28/28c2_003.htm). |  |
**city** | Option<**String**> | This is the city name of the address. |  |
**urbanization** | Option<**String**> | This is the urbanization code relevant only for Puerto Rico addresses. |  |
**zip_code** | Option<**String**> | This is the 5-digit ZIP code. |  |
**zip_plus4** | Option<**String**> | This is the 4-digit component of the ZIP+4 code. Using the correct Zip+4 reduces the number of times your mail is handled and can decrease the chance of a misdelivery or error. |  |

### Return type

[**models::AddressResponse**](AddressResponse.md)

### Authorization

[OAuth](../README.md#OAuth), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_city_state

> models::CityStateResponse get_city_state(zip_code)
Returns the city and state for a given ZIP Code

Returns the city and state corresponding to the given ZIP Code&#8482;.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zip_code** | **String** | This is the 5-digit ZIP code. | [required] |

### Return type

[**models::CityStateResponse**](CityStateResponse.md)

### Authorization

[OAuth](../README.md#OAuth), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zip_code

> models::ZipCodeResponse get_zip_code(street_address, city, state, secondary_address, zip_code, zip_plus4)
Returns the ZIP Code for a given address.

Returns the ZIP Code&#8482; and ZIP + 4&#174; corresponding to the given address, city, and state (use USPS state abbreviations).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**street_address** | **String** | The number of a building along with the name of the road or street on which it is located. | [required] |
**city** | **String** | This is the city name of the address. | [required] |
**state** | **String** | This is two-character state code of the address. | [required] |
**secondary_address** | Option<**String**> | The secondary unit designator, such as apartment(APT) or suite(STE) number, defining the exact location of the address within a building.  For more information please see [Postal Explorer](https://pe.usps.com/text/pub28/28c2_003.htm). |  |
**zip_code** | Option<**String**> | This is the 5-digit ZIP code. |  |
**zip_plus4** | Option<**String**> | This is the 4-digit component of the ZIP+4 code. Using the correct Zip+4 reduces the number of times your mail is handled and can decrease the chance of a misdelivery or error. |  |

### Return type

[**models::ZipCodeResponse**](ZIPCodeResponse.md)

### Authorization

[OAuth](../README.md#OAuth), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

