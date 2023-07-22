//! Structs representing a model with only an ID
//!
//! Base structs are useful when you want to call a method on a model but don’t
//! want to send any unnecessary requests to fetch the model’s data.
//! For example, see this code:
//!
//! ```
//! let user = oxid_roblox::user_from_id(1).await.unwrap();
//! println!("Follower count: {}", user.follower_count().await.unwrap());
//! ```
//!
//! The [user_from_id](crate::user_from_id) function will send a request to the Roblox API to fetch
//! the user’s data, yet we do not access any of its fields; we only use it to
//! call a method. This is where base structures come in handy:
//!
//! ```
//! let user = oxid_roblox::base_user(1);
//! println!("Follower count: {}", user.follower_count().await.unwrap());
//! ```
//!
//! Instead of sending two requests, this code only sends one. This may be
//! useful in situations where you could be rate-limited.
//!
//! To know which methods send requests, they are usually marked as `async`.
//! Examples of methods that do not send requests are ones that return bases
//! or a [PageIterator](crate::util::paging::PageIterator`).

mod base_asset;
mod base_group;
mod base_plugin;
mod base_universe;
mod base_user;

pub use base_asset::BaseAsset;
pub use base_group::BaseGroup;
pub use base_plugin::BasePlugin;
pub use base_universe::BaseUniverse;
pub use base_user::BaseUser;
