use crate::{Crunchyroll, Executor, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) use proc_macros::{Available, FromId, Request};

/// Contains a variable amount of items and the maximum / total of item which are available.
/// Mostly used when fetching pagination results.
#[derive(Debug, Deserialize)]
#[serde(bound = "T: Request + DeserializeOwned")]
#[cfg_attr(feature = "__test_strict", serde(deny_unknown_fields))]
#[cfg_attr(
    not(feature = "__test_strict"),
    serde(default),
    derive(smart_default::SmartDefault)
)]
pub struct BulkResult<T: Request + DeserializeOwned> {
    #[cfg_attr(not(feature = "__test_strict"), default(Vec::new()))]
    pub items: Vec<T>,
    pub total: u32,
}

impl<T: Request + DeserializeOwned> Request for BulkResult<T> {
    fn __set_executor(&mut self, executor: Arc<Executor>) {
        for item in self.items.iter_mut() {
            item.__set_executor(executor.clone())
        }
    }
}

/// The standard representation of images how the api returns them.
#[derive(Clone, Debug, Deserialize, Default)]
#[cfg_attr(feature = "__test_strict", serde(deny_unknown_fields))]
#[cfg_attr(not(feature = "__test_strict"), serde(default))]
pub struct Image {
    pub source: String,
    #[serde(rename(deserialize = "type"))]
    pub image_type: String,
    pub height: u32,
    pub width: u32,
}

/// Helper trait for [`Crunchyroll::request`] generic returns.
/// Must be implemented for every struct which is used as generic parameter for [`Crunchyroll::request`].
#[doc(hidden)]
pub trait Request {
    /// Set a usable [`Executor`] instance to the struct if required
    fn __set_executor(&mut self, _: Arc<Executor>) {}

    /// Get the [`Executor`] instance of the struct which implements this trait (if available).
    fn __get_executor(&self) -> Option<Arc<Executor>> {
        None
    }
}

/// Implement [`Request`] for cases where only the request must be done without needing an
/// explicit result.
impl Request for () {}

impl<K, V> Request for HashMap<K, V> {}

/// Check if further actions with the struct which implements this are available.
pub trait Available: Request {
    /// Returns if the current episode, series, ... is available.
    fn available(&self) -> bool;
}

/// Every instance of the struct which implements this can be constructed by an id
#[async_trait::async_trait]
pub trait FromId {
    /// Creates a new [`Self`] by the provided id or returns an [`CrunchyrollError`] if something
    /// caused an issue.
    async fn from_id(crunchy: &Crunchyroll, id: String) -> Result<Self>
    where
        Self: Sized;
}
