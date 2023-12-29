use crate::{
    Result,
    activity::Activity,
    pack_unpack::{pack, unpack},
};
use serde::{Deserializer, de::IntoDeserializer, Serialize, Deserialize};
use serde_json::Value;
use strum::FromRepr;
use uuid::Uuid;

/// The handshake data sent to Discord. for connection.
#[derive(Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HandshakeData {
    v: u32,
    client_id: String,
}
impl HandshakeData {
    fn new<S: Into<String>>(client_id: S) -> Self {
        Self {
            v: 1,
            client_id: client_id.into(),
        }
    }
}

#[derive(Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct ActivityArgs<'a> {
    pid: u32,
    activity: Option<Activity<'a>>,
}

/// This defines a message coming from Discord into your app.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum ActivityCmd {
    Dispatch,
    Authorize,
    Authenticate,
    GetGuild,
    GetGuilds,
    GetChannel,
    GetChannels,
    CreateChannelInvite,
    GetRelationships,
    GetUser,
    Subscribe,
    Unsubscribe,
    SetUserVoiceSettings,
    SetUserVoiceSettings2,
    SelectVoiceChannel,
    GetSelectedVoiceChannel,
    SelectTextChannel,
    GetVoiceSettings,
    SetVoiceSettings2,
    SetVoiceSettings,
    CaptureShortcut,
    SetActivity,
    SendActivityJoinInvite,
    CloseActivityJoinRequest,
    ActivityInviteUser,
    AcceptActivityInvite,
    InviteBrowser,
    DeepLink,
    ConnectionsCallback,
    BraintreePopupBridgeCallback,
    GiftCodeBrowser,
    GuildTemplateBrowser,
    Overlay,
    BrowserHandoff,
    SetCertifiedDevices,
    GetImage,
    CreateLobby,
    UpdateLobby,
    DeleteLobby,
    UpdateLobbyMember,
    ConnectToLobby,
    DisconnectFromLobby,
    SendToLobby,
    SearchLobbies,
    ConnectToLobbyVoice,
    DisconnectFromLobbyVoice,
    SetOverlayLocked,
    OpenOverlayActivityInvite,
    OpenOverlayGuildInvite,
    OpenOverlayVoiceSettings,
    ValidateApplication,
    GetEntitlementTicket,
    GetApplicationTicket,
    StartPurchase,
    GetSkus,
    GetEntitlements,
    GetNetworkingConfig,
    NetworkingSystemMetrics,
    NetworkingPeerMetrics,
    NetworkingCreateToken,
    SetUserAchievement,
    GetUserAchievements,
}

/// All of the valid events for the IPC.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum ActivityEvent {
    CreateUserUpdate,
    GuildStatus,
    GuildCrate,
    ChannelCreate,
    RelationshipUpdate,
    VoiceChannelSelect,
    VoiceStateCreate,
    VoiceStateDelete,
    VoiceStateUpdate,
    VoiceSettingsUpdate,
    VoiceSettingsUpdate2,
    VoiceConnectionStatus,
    SpeakingStart,
    SpeakingStop,
    GameJoin,
    GameSpectate,
    ActivityJoin,
    ActivityJoinRequest,
    ActivitySpectate,
    ActivityInvite,
    NotificationCreate,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    LobbyDelete,
    LobbyUpdate,
    LobbyMemberConnect,
    LobbyMemberDisconnect,
    LobbyMemberUpdate,
    LobbyMessage,
    CaptureShortcutChange,
    Overlay,
    OverlayUpdate,
    EntitlementCreate,
    EntitlementDelete,
    UserAchievementUpdate,
    Ready,
    Error,
}   

/// This defines a message sent from your app to Discord.
#[derive(Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DiscordIPCCommandOutgoing<'a> {
    /// The command ID of this request.
    cmd: ActivityCmd,
    /// The unique ID of this request, a response will be sent with the matching ID.
    args: ActivityArgs<'a>,
    /// The arguments of this request.
    nonce: String,
    /// Events with `cmd` `SUBSCRIBE` will have an `event` parameter defining the name of the event (e.g. [MessageCreate](ActivityEvent::MessageCreate)).
    evt: Option<ActivityEvent>,
}
impl<'a> DiscordIPCCommandOutgoing<'a> {
    fn set_activity(activity: Activity<'a>) -> Self {
        Self {
            cmd: ActivityCmd::SetActivity,
            args: ActivityArgs {
                pid: std::process::id(),
                activity: Some(activity),
            },
            nonce: Uuid::new_v4().to_string(),
            evt: None,
        }
    }

    fn clear_activity() -> Self {
        Self {
            cmd: ActivityCmd::SetActivity,
            args: ActivityArgs {
                pid: std::process::id(),
                activity: None,
            },
            nonce: Uuid::new_v4().to_string(),
            evt: None,
        }
    }
}

/// This defines all of the messages you can send to Discord from your app.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DiscordIPCPayload<'a> {
    /// Connection handshake.
    Handshake(HandshakeData),
    /// A command.
    Command(DiscordIPCCommandOutgoing<'a>),
}
impl serde::Serialize for DiscordIPCPayload<'_> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DiscordIPCPayload::Handshake(data) => data.serialize(serializer),
            DiscordIPCPayload::Command(data) => data.serialize(serializer),
        }
    }
}

/// This defines a message coming from Discord into your app.
#[derive(Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DiscordIPCCommandIncoming {
    /// The command ID of this response.
    pub cmd: ActivityCmd,
    /// The unique ID of the request that triggered this response.
    pub nonce: Option<String>,
    /// The arguments of the request that triggered this response.
    pub args: Option<Value>,
    /// The payload of this response
    pub data: Value,
    /// The type of event this is.
    pub evt: Option<ActivityEvent>,
}

/// All of the possible errors from Discord (critical).
#[allow(missing_docs)]
#[derive(FromRepr, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, strum::Display, Debug)]
#[repr(u16)]
pub enum DiscordIPCErrorCodeCritical {
    // crtical errors
    CloseNormal = 1000,
    CloseUnsupported = 1003,
    CloseAbnormal = 1006,
    InvalidClientId = 4000,
    InvalidOrigin = 4001,
    RateLimited = 4002,
    TokenRevoked = 4003,
    InvalidVersion = 4004,
    InvalidEncoding = 4005,
}

/// All of the possible errors from Discord (non-critical).
#[allow(missing_docs)]
#[derive(FromRepr, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, strum::Display, Debug)]
#[repr(u16)]
pub enum DiscordIPCErrorCodeNonCritical {
    // non-crtical errors
    CaptureShortcutAlreadyListening = 5004,
    GetGuildTimedOut = 5002,
    InvalidActivityJoinRequest = 4012,
    InvalidActivitySecret = 5005,
    InvalidChannel = 4005,
    InvalidClientId = 4007,
    InvalidCommand = 4002,
    InvalidEntitlement = 4015,
    InvalidEvent = 4004,
    InvalidGiftCode = 4016,
    InvalidGuild = 4003,
    InvalidInvite = 4011,
    InvalidLobby = 4013,
    InvalidLobbySecret = 4014,
    InvalidOrigin = 4008,
    InvalidPayload = 4000,
    InvalidPermissions = 4006,
    InvalidToken = 4009,
    InvalidUser = 4010,
    LobbyFull = 5007,
    NoEligibleActivity = 5006,
    Oauth2Error = 5000,
    PurchaseCanceled = 5008,
    PurchaseError = 5009,
    RateLimited = 5011,
    SelectChannelTimedOut = 5001,
    SelectVoiceForceRequired = 5003,
    ServiceUnavailable = 1001,
    TransactionAborted = 1002,
    UnauthorizedForAchievement = 5010,
    UnknownError = 1000,
}

/// All of the possible errors from Discord.
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, strum::Display, Debug)]
pub enum DiscordIPCErrorCode {
    Critical(DiscordIPCErrorCodeCritical),
    NonCritical(DiscordIPCErrorCodeNonCritical),
}
impl<'de> serde::Deserialize<'de> for DiscordIPCErrorCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        // this is probably wrong... view: https://github.com/foxt/easy-presence/blob/master/ipc.md#error-handling
        let code = u16::deserialize(deserializer)?;

        // Attempt to convert to a critical error
        if let Some(critical) = DiscordIPCErrorCodeCritical::from_repr(code) {
            Ok(DiscordIPCErrorCode::Critical(critical))
        } else if let Some(non_critical) = DiscordIPCErrorCodeNonCritical::from_repr(code) {
            Ok(DiscordIPCErrorCode::NonCritical(non_critical))
        } else {
            Err(serde::de::Error::custom("Invalid error code"))
        }
    }
}

/// The error response from Discord.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DiscordIPCError {
    /// The error code.
    pub code: DiscordIPCErrorCode,
    /// A human readable message.
    pub message: String,
}
impl std::fmt::Display for DiscordIPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Discord IPC error [{}]: {}", self.code, self.message)
    }
}
impl std::error::Error for DiscordIPCError {}

/// The response from Discord.
#[derive(Clone, Eq, PartialEq, strum::Display, Debug)]
pub enum DiscordIPCResponse {
    /// A response to a command.
    Command(DiscordIPCCommandIncoming),
    /// An error response.
    Error(DiscordIPCError),
}
impl<'de> serde::Deserialize<'de> for DiscordIPCResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DiscordIPCResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a DiscordIPCResponse object")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> std::result::Result<Self::Value, A::Error> {
                let mut error = None;
                let mut message = None;
                let mut remaining = serde_json::Map::new();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "error" => error = Some(map.next_value::<DiscordIPCErrorCode>()?),
                        "message" => message = Some(map.next_value::<String>()?),
                        _ => { remaining.insert(key, map.next_value::<serde_json::Value>()?); }
                    }

                    if error.is_some() && message.is_some() {
                        break;
                    }
                }

                if let (Some(error), Some(message)) = (error, message) {
                    Ok(DiscordIPCResponse::Error(DiscordIPCError { code: error, message }))
                } else {
                    // this can probably be optimised
                    let x = DiscordIPCCommandIncoming::deserialize(serde_json::Value::Object(remaining).into_deserializer())
                        .map_err(|e| serde::de::Error::custom(format!("invalid response: {}", e)))?;
                    Ok(DiscordIPCResponse::Command(x))
                }
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

/// A client that connects to and communicates with the Discord IPC.
///
/// Implemented via the [`DiscordIpcClient`](struct@crate::DiscordIpcClient) struct.
pub trait DiscordIpc {
    /// Connects the client to the Discord IPC.
    ///
    /// This method attempts to first establish a connection,
    /// and then sends a handshake.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the client
    /// fails to connect to the socket, or if it fails to
    /// send a handshake.
    ///
    /// # Examples
    /// ```
    /// let mut client = discord_ipc::new_client("<some client id>");
    /// client.connect()?;
    /// ```
    fn connect(&mut self) -> Result<()> {
        self.connect_ipc()?;
        log::debug!("Connected to Discord IPC");
        self.send_handshake()?;
        log::debug!("Sent handshake to Discord IPC");

        Ok(())
    }

    /// Reconnects to the Discord IPC.
    ///
    /// This method closes the client's active connection,
    /// then re-connects it and re-sends a handshake.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the client
    /// failed to connect to the socket, or if it failed to
    /// send a handshake.
    ///
    /// # Examples
    /// ```
    /// let mut client = discord_ipc::new_client("<some client id>");
    /// client.connect()?;
    ///
    /// client.close()?;
    /// client.reconnect()?;
    /// ```
    fn reconnect(&mut self) -> Result<()> {
        log::debug!("Reconnecting to Discord IPC...");
        self.close()?;
        log::debug!("Closed connection to Discord IPC");
        self.connect_ipc()?;
        log::debug!("Reconnected to Discord IPC");
        self.send_handshake()?;
        log::debug!("Sent handshake to Discord IPC");

        Ok(())
    }

    #[doc(hidden)]
    fn get_client_id(&self) -> &String;

    #[doc(hidden)]
    fn connect_ipc(&mut self) -> Result<()>;

    /// Handshakes the Discord IPC.
    ///
    /// This method sends the handshake signal to the IPC.
    /// It is usually not called manually, as it is automatically
    /// called by [`connect`] and/or [`reconnect`].
    ///
    /// [`connect`]: #method.connect
    /// [`reconnect`]: #method.reconnect
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if sending the handshake failed.
    fn send_handshake(&mut self) -> Result<()> {
        self.send(
            &HandshakeData::new(self.get_client_id()),
            0,
        )?;
        // TODO: Return an Err if the handshake is rejected
        self.recv()?;

        Ok(())
    }

    /// Sends JSON data to the Discord IPC.
    ///
    /// This method takes data (`serde_json::Value`) and
    /// an opcode as its parameters.
    ///
    /// # Errors
    /// Returns an `Err` variant if writing to the socket failed
    ///
    /// # Examples
    /// ```
    /// let payload = serde_json::json!({ "field": "value" });
    /// client.send(payload, 0)?;
    /// ```
    // TODO: Refine the `data` argument to make it clear what the user can send.
    fn send<T: ?Sized + serde::Serialize>(&mut self, data: &T, opcode: u8) -> Result<()> {
        let data_string = serde_json::to_string(data)?;

        log::debug!("Sending IPC message [{}]: {}", opcode, data_string);

        let header = pack(opcode.into(), data_string.len() as u32);

        self.write(&header)?;
        self.write(data_string.as_bytes())?;

        Ok(())
    }

    #[doc(hidden)]
    fn write(&mut self, data: &[u8]) -> Result<()>;

    /// Receives an opcode and JSON data from the Discord IPC.
    ///
    /// This method returns any data received from the IPC.
    /// It returns a tuple containing the opcode, and the JSON data.
    ///
    /// # Errors
    /// Returns an `Err` variant if reading the socket was
    /// unsuccessful.
    ///
    /// # Examples
    /// ```
    /// client.connect_ipc()?;
    /// client.send_handshake()?;
    ///
    /// println!("{:?}", client.recv()?);
    /// ```
    fn recv(&mut self) -> Result<(u32, DiscordIPCResponse)> {
        let mut header = [0; 8];

        self.read(&mut header)?;
        let (op, length) = unpack(header.to_vec())?;

        let mut data = vec![0u8; length as usize];
        self.read(&mut data)?;

        let json_data = serde_json::from_slice::<DiscordIPCResponse>(&data)?;

        log::debug!("Received IPC message [{}]: {:?}", op, json_data);

        Ok((op, json_data))
    }

    #[doc(hidden)]
    fn read(&mut self, buffer: &mut [u8]) -> Result<()>;

    /// Sets a Discord activity.
    ///
    /// This method is an abstraction of [`send`],
    /// wrapping it such that only an activity payload
    /// is required.
    ///
    /// [`send`]: #method.send
    ///
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    fn set_activity(&mut self, activity_payload: Activity) -> Result<()> {
        self.send(&DiscordIPCCommandOutgoing::set_activity(activity_payload), 1)
    }

    /// Works the same as as [`set_activity`] but clears activity instead.
    /// 
    /// [`set_activity`]: #method.set_activity
    /// 
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    fn clear_activity(&mut self) -> Result<()> {
        self.send(&DiscordIPCCommandOutgoing::clear_activity(), 1)
    }

    /// Closes the Discord IPC connection. Implementation is dependent on platform.
    fn close(&mut self) -> Result<()>;
}
