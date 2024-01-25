use chrono::{DateTime, Utc};
use feed_rs::model as feedrs;

pub use feedrs::FeedType;

pub type Time = DateTime<Utc>;

pub struct EntryRef<'a> {
    entry: &'a feedrs::Entry,
}

impl<'a> EntryRef<'a> {
    pub fn title(&self) -> Option<&str> {
        self.entry.title.as_ref().map(|text| text.content.as_str())
    }

    pub fn published(&self) -> Option<Time> {
        self.entry.published
    }

    pub fn summary(&self) -> Option<&str> {
        self.entry
            .summary
            .as_ref()
            .map(|text| text.content.as_str())
    }

    /// Return approximate entry bytes size
    pub fn approximate_size(&self) -> usize {
        let content_size = self
            .entry
            .content
            .as_ref()
            .and_then(|content| content.body.as_deref())
            .map(|body| body.len())
            .unwrap_or(0);

        let summary_size = self
            .entry
            .summary
            .as_ref()
            .map(|summary| summary.content.len())
            .unwrap_or(0);

        content_size + summary_size
    }
}

#[derive(Debug, Clone)]
pub struct Feed {
    url: String,
    #[allow(dead_code)]
    feed: feedrs::Feed,
}

impl Feed {
    pub fn r#type(&self) -> FeedType {
        self.feed.feed_type.clone()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn title(&self) -> Option<&str> {
        self.feed.title.as_ref().map(|text| text.content.as_str())
    }

    pub fn updated(&self) -> Option<Time> {
        self.feed.updated
    }

    pub fn entry_refs(&self) -> impl Iterator<Item = EntryRef> {
        self.feed.entries.iter().map(|entry| EntryRef { entry })
    }

    pub fn authors(&self) -> impl Iterator<Item = &str> {
        self.feed.authors.iter().map(|person| person.name.as_str())
    }

    pub fn description(&self) -> Option<&str> {
        self.feed
            .description
            .as_ref()
            .map(|text| text.content.as_str())
    }

    pub fn links(&self) -> impl Iterator<Item = &feedrs::Link> {
        self.feed.links.iter()
    }

    /// Return website link to which feed syndicate
    pub fn website_url(&self) -> Option<&str> {
        link::find_website_url(self.r#type(), &self.feed.links)
    }

    // for debug
    pub fn without_entries(&self) -> Self {
        let mut clone = self.clone();
        clone.feed.entries.clear();
        clone
    }

    /// Return approximate Feed byte size
    pub fn approximate_size(&self) -> usize {
        self.entry_refs()
            .map(|entry| entry.approximate_size())
            .sum()
    }
}

impl From<(String, feed_rs::model::Feed)> for Feed {
    fn from(feed: (String, feed_rs::model::Feed)) -> Self {
        Feed {
            url: feed.0,
            feed: feed.1,
        }
    }
}

mod link {
    use feed_rs::model::{FeedType, Link};

    pub fn find_website_url<'a>(
        feed_type: FeedType,
        links: impl IntoIterator<Item = &'a Link>,
    ) -> Option<&'a str> {
        let mut links = links.into_iter();
        match feed_type {
            // Find rel == alternate link
            FeedType::Atom => links
                .find(|link| link.rel.as_deref() == Some("alternate"))
                .map(|link| link.href.as_str()),

            // TODO
            FeedType::JSON => todo!(),

            // TODO
            FeedType::RSS0 => todo!(),

            // Use the first link whose rel is not "self"
            FeedType::RSS1 | FeedType::RSS2 => links
                .find(|link| link.rel.as_deref() != Some("self"))
                .map(|link| link.href.as_str()),
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn rss_ignore_rel_self() {
            let links = vec![
                Link {
                    href: "https://syndicationd.ymgyt.io/".into(),
                    title: None,
                    rel: None,
                    media_type: None,
                    href_lang: None,
                    length: None,
                },
                Link {
                    href: "https://syndicationd.ymgyt.io/atom.xml".into(),
                    title: None,
                    rel: Some("self".into()),
                    media_type: None,
                    href_lang: None,
                    length: None,
                },
            ];

            assert_eq!(
                find_website_url(FeedType::RSS1, &links),
                Some("https://syndicationd.ymgyt.io/")
            );
            assert_eq!(
                find_website_url(FeedType::RSS2, &links),
                Some("https://syndicationd.ymgyt.io/")
            );
        }

        #[test]
        fn atom_use_rel_alternate() {
            let links = vec![
                Link {
                    href: "https://syndicationd.ymgyt.io/atom.xml".into(),
                    title: None,
                    rel: Some("self".into()),
                    media_type: None,
                    href_lang: None,
                    length: None,
                },
                Link {
                    href: "https://syndicationd.ymgyt.io/".into(),
                    title: None,
                    rel: Some("alternate".into()),
                    media_type: None,
                    href_lang: None,
                    length: None,
                },
            ];

            assert_eq!(
                find_website_url(FeedType::Atom, &links),
                Some("https://syndicationd.ymgyt.io/")
            );
        }
    }
}
