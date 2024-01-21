use async_graphql::{InputObject, Object, Union};

use crate::{
    gql::{
        mutation::ResponseStatus,
        object::{self, Feed},
    },
    usecase,
};

#[derive(InputObject)]
pub struct SubscribeFeedInput {
    /// Feed url to subscribe
    pub url: String,
}

impl From<SubscribeFeedInput> for usecase::SubscribeFeedInput {
    fn from(value: SubscribeFeedInput) -> Self {
        usecase::SubscribeFeedInput { url: value.url }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
pub enum SubscribeFeedResponse {
    Success(SubscribeFeedSuccess),
    Error(SubscribeFeedError),
}

pub struct SubscribeFeedSuccess {
    pub status: ResponseStatus,
    /// Subscribed feed
    pub feed: object::Feed,
}

#[Object]
impl SubscribeFeedSuccess {
    pub async fn status(&self) -> ResponseStatus {
        self.status.clone()
    }

    pub async fn feed(&self) -> &object::Feed {
        &self.feed
    }
}

pub struct SubscribeFeedError {
    pub status: ResponseStatus,
    pub message: String,
}

#[Object]
impl SubscribeFeedError {
    pub async fn status(&self) -> ResponseStatus {
        self.status.clone()
    }

    /// Error message
    pub async fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<ResponseStatus> for SubscribeFeedResponse {
    fn from(status: ResponseStatus) -> Self {
        SubscribeFeedResponse::Error(SubscribeFeedError {
            status,
            message: "Unauthorized".into(),
        })
    }
}

impl From<usecase::Output<usecase::SubscribeFeedOutput>> for SubscribeFeedResponse {
    fn from(output: usecase::Output<usecase::SubscribeFeedOutput>) -> Self {
        SubscribeFeedResponse::Success(SubscribeFeedSuccess {
            status: ResponseStatus::ok(),
            feed: Feed::from(output.output.feed),
        })
    }
}
