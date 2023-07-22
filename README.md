<div align="center">
  <img src="https://raw.githubusercontent.com/roblox-api-wrappers/.github/main/res/oxid_roblox_logo.png" alt="rublox" width="15%" height="15%"/>
</div>

<br>

<div align="center">
  <a href="https://crates.io/crates/oxid_roblox">
    <img alt="downloads" src="https://img.shields.io/crates/d/oxid_roblox?style=flat-square&color=%23ff3a16">
  </a>

  <a href="https://crates.io/crates/oxid_roblox">
    <img alt="crate version" src="https://img.shields.io/crates/v/oxid_roblox?style=flat-square&label=version&color=%23ff3a16">
  </a>

  <a href="https://github.com/zmadie/oxid_roblox/blob/main/LICENSE">
    <img alt="license: MIT" src="https://img.shields.io/github/license/zmadie/oxid_roblox?color=%23ff3a16&logo=github&style=flat-square">
  </a>

  <img alt="commit activity" src="https://img.shields.io/github/commit-activity/m/zmadie/oxid_roblox?color=%23ff3a16&logo=github&style=flat-square">

  <img alt="repo stars" src="https://img.shields.io/github/stars/zmadie/oxid_roblox?color=%23ff3a16&logo=github&style=flat-square">

  <a href="https://docs.rs/oxid_roblox">
    <img alt="docs" src="https://img.shields.io/docsrs/oxid_roblox?logo=docs.rs&label=docs.rs&color=%23ff3a16">
  </a>
</div>

<div align="center"> <i>oxid_roblox is a Roblox web API wrapper written in Rust. It aims to provide an interface to get and modify data from Roblox's web API. </i> </div>

<br>

# Getting Started

Authentication can be done by setting the .ROBLOSECURITY cookie through `oxid_roblox::set_roblosecurity`:

```rust
oxid_roblox::set_roblosecurity("cookie");

// with dotenv
use dotenv::dotenv;
use std::env;

dotenv().ok();

oxid_roblox::set_roblosecurity(&env::var("COOKIE").unwrap());
```

If a method that requires authentication is called without setting the .ROBLOSECURITY cookie first, the program will panic.

All structs representing the Roblox API models should not be initialised by the user. Instead, they are all built through methods in modules and other structs.

Models can be found in the `oxid_roblox::models` module:

![](https://cdn.discordapp.com/attachments/827652175609856053/1196655346987573248/image.png)

And methods for getting the corresponding model can be found by searching for the model in the "In Names" category:

![](https://cdn.discordapp.com/attachments/827652175609856053/1196655956881313812/image.png)

# Documentation

You can access the documentation at <https://docs.rs/oxid_roblox>.
