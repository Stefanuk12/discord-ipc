//! This library provides easy access to the Discord IPC.
//!
//! It provides implementations for both Unix and Windows
//! operating systems, with both implementations using the
//! same API. Thus, this crate can be used in a platform-agnostic
//! manner.
//!
//! # Hello world
//! ```
//! use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = DiscordIpcClient::new("<some client id>");
//!     client.connect()?;
//!
//!     let payload = activity::Activity::new().state("Hello world!");
//!     client.set_activity(payload)?;
//! }
//! ```
#![deny(missing_docs)]

mod discord_ipc;
mod pack_unpack;
pub use discord_ipc::*;
pub mod activity;

#[cfg(unix)]
mod ipc_unix;
#[cfg(unix)]
use ipc_unix as ipc;

#[cfg(windows)]
mod ipc_windows;
#[cfg(windows)]
use ipc_windows as ipc;

pub use ipc::DiscordIpcClient;

#[deprecated(since = "0.2.0", note = "use DiscordIpcClient::new() instead")]
/// Creates a new client to connect to the Discord IPC. Functionally
/// identical to [`DiscordIpcClient::new()`].
///
/// # Examples
/// ```
/// let ipc_client = discord_ipc_client::new_client("<some client id>");
/// ```
pub fn new_client(client_id: &str) -> impl DiscordIpc {
    ipc::DiscordIpcClient::new(client_id)
}

/// The error type for this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Any IO errors.
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// Any String FromUtf8 errors.
    #[error(transparent)]
    String(#[from] std::string::FromUtf8Error),
    /// Errors from the serde_json crate.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// The opcode was malformed while attempting to unpack.
    #[error("malformed opcode")]
    MalformedOpcode,
    /// The header was malformed while attempting to unpack.
    #[error("malformed header")]
    MalformedHeader,
    /// Could not connect to the Discord IPC socket.
    #[error("could not connect to the Discord IPC socket: {0}")]
    CouldNotConnect(std::io::Error),
    /// Could not resolve the pipe pattern (exclusive to unix)
    #[cfg(unix)]
    #[error("could not resolve the pipe pattern")]
    CouldNotResolvePipePattern,
}

/// The result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;