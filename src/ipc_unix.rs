use crate::Empty;
use crate::{Error, Result, discord_ipc::DiscordIpc};
use std::os::unix::net::UnixStream;
use std::{
    env::var,
    io::{Read, Write},
    net::Shutdown,
    path::PathBuf,
};

// Environment keys to search for the Discord pipe
const ENV_KEYS: [&str; 4] = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];


#[allow(dead_code)]
/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
    /// Client ID of the IPC client.
    pub client_id: String,
    connected: bool,
    socket: Option<UnixStream>,
}

impl DiscordIpcClient {
    /// Creates a new `DiscordIpcClient`.
    ///
    /// # Examples
    /// ```
    /// let ipc_client = DiscordIpcClient::new("<some client id>");
    /// ```
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            connected: false,
            socket: None,
        }
    }

    fn get_pipe_pattern() -> Result<PathBuf> {
        ENV_KEYS
            .iter()
            .find_map(|key| var(key).ok().map(|val| PathBuf::from(val)))
            .ok_or(Error::CouldNotResolvePipePattern)
    }
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self) -> Result<()> {
        let iter = 0..10;
        let last = iter.end - 1;
        let base_path = DiscordIpcClient::get_pipe_pattern()?;
        for i in iter {
            let path = base_path.join(format!("discord-ipc-{}", i));

            match UnixStream::connect(&path) {
                Ok(socket) => {
                    self.socket = Some(socket);
                    return Ok(());
                }
                Err(e) => if i == last { Err(e)? } else { continue }
            }
        }

        // this should never happen
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let socket = self.socket.as_mut().expect("Client not connected");

        socket.write_all(data)?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let socket = self.socket.as_mut().unwrap();

        socket.read_exact(buffer)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        if self.send(&Empty, 2).is_ok() {}

        let socket = self.socket.as_mut().unwrap();

        socket.flush()?;
        socket.shutdown(Shutdown::Both)?;

        log::debug!("Closed IPC socket");

        Ok(())
    }

    fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
