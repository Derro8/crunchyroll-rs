use serde::Deserialize;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::common::{Available, FromId, Image, Request};
use crate::{Crunchyroll, enum_values, Executor, Locale};
use crate::error::Result;

enum_values!{
    MediaType,
    Series = "series",
    Movie = "movie_listing"
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "__test_strict", serde(deny_unknown_fields))]
#[cfg_attr(not(feature = "__test_strict"), serde(default), derive(Default))]
pub struct MovieListingImages {
    pub poster_tall: Vec<Vec<Image>>,
    pub poster_wide: Vec<Vec<Image>>
}

/// This struct represents a movie collection.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "__test_strict", serde(deny_unknown_fields))]
#[cfg_attr(not(feature = "__test_strict"), serde(default), derive(smart_default::SmartDefault))]
pub struct MovieListing {
    #[serde(skip)]
    executor: Arc<Executor>,

    pub id: String,
    pub channel_id: String,

    pub slug: String,
    pub title: String,
    pub slug_title: String,
    pub seo_title: String,
    pub description: String,
    pub seo_description: String,
    pub extended_description: String,

    pub movie_release_year: u32,
    pub content_provider: String,

    pub keywords: Vec<String>,
    pub season_tags: Vec<String>,

    pub images: MovieListingImages,

    pub is_subbed: bool,
    pub is_dubbed: bool,
    pub subtitle_locales: Vec<Locale>,

    pub hd_flag: bool,
    pub is_premium_only: bool,

    pub maturity_ratings: Vec<String>,
    pub is_mature: bool,
    pub mature_blocked: bool,

    #[cfg_attr(not(feature = "__test_strict"), default(DateTime::<Utc>::from(std::time::SystemTime::UNIX_EPOCH)))]
    pub free_available_date: DateTime<Utc>,
    #[cfg_attr(not(feature = "__test_strict"), default(DateTime::<Utc>::from(std::time::SystemTime::UNIX_EPOCH)))]
    pub premium_available_date: DateTime<Utc>,

    pub available_offline: bool,
    pub availability_notes: String,

    #[cfg(feature = "__test_strict")]
    extended_maturity_rating: crate::StrictValue,
    #[cfg(feature = "__test_strict")]
    available_date: crate::StrictValue,
    #[cfg(feature = "__test_strict")]
    premium_date: crate::StrictValue
}

impl Request for MovieListing {
    fn set_executor(&mut self, executor: Arc<Executor>) {
        self.executor = executor
    }
}

impl Available for MovieListing {
    fn available(&self) -> bool {
        !self.is_premium_only || self.executor.config.clone().premium
    }
}

#[async_trait::async_trait]
impl FromId for MovieListing {
    async fn from_id(crunchy: &Crunchyroll, id: String) -> Result<Self> {
        let executor = crunchy.executor.clone();

        let endpoint = format!("https://beta-api.crunchyroll.com/cms/v2/{}/movie_listings/{}", executor.config.bucket, id);
        let builder = executor.client
            .get(endpoint)
            .query(&executor.media_query());

        executor.request(builder).await
    }
}

type SeriesImages = MovieListingImages;

/// This struct represents a crunchyroll series.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "__test_strict", serde(deny_unknown_fields))]
#[cfg_attr(not(feature = "__test_strict"), serde(default), derive(smart_default::SmartDefault))]
pub struct Series {
    #[serde(skip)]
    executor: Arc<Executor>,

    pub id: String,
    pub channel_id: String,

    pub slug: String,
    pub title: String,
    pub slug_title: String,
    pub seo_title: String,
    pub description: String,
    pub seo_description: String,
    pub extended_description: String,

    pub series_launch_year: u32,
    pub content_provider: String,

    pub episode_count: u32,
    pub season_count: u32,
    pub media_count: u32,

    pub keywords: Vec<String>,
    pub season_tags: Vec<String>,

    pub images: SeriesImages,

    pub is_subbed: bool,
    pub is_dubbed: bool,
    pub is_simulcast: bool,
    pub audio_locales: Vec<Locale>,
    pub subtitle_locales: Vec<Locale>,

    pub maturity_ratings: Vec<String>,
    pub is_mature: bool,
    pub mature_blocked: bool,

    pub availability_notes: String,

    #[cfg(feature = "__test_strict")]
    extended_maturity_rating: crate::StrictValue
}

impl Request for Series {
    fn set_executor(&mut self, executor: Arc<Executor>) {
        self.executor = executor
    }
}

impl Available for Series {
    fn available(&self) -> bool {
        self.channel_id.is_empty() || self.executor.config.premium
    }
}

#[async_trait::async_trait]
impl FromId for Series {
    async fn from_id(crunchy: &Crunchyroll, id: String) -> Result<Self> {
        let executor = crunchy.executor.clone();

        let endpoint = format!("https://beta-api.crunchyroll.com/cms/v2/{}/series/{}", executor.config.bucket, id);
        let builder = executor.client
            .get(endpoint)
            .query(&executor.media_query());

        executor.request(builder).await
    }
}
