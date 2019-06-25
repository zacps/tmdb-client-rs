# \TVEpisodesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tv_season_episode_rating**](TVEpisodesApi.md#delete_tv_season_episode_rating) | **delete** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating | Delete Rating
[**get_tv_episode_changes**](TVEpisodesApi.md#get_tv_episode_changes) | **get** /tv/episode/{episode_id}/changes | Get Changes
[**get_tv_season_episode_account_states**](TVEpisodesApi.md#get_tv_season_episode_account_states) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/account_states | Get Account States
[**get_tv_season_episode_credits**](TVEpisodesApi.md#get_tv_season_episode_credits) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/credits | Get Credits
[**get_tv_season_episode_details**](TVEpisodesApi.md#get_tv_season_episode_details) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number} | Get Details
[**get_tv_season_episode_external_ids**](TVEpisodesApi.md#get_tv_season_episode_external_ids) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/external_ids | Get TV Episode External IDs
[**get_tv_season_episode_images**](TVEpisodesApi.md#get_tv_season_episode_images) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/images | Get Images
[**get_tv_season_episode_videos_list**](TVEpisodesApi.md#get_tv_season_episode_videos_list) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/videos | Get  Videos
[**post_tv_season_episode_rating**](TVEpisodesApi.md#post_tv_season_episode_rating) | **post** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating | Rate TV Episode



## delete_tv_season_episode_rating

> ::models::InlineResponse401 delete_tv_season_episode_rating(ctx, tv_id, season_number, episode_number, content_type, optional)
Delete Rating

Remove your rating for a TV episode.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **episode_number** | **i32**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_episode_changes

> ::models::ChangeDetails get_tv_episode_changes(ctx, episode_id, optional)
Get Changes

Get the changes for a TV episode. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **episode_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **episode_id** | **i32**|  | 
 **start_date** | **String**| Filter the results with a start date. | 
 **end_date** | **String**| Filter the results with a end date. | 
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::ChangeDetails**](change-details.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_account_states

> ::models::EpisodeRatingList get_tv_season_episode_account_states(ctx, tv_id, season_number, episode_number, optional)
Get Account States

Get your rating for a episode.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **episode_number** | **i32**|  | 
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 

### Return type

[**::models::EpisodeRatingList**](episode-rating-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_credits

> ::models::Credits get_tv_season_episode_credits(ctx, tv_id, season_number, episode_number)
Get Credits

Get the credits (cast, crew and guest stars) for a TV episode.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 

### Return type

[**::models::Credits**](credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_details

> ::models::EpisodeDetails get_tv_season_episode_details(ctx, tv_id, season_number, episode_number, optional)
Get Details

Get the TV episode details by id.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **episode_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **append_to_response** | **String**| Append requests within the same namespace to the response. | 

### Return type

[**::models::EpisodeDetails**](episode-details.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_external_ids

> ::models::MovieTvExternalIds get_tv_season_episode_external_ids(ctx, tv_id, season_number, episode_number)
Get TV Episode External IDs

Get the external ids for a TV episode. We currently support the following external sources.  | **External Sources** | | -------------------- | | IMDB ID              | | Freebase MID         | | Freebase ID          | | TVDB ID              | | TVRage ID            |

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 

### Return type

[**::models::MovieTvExternalIds**](movie-tv-external-ids.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_images

> ::models::Images get_tv_season_episode_images(ctx, tv_id, season_number, episode_number)
Get Images

Get the images that belong to a TV episode.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 

### Return type

[**::models::Images**](images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_videos_list

> ::models::VideosList get_tv_season_episode_videos_list(ctx, tv_id, season_number, episode_number, optional)
Get  Videos

Get the videos that have been added to a TV episode.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **episode_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::VideosList**](videos-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tv_season_episode_rating

> ::models::InlineResponse401 post_tv_season_episode_rating(ctx, tv_id, season_number, episode_number, content_type, optional)
Rate TV Episode

Rate a TV episode.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
  **episode_number** | **i32**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **episode_number** | **i32**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 
 **body** | [**InlineObject7**](InlineObject7.md)|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
