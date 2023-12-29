# Discord Rich Presence
[![crates.io](https://img.shields.io/crates/v/discord-rich-presence.svg)](https://crates.io/crates/discord-ipc-rp)
[![Docs](https://docs.rs/discord-rich-presence/badge.svg?version=0.2.3)](https://docs.rs/discord-ipc-rp)


A simple, cross-platform crate to connect and send data to Discord's IPC. Special attention is given to sending rich presence data.

## Example
```rust
use discord_ipc::{Result, activity, DiscordIpc, DiscordIpcClient};

fn main() -> Result<()> {
    let mut client = DiscordIpcClient::new("<some application ID>");

    client.connect()?;
    client.set_activity(activity::Activity::new()
        .state("foo")
        .details("bar")
    )?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    client.close()?;

    Ok(())
}
```

## Contributions

Contributions are welcome! Please open an issue if you have any questions or suggestions. Here's a list of things that need to be done:
- [ ] Optimisations
- [ ] Reduce crate size
- [ ] Add more descriptive structs (instead of using `serde_json::Value`)
