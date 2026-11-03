#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[allow(dead_code)]
pub type SSE = std::pin::Pin<
    std::boxed::Box<
        dyn futures_util::Stream<
                Item = std::result::Result<axum::response::sse::Event, std::convert::Infallible>,
            > + std::marker::Send
            + std::marker::Sync,
    >,
>;

#[allow(dead_code)]
fn from_validation_error(e: validator::ValidationError) -> validator::ValidationErrors {
    let mut errs = validator::ValidationErrors::new();
    errs.add("na", e);
    errs
}

#[allow(dead_code)]
pub fn check_xss_string(v: &str) -> std::result::Result<(), validator::ValidationError> {
    if ammonia::is_html(v) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_vec_string(v: &[String]) -> std::result::Result<(), validator::ValidationError> {
    if v.iter().any(|i| ammonia::is_html(i)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_string(
    v: &std::collections::HashMap<String, String>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| ammonia::is_html(v)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_nested<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError>
where
    T: validator::Validate,
{
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| v.validate().is_err()) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteApplicationPathParams {
    pub application_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetApplicationPathParams {
    pub application_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecordApplicationAccessPathParams {
    pub application_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateApplicationPathParams {
    pub application_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateBookmarkPathParams {
    pub category_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteBookmarkPathParams {
    pub category_id: uuid::Uuid,
    pub bookmark_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteCategoryPathParams {
    pub category_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecordBookmarkAccessPathParams {
    pub category_id: uuid::Uuid,
    pub bookmark_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReorderBookmarksPathParams {
    pub category_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateBookmarkPathParams {
    pub category_id: uuid::Uuid,
    pub bookmark_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateCategoryPathParams {
    pub category_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CompleteLoginQueryParams {
    /// Temporary sign-in code supplied by your account provider.
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Sign-in check supplied by your account provider.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "error_description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DowngradeSessionPathParams {
    pub session_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InvalidateSessionPathParams {
    pub session_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TerminateSessionPathParams {
    pub session_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpgradeSessionPathParams {
    pub session_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteThemePathParams {
    pub theme_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetThemePathParams {
    pub theme_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateApplication201Response {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,

    #[serde(rename = "health_check")]
    pub health_check: Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck>,

    /// Omitted until a configured health check has run.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "health")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,

    /// Choose which groups can see this app. An empty list makes it visible to everyone.
    #[serde(rename = "visible_to_groups")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub visible_to_groups: Vec<String>,
}

impl CreateApplication201Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        icon: String,
        address: String,
        health_check: Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck>,
        visible_to_groups: Vec<String>,
    ) -> CreateApplication201Response {
        CreateApplication201Response {
            id,
            name,
            icon,
            address,
            health_check,
            health: None,
            visible_to_groups,
        }
    }
}

/// Converts the CreateApplication201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateApplication201Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
            // Skipping health_check in query parameter serialization
            self.health
                .as_ref()
                .map(|health| ["health".to_string(), health.to_string()].join(",")),
            Some("visible_to_groups".to_string()),
            Some(
                self.visible_to_groups
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateApplication201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateApplication201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
            pub health_check: Vec<models::ListApplications200ResponseApplicationsInnerHealthCheck>,
            pub health: Vec<String>,
            pub visible_to_groups: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateApplication201Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "health_check" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in CreateApplication201Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "health" => intermediate_rep.health.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "visible_to_groups" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateApplication201Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateApplication201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateApplication201Response {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in CreateApplication201Response".to_string())?,
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateApplication201Response".to_string())?,
            icon: intermediate_rep
                .icon
                .into_iter()
                .next()
                .ok_or_else(|| "icon missing in CreateApplication201Response".to_string())?,
            address: intermediate_rep
                .address
                .into_iter()
                .next()
                .ok_or_else(|| "address missing in CreateApplication201Response".to_string())?,
            health_check: std::result::Result::Err(
                "Nullable types not supported in CreateApplication201Response".to_string(),
            )?,
            health: intermediate_rep.health.into_iter().next(),
            visible_to_groups: intermediate_rep
                .visible_to_groups
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "visible_to_groups missing in CreateApplication201Response".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateApplication201Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateApplication201Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateApplication201Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateApplication201Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateApplication201Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateApplication201Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateApplication201Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateApplicationRequest {
    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,

    #[serde(rename = "health_check")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check:
        Option<Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck>>,

    /// Choose which groups can see this app. An empty list makes it visible to everyone.
    #[serde(rename = "visible_to_groups")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub visible_to_groups: Vec<String>,
}

impl CreateApplicationRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        icon: String,
        address: String,
        visible_to_groups: Vec<String>,
    ) -> CreateApplicationRequest {
        CreateApplicationRequest {
            name,
            icon,
            address,
            health_check: None,
            visible_to_groups,
        }
    }
}

/// Converts the CreateApplicationRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateApplicationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
            // Skipping health_check in query parameter serialization
            Some("visible_to_groups".to_string()),
            Some(
                self.visible_to_groups
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateApplicationRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateApplicationRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
            pub health_check: Vec<models::ListApplications200ResponseApplicationsInnerHealthCheck>,
            pub visible_to_groups: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateApplicationRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "health_check" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in CreateApplicationRequest".to_string()),
                    "visible_to_groups" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateApplicationRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateApplicationRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateApplicationRequest {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateApplicationRequest".to_string())?,
            icon: intermediate_rep
                .icon
                .into_iter()
                .next()
                .ok_or_else(|| "icon missing in CreateApplicationRequest".to_string())?,
            address: intermediate_rep
                .address
                .into_iter()
                .next()
                .ok_or_else(|| "address missing in CreateApplicationRequest".to_string())?,
            health_check: std::result::Result::Err(
                "Nullable types not supported in CreateApplicationRequest".to_string(),
            )?,
            visible_to_groups: intermediate_rep
                .visible_to_groups
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "visible_to_groups missing in CreateApplicationRequest".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateApplicationRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateApplicationRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateApplicationRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateApplicationRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateApplicationRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateApplicationRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateApplicationRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateBookmark201Response {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    /// Icon name used to display this bookmark.
    #[serde(rename = "icon")]
    #[validate(custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,
}

impl CreateBookmark201Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        icon: String,
        address: String,
    ) -> CreateBookmark201Response {
        CreateBookmark201Response {
            id,
            name,
            icon,
            address,
        }
    }
}

/// Converts the CreateBookmark201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateBookmark201Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateBookmark201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateBookmark201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateBookmark201Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CreateBookmark201Response".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateBookmark201Response {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in CreateBookmark201Response".to_string())?,
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateBookmark201Response".to_string())?,
            icon: intermediate_rep
                .icon
                .into_iter()
                .next()
                .ok_or_else(|| "icon missing in CreateBookmark201Response".to_string())?,
            address: intermediate_rep
                .address
                .into_iter()
                .next()
                .ok_or_else(|| "address missing in CreateBookmark201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateBookmark201Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateBookmark201Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateBookmark201Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateBookmark201Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateBookmark201Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateBookmark201Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateBookmark201Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateBookmarkRequest {
    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,
}

impl CreateBookmarkRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, icon: String, address: String) -> CreateBookmarkRequest {
        CreateBookmarkRequest {
            name,
            icon,
            address,
        }
    }
}

/// Converts the CreateBookmarkRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateBookmarkRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateBookmarkRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateBookmarkRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateBookmarkRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CreateBookmarkRequest".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateBookmarkRequest {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateBookmarkRequest".to_string())?,
            icon: intermediate_rep
                .icon
                .into_iter()
                .next()
                .ok_or_else(|| "icon missing in CreateBookmarkRequest".to_string())?,
            address: intermediate_rep
                .address
                .into_iter()
                .next()
                .ok_or_else(|| "address missing in CreateBookmarkRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateBookmarkRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateBookmarkRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateBookmarkRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateBookmarkRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateBookmarkRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateBookmarkRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateBookmarkRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateCategory201Response {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "bookmarks_sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    pub bookmarks_sort_order: String,

    #[serde(rename = "bookmarks")]
    #[validate(nested)]
    pub bookmarks: Vec<models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>,
}

impl CreateCategory201Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        bookmarks_sort_order: String,
        bookmarks: Vec<models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>,
    ) -> CreateCategory201Response {
        CreateCategory201Response {
            id,
            name,
            bookmarks_sort_order,
            bookmarks,
        }
    }
}

/// Converts the CreateCategory201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateCategory201Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("bookmarks_sort_order".to_string()),
            Some(self.bookmarks_sort_order.to_string()),
            // Skipping bookmarks in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateCategory201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateCategory201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub name: Vec<String>,
            pub bookmarks_sort_order: Vec<String>,
            pub bookmarks:
                Vec<Vec<models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateCategory201Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bookmarks_sort_order" => intermediate_rep.bookmarks_sort_order.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "bookmarks" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateCategory201Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateCategory201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateCategory201Response {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in CreateCategory201Response".to_string())?,
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateCategory201Response".to_string())?,
            bookmarks_sort_order: intermediate_rep
                .bookmarks_sort_order
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "bookmarks_sort_order missing in CreateCategory201Response".to_string()
                })?,
            bookmarks: intermediate_rep
                .bookmarks
                .into_iter()
                .next()
                .ok_or_else(|| "bookmarks missing in CreateCategory201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateCategory201Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateCategory201Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateCategory201Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateCategory201Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateCategory201Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateCategory201Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateCategory201Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateCategoryRequest {
    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "shelved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shelved: Option<bool>,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl CreateCategoryRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String) -> CreateCategoryRequest {
        CreateCategoryRequest {
            name,
            shelved: Some(false),
            sort_order: None,
        }
    }
}

/// Converts the CreateCategoryRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateCategoryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            self.shelved
                .as_ref()
                .map(|shelved| ["shelved".to_string(), shelved.to_string()].join(",")),
            self.sort_order
                .as_ref()
                .map(|sort_order| ["sort_order".to_string(), sort_order.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateCategoryRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateCategoryRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub shelved: Vec<bool>,
            pub sort_order: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateCategoryRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shelved" => intermediate_rep.shelved.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "sort_order" => intermediate_rep.sort_order.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CreateCategoryRequest".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateCategoryRequest {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateCategoryRequest".to_string())?,
            shelved: intermediate_rep.shelved.into_iter().next(),
            sort_order: intermediate_rep.sort_order.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateCategoryRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateCategoryRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateCategoryRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateCategoryRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateCategoryRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateCategoryRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateCategoryRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateTheme201Response {
    /// Namespaced theme ID, either global:<name> or user:<uuid>.
    #[serde(rename = "id")]
    #[validate(custom(function = "check_xss_string"))]
    pub id: String,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "mode")]
    #[validate(custom(function = "check_xss_string"))]
    pub mode: String,

    #[serde(rename = "palette")]
    #[validate(nested)]
    pub palette: models::ListThemes200ResponseInnerPalette,

    /// Whether this theme comes with Dash or was created by you.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "scope")]
    #[validate(custom(function = "check_xss_string"))]
    pub scope: String,
}

impl CreateTheme201Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: String,
        name: String,
        mode: String,
        palette: models::ListThemes200ResponseInnerPalette,
        scope: String,
    ) -> CreateTheme201Response {
        CreateTheme201Response {
            id,
            name,
            mode,
            palette,
            scope,
        }
    }
}

/// Converts the CreateTheme201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateTheme201Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("mode".to_string()),
            Some(self.mode.to_string()),
            // Skipping palette in query parameter serialization
            Some("scope".to_string()),
            Some(self.scope.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateTheme201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateTheme201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub mode: Vec<String>,
            pub palette: Vec<models::ListThemes200ResponseInnerPalette>,
            pub scope: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateTheme201Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "mode" => intermediate_rep.mode.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "palette" => intermediate_rep.palette.push(
                        <models::ListThemes200ResponseInnerPalette as std::str::FromStr>::from_str(
                            val,
                        )
                        .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "scope" => intermediate_rep.scope.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CreateTheme201Response".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateTheme201Response {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in CreateTheme201Response".to_string())?,
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateTheme201Response".to_string())?,
            mode: intermediate_rep
                .mode
                .into_iter()
                .next()
                .ok_or_else(|| "mode missing in CreateTheme201Response".to_string())?,
            palette: intermediate_rep
                .palette
                .into_iter()
                .next()
                .ok_or_else(|| "palette missing in CreateTheme201Response".to_string())?,
            scope: intermediate_rep
                .scope
                .into_iter()
                .next()
                .ok_or_else(|| "scope missing in CreateTheme201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateTheme201Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateTheme201Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateTheme201Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateTheme201Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateTheme201Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateTheme201Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateTheme201Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateThemeRequest {
    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "mode")]
    #[validate(custom(function = "check_xss_string"))]
    pub mode: String,

    #[serde(rename = "palette")]
    #[validate(nested)]
    pub palette: models::ListThemes200ResponseInnerPalette,
}

impl CreateThemeRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        mode: String,
        palette: models::ListThemes200ResponseInnerPalette,
    ) -> CreateThemeRequest {
        CreateThemeRequest {
            name,
            mode,
            palette,
        }
    }
}

/// Converts the CreateThemeRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CreateThemeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("mode".to_string()),
            Some(self.mode.to_string()),
            // Skipping palette in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateThemeRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateThemeRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub mode: Vec<String>,
            pub palette: Vec<models::ListThemes200ResponseInnerPalette>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CreateThemeRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "mode" => intermediate_rep.mode.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "palette" => intermediate_rep.palette.push(
                        <models::ListThemes200ResponseInnerPalette as std::str::FromStr>::from_str(
                            val,
                        )
                        .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CreateThemeRequest".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateThemeRequest {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CreateThemeRequest".to_string())?,
            mode: intermediate_rep
                .mode
                .into_iter()
                .next()
                .ok_or_else(|| "mode missing in CreateThemeRequest".to_string())?,
            palette: intermediate_rep
                .palette
                .into_iter()
                .next()
                .ok_or_else(|| "palette missing in CreateThemeRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateThemeRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateThemeRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CreateThemeRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CreateThemeRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CreateThemeRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CreateThemeRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into CreateThemeRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

/// Current backup file format used for downloads and restores.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExportTransfer200Response {
    /// Identifies this as a current Dash backup.
    #[serde(rename = "version")]
    pub version: crate::types::Object,

    /// When Dash created this backup.
    #[serde(rename = "exported_at")]
    pub exported_at: chrono::DateTime<chrono::Utc>,

    /// Account name shown in the backup; it does not change its owner.
    #[serde(rename = "username")]
    #[validate(custom(function = "check_xss_string"))]
    pub username: String,

    #[serde(rename = "settings")]
    #[validate(nested)]
    pub settings: models::ExportTransfer200ResponseSettings,

    #[serde(rename = "themes")]
    #[validate(nested)]
    pub themes: Vec<models::ExportTransfer200ResponseThemesInner>,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "categories_sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    pub categories_sort_order: String,

    /// When categories_sort_order is user_defined, this array's order is the saved category order. Recently accessed order is derived from bookmark last_accessed_at values.
    #[serde(rename = "categories")]
    #[validate(nested)]
    pub categories: Vec<models::ExportTransfer200ResponseCategoriesInner>,

    /// Included in administrator exports. Controls sorting of the shared app catalog.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "applications_sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications_sort_order: Option<String>,

    /// For user_defined sorting, apps are listed in their custom order. Alphabetical sorting is derived from app names. Recently accessed sorting is per-user, so this array's order is not used for it.
    #[serde(rename = "applications")]
    #[validate(nested)]
    pub applications: Vec<models::ExportTransfer200ResponseApplicationsInner>,

    /// Dash uses this value to check that the backup is unchanged.
    #[serde(rename = "signature")]
    #[validate(custom(function = "check_xss_string"))]
    pub signature: String,
}

impl ExportTransfer200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        version: crate::types::Object,
        exported_at: chrono::DateTime<chrono::Utc>,
        username: String,
        settings: models::ExportTransfer200ResponseSettings,
        themes: Vec<models::ExportTransfer200ResponseThemesInner>,
        categories_sort_order: String,
        categories: Vec<models::ExportTransfer200ResponseCategoriesInner>,
        applications: Vec<models::ExportTransfer200ResponseApplicationsInner>,
        signature: String,
    ) -> ExportTransfer200Response {
        ExportTransfer200Response {
            version,
            exported_at,
            username,
            settings,
            themes,
            categories_sort_order,
            categories,
            applications_sort_order: None,
            applications,
            signature,
        }
    }
}

/// Converts the ExportTransfer200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExportTransfer200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping version in query parameter serialization

            // Skipping exported_at in query parameter serialization
            Some("username".to_string()),
            Some(self.username.to_string()),
            // Skipping settings in query parameter serialization

            // Skipping themes in query parameter serialization
            Some("categories_sort_order".to_string()),
            Some(self.categories_sort_order.to_string()),
            // Skipping categories in query parameter serialization
            self.applications_sort_order
                .as_ref()
                .map(|applications_sort_order| {
                    [
                        "applications_sort_order".to_string(),
                        applications_sort_order.to_string(),
                    ]
                    .join(",")
                }),
            // Skipping applications in query parameter serialization
            Some("signature".to_string()),
            Some(self.signature.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExportTransfer200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExportTransfer200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub version: Vec<crate::types::Object>,
            pub exported_at: Vec<chrono::DateTime<chrono::Utc>>,
            pub username: Vec<String>,
            pub settings: Vec<models::ExportTransfer200ResponseSettings>,
            pub themes: Vec<Vec<models::ExportTransfer200ResponseThemesInner>>,
            pub categories_sort_order: Vec<String>,
            pub categories: Vec<Vec<models::ExportTransfer200ResponseCategoriesInner>>,
            pub applications_sort_order: Vec<String>,
            pub applications: Vec<Vec<models::ExportTransfer200ResponseApplicationsInner>>,
            pub signature: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ExportTransfer200Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "exported_at" => intermediate_rep.exported_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "settings" => intermediate_rep.settings.push(<models::ExportTransfer200ResponseSettings as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "themes" => return std::result::Result::Err("Parsing a container in this style is not supported in ExportTransfer200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "categories_sort_order" => intermediate_rep.categories_sort_order.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "categories" => return std::result::Result::Err("Parsing a container in this style is not supported in ExportTransfer200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "applications_sort_order" => intermediate_rep.applications_sort_order.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "applications" => return std::result::Result::Err("Parsing a container in this style is not supported in ExportTransfer200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "signature" => intermediate_rep.signature.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExportTransfer200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExportTransfer200Response {
            version: intermediate_rep
                .version
                .into_iter()
                .next()
                .ok_or_else(|| "version missing in ExportTransfer200Response".to_string())?,
            exported_at: intermediate_rep
                .exported_at
                .into_iter()
                .next()
                .ok_or_else(|| "exported_at missing in ExportTransfer200Response".to_string())?,
            username: intermediate_rep
                .username
                .into_iter()
                .next()
                .ok_or_else(|| "username missing in ExportTransfer200Response".to_string())?,
            settings: intermediate_rep
                .settings
                .into_iter()
                .next()
                .ok_or_else(|| "settings missing in ExportTransfer200Response".to_string())?,
            themes: intermediate_rep
                .themes
                .into_iter()
                .next()
                .ok_or_else(|| "themes missing in ExportTransfer200Response".to_string())?,
            categories_sort_order: intermediate_rep
                .categories_sort_order
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "categories_sort_order missing in ExportTransfer200Response".to_string()
                })?,
            categories: intermediate_rep
                .categories
                .into_iter()
                .next()
                .ok_or_else(|| "categories missing in ExportTransfer200Response".to_string())?,
            applications_sort_order: intermediate_rep.applications_sort_order.into_iter().next(),
            applications: intermediate_rep
                .applications
                .into_iter()
                .next()
                .ok_or_else(|| "applications missing in ExportTransfer200Response".to_string())?,
            signature: intermediate_rep
                .signature
                .into_iter()
                .next()
                .ok_or_else(|| "signature missing in ExportTransfer200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExportTransfer200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExportTransfer200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ExportTransfer200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ExportTransfer200Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ExportTransfer200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ExportTransfer200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ExportTransfer200Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExportTransfer200ResponseApplicationsInner {
    #[serde(rename = "hash")]
    #[validate(custom(function = "check_xss_string"))]
    pub hash: String,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,

    #[serde(rename = "visible_to_groups")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub visible_to_groups: Vec<String>,

    #[serde(rename = "health_check")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check:
        Option<Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck>>,
}

impl ExportTransfer200ResponseApplicationsInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        hash: String,
        name: String,
        icon: String,
        address: String,
        visible_to_groups: Vec<String>,
    ) -> ExportTransfer200ResponseApplicationsInner {
        ExportTransfer200ResponseApplicationsInner {
            hash,
            name,
            icon,
            address,
            visible_to_groups,
            health_check: None,
        }
    }
}

/// Converts the ExportTransfer200ResponseApplicationsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExportTransfer200ResponseApplicationsInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("hash".to_string()),
            Some(self.hash.to_string()),
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
            Some("visible_to_groups".to_string()),
            Some(
                self.visible_to_groups
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            // Skipping health_check in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExportTransfer200ResponseApplicationsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExportTransfer200ResponseApplicationsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub hash: Vec<String>,
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
            pub visible_to_groups: Vec<Vec<String>>,
            pub health_check: Vec<models::ListApplications200ResponseApplicationsInnerHealthCheck>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val =
                match string_iter.next() {
                    Some(x) => x,
                    None => return std::result::Result::Err(
                        "Missing value while parsing ExportTransfer200ResponseApplicationsInner"
                            .to_string(),
                    ),
                };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "hash" => intermediate_rep.hash.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "visible_to_groups" => return std::result::Result::Err("Parsing a container in this style is not supported in ExportTransfer200ResponseApplicationsInner".to_string()),
                    "health_check" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ExportTransfer200ResponseApplicationsInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExportTransfer200ResponseApplicationsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExportTransfer200ResponseApplicationsInner {
            hash: intermediate_rep.hash.into_iter().next().ok_or_else(|| {
                "hash missing in ExportTransfer200ResponseApplicationsInner".to_string()
            })?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| {
                "name missing in ExportTransfer200ResponseApplicationsInner".to_string()
            })?,
            icon: intermediate_rep.icon.into_iter().next().ok_or_else(|| {
                "icon missing in ExportTransfer200ResponseApplicationsInner".to_string()
            })?,
            address: intermediate_rep.address.into_iter().next().ok_or_else(|| {
                "address missing in ExportTransfer200ResponseApplicationsInner".to_string()
            })?,
            visible_to_groups: intermediate_rep
                .visible_to_groups
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "visible_to_groups missing in ExportTransfer200ResponseApplicationsInner"
                        .to_string()
                })?,
            health_check: std::result::Result::Err(
                "Nullable types not supported in ExportTransfer200ResponseApplicationsInner"
                    .to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExportTransfer200ResponseApplicationsInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExportTransfer200ResponseApplicationsInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ExportTransfer200ResponseApplicationsInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ExportTransfer200ResponseApplicationsInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ExportTransfer200ResponseApplicationsInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ExportTransfer200ResponseApplicationsInner as std::str::FromStr>::from_str(
                    value,
                ) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ExportTransfer200ResponseApplicationsInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExportTransfer200ResponseCategoriesInner {
    #[serde(rename = "hash")]
    #[validate(custom(function = "check_xss_string"))]
    pub hash: String,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "shelved")]
    pub shelved: bool,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "bookmarks_sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarks_sort_order: Option<String>,

    /// When bookmarks_sort_order is user_defined, this array's order is the saved bookmark order. Recently accessed order is derived from each bookmark's last_accessed_at value.
    #[serde(rename = "bookmarks")]
    #[validate(nested)]
    pub bookmarks: Vec<models::ExportTransfer200ResponseCategoriesInnerBookmarksInner>,
}

impl ExportTransfer200ResponseCategoriesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        hash: String,
        name: String,
        shelved: bool,
        bookmarks: Vec<models::ExportTransfer200ResponseCategoriesInnerBookmarksInner>,
    ) -> ExportTransfer200ResponseCategoriesInner {
        ExportTransfer200ResponseCategoriesInner {
            hash,
            name,
            shelved,
            bookmarks_sort_order: None,
            bookmarks,
        }
    }
}

/// Converts the ExportTransfer200ResponseCategoriesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExportTransfer200ResponseCategoriesInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("hash".to_string()),
            Some(self.hash.to_string()),
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("shelved".to_string()),
            Some(self.shelved.to_string()),
            self.bookmarks_sort_order
                .as_ref()
                .map(|bookmarks_sort_order| {
                    [
                        "bookmarks_sort_order".to_string(),
                        bookmarks_sort_order.to_string(),
                    ]
                    .join(",")
                }),
            // Skipping bookmarks in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExportTransfer200ResponseCategoriesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExportTransfer200ResponseCategoriesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub hash: Vec<String>,
            pub name: Vec<String>,
            pub shelved: Vec<bool>,
            pub bookmarks_sort_order: Vec<String>,
            pub bookmarks: Vec<Vec<models::ExportTransfer200ResponseCategoriesInnerBookmarksInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ExportTransfer200ResponseCategoriesInner"
                            .to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "hash" => intermediate_rep.hash.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "shelved" => intermediate_rep.shelved.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bookmarks_sort_order" => intermediate_rep.bookmarks_sort_order.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "bookmarks" => return std::result::Result::Err("Parsing a container in this style is not supported in ExportTransfer200ResponseCategoriesInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExportTransfer200ResponseCategoriesInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExportTransfer200ResponseCategoriesInner {
            hash: intermediate_rep.hash.into_iter().next().ok_or_else(|| {
                "hash missing in ExportTransfer200ResponseCategoriesInner".to_string()
            })?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| {
                "name missing in ExportTransfer200ResponseCategoriesInner".to_string()
            })?,
            shelved: intermediate_rep.shelved.into_iter().next().ok_or_else(|| {
                "shelved missing in ExportTransfer200ResponseCategoriesInner".to_string()
            })?,
            bookmarks_sort_order: intermediate_rep.bookmarks_sort_order.into_iter().next(),
            bookmarks: intermediate_rep
                .bookmarks
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "bookmarks missing in ExportTransfer200ResponseCategoriesInner".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ExportTransfer200ResponseCategoriesInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ExportTransfer200ResponseCategoriesInner as std::str::FromStr>::from_str(
                    value,
                ) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ExportTransfer200ResponseCategoriesInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExportTransfer200ResponseCategoriesInnerBookmarksInner {
    #[serde(rename = "hash")]
    #[validate(custom(function = "check_xss_string"))]
    pub hash: String,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,

    #[serde(rename = "last_accessed_at")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<Nullable<chrono::DateTime<chrono::Utc>>>,
}

impl ExportTransfer200ResponseCategoriesInnerBookmarksInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        hash: String,
        name: String,
        icon: String,
        address: String,
    ) -> ExportTransfer200ResponseCategoriesInnerBookmarksInner {
        ExportTransfer200ResponseCategoriesInnerBookmarksInner {
            hash,
            name,
            icon,
            address,
            last_accessed_at: None,
        }
    }
}

/// Converts the ExportTransfer200ResponseCategoriesInnerBookmarksInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExportTransfer200ResponseCategoriesInnerBookmarksInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("hash".to_string()),
            Some(self.hash.to_string()),
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
            // Skipping last_accessed_at in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExportTransfer200ResponseCategoriesInnerBookmarksInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExportTransfer200ResponseCategoriesInnerBookmarksInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub hash: Vec<String>,
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
            pub last_accessed_at: Vec<chrono::DateTime<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "hash" => intermediate_rep.hash.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "last_accessed_at" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExportTransfer200ResponseCategoriesInnerBookmarksInner {
            hash: intermediate_rep.hash.into_iter().next().ok_or_else(|| "hash missing in ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string())?,
            icon: intermediate_rep.icon.into_iter().next().ok_or_else(|| "icon missing in ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string())?,
            address: intermediate_rep.address.into_iter().next().ok_or_else(|| "address missing in ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string())?,
            last_accessed_at: std::result::Result::Err("Nullable types not supported in ExportTransfer200ResponseCategoriesInnerBookmarksInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInnerBookmarksInner> and HeaderValue

#[cfg(feature = "server")]
impl
    std::convert::TryFrom<
        header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInnerBookmarksInner>,
    > for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInnerBookmarksInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ExportTransfer200ResponseCategoriesInnerBookmarksInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ExportTransfer200ResponseCategoriesInnerBookmarksInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExportTransfer200ResponseCategoriesInnerBookmarksInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ExportTransfer200ResponseCategoriesInnerBookmarksInner - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExportTransfer200ResponseSettings {
    #[serde(rename = "language")]
    #[validate(custom(function = "check_xss_string"))]
    pub language: String,

    #[serde(rename = "time_zone")]
    #[validate(custom(function = "check_xss_string"))]
    pub time_zone: String,

    #[serde(rename = "light_theme")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_theme: Option<Nullable<String>>,

    #[serde(rename = "dark_theme")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_theme: Option<Nullable<String>>,
}

impl ExportTransfer200ResponseSettings {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(language: String, time_zone: String) -> ExportTransfer200ResponseSettings {
        ExportTransfer200ResponseSettings {
            language,
            time_zone,
            light_theme: None,
            dark_theme: None,
        }
    }
}

/// Converts the ExportTransfer200ResponseSettings value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExportTransfer200ResponseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("language".to_string()),
            Some(self.language.to_string()),
            Some("time_zone".to_string()),
            Some(self.time_zone.to_string()),
            self.light_theme.as_ref().map(|light_theme| {
                [
                    "light_theme".to_string(),
                    light_theme
                        .as_ref()
                        .map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
            self.dark_theme.as_ref().map(|dark_theme| {
                [
                    "dark_theme".to_string(),
                    dark_theme
                        .as_ref()
                        .map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExportTransfer200ResponseSettings value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExportTransfer200ResponseSettings {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub language: Vec<String>,
            pub time_zone: Vec<String>,
            pub light_theme: Vec<String>,
            pub dark_theme: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ExportTransfer200ResponseSettings".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "language" => intermediate_rep.language.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "time_zone" => intermediate_rep.time_zone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "light_theme" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ExportTransfer200ResponseSettings".to_string()),
                    "dark_theme" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ExportTransfer200ResponseSettings".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExportTransfer200ResponseSettings".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExportTransfer200ResponseSettings {
            language: intermediate_rep
                .language
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "language missing in ExportTransfer200ResponseSettings".to_string()
                })?,
            time_zone: intermediate_rep
                .time_zone
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "time_zone missing in ExportTransfer200ResponseSettings".to_string()
                })?,
            light_theme: std::result::Result::Err(
                "Nullable types not supported in ExportTransfer200ResponseSettings".to_string(),
            )?,
            dark_theme: std::result::Result::Err(
                "Nullable types not supported in ExportTransfer200ResponseSettings".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExportTransfer200ResponseSettings> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExportTransfer200ResponseSettings>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ExportTransfer200ResponseSettings>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ExportTransfer200ResponseSettings - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ExportTransfer200ResponseSettings>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ExportTransfer200ResponseSettings as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ExportTransfer200ResponseSettings - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExportTransfer200ResponseThemesInner {
    #[serde(rename = "hash")]
    #[validate(custom(function = "check_xss_string"))]
    pub hash: String,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "mode")]
    #[validate(custom(function = "check_xss_string"))]
    pub mode: String,

    #[serde(rename = "primary")]
    #[validate(
            regex(path = *RE_EXPORTTRANSFER200RESPONSETHEMESINNER_PRIMARY),
          custom(function = "check_xss_string"),
    )]
    pub primary: String,

    #[serde(rename = "secondary")]
    #[validate(
            regex(path = *RE_EXPORTTRANSFER200RESPONSETHEMESINNER_SECONDARY),
          custom(function = "check_xss_string"),
    )]
    pub secondary: String,

    #[serde(rename = "tertiary")]
    #[validate(
            regex(path = *RE_EXPORTTRANSFER200RESPONSETHEMESINNER_TERTIARY),
          custom(function = "check_xss_string"),
    )]
    pub tertiary: String,
}

lazy_static::lazy_static! {
    static ref RE_EXPORTTRANSFER200RESPONSETHEMESINNER_PRIMARY: regex::Regex = regex::Regex::new("^#[0-9A-Fa-f]{3}([0-9A-Fa-f]{3})?$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_EXPORTTRANSFER200RESPONSETHEMESINNER_SECONDARY: regex::Regex = regex::Regex::new("^#[0-9A-Fa-f]{3}([0-9A-Fa-f]{3})?$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_EXPORTTRANSFER200RESPONSETHEMESINNER_TERTIARY: regex::Regex = regex::Regex::new("^#[0-9A-Fa-f]{3}([0-9A-Fa-f]{3})?$").unwrap();
}

impl ExportTransfer200ResponseThemesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        hash: String,
        name: String,
        mode: String,
        primary: String,
        secondary: String,
        tertiary: String,
    ) -> ExportTransfer200ResponseThemesInner {
        ExportTransfer200ResponseThemesInner {
            hash,
            name,
            mode,
            primary,
            secondary,
            tertiary,
        }
    }
}

/// Converts the ExportTransfer200ResponseThemesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExportTransfer200ResponseThemesInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("hash".to_string()),
            Some(self.hash.to_string()),
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("mode".to_string()),
            Some(self.mode.to_string()),
            Some("primary".to_string()),
            Some(self.primary.to_string()),
            Some("secondary".to_string()),
            Some(self.secondary.to_string()),
            Some("tertiary".to_string()),
            Some(self.tertiary.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExportTransfer200ResponseThemesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExportTransfer200ResponseThemesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub hash: Vec<String>,
            pub name: Vec<String>,
            pub mode: Vec<String>,
            pub primary: Vec<String>,
            pub secondary: Vec<String>,
            pub tertiary: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ExportTransfer200ResponseThemesInner"
                            .to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "hash" => intermediate_rep.hash.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "mode" => intermediate_rep.mode.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "primary" => intermediate_rep.primary.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "secondary" => intermediate_rep.secondary.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "tertiary" => intermediate_rep.tertiary.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ExportTransfer200ResponseThemesInner"
                                .to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExportTransfer200ResponseThemesInner {
            hash: intermediate_rep.hash.into_iter().next().ok_or_else(|| {
                "hash missing in ExportTransfer200ResponseThemesInner".to_string()
            })?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| {
                "name missing in ExportTransfer200ResponseThemesInner".to_string()
            })?,
            mode: intermediate_rep.mode.into_iter().next().ok_or_else(|| {
                "mode missing in ExportTransfer200ResponseThemesInner".to_string()
            })?,
            primary: intermediate_rep.primary.into_iter().next().ok_or_else(|| {
                "primary missing in ExportTransfer200ResponseThemesInner".to_string()
            })?,
            secondary: intermediate_rep
                .secondary
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "secondary missing in ExportTransfer200ResponseThemesInner".to_string()
                })?,
            tertiary: intermediate_rep
                .tertiary
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "tertiary missing in ExportTransfer200ResponseThemesInner".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExportTransfer200ResponseThemesInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExportTransfer200ResponseThemesInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ExportTransfer200ResponseThemesInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ExportTransfer200ResponseThemesInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ExportTransfer200ResponseThemesInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ExportTransfer200ResponseThemesInner as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ExportTransfer200ResponseThemesInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetCurrentUser200Response {
    #[serde(rename = "id")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub id: String,

    #[serde(rename = "username")]
    #[validate(custom(function = "check_xss_string"))]
    pub username: String,

    #[serde(rename = "first_name")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Nullable<String>>,

    #[serde(rename = "last_name")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Nullable<String>>,

    #[serde(rename = "profile_picture")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<Nullable<String>>,

    #[serde(rename = "groups")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub groups: Vec<String>,
}

impl GetCurrentUser200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, username: String, groups: Vec<String>) -> GetCurrentUser200Response {
        GetCurrentUser200Response {
            id,
            username,
            first_name: None,
            last_name: None,
            profile_picture: None,
            groups,
        }
    }
}

/// Converts the GetCurrentUser200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetCurrentUser200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            Some("username".to_string()),
            Some(self.username.to_string()),
            self.first_name.as_ref().map(|first_name| {
                [
                    "first_name".to_string(),
                    first_name
                        .as_ref()
                        .map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
            self.last_name.as_ref().map(|last_name| {
                [
                    "last_name".to_string(),
                    last_name
                        .as_ref()
                        .map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
            self.profile_picture.as_ref().map(|profile_picture| {
                [
                    "profile_picture".to_string(),
                    profile_picture
                        .as_ref()
                        .map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
            Some("groups".to_string()),
            Some(
                self.groups
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetCurrentUser200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetCurrentUser200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub username: Vec<String>,
            pub first_name: Vec<String>,
            pub last_name: Vec<String>,
            pub profile_picture: Vec<String>,
            pub groups: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GetCurrentUser200Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "first_name" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in GetCurrentUser200Response".to_string()),
                    "last_name" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in GetCurrentUser200Response".to_string()),
                    "profile_picture" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in GetCurrentUser200Response".to_string()),
                    "groups" => return std::result::Result::Err("Parsing a container in this style is not supported in GetCurrentUser200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetCurrentUser200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetCurrentUser200Response {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in GetCurrentUser200Response".to_string())?,
            username: intermediate_rep
                .username
                .into_iter()
                .next()
                .ok_or_else(|| "username missing in GetCurrentUser200Response".to_string())?,
            first_name: std::result::Result::Err(
                "Nullable types not supported in GetCurrentUser200Response".to_string(),
            )?,
            last_name: std::result::Result::Err(
                "Nullable types not supported in GetCurrentUser200Response".to_string(),
            )?,
            profile_picture: std::result::Result::Err(
                "Nullable types not supported in GetCurrentUser200Response".to_string(),
            )?,
            groups: intermediate_rep
                .groups
                .into_iter()
                .next()
                .ok_or_else(|| "groups missing in GetCurrentUser200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetCurrentUser200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetCurrentUser200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GetCurrentUser200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GetCurrentUser200Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<GetCurrentUser200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GetCurrentUser200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into GetCurrentUser200Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDashboard200Response {
    #[serde(rename = "applications")]
    #[validate(nested)]
    pub applications: Vec<models::GetDashboard200ResponseApplicationsInner>,

    /// How your active and shelved categories are sorted.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "categories_sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    pub categories_sort_order: String,

    #[serde(rename = "shelved_categories")]
    #[validate(nested)]
    pub shelved_categories: Vec<models::GetDashboard200ResponseShelvedCategoriesInner>,

    #[serde(rename = "categories")]
    #[validate(nested)]
    pub categories: Vec<models::GetDashboard200ResponseShelvedCategoriesInner>,
}

impl GetDashboard200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        applications: Vec<models::GetDashboard200ResponseApplicationsInner>,
        categories_sort_order: String,
        shelved_categories: Vec<models::GetDashboard200ResponseShelvedCategoriesInner>,
        categories: Vec<models::GetDashboard200ResponseShelvedCategoriesInner>,
    ) -> GetDashboard200Response {
        GetDashboard200Response {
            applications,
            categories_sort_order,
            shelved_categories,
            categories,
        }
    }
}

/// Converts the GetDashboard200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetDashboard200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping applications in query parameter serialization
            Some("categories_sort_order".to_string()),
            Some(self.categories_sort_order.to_string()),
            // Skipping shelved_categories in query parameter serialization

            // Skipping categories in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetDashboard200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetDashboard200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub applications: Vec<Vec<models::GetDashboard200ResponseApplicationsInner>>,
            pub categories_sort_order: Vec<String>,
            pub shelved_categories: Vec<Vec<models::GetDashboard200ResponseShelvedCategoriesInner>>,
            pub categories: Vec<Vec<models::GetDashboard200ResponseShelvedCategoriesInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GetDashboard200Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "applications" => return std::result::Result::Err("Parsing a container in this style is not supported in GetDashboard200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "categories_sort_order" => intermediate_rep.categories_sort_order.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "shelved_categories" => return std::result::Result::Err("Parsing a container in this style is not supported in GetDashboard200Response".to_string()),
                    "categories" => return std::result::Result::Err("Parsing a container in this style is not supported in GetDashboard200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetDashboard200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetDashboard200Response {
            applications: intermediate_rep
                .applications
                .into_iter()
                .next()
                .ok_or_else(|| "applications missing in GetDashboard200Response".to_string())?,
            categories_sort_order: intermediate_rep
                .categories_sort_order
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "categories_sort_order missing in GetDashboard200Response".to_string()
                })?,
            shelved_categories: intermediate_rep
                .shelved_categories
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "shelved_categories missing in GetDashboard200Response".to_string()
                })?,
            categories: intermediate_rep
                .categories
                .into_iter()
                .next()
                .ok_or_else(|| "categories missing in GetDashboard200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetDashboard200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetDashboard200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GetDashboard200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GetDashboard200Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<GetDashboard200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GetDashboard200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into GetDashboard200Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDashboard200ResponseApplicationsInner {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,

    /// Omitted when no health check is configured.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "health")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
}

impl GetDashboard200ResponseApplicationsInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        icon: String,
        address: String,
    ) -> GetDashboard200ResponseApplicationsInner {
        GetDashboard200ResponseApplicationsInner {
            id,
            name,
            icon,
            address,
            health: None,
        }
    }
}

/// Converts the GetDashboard200ResponseApplicationsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetDashboard200ResponseApplicationsInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
            self.health
                .as_ref()
                .map(|health| ["health".to_string(), health.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetDashboard200ResponseApplicationsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetDashboard200ResponseApplicationsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
            pub health: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GetDashboard200ResponseApplicationsInner"
                            .to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "health" => intermediate_rep.health.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GetDashboard200ResponseApplicationsInner"
                                .to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetDashboard200ResponseApplicationsInner {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| {
                "id missing in GetDashboard200ResponseApplicationsInner".to_string()
            })?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| {
                "name missing in GetDashboard200ResponseApplicationsInner".to_string()
            })?,
            icon: intermediate_rep.icon.into_iter().next().ok_or_else(|| {
                "icon missing in GetDashboard200ResponseApplicationsInner".to_string()
            })?,
            address: intermediate_rep.address.into_iter().next().ok_or_else(|| {
                "address missing in GetDashboard200ResponseApplicationsInner".to_string()
            })?,
            health: intermediate_rep.health.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetDashboard200ResponseApplicationsInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetDashboard200ResponseApplicationsInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GetDashboard200ResponseApplicationsInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GetDashboard200ResponseApplicationsInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<GetDashboard200ResponseApplicationsInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GetDashboard200ResponseApplicationsInner as std::str::FromStr>::from_str(
                    value,
                ) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into GetDashboard200ResponseApplicationsInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDashboard200ResponseShelvedCategoriesInner {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "bookmarks_sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    pub bookmarks_sort_order: String,

    #[serde(rename = "bookmarks")]
    #[validate(nested)]
    pub bookmarks: Vec<models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>,
}

impl GetDashboard200ResponseShelvedCategoriesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        bookmarks_sort_order: String,
        bookmarks: Vec<models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>,
    ) -> GetDashboard200ResponseShelvedCategoriesInner {
        GetDashboard200ResponseShelvedCategoriesInner {
            id,
            name,
            bookmarks_sort_order,
            bookmarks,
        }
    }
}

/// Converts the GetDashboard200ResponseShelvedCategoriesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetDashboard200ResponseShelvedCategoriesInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("bookmarks_sort_order".to_string()),
            Some(self.bookmarks_sort_order.to_string()),
            // Skipping bookmarks in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetDashboard200ResponseShelvedCategoriesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetDashboard200ResponseShelvedCategoriesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub name: Vec<String>,
            pub bookmarks_sort_order: Vec<String>,
            pub bookmarks:
                Vec<Vec<models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val =
                match string_iter.next() {
                    Some(x) => x,
                    None => return std::result::Result::Err(
                        "Missing value while parsing GetDashboard200ResponseShelvedCategoriesInner"
                            .to_string(),
                    ),
                };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bookmarks_sort_order" => intermediate_rep.bookmarks_sort_order.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "bookmarks" => return std::result::Result::Err("Parsing a container in this style is not supported in GetDashboard200ResponseShelvedCategoriesInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetDashboard200ResponseShelvedCategoriesInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetDashboard200ResponseShelvedCategoriesInner {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| {
                "id missing in GetDashboard200ResponseShelvedCategoriesInner".to_string()
            })?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| {
                "name missing in GetDashboard200ResponseShelvedCategoriesInner".to_string()
            })?,
            bookmarks_sort_order: intermediate_rep
                .bookmarks_sort_order
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "bookmarks_sort_order missing in GetDashboard200ResponseShelvedCategoriesInner"
                        .to_string()
                })?,
            bookmarks: intermediate_rep
                .bookmarks
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "bookmarks missing in GetDashboard200ResponseShelvedCategoriesInner".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetDashboard200ResponseShelvedCategoriesInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetDashboard200ResponseShelvedCategoriesInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GetDashboard200ResponseShelvedCategoriesInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GetDashboard200ResponseShelvedCategoriesInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<GetDashboard200ResponseShelvedCategoriesInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GetDashboard200ResponseShelvedCategoriesInner as std::str::FromStr>::from_str(
                    value,
                ) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into GetDashboard200ResponseShelvedCategoriesInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    /// Icon name used to display this bookmark.
    #[serde(rename = "icon")]
    #[validate(custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,
}

impl GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        icon: String,
        address: String,
    ) -> GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner {
        GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner {
            id,
            name,
            icon,
            address,
        }
    }
}

/// Converts the GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner".to_string())?,
            icon: intermediate_rep.icon.into_iter().next().ok_or_else(|| "icon missing in GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner".to_string())?,
            address: intermediate_rep.address.into_iter().next().ok_or_else(|| "address missing in GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner> and HeaderValue

#[cfg(feature = "server")]
impl
    std::convert::TryFrom<
        header::IntoHeaderValue<GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>,
    > for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<
            GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner,
        >,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetPreferences200Response {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "language")]
    #[validate(custom(function = "check_xss_string"))]
    pub language: String,

    /// Choose your local time zone or let Dash detect it.
    #[serde(rename = "time_zone")]
    #[validate(custom(function = "check_xss_string"))]
    pub time_zone: String,

    #[serde(rename = "selected_themes")]
    #[validate(nested)]
    pub selected_themes: models::GetPreferences200ResponseSelectedThemes,
}

impl GetPreferences200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        language: String,
        time_zone: String,
        selected_themes: models::GetPreferences200ResponseSelectedThemes,
    ) -> GetPreferences200Response {
        GetPreferences200Response {
            language,
            time_zone,
            selected_themes,
        }
    }
}

/// Converts the GetPreferences200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetPreferences200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("language".to_string()),
            Some(self.language.to_string()),
            Some("time_zone".to_string()),
            Some(self.time_zone.to_string()),
            // Skipping selected_themes in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetPreferences200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetPreferences200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub language: Vec<String>,
            pub time_zone: Vec<String>,
            pub selected_themes: Vec<models::GetPreferences200ResponseSelectedThemes>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GetPreferences200Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "language" => intermediate_rep.language.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "time_zone" => intermediate_rep.time_zone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "selected_themes" => intermediate_rep.selected_themes.push(<models::GetPreferences200ResponseSelectedThemes as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetPreferences200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetPreferences200Response {
            language: intermediate_rep
                .language
                .into_iter()
                .next()
                .ok_or_else(|| "language missing in GetPreferences200Response".to_string())?,
            time_zone: intermediate_rep
                .time_zone
                .into_iter()
                .next()
                .ok_or_else(|| "time_zone missing in GetPreferences200Response".to_string())?,
            selected_themes: intermediate_rep
                .selected_themes
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "selected_themes missing in GetPreferences200Response".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetPreferences200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetPreferences200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GetPreferences200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GetPreferences200Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<GetPreferences200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GetPreferences200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into GetPreferences200Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetPreferences200ResponseSelectedThemes {
    #[serde(rename = "light")]
    pub light: Nullable<String>,

    #[serde(rename = "dark")]
    pub dark: Nullable<String>,
}

impl GetPreferences200ResponseSelectedThemes {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        light: Nullable<String>,
        dark: Nullable<String>,
    ) -> GetPreferences200ResponseSelectedThemes {
        GetPreferences200ResponseSelectedThemes { light, dark }
    }
}

/// Converts the GetPreferences200ResponseSelectedThemes value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GetPreferences200ResponseSelectedThemes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("light".to_string()),
            Some(
                self.light
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("dark".to_string()),
            Some(
                self.dark
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetPreferences200ResponseSelectedThemes value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetPreferences200ResponseSelectedThemes {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub light: Vec<String>,
            pub dark: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GetPreferences200ResponseSelectedThemes"
                            .to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "light" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in GetPreferences200ResponseSelectedThemes".to_string()),
                    "dark" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in GetPreferences200ResponseSelectedThemes".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetPreferences200ResponseSelectedThemes".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetPreferences200ResponseSelectedThemes {
            light: std::result::Result::Err(
                "Nullable types not supported in GetPreferences200ResponseSelectedThemes"
                    .to_string(),
            )?,
            dark: std::result::Result::Err(
                "Nullable types not supported in GetPreferences200ResponseSelectedThemes"
                    .to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetPreferences200ResponseSelectedThemes> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GetPreferences200ResponseSelectedThemes>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GetPreferences200ResponseSelectedThemes>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GetPreferences200ResponseSelectedThemes - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<GetPreferences200ResponseSelectedThemes>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GetPreferences200ResponseSelectedThemes as std::str::FromStr>::from_str(
                    value,
                ) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into GetPreferences200ResponseSelectedThemes - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListApplications200Response {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "applications_sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    pub applications_sort_order: String,

    #[serde(rename = "applications")]
    #[validate(nested)]
    pub applications: Vec<models::ListApplications200ResponseApplicationsInner>,
}

impl ListApplications200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        applications_sort_order: String,
        applications: Vec<models::ListApplications200ResponseApplicationsInner>,
    ) -> ListApplications200Response {
        ListApplications200Response {
            applications_sort_order,
            applications,
        }
    }
}

/// Converts the ListApplications200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ListApplications200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("applications_sort_order".to_string()),
            Some(self.applications_sort_order.to_string()),
            // Skipping applications in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListApplications200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListApplications200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub applications_sort_order: Vec<String>,
            pub applications: Vec<Vec<models::ListApplications200ResponseApplicationsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ListApplications200Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "applications_sort_order" => intermediate_rep.applications_sort_order.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "applications" => return std::result::Result::Err("Parsing a container in this style is not supported in ListApplications200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListApplications200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListApplications200Response {
            applications_sort_order: intermediate_rep
                .applications_sort_order
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "applications_sort_order missing in ListApplications200Response".to_string()
                })?,
            applications: intermediate_rep
                .applications
                .into_iter()
                .next()
                .ok_or_else(|| "applications missing in ListApplications200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListApplications200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ListApplications200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ListApplications200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ListApplications200Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ListApplications200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ListApplications200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ListApplications200Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListApplications200ResponseApplicationsInner {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,

    #[serde(rename = "health_check")]
    pub health_check: Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck>,

    /// Omitted until a configured health check has run.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "health")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,

    /// Choose which groups can see this app. An empty list makes it visible to everyone.
    #[serde(rename = "visible_to_groups")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub visible_to_groups: Vec<String>,
}

impl ListApplications200ResponseApplicationsInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        name: String,
        icon: String,
        address: String,
        health_check: Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck>,
        visible_to_groups: Vec<String>,
    ) -> ListApplications200ResponseApplicationsInner {
        ListApplications200ResponseApplicationsInner {
            id,
            name,
            icon,
            address,
            health_check,
            health: None,
            visible_to_groups,
        }
    }
}

/// Converts the ListApplications200ResponseApplicationsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ListApplications200ResponseApplicationsInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
            // Skipping health_check in query parameter serialization
            self.health
                .as_ref()
                .map(|health| ["health".to_string(), health.to_string()].join(",")),
            Some("visible_to_groups".to_string()),
            Some(
                self.visible_to_groups
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListApplications200ResponseApplicationsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListApplications200ResponseApplicationsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
            pub health_check: Vec<models::ListApplications200ResponseApplicationsInnerHealthCheck>,
            pub health: Vec<String>,
            pub visible_to_groups: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val =
                match string_iter.next() {
                    Some(x) => x,
                    None => return std::result::Result::Err(
                        "Missing value while parsing ListApplications200ResponseApplicationsInner"
                            .to_string(),
                    ),
                };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "health_check" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ListApplications200ResponseApplicationsInner".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "health" => intermediate_rep.health.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "visible_to_groups" => return std::result::Result::Err("Parsing a container in this style is not supported in ListApplications200ResponseApplicationsInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListApplications200ResponseApplicationsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListApplications200ResponseApplicationsInner {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| {
                "id missing in ListApplications200ResponseApplicationsInner".to_string()
            })?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| {
                "name missing in ListApplications200ResponseApplicationsInner".to_string()
            })?,
            icon: intermediate_rep.icon.into_iter().next().ok_or_else(|| {
                "icon missing in ListApplications200ResponseApplicationsInner".to_string()
            })?,
            address: intermediate_rep.address.into_iter().next().ok_or_else(|| {
                "address missing in ListApplications200ResponseApplicationsInner".to_string()
            })?,
            health_check: std::result::Result::Err(
                "Nullable types not supported in ListApplications200ResponseApplicationsInner"
                    .to_string(),
            )?,
            health: intermediate_rep.health.into_iter().next(),
            visible_to_groups: intermediate_rep
                .visible_to_groups
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "visible_to_groups missing in ListApplications200ResponseApplicationsInner"
                        .to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListApplications200ResponseApplicationsInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ListApplications200ResponseApplicationsInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ListApplications200ResponseApplicationsInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ListApplications200ResponseApplicationsInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ListApplications200ResponseApplicationsInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ListApplications200ResponseApplicationsInner as std::str::FromStr>::from_str(
                    value,
                ) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ListApplications200ResponseApplicationsInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

/// Omit or set to null to turn checks off. If `url` is omitted, Dash checks the app's address.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListApplications200ResponseApplicationsInnerHealthCheck {
    /// Optional HTTP or HTTPS address to check instead of the app address.
    #[serde(rename = "url")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Nullable<String>>,

    /// HTTP response class that means the app is up.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "expected_status_class")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_status_class: Option<String>,

    /// Allow invalid or self-signed certificates. Use only for trusted internal services.
    #[serde(rename = "ignore_tls_errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_tls_errors: Option<bool>,
}

impl ListApplications200ResponseApplicationsInnerHealthCheck {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ListApplications200ResponseApplicationsInnerHealthCheck {
        ListApplications200ResponseApplicationsInnerHealthCheck {
            url: None,
            expected_status_class: None,
            ignore_tls_errors: Some(false),
        }
    }
}

/// Converts the ListApplications200ResponseApplicationsInnerHealthCheck value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ListApplications200ResponseApplicationsInnerHealthCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.url.as_ref().map(|url| {
                [
                    "url".to_string(),
                    url.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
            self.expected_status_class
                .as_ref()
                .map(|expected_status_class| {
                    [
                        "expected_status_class".to_string(),
                        expected_status_class.to_string(),
                    ]
                    .join(",")
                }),
            self.ignore_tls_errors.as_ref().map(|ignore_tls_errors| {
                [
                    "ignore_tls_errors".to_string(),
                    ignore_tls_errors.to_string(),
                ]
                .join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListApplications200ResponseApplicationsInnerHealthCheck value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListApplications200ResponseApplicationsInnerHealthCheck {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub url: Vec<String>,
            pub expected_status_class: Vec<String>,
            pub ignore_tls_errors: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListApplications200ResponseApplicationsInnerHealthCheck".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "url" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ListApplications200ResponseApplicationsInnerHealthCheck".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "expected_status_class" => intermediate_rep.expected_status_class.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ignore_tls_errors" => intermediate_rep.ignore_tls_errors.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListApplications200ResponseApplicationsInnerHealthCheck".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListApplications200ResponseApplicationsInnerHealthCheck {
            url: std::result::Result::Err("Nullable types not supported in ListApplications200ResponseApplicationsInnerHealthCheck".to_string())?,
            expected_status_class: intermediate_rep.expected_status_class.into_iter().next(),
            ignore_tls_errors: intermediate_rep.ignore_tls_errors.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListApplications200ResponseApplicationsInnerHealthCheck> and HeaderValue

#[cfg(feature = "server")]
impl
    std::convert::TryFrom<
        header::IntoHeaderValue<ListApplications200ResponseApplicationsInnerHealthCheck>,
    > for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ListApplications200ResponseApplicationsInnerHealthCheck>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ListApplications200ResponseApplicationsInnerHealthCheck - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ListApplications200ResponseApplicationsInnerHealthCheck>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListApplications200ResponseApplicationsInnerHealthCheck as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ListApplications200ResponseApplicationsInnerHealthCheck - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListSessions200ResponseInner {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    /// When this device signed in.
    #[serde(rename = "issued_at")]
    pub issued_at: chrono::DateTime<chrono::Utc>,

    /// Most recent activity from this device.
    #[serde(rename = "last_accessed_at")]
    pub last_accessed_at: chrono::DateTime<chrono::Utc>,

    /// When this device will need to sign in again.
    #[serde(rename = "expires_at")]
    pub expires_at: chrono::DateTime<chrono::Utc>,

    /// Whether this device is still signed in.
    #[serde(rename = "is_active")]
    pub is_active: bool,

    /// Whether this is the device making this request.
    #[serde(rename = "is_current")]
    pub is_current: bool,

    /// Whether this device uses a short or long sign-in.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "lifetime")]
    #[validate(custom(function = "check_xss_string"))]
    pub lifetime: String,

    #[serde(rename = "metadata")]
    #[validate(nested)]
    pub metadata: models::ListSessions200ResponseInnerMetadata,
}

impl ListSessions200ResponseInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: uuid::Uuid,
        issued_at: chrono::DateTime<chrono::Utc>,
        last_accessed_at: chrono::DateTime<chrono::Utc>,
        expires_at: chrono::DateTime<chrono::Utc>,
        is_active: bool,
        is_current: bool,
        lifetime: String,
        metadata: models::ListSessions200ResponseInnerMetadata,
    ) -> ListSessions200ResponseInner {
        ListSessions200ResponseInner {
            id,
            issued_at,
            last_accessed_at,
            expires_at,
            is_active,
            is_current,
            lifetime,
            metadata,
        }
    }
}

/// Converts the ListSessions200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ListSessions200ResponseInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization

            // Skipping issued_at in query parameter serialization

            // Skipping last_accessed_at in query parameter serialization

            // Skipping expires_at in query parameter serialization
            Some("is_active".to_string()),
            Some(self.is_active.to_string()),
            Some("is_current".to_string()),
            Some(self.is_current.to_string()),
            Some("lifetime".to_string()),
            Some(self.lifetime.to_string()),
            // Skipping metadata in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListSessions200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListSessions200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub issued_at: Vec<chrono::DateTime<chrono::Utc>>,
            pub last_accessed_at: Vec<chrono::DateTime<chrono::Utc>>,
            pub expires_at: Vec<chrono::DateTime<chrono::Utc>>,
            pub is_active: Vec<bool>,
            pub is_current: Vec<bool>,
            pub lifetime: Vec<String>,
            pub metadata: Vec<models::ListSessions200ResponseInnerMetadata>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ListSessions200ResponseInner".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "issued_at" => intermediate_rep.issued_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last_accessed_at" => intermediate_rep.last_accessed_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expires_at" => intermediate_rep.expires_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_active" => intermediate_rep.is_active.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_current" => intermediate_rep.is_current.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lifetime" => intermediate_rep.lifetime.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "metadata" => intermediate_rep.metadata.push(<models::ListSessions200ResponseInnerMetadata as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListSessions200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListSessions200ResponseInner {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in ListSessions200ResponseInner".to_string())?,
            issued_at: intermediate_rep
                .issued_at
                .into_iter()
                .next()
                .ok_or_else(|| "issued_at missing in ListSessions200ResponseInner".to_string())?,
            last_accessed_at: intermediate_rep
                .last_accessed_at
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "last_accessed_at missing in ListSessions200ResponseInner".to_string()
                })?,
            expires_at: intermediate_rep
                .expires_at
                .into_iter()
                .next()
                .ok_or_else(|| "expires_at missing in ListSessions200ResponseInner".to_string())?,
            is_active: intermediate_rep
                .is_active
                .into_iter()
                .next()
                .ok_or_else(|| "is_active missing in ListSessions200ResponseInner".to_string())?,
            is_current: intermediate_rep
                .is_current
                .into_iter()
                .next()
                .ok_or_else(|| "is_current missing in ListSessions200ResponseInner".to_string())?,
            lifetime: intermediate_rep
                .lifetime
                .into_iter()
                .next()
                .ok_or_else(|| "lifetime missing in ListSessions200ResponseInner".to_string())?,
            metadata: intermediate_rep
                .metadata
                .into_iter()
                .next()
                .ok_or_else(|| "metadata missing in ListSessions200ResponseInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListSessions200ResponseInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ListSessions200ResponseInner>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ListSessions200ResponseInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ListSessions200ResponseInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ListSessions200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ListSessions200ResponseInner as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ListSessions200ResponseInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListSessions200ResponseInnerMetadata {
    /// Browser used by this device, when available.
    #[serde(rename = "browser")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<Nullable<String>>,

    /// Operating system used by this device, when available.
    #[serde(rename = "operating_system")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<Nullable<String>>,

    #[serde(rename = "ip_address")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Nullable<models::ListSessions200ResponseInnerMetadataIpAddress>>,
}

impl ListSessions200ResponseInnerMetadata {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ListSessions200ResponseInnerMetadata {
        ListSessions200ResponseInnerMetadata {
            browser: None,
            operating_system: None,
            ip_address: None,
        }
    }
}

/// Converts the ListSessions200ResponseInnerMetadata value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ListSessions200ResponseInnerMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.browser.as_ref().map(|browser| {
                [
                    "browser".to_string(),
                    browser
                        .as_ref()
                        .map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
            self.operating_system.as_ref().map(|operating_system| {
                [
                    "operating_system".to_string(),
                    operating_system
                        .as_ref()
                        .map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
            // Skipping ip_address in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListSessions200ResponseInnerMetadata value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListSessions200ResponseInnerMetadata {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub browser: Vec<String>,
            pub operating_system: Vec<String>,
            pub ip_address: Vec<models::ListSessions200ResponseInnerMetadataIpAddress>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ListSessions200ResponseInnerMetadata"
                            .to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "browser" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ListSessions200ResponseInnerMetadata".to_string()),
                    "operating_system" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ListSessions200ResponseInnerMetadata".to_string()),
                    "ip_address" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ListSessions200ResponseInnerMetadata".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListSessions200ResponseInnerMetadata".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListSessions200ResponseInnerMetadata {
            browser: std::result::Result::Err(
                "Nullable types not supported in ListSessions200ResponseInnerMetadata".to_string(),
            )?,
            operating_system: std::result::Result::Err(
                "Nullable types not supported in ListSessions200ResponseInnerMetadata".to_string(),
            )?,
            ip_address: std::result::Result::Err(
                "Nullable types not supported in ListSessions200ResponseInnerMetadata".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListSessions200ResponseInnerMetadata> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ListSessions200ResponseInnerMetadata>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ListSessions200ResponseInnerMetadata>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ListSessions200ResponseInnerMetadata - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ListSessions200ResponseInnerMetadata>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ListSessions200ResponseInnerMetadata as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ListSessions200ResponseInnerMetadata - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

/// Network address seen the last time this device connected.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum ListSessions200ResponseInnerMetadataIpAddress {
    String(String),
    String1(String),
}

impl validator::Validate for ListSessions200ResponseInnerMetadataIpAddress {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::String(_) => std::result::Result::Ok(()),
            Self::String1(_) => std::result::Result::Ok(()),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListSessions200ResponseInnerMetadataIpAddress value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListSessions200ResponseInnerMetadataIpAddress {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListThemes200ResponseInner {
    /// Namespaced theme ID, either global:<name> or user:<uuid>.
    #[serde(rename = "id")]
    #[validate(custom(function = "check_xss_string"))]
    pub id: String,

    #[serde(rename = "name")]
    #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "mode")]
    #[validate(custom(function = "check_xss_string"))]
    pub mode: String,

    #[serde(rename = "palette")]
    #[validate(nested)]
    pub palette: models::ListThemes200ResponseInnerPalette,

    /// Whether this theme comes with Dash or was created by you.
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "scope")]
    #[validate(custom(function = "check_xss_string"))]
    pub scope: String,
}

impl ListThemes200ResponseInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: String,
        name: String,
        mode: String,
        palette: models::ListThemes200ResponseInnerPalette,
        scope: String,
    ) -> ListThemes200ResponseInner {
        ListThemes200ResponseInner {
            id,
            name,
            mode,
            palette,
            scope,
        }
    }
}

/// Converts the ListThemes200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ListThemes200ResponseInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("mode".to_string()),
            Some(self.mode.to_string()),
            // Skipping palette in query parameter serialization
            Some("scope".to_string()),
            Some(self.scope.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListThemes200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListThemes200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub mode: Vec<String>,
            pub palette: Vec<models::ListThemes200ResponseInnerPalette>,
            pub scope: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ListThemes200ResponseInner".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "mode" => intermediate_rep.mode.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "palette" => intermediate_rep.palette.push(
                        <models::ListThemes200ResponseInnerPalette as std::str::FromStr>::from_str(
                            val,
                        )
                        .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "scope" => intermediate_rep.scope.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ListThemes200ResponseInner".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListThemes200ResponseInner {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in ListThemes200ResponseInner".to_string())?,
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ListThemes200ResponseInner".to_string())?,
            mode: intermediate_rep
                .mode
                .into_iter()
                .next()
                .ok_or_else(|| "mode missing in ListThemes200ResponseInner".to_string())?,
            palette: intermediate_rep
                .palette
                .into_iter()
                .next()
                .ok_or_else(|| "palette missing in ListThemes200ResponseInner".to_string())?,
            scope: intermediate_rep
                .scope
                .into_iter()
                .next()
                .ok_or_else(|| "scope missing in ListThemes200ResponseInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListThemes200ResponseInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ListThemes200ResponseInner>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ListThemes200ResponseInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ListThemes200ResponseInner - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ListThemes200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ListThemes200ResponseInner as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ListThemes200ResponseInner - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

/// The three colors used to draw your theme.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListThemes200ResponseInnerPalette {
    #[serde(rename = "primary")]
    #[validate(
            regex(path = *RE_LISTTHEMES200RESPONSEINNERPALETTE_PRIMARY),
          custom(function = "check_xss_string"),
    )]
    pub primary: String,

    #[serde(rename = "secondary")]
    #[validate(
            regex(path = *RE_LISTTHEMES200RESPONSEINNERPALETTE_SECONDARY),
          custom(function = "check_xss_string"),
    )]
    pub secondary: String,

    #[serde(rename = "tertiary")]
    #[validate(
            regex(path = *RE_LISTTHEMES200RESPONSEINNERPALETTE_TERTIARY),
          custom(function = "check_xss_string"),
    )]
    pub tertiary: String,
}

lazy_static::lazy_static! {
    static ref RE_LISTTHEMES200RESPONSEINNERPALETTE_PRIMARY: regex::Regex = regex::Regex::new("^#[0-9A-Fa-f]{3}([0-9A-Fa-f]{3})?$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_LISTTHEMES200RESPONSEINNERPALETTE_SECONDARY: regex::Regex = regex::Regex::new("^#[0-9A-Fa-f]{3}([0-9A-Fa-f]{3})?$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_LISTTHEMES200RESPONSEINNERPALETTE_TERTIARY: regex::Regex = regex::Regex::new("^#[0-9A-Fa-f]{3}([0-9A-Fa-f]{3})?$").unwrap();
}

impl ListThemes200ResponseInnerPalette {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        primary: String,
        secondary: String,
        tertiary: String,
    ) -> ListThemes200ResponseInnerPalette {
        ListThemes200ResponseInnerPalette {
            primary,
            secondary,
            tertiary,
        }
    }
}

/// Converts the ListThemes200ResponseInnerPalette value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ListThemes200ResponseInnerPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("primary".to_string()),
            Some(self.primary.to_string()),
            Some("secondary".to_string()),
            Some(self.secondary.to_string()),
            Some("tertiary".to_string()),
            Some(self.tertiary.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListThemes200ResponseInnerPalette value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListThemes200ResponseInnerPalette {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub primary: Vec<String>,
            pub secondary: Vec<String>,
            pub tertiary: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ListThemes200ResponseInnerPalette".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "primary" => intermediate_rep.primary.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "secondary" => intermediate_rep.secondary.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "tertiary" => intermediate_rep.tertiary.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ListThemes200ResponseInnerPalette"
                                .to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListThemes200ResponseInnerPalette {
            primary: intermediate_rep.primary.into_iter().next().ok_or_else(|| {
                "primary missing in ListThemes200ResponseInnerPalette".to_string()
            })?,
            secondary: intermediate_rep
                .secondary
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "secondary missing in ListThemes200ResponseInnerPalette".to_string()
                })?,
            tertiary: intermediate_rep
                .tertiary
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "tertiary missing in ListThemes200ResponseInnerPalette".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListThemes200ResponseInnerPalette> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ListThemes200ResponseInnerPalette>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ListThemes200ResponseInnerPalette>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ListThemes200ResponseInnerPalette - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ListThemes200ResponseInnerPalette>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ListThemes200ResponseInnerPalette as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ListThemes200ResponseInnerPalette - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

/// Dash returns this information when it can't complete a request. The app can use `code` to respond appropriately and show `message` to you.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Problem {
    #[serde(rename = "type")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_type: Option<String>,

    #[serde(rename = "title")]
    #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    #[serde(rename = "status")]
    #[validate(range(min = 400u16, max = 599u16))]
    pub status: u16,

    #[serde(rename = "detail")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    #[serde(rename = "instance")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Short label that helps the app handle this problem.
    #[serde(rename = "code")]
    #[validate(custom(function = "check_xss_string"))]
    pub code: String,

    /// Short, user-readable explanation of the error.
    #[serde(rename = "message")]
    #[validate(custom(function = "check_xss_string"))]
    pub message: String,
}

impl Problem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, status: u16, code: String, message: String) -> Problem {
        Problem {
            r_type: Some(r#"about:blank"#.to_string()),
            title,
            status,
            detail: None,
            instance: None,
            code,
            message,
        }
    }
}

/// Converts the Problem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.r_type
                .as_ref()
                .map(|r_type| ["type".to_string(), r_type.to_string()].join(",")),
            Some("title".to_string()),
            Some(self.title.to_string()),
            Some("status".to_string()),
            Some(self.status.to_string()),
            self.detail
                .as_ref()
                .map(|detail| ["detail".to_string(), detail.to_string()].join(",")),
            self.instance
                .as_ref()
                .map(|instance| ["instance".to_string(), instance.to_string()].join(",")),
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("message".to_string()),
            Some(self.message.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Problem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<u16>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
            pub code: Vec<String>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Problem".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <u16 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Problem".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Problem {
            r_type: intermediate_rep.r_type.into_iter().next(),
            title: intermediate_rep
                .title
                .into_iter()
                .next()
                .ok_or_else(|| "title missing in Problem".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in Problem".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in Problem".to_string())?,
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or_else(|| "message missing in Problem".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Problem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Problem>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Problem>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for Problem - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Problem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Problem as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into Problem - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReorderBookmarksRequest {
    #[serde(rename = "bookmark_ids")]
    #[validate()]
    pub bookmark_ids: Vec<uuid::Uuid>,
}

impl ReorderBookmarksRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(bookmark_ids: Vec<uuid::Uuid>) -> ReorderBookmarksRequest {
        ReorderBookmarksRequest { bookmark_ids }
    }
}

/// Converts the ReorderBookmarksRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ReorderBookmarksRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping bookmark_ids in query parameter serialization

        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ReorderBookmarksRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ReorderBookmarksRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bookmark_ids: Vec<Vec<uuid::Uuid>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ReorderBookmarksRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "bookmark_ids" => return std::result::Result::Err("Parsing a container in this style is not supported in ReorderBookmarksRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ReorderBookmarksRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ReorderBookmarksRequest {
            bookmark_ids: intermediate_rep
                .bookmark_ids
                .into_iter()
                .next()
                .ok_or_else(|| "bookmark_ids missing in ReorderBookmarksRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ReorderBookmarksRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ReorderBookmarksRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ReorderBookmarksRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ReorderBookmarksRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ReorderBookmarksRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ReorderBookmarksRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ReorderBookmarksRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReorderCategoriesRequest {
    /// List every category ID in the order you want. Include shelved categories.
    #[serde(rename = "category_ids")]
    #[validate()]
    pub category_ids: Vec<uuid::Uuid>,
}

impl ReorderCategoriesRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(category_ids: Vec<uuid::Uuid>) -> ReorderCategoriesRequest {
        ReorderCategoriesRequest { category_ids }
    }
}

/// Converts the ReorderCategoriesRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ReorderCategoriesRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping category_ids in query parameter serialization

        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ReorderCategoriesRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ReorderCategoriesRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub category_ids: Vec<Vec<uuid::Uuid>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ReorderCategoriesRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "category_ids" => return std::result::Result::Err("Parsing a container in this style is not supported in ReorderCategoriesRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ReorderCategoriesRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ReorderCategoriesRequest {
            category_ids: intermediate_rep
                .category_ids
                .into_iter()
                .next()
                .ok_or_else(|| "category_ids missing in ReorderCategoriesRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ReorderCategoriesRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ReorderCategoriesRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ReorderCategoriesRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ReorderCategoriesRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ReorderCategoriesRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ReorderCategoriesRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ReorderCategoriesRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateApplicationCatalogOrderRequest {
    /// Complete custom order of every application in the catalog.
    #[serde(rename = "application_ids")]
    #[validate()]
    pub application_ids: Vec<uuid::Uuid>,
}

impl UpdateApplicationCatalogOrderRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(application_ids: Vec<uuid::Uuid>) -> UpdateApplicationCatalogOrderRequest {
        UpdateApplicationCatalogOrderRequest { application_ids }
    }
}

/// Converts the UpdateApplicationCatalogOrderRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UpdateApplicationCatalogOrderRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping application_ids in query parameter serialization

        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateApplicationCatalogOrderRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateApplicationCatalogOrderRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub application_ids: Vec<Vec<uuid::Uuid>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UpdateApplicationCatalogOrderRequest"
                            .to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "application_ids" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateApplicationCatalogOrderRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateApplicationCatalogOrderRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateApplicationCatalogOrderRequest {
            application_ids: intermediate_rep
                .application_ids
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "application_ids missing in UpdateApplicationCatalogOrderRequest".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateApplicationCatalogOrderRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateApplicationCatalogOrderRequest>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UpdateApplicationCatalogOrderRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for UpdateApplicationCatalogOrderRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<UpdateApplicationCatalogOrderRequest>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UpdateApplicationCatalogOrderRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into UpdateApplicationCatalogOrderRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateApplicationRequest {
    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "icon")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub icon: String,

    #[serde(rename = "address")]
    #[validate(custom(function = "check_xss_string"))]
    pub address: String,

    #[serde(rename = "health_check")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check:
        Option<Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck>>,

    /// Choose which groups can see this app. An empty list makes it visible to everyone.
    #[serde(rename = "visible_to_groups")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub visible_to_groups: Vec<String>,
}

impl UpdateApplicationRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        icon: String,
        address: String,
        visible_to_groups: Vec<String>,
    ) -> UpdateApplicationRequest {
        UpdateApplicationRequest {
            name,
            icon,
            address,
            health_check: None,
            visible_to_groups,
        }
    }
}

/// Converts the UpdateApplicationRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UpdateApplicationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("icon".to_string()),
            Some(self.icon.to_string()),
            Some("address".to_string()),
            Some(self.address.to_string()),
            // Skipping health_check in query parameter serialization
            Some("visible_to_groups".to_string()),
            Some(
                self.visible_to_groups
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateApplicationRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateApplicationRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub icon: Vec<String>,
            pub address: Vec<String>,
            pub health_check: Vec<models::ListApplications200ResponseApplicationsInnerHealthCheck>,
            pub visible_to_groups: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UpdateApplicationRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "health_check" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in UpdateApplicationRequest".to_string()),
                    "visible_to_groups" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateApplicationRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateApplicationRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateApplicationRequest {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in UpdateApplicationRequest".to_string())?,
            icon: intermediate_rep
                .icon
                .into_iter()
                .next()
                .ok_or_else(|| "icon missing in UpdateApplicationRequest".to_string())?,
            address: intermediate_rep
                .address
                .into_iter()
                .next()
                .ok_or_else(|| "address missing in UpdateApplicationRequest".to_string())?,
            health_check: std::result::Result::Err(
                "Nullable types not supported in UpdateApplicationRequest".to_string(),
            )?,
            visible_to_groups: intermediate_rep
                .visible_to_groups
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "visible_to_groups missing in UpdateApplicationRequest".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateApplicationRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateApplicationRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UpdateApplicationRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for UpdateApplicationRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UpdateApplicationRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UpdateApplicationRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into UpdateApplicationRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateCategoriesSortOrderRequest {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    pub sort_order: String,
}

impl UpdateCategoriesSortOrderRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(sort_order: String) -> UpdateCategoriesSortOrderRequest {
        UpdateCategoriesSortOrderRequest { sort_order }
    }
}

/// Converts the UpdateCategoriesSortOrderRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UpdateCategoriesSortOrderRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("sort_order".to_string()),
            Some(self.sort_order.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateCategoriesSortOrderRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateCategoriesSortOrderRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub sort_order: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UpdateCategoriesSortOrderRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "sort_order" => intermediate_rep.sort_order.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UpdateCategoriesSortOrderRequest"
                                .to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateCategoriesSortOrderRequest {
            sort_order: intermediate_rep
                .sort_order
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "sort_order missing in UpdateCategoriesSortOrderRequest".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateCategoriesSortOrderRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateCategoriesSortOrderRequest>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UpdateCategoriesSortOrderRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for UpdateCategoriesSortOrderRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<UpdateCategoriesSortOrderRequest>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UpdateCategoriesSortOrderRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into UpdateCategoriesSortOrderRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateCategoryRequest {
    #[serde(rename = "name")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "shelved")]
    pub shelved: bool,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "sort_order")]
    #[validate(custom(function = "check_xss_string"))]
    pub sort_order: String,
}

impl UpdateCategoryRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, shelved: bool, sort_order: String) -> UpdateCategoryRequest {
        UpdateCategoryRequest {
            name,
            shelved,
            sort_order,
        }
    }
}

/// Converts the UpdateCategoryRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UpdateCategoryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("shelved".to_string()),
            Some(self.shelved.to_string()),
            Some("sort_order".to_string()),
            Some(self.sort_order.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateCategoryRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateCategoryRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub shelved: Vec<bool>,
            pub sort_order: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UpdateCategoryRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shelved" => intermediate_rep.shelved.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "sort_order" => intermediate_rep.sort_order.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UpdateCategoryRequest".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateCategoryRequest {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in UpdateCategoryRequest".to_string())?,
            shelved: intermediate_rep
                .shelved
                .into_iter()
                .next()
                .ok_or_else(|| "shelved missing in UpdateCategoryRequest".to_string())?,
            sort_order: intermediate_rep
                .sort_order
                .into_iter()
                .next()
                .ok_or_else(|| "sort_order missing in UpdateCategoryRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateCategoryRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateCategoryRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UpdateCategoryRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for UpdateCategoryRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UpdateCategoryRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UpdateCategoryRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into UpdateCategoryRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdatePreferencesRequest {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "language")]
    #[validate(custom(function = "check_xss_string"))]
    pub language: String,

    /// Choose your local time zone or let Dash detect it.
    #[serde(rename = "time_zone")]
    #[validate(custom(function = "check_xss_string"))]
    pub time_zone: String,

    #[serde(rename = "selected_themes")]
    #[validate(nested)]
    pub selected_themes: models::GetPreferences200ResponseSelectedThemes,
}

impl UpdatePreferencesRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        language: String,
        time_zone: String,
        selected_themes: models::GetPreferences200ResponseSelectedThemes,
    ) -> UpdatePreferencesRequest {
        UpdatePreferencesRequest {
            language,
            time_zone,
            selected_themes,
        }
    }
}

/// Converts the UpdatePreferencesRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UpdatePreferencesRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("language".to_string()),
            Some(self.language.to_string()),
            Some("time_zone".to_string()),
            Some(self.time_zone.to_string()),
            // Skipping selected_themes in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdatePreferencesRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdatePreferencesRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub language: Vec<String>,
            pub time_zone: Vec<String>,
            pub selected_themes: Vec<models::GetPreferences200ResponseSelectedThemes>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UpdatePreferencesRequest".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "language" => intermediate_rep.language.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "time_zone" => intermediate_rep.time_zone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "selected_themes" => intermediate_rep.selected_themes.push(<models::GetPreferences200ResponseSelectedThemes as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdatePreferencesRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdatePreferencesRequest {
            language: intermediate_rep
                .language
                .into_iter()
                .next()
                .ok_or_else(|| "language missing in UpdatePreferencesRequest".to_string())?,
            time_zone: intermediate_rep
                .time_zone
                .into_iter()
                .next()
                .ok_or_else(|| "time_zone missing in UpdatePreferencesRequest".to_string())?,
            selected_themes: intermediate_rep
                .selected_themes
                .into_iter()
                .next()
                .ok_or_else(|| "selected_themes missing in UpdatePreferencesRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdatePreferencesRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdatePreferencesRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UpdatePreferencesRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for UpdatePreferencesRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UpdatePreferencesRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UpdatePreferencesRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into UpdatePreferencesRequest - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

/// Dash returns this information when it can't complete a request. The app can use `code` to respond appropriately and show `message` to you.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpgradeSession409Response {
    #[serde(rename = "type")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_type: Option<String>,

    #[serde(rename = "title")]
    #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    #[serde(rename = "status")]
    #[validate(range(min = 400u16, max = 599u16))]
    pub status: u16,

    #[serde(rename = "detail")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    #[serde(rename = "instance")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Short label that helps the app handle this problem.
    #[serde(rename = "code")]
    #[validate(custom(function = "check_xss_string"))]
    pub code: String,

    /// Short, user-readable explanation of the error.
    #[serde(rename = "message")]
    #[validate(custom(function = "check_xss_string"))]
    pub message: String,
}

impl UpgradeSession409Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        title: String,
        status: u16,
        code: String,
        message: String,
    ) -> UpgradeSession409Response {
        UpgradeSession409Response {
            r_type: Some(r#"about:blank"#.to_string()),
            title,
            status,
            detail: None,
            instance: None,
            code,
            message,
        }
    }
}

/// Converts the UpgradeSession409Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UpgradeSession409Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.r_type
                .as_ref()
                .map(|r_type| ["type".to_string(), r_type.to_string()].join(",")),
            Some("title".to_string()),
            Some(self.title.to_string()),
            Some("status".to_string()),
            Some(self.status.to_string()),
            self.detail
                .as_ref()
                .map(|detail| ["detail".to_string(), detail.to_string()].join(",")),
            self.instance
                .as_ref()
                .map(|instance| ["instance".to_string(), instance.to_string()].join(",")),
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("message".to_string()),
            Some(self.message.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpgradeSession409Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpgradeSession409Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<u16>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
            pub code: Vec<String>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UpgradeSession409Response".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <u16 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UpgradeSession409Response".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpgradeSession409Response {
            r_type: intermediate_rep.r_type.into_iter().next(),
            title: intermediate_rep
                .title
                .into_iter()
                .next()
                .ok_or_else(|| "title missing in UpgradeSession409Response".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in UpgradeSession409Response".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in UpgradeSession409Response".to_string())?,
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or_else(|| "message missing in UpgradeSession409Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpgradeSession409Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UpgradeSession409Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UpgradeSession409Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for UpgradeSession409Response - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UpgradeSession409Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UpgradeSession409Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into UpgradeSession409Response - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}
