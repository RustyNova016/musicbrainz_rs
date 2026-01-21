use core::fmt::Write as _;
use core::marker::PhantomData;
use core::str::FromStr;

use api_bindium::ApiRequest;
use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::UriBuilderError;
use api_bindium::ureq::http::Uri;

use crate::APIPath;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::MusicBrainzClient;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::api::ApiEndpointError;
use crate::entity::CoverartResolution;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::entity::CoverartResponse;
use crate::entity::CoverartTarget;
use crate::entity::CoverartType;
use crate::entity::coverart::Coverart;

/// Perform a lookup of an entity's coverart when you have the MBID for that entity
///
/// # Lookups
///
/// You can perform a lookup of an entity's coverart when you have the MBID for that entity.
///
/// ## Example
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[cfg(feature = "sync")]
/// # fn main() -> Result<(), musicbrainz_rs::ApiEndpointError> {
/// # use musicbrainz_rs::entity::release::Release;
/// # use musicbrainz_rs::entity::CoverartResponse;
/// let in_utero_coverart = Release::fetch_coverart()
///         .id("ee1be6eb-a435-49f8-9053-5117f60e83c2")
///         .execute()?;
///
/// if let CoverartResponse::Json(coverart) = in_utero_coverart {
///     assert_eq!(coverart.images[0].front, true);
///     assert_eq!(coverart.images[0].back, false);
/// } else {
///     assert!(false);
/// }
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct CoverartQuery<T> {
    pub path: String,
    pub target: CoverartTarget,
    pub phantom: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct FetchCoverartQuery<T>(pub CoverartQuery<T>);

impl<T> FetchCoverartQuery<T>
where
    T: Clone + FetchCoverart,
{
    pub fn id(&mut self, id: &str) -> &mut Self {
        let _ = write!(self.0.path, "/{id}");
        self
    }

    pub fn front(&mut self) -> &mut Self {
        if self.0.target.img_type.is_some() {
            println!("ignoring call to `front`, since coverart type has already been set");
        }
        self.0.target.img_type = Some(CoverartType::Front);
        self
    }

    pub fn back(&mut self) -> &mut Self {
        if self.0.target.img_type.is_some() {
            println!("ignoring call to `back`, since coverart type has already been set");
        }
        self.0.target.img_type = Some(CoverartType::Back);
        self
    }

    pub fn res_250(&mut self) -> &mut Self {
        if self.0.target.img_res.is_some() {
            println!("ignoring call to `res_250`, since resolution has already been set");
        }
        self.0.target.img_res = Some(CoverartResolution::Res250);
        self
    }

    pub fn res_500(&mut self) -> &mut Self {
        if self.0.target.img_res.is_some() {
            println!("ignoring call to `res_500`, since resolution has already been set");
        }
        self.0.target.img_res = Some(CoverartResolution::Res500);
        self
    }

    pub fn res_1200(&mut self) -> &mut Self {
        if self.0.target.img_res.is_some() {
            println!("ignoring call to `res_1200`, since resolution has already been set");
        }
        self.0.target.img_res = Some(CoverartResolution::Res1200);
        self
    }

    pub fn validate(&mut self) {
        if let Some(img_type) = &self.0.target.img_type {
            let _ = write!(self.0.path, "/{}", img_type.as_str());
            if let Some(img_res) = &self.0.target.img_res {
                let _ = write!(self.0.path, "-{}", img_res.as_str());
            }
        } else if self.0.target.img_res.is_some() {
            // Implicitly assume coverart type as front in the case when resolution is
            // explicitly specified but coverart type is not.
            self.front().validate();
        }
    }

    /// Turn the query into an [`api_bindium::ApiRequest`]
    pub fn as_api_request(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<ApiRequest<JsonParser<Coverart>>, UriBuilderError> {
        self.validate();

        let url = format!("{}/{}", client.coverart_archive_url, &self.0.path);

        Ok(ApiRequest::builder()
            .uri(Uri::from_str(&url).unwrap())
            .verb(api_bindium::HTTPVerb::Get)
            .build())
    }

    #[cfg(feature = "sync")]
    pub fn execute(&mut self) -> Result<CoverartResponse, ApiEndpointError> {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client(&MUSICBRAINZ_CLIENT)
    }

    #[cfg(feature = "sync")]
    pub fn execute_with_client(
        &mut self,
        client: &MusicBrainzClient,
    ) -> Result<CoverartResponse, ApiEndpointError> {
        use snafu::ResultExt;

        use crate::api::ApiRequestSnafu;
        use crate::api::InvalidUriSnafu;

        let mut req = self.as_api_request(client).context(InvalidUriSnafu)?;

        let response = req
            .send_with_retries(&client.api_client)
            .context(ApiRequestSnafu)?;

        // If we requested a specific image, we have a redirect in return
        if self.0.target.img_type.is_some() {
            use api_bindium::ureq::ResponseExt;

            let redirect = response.get_uri();
            Ok(CoverartResponse::Url(redirect.to_string()))
        } else {
            Ok(CoverartResponse::Json(
                req.parse_response(response).context(ApiRequestSnafu)?,
            ))
        }
    }

    #[cfg(feature = "async")]
    pub async fn execute_async(&mut self) -> Result<CoverartResponse, ApiEndpointError> {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client_async(&MUSICBRAINZ_CLIENT).await
    }

    #[cfg(feature = "async")]
    pub async fn execute_with_client_async(
        &mut self,
        client: &MusicBrainzClient,
    ) -> Result<CoverartResponse, ApiEndpointError> {
        use snafu::ResultExt;

        use crate::api::ApiRequestSnafu;
        use crate::api::InvalidUriSnafu;

        let mut req = self.as_api_request(client).context(InvalidUriSnafu)?;

        let response = req
            .send_with_retries_async(&client.api_client)
            .await
            .context(ApiRequestSnafu)?;

        // If we requested a specific image, we have a redirect in return
        if self.0.target.img_type.is_some() {
            use api_bindium::ureq::ResponseExt;

            let redirect = response.get_uri();
            Ok(CoverartResponse::Url(redirect.to_string()))
        } else {
            Ok(CoverartResponse::Json(
                req.parse_response(response).context(ApiRequestSnafu)?,
            ))
        }
    }
}

/// Implemented by all fetchable coverart entities (see [`FetchCoverartQuery`])
pub trait FetchCoverart {
    fn fetch_coverart() -> FetchCoverartQuery<Self>
    where
        Self: Sized + APIPath,
    {
        FetchCoverartQuery(CoverartQuery {
            path: Self::path().to_string(),
            phantom: PhantomData,
            target: CoverartTarget {
                img_type: None,
                img_res: None,
            },
        })
    }

    fn get_coverart(&self) -> FetchCoverartQuery<Self>
    where
        Self: Sized + APIPath,
        Self: Clone,
    {
        FetchCoverartQuery(CoverartQuery {
            path: Self::path().to_string(),
            phantom: PhantomData,
            target: CoverartTarget {
                img_type: None,
                img_res: None,
            },
        })
    }
}
