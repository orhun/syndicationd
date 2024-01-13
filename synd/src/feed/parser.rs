use std::time::Duration;

use async_trait::async_trait;
use feed_rs::parser::Parser;

use crate::types::Feed;

pub type ParseResult<T> = std::result::Result<T, ParserError>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("fetch failed")]
    Fetch(#[from] reqwest::Error),
    #[error("response size limit exceeded")]
    ResponseLimitExceed,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub trait FetchFeed: Send + Sync {
    async fn fetch(&self, url: String) -> ParseResult<Feed>;
}

/// Feed Process entry point
pub struct FeedService {
    http: reqwest::Client,
    buff_limit: usize,
}

#[async_trait]
impl FetchFeed for FeedService {
    async fn fetch(&self, url: String) -> ParseResult<Feed> {
        use futures::StreamExt;
        let mut stream = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(ParserError::Fetch)?
            .error_for_status()
            .map_err(ParserError::Fetch)?
            .bytes_stream();

        let mut buff = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(ParserError::Fetch)?;
            if buff.len() + chunk.len() > self.buff_limit {
                return Err(ParserError::ResponseLimitExceed);
            }
            buff.extend(chunk);
        }

        self.parse(url, buff.as_slice())
    }
}

impl FeedService {
    pub fn new(user_agent: &str, buff_limit: usize) -> Self {
        let http = reqwest::ClientBuilder::new()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self { http, buff_limit }
    }

    pub fn parse<S>(&self, url: impl Into<String>, source: S) -> ParseResult<Feed>
    where
        S: std::io::Read,
    {
        let url = url.into();
        let parser = self.build_parser(&url);

        match parser.parse(source) {
            Ok(feed) => Ok(Feed::from((url, feed))),
            // TODO: handle error
            Err(err) => Err(ParserError::Other(err.into())),
        }
    }

    fn build_parser(&self, base_uri: impl AsRef<str>) -> Parser {
        feed_rs::parser::Builder::new()
            .base_uri(Some(base_uri))
            .build()
    }
}
