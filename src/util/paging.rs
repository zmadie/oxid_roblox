//! Paging functionality
//!
//! To consume a [PageIterator] returned by a function, you can use the
//! [PageIterator::into_stream] method to get a [Stream](https://docs.rs/futures-core/latest/futures_core/stream/trait.Stream.html)
//! of the items. In conjunction with [StreamExt](https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html)
//! from the `futures_util` crate, you can do many sorts of manipulations
//! on the stream. A full example:
//!
//! ```
//! use futures_util::{pin_mut, StreamExt};
//! use oxid_roblox::derives::{GroupDerive, UserDerive};
//!
//! #[tokio::main]
//! async fn main() {
//!     let user = oxid_roblox::base_user(1);
//!     let username_iterator = user.username_history();
//!
//!     let first_5_usernames = username_iterator
//!         .into_stream()
//!         .take(5)
//!         .map(|result| result.unwrap())
//!         .collect::<Vec<_>>()
//!         .await;
//!
//!     println!("{:?}", first_5_usernames);
//!
//!     let group = oxid_roblox::base_group(1);
//!     let members_stream = group.members().into_stream();
//!     pin_mut!(members_stream);
//!
//!     while let Some(Ok(member)) = members_stream.next().await {
//!         println!("{:?}", member);
//!     }
//! }
//! ```
//!
//! To change how many items per page should be fetched, use the
//! [PageIterator::page_size] method. To change the sort order, use the
//! [PageIterator::sort_order] method. For example:
//!
//! ```
//! let usernames_iterator = oxid_roblox::base_user(1)
//!     .username_history()
//!     .page_size(PageSize::OneHundred)
//!     .sort_order(SortOrder::Descending);
//! ```

use async_stream::stream;
use async_trait::async_trait;
use futures_core::stream::Stream;

use super::{api_helper, responses::PageResponse, RobloxResult};

pub(crate) fn identity_mapper<T: Clone>(data: &T) -> T {
    data.clone()
}

pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    fn serialize(&self) -> String {
        match self {
            SortOrder::Ascending => "Asc",
            SortOrder::Descending => "Desc",
        }
        .to_owned()
    }
}

pub enum PageSize {
    Ten = 10,
    TwentyFive = 25,
    Fifty = 50,
    OneHundred = 100,
}

impl PageSize {
    fn serialize(&self) -> String {
        match self {
            PageSize::Ten => "10",
            PageSize::TwentyFive => "25",
            PageSize::Fifty => "50",
            PageSize::OneHundred => "100",
        }
        .to_owned()
    }
}

// An iterator for all pages of a PageIterator
struct PagesIterator<T> {
    iterator: Box<dyn BasePageIterator<T>>,
    current_page_position: i32,
    current_page_data: Vec<T>,
}

impl<T: Clone> PagesIterator<T> {
    fn new(iterator: Box<dyn BasePageIterator<T>>) -> Self {
        Self {
            iterator,
            current_page_position: 0,
            current_page_data: Vec::new(),
        }
    }

    fn into_stream(mut self) -> impl Stream<Item = RobloxResult<T>> {
        stream! {
            loop {
                if self.current_page_position == self.current_page_data.len() as i32 {
                    match self.iterator.next_page().await.transpose() {
                        Some(Ok(data)) => self.current_page_data = data,
                        Some(Err(err)) => {
                            yield Err(err);
                            break
                        },
                        None => break,
                    };

                    if self.current_page_data.is_empty() {
                        break;
                    }
                    self.current_page_position = 0;
                }

                yield Ok(self.current_page_data[self.current_page_position as usize].clone());
                self.current_page_position += 1;
            }
        }
    }
}

#[async_trait(?Send)]
trait BasePageIterator<T> {
    async fn next_page(&mut self) -> RobloxResult<Option<Vec<T>>>;
}

pub struct PageIterator<T, U>
where
    T: serde::de::DeserializeOwned,
    U: Clone,
{
    url: String,
    mapper: fn(&T) -> U,
    sort_order: SortOrder,
    page_size: PageSize,
    iteration_started: bool,
    next_cursor: Option<String>,
}

impl<T, U> PageIterator<T, U>
where
    T: serde::de::DeserializeOwned + 'static,
    U: Clone + 'static,
{
    pub fn new(url: String, mapper: fn(&T) -> U) -> Self {
        Self {
            url,
            mapper,
            sort_order: SortOrder::Ascending,
            page_size: PageSize::Ten,
            iteration_started: false,
            next_cursor: None,
        }
    }

    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = sort_order;
        self
    }

    pub fn page_size(mut self, page_size: PageSize) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn into_stream(self) -> impl Stream<Item = RobloxResult<U>> {
        PagesIterator::new(Box::new(self)).into_stream()
    }
}

#[async_trait(?Send)]
impl<T, U> BasePageIterator<U> for PageIterator<T, U>
where
    T: serde::de::DeserializeOwned,
    U: Clone,
{
    async fn next_page(&mut self) -> RobloxResult<Option<Vec<U>>> {
        // Just checking for self.next_cursor.is_none() would make single-page sized results return no data. This logic allows for fetching atleast one page
        if self.iteration_started && self.next_cursor.is_none() {
            return Ok(None);
        }
        self.iteration_started = true;

        let page = api_helper::deserialize_body::<PageResponse<T>>(
            api_helper::get(format!(
                "{}?sortOrder={}&limit={}&cursor={}",
                self.url,
                self.sort_order.serialize(),
                self.page_size.serialize(),
                self.next_cursor.clone().unwrap_or(String::new())
            ))
            .await?,
        )
        .await;

        self.next_cursor = page.next_page_cursor;

        Ok(Some(page.data.iter().map(self.mapper).collect()))
    }
}
