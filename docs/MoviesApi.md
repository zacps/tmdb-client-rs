# \MoviesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_movie_rating**](MoviesApi.md#delete_movie_rating) | **delete** /movie/{movie_id}/rating | Delete Rating
[**get_movie_account_states**](MoviesApi.md#get_movie_account_states) | **get** /movie/{movie_id}/account_states | Get Account States
[**get_movie_alternative_titles_list**](MoviesApi.md#get_movie_alternative_titles_list) | **get** /movie/{movie_id}/alternative_titles | Get Alternative Titles
[**get_movie_changes_list**](MoviesApi.md#get_movie_changes_list) | **get** /movie/{movie_id}/changes | Get Changes
[**get_movie_credits**](MoviesApi.md#get_movie_credits) | **get** /movie/{movie_id}/credits | Get Credits
[**get_movie_details**](MoviesApi.md#get_movie_details) | **get** /movie/{movie_id} | Get Details
[**get_movie_images**](MoviesApi.md#get_movie_images) | **get** /movie/{movie_id}/images | Get Images
[**get_movie_keywords_list**](MoviesApi.md#get_movie_keywords_list) | **get** /movie/{movie_id}/keywords | Get Keywords
[**get_movie_latest_details**](MoviesApi.md#get_movie_latest_details) | **get** /movie/latest | Get Latest
[**get_movie_lists_paginated**](MoviesApi.md#get_movie_lists_paginated) | **get** /movie/{movie_id}/lists | Get Lists
[**get_movie_now_playing_paginated**](MoviesApi.md#get_movie_now_playing_paginated) | **get** /movie/now_playing | Get Now Playing
[**get_movie_popular_paginated**](MoviesApi.md#get_movie_popular_paginated) | **get** /movie/popular | Get Popular
[**get_movie_recommendations_paginated**](MoviesApi.md#get_movie_recommendations_paginated) | **get** /movie/{movie_id}/recommendations | Get Recommendations
[**get_movie_release_dates**](MoviesApi.md#get_movie_release_dates) | **get** /movie/{movie_id}/release_dates | Get Release Dates
[**get_movie_reviews_paginated**](MoviesApi.md#get_movie_reviews_paginated) | **get** /movie/{movie_id}/reviews | Get Reviews
[**get_movie_similar_paginated**](MoviesApi.md#get_movie_similar_paginated) | **get** /movie/{movie_id}/similar | Get Similar Movies
[**get_movie_top_rated_paginated**](MoviesApi.md#get_movie_top_rated_paginated) | **get** /movie/top_rated | Get Top Rated
[**get_movie_translations_list**](MoviesApi.md#get_movie_translations_list) | **get** /movie/{movie_id}/translations | Get Translations
[**get_movie_upcoming_paginated**](MoviesApi.md#get_movie_upcoming_paginated) | **get** /movie/upcoming | Get Upcoming
[**get_movie_videos_list**](MoviesApi.md#get_movie_videos_list) | **get** /movie/{movie_id}/videos | Get Videos
[**post_movie_rating**](MoviesApi.md#post_movie_rating) | **post** /movie/{movie_id}/rating | Rate Movie



## delete_movie_rating

> ::models::InlineResponse401 delete_movie_rating(ctx, movie_id, content_type, optional)
Delete Rating

Remove your rating for a movie.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
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


## get_movie_account_states

> ::models::AccountStates get_movie_account_states(ctx, movie_id, optional)
Get Account States

Grab the following account states for a session:  - Movie rating - If it belongs to your watchlist - If it belongs to your favourite list

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **String**|  | 
 **session_id** | **String**|  | 
 **guest_session_id** | **String**|  | 

### Return type

[**::models::AccountStates**](account-states.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_alternative_titles_list

> ::models::AlternativeTitlesList get_movie_alternative_titles_list(ctx, movie_id, optional)
Get Alternative Titles

Get all of the alternative titles for a movie.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **country** | **String**|  | 

### Return type

[**::models::AlternativeTitlesList**](alternative-titles-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_changes_list

> ::models::ChangeDetails get_movie_changes_list(ctx, movie_id, optional)
Get Changes

Get the changes for a movie. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **String**|  | 
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


## get_movie_credits

> ::models::Credits get_movie_credits(ctx, movie_id)
Get Credits

Get the cast and crew for a movie.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 

### Return type

[**::models::Credits**](credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_details

> ::models::MovieDetails get_movie_details(ctx, movie_id, optional)
Get Details

Get the primary information about a movie.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **append_to_response** | **String**| Append requests within the same namespace to the response. | 

### Return type

[**::models::MovieDetails**](movie-details.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_images

> ::models::Images get_movie_images(ctx, movie_id, optional)
Get Images

Get the images that belong to a movie.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **include_image_language** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::Images**](images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_keywords_list

> ::models::KeywordsList get_movie_keywords_list(ctx, movie_id, optional)
Get Keywords

Get the keywords that have been added to a movie.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **api_key** | **String**|  | 

### Return type

[**::models::KeywordsList**](keywords-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_latest_details

> ::models::MovieDetails get_movie_latest_details(ctx, optional)
Get Latest

Get the most newly created movie. This is a live response and will continuously change.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::MovieDetails**](movie-details.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_lists_paginated

> ::models::ListsPaginated get_movie_lists_paginated(ctx, movie_id, optional)
Get Lists

Get a list of lists that this movie belongs to.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **api_key** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::ListsPaginated**](lists-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_now_playing_paginated

> ::models::MoviePaginated get_movie_now_playing_paginated(ctx, optional)
Get Now Playing

Get a list of movies in theatres. This is a release type query that looks for all movies that have a release type of 2 or 3 within the specified date range.  You can optionally specify a `region` prameter which will narrow the search to only look for theatrical release dates within the specified country.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_popular_paginated

> ::models::MoviePaginated get_movie_popular_paginated(ctx, optional)
Get Popular

Get a list of the current popular movies on TMDb. This list updates daily.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_recommendations_paginated

> ::models::MoviePaginated get_movie_recommendations_paginated(ctx, movie_id, optional)
Get Recommendations

Get a list of recommended movies for a movie.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **api_key** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_release_dates

> ::models::ReleaseDatesList get_movie_release_dates(ctx, movie_id, optional)
Get Release Dates

Get the release date along with the certification for a movie.  Release dates support different types:  1. Premiere 2. Theatrical (limited) 3. Theatrical 4. Digital 5. Physical 6. TV

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **api_key** | **String**|  | 

### Return type

[**::models::ReleaseDatesList**](release-dates-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_reviews_paginated

> ::models::ReviewsPaginated get_movie_reviews_paginated(ctx, movie_id, optional)
Get Reviews

Get the user reviews for a movie.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **api_key** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::ReviewsPaginated**](reviews-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_similar_paginated

> ::models::MoviePaginated get_movie_similar_paginated(ctx, movie_id, optional)
Get Similar Movies

Get a list of similar movies. This is **not** the same as the \"Recommendation\" system you see on the website.  These items are assembled by looking at keywords and genres.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **api_key** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_top_rated_paginated

> ::models::MoviePaginated get_movie_top_rated_paginated(ctx, optional)
Get Top Rated

Get the top rated movies on TMDb.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_translations_list

> ::models::Translations get_movie_translations_list(ctx, movie_id, optional)
Get Translations

Get a list of translations that have been created for a movie.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **api_key** | **String**|  | 

### Return type

[**::models::Translations**](translations.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_upcoming_paginated

> ::models::MoviePaginated get_movie_upcoming_paginated(ctx, optional)
Get Upcoming

Get a list of upcoming movies in theatres. This is a release type query that looks for all movies that have a release type of 2 or 3 within the specified date range.  You can optionally specify a `region` prameter which will narrow the search to only look for theatrical release dates within the specified country.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_videos_list

> ::models::VideosList get_movie_videos_list(ctx, movie_id, optional)
Get Videos

Get the videos that have been added to a movie.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **String**|  | 
 **api_key** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::VideosList**](videos-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_movie_rating

> ::models::InlineResponse401 post_movie_rating(ctx, movie_id, content_type, optional)
Rate Movie

Rate a movie.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **movie_id** | **i32**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **movie_id** | **i32**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 
 **body** | [**InlineObject5**](InlineObject5.md)|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
