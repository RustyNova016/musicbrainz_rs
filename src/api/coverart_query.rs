use core::fmt::Write as _;
use core::marker::PhantomData;

use crate::client::MUSICBRAINZ_CLIENT;
use crate::entity::CoverartResolution;
use crate::entity::CoverartResponse;
use crate::entity::CoverartTarget;
use crate::entity::CoverartType;
use crate::APIPath;
use crate::ApiRequest;
use crate::MusicBrainzClient;

/// Perform a lookup of an entity's coverart when you have the MBID for that entity
///
/// # Lookups
///
/// You can perform a lookup of an entity's coverart when you have the MBID for that entity.
///
/// ## Example
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[tokio::main]
/// # #[cfg(feature = "async")]
/// # async fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::release::Release;
/// # use musicbrainz_rs::entity::CoverartResponse;
/// let in_utero_coverart = Release::fetch_coverart()
///         .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
///         .execute()
///         .await?;
///
/// if let CoverartResponse::Json(coverart) = in_utero_coverart {
///     assert_eq!(coverart.images[0].front, true);
///     assert_eq!(coverart.images[0].back, false);
/// } else {
///     assert!(false);
/// }
/// #   Ok(())
/// # }
/// # #[cfg(feature = "blocking")]
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::release::Release;
/// # use musicbrainz_rs::entity::CoverartResponse;
/// let in_utero_coverart = Release::fetch_coverart()
///         .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
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

    /// Turn the query into an [`crate::ApiRequest`]
    pub fn as_api_request(&mut self, client: &crate::MusicBrainzClient) -> ApiRequest {
        self.validate();

        let url = format!("{}/{}", client.coverart_archive_url, &self.0.path);

        ApiRequest::new(url)
    }

    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<CoverartResponse, crate::Error> {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &MusicBrainzClient,
    ) -> Result<CoverartResponse, crate::Error> {
        let response = client
            .send_with_retries(self.as_api_request(client))
            .await?;

        let coverart_response = if self.0.target.img_type.is_some() {
            let url = response.url().clone();
            CoverartResponse::Url(url.to_string())
        } else {
            CoverartResponse::Json(response.json().await?)
        };

        Ok(coverart_response)
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
