//! Traits that provide shared methods for structs
//!
//! Because of the `async_trait` crate, the definition of each function gets
//! messy. The actual return of each function is after `Future<Output = ...>`.
//!
//! As an example of interpretation:
//!
//! ```
//! fn update_shout<'life0, 'async_trait>(
//!     &'life0 mut self,
//!     message: String
//! ) -> Pin<Box<dyn Future<Output = RobloxResult<GroupShout>> + 'async_trait>>
//! where
//!     Self: 'async_trait,
//!     'life0: 'async_trait
//! ```
//!
//! Becomes this:
//!
//! ```
//! async fn update_shout(
//!     &mut self,
//!     message: String
//! ) -> RobloxResult<GroupShout>>
//! ```
//!
//! You can check out all the traits provided below. To see which structs
//! implement each trait, check the "Implementors" section (at the bottom of
//! each page).

mod asset;
mod group;
mod plugin;
mod universe;
mod user;

pub use asset::Asset as AssetDerive;
pub use group::Group as GroupDerive;
pub use plugin::Plugin as PluginDerive;
pub use universe::Universe as UniverseDerive;
pub use user::User as UserDerive;
