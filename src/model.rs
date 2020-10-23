//! Models to (de)serialize incoming/outgoing websocket events and HTTP
//! responses.

use serde::{Deserialize, Serialize};

/// The type of event that something is.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Opcode {
    /// Default empty op code.
    Empty,
    /// A combined voice server and voice state update.
    VoiceUpdate,
    /// Play a track.
    Play,
    /// Stop a player.
    Stop,
    /// Pause a player.
    Pause,
    /// Seek a player's active track to a new position.
    Seek,
    /// Set the volume of a player.
    Volume,
    /// Set the filter of a player.
    Filters,
    /// Destroy a player from a node.
    Destroy,
    /// An update about a player's current track.
    PlayerUpdate,
    /// Meta information about a track starting or ending.
    Event,
    /// Updated statistics about a node.
    Stats,
}

impl Default for Opcode {
    fn default() -> Self {
        Self::Empty
    }
}

pub mod outgoing {
    //! Events that clients send to Lavalink.

    use super::Opcode;
    use serde::{Deserialize, Serialize};
    use twilight_model::{gateway::payload::VoiceServerUpdate, id::GuildId};

    /// An outgoing event to send to Lavalink.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutgoingEvent {
        /// A combined voice server and voice state update.
        VoiceUpdate(VoiceUpdate),
        /// Play a track.
        Play(Play),
        /// Stop a player.
        Stop(Stop),
        /// Pause or unpause a player.
        Pause(Pause),
        /// Seek a player's active track to a new position.
        Seek(Seek),
        /// Set the volume of a player.
        Volume(Volume),
        /// Set the filter of a player.
        Filters(Filters),
        /// Destroy a player for a guild.
        Destroy(Destroy),
    }

    impl From<VoiceUpdate> for OutgoingEvent {
        fn from(event: VoiceUpdate) -> OutgoingEvent {
            Self::VoiceUpdate(event)
        }
    }

    impl From<Play> for OutgoingEvent {
        fn from(event: Play) -> OutgoingEvent {
            Self::Play(event)
        }
    }

    impl From<Stop> for OutgoingEvent {
        fn from(event: Stop) -> OutgoingEvent {
            Self::Stop(event)
        }
    }

    impl From<Pause> for OutgoingEvent {
        fn from(event: Pause) -> OutgoingEvent {
            Self::Pause(event)
        }
    }

    impl From<Seek> for OutgoingEvent {
        fn from(event: Seek) -> OutgoingEvent {
            Self::Seek(event)
        }
    }

    impl From<Volume> for OutgoingEvent {
        fn from(event: Volume) -> OutgoingEvent {
            Self::Volume(event)
        }
    }

    impl From<Filters> for OutgoingEvent {
        fn from(event: Filters) -> OutgoingEvent {
            Self::Filters(event)
        }
    }

    impl From<Destroy> for OutgoingEvent {
        fn from(event: Destroy) -> OutgoingEvent {
            Self::Destroy(event)
        }
    }

    /// A combined voice server and voice state update.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VoiceUpdate {
        /// The opcode of the event.
        pub op: Opcode,
        /// The session ID of the voice channel.
        pub session_id: String,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The inner event being forwarded to a node.
        pub event: SlimVoiceServerUpdate,
    }

    impl VoiceUpdate {
        /// Create a new voice update event.
        pub fn new(
            guild_id: GuildId,
            session_id: impl Into<String>,
            event: SlimVoiceServerUpdate,
        ) -> Self {
            Self::from((guild_id, session_id, event))
        }
    }

    impl<T: Into<String>> From<(GuildId, T, SlimVoiceServerUpdate)> for VoiceUpdate {
        fn from((guild_id, session_id, event): (GuildId, T, SlimVoiceServerUpdate)) -> Self {
            Self {
                op: Opcode::VoiceUpdate,
                session_id: session_id.into(),
                guild_id,
                event,
            }
        }
    }

    /// A slimmed version of a twilight voice server update.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub struct SlimVoiceServerUpdate {
        /// The endpoint of the Discord voice server.
        pub endpoint: Option<String>,
        /// The authentication token used by the bot to connect to the Discord
        /// voice server.
        pub token: String,
    }

    impl From<VoiceServerUpdate> for SlimVoiceServerUpdate {
        fn from(update: VoiceServerUpdate) -> Self {
            Self {
                endpoint: update.endpoint,
                token: update.token,
            }
        }
    }

    /// Play a track, optionally specifying to not skip the current track.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Play {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The base64 track information.
        pub track: String,
        /// The position in milliseconds to start the track from.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<u64>,
        /// The position in milliseconds to end the track. Does nothing.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<u64>,
        /// Whether or not to replace the currently playing track with this new
        /// track.
        ///
        /// Set to `true` to keep playing the current playing track, or `false`
        /// to replace the current playing track with a new one.
        pub no_replace: bool,
    }

    impl Play {
        /// Create a new play event.
        pub fn new(
            guild_id: GuildId,
            track: impl Into<String>,
            start_time: impl Into<Option<u64>>,
            end_time: impl Into<Option<u64>>,
            no_replace: bool,
        ) -> Self {
            Self::from((guild_id, track, start_time, end_time, no_replace))
        }
    }

    impl<T: Into<String>> From<(GuildId, T)> for Play {
        fn from((guild_id, track): (GuildId, T)) -> Self {
            Self::from((guild_id, track, None, None, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>> From<(GuildId, T, S)> for Play {
        fn from((guild_id, track, start_time): (GuildId, T, S)) -> Self {
            Self::from((guild_id, track, start_time, None, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>> From<(GuildId, T, S, E)>
        for Play
    {
        fn from((guild_id, track, start_time, end_time): (GuildId, T, S, E)) -> Self {
            Self::from((guild_id, track, start_time, end_time, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>> From<(GuildId, T, S, E, bool)>
        for Play
    {
        fn from(
            (guild_id, track, start_time, end_time, no_replace): (GuildId, T, S, E, bool),
        ) -> Self {
            Self {
                op: Opcode::Play,
                guild_id,
                track: track.into(),
                start_time: start_time.into(),
                end_time: end_time.into(),
                no_replace,
            }
        }
    }

    /// Stop a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Stop {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
    }

    impl Stop {
        /// Create a new stop event.
        pub fn new(guild_id: GuildId) -> Self {
            Self::from(guild_id)
        }
    }

    impl From<GuildId> for Stop {
        fn from(guild_id: GuildId) -> Self {
            Self {
                guild_id,
                op: Opcode::Stop,
            }
        }
    }

    /// Pause or unpause a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Pause {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// Whether to pause the player.
        ///
        /// Set to `true` to pause or `false` to resume.
        pub pause: bool,
    }

    impl Pause {
        /// Create a new pause event.
        ///
        /// Set to `true` to pause the player or `false` to resume it.
        pub fn new(guild_id: GuildId, pause: bool) -> Self {
            Self::from((guild_id, pause))
        }
    }

    impl From<(GuildId, bool)> for Pause {
        fn from((guild_id, pause): (GuildId, bool)) -> Self {
            Self {
                op: Opcode::Pause,
                guild_id,
                pause,
            }
        }
    }

    /// Seek a player's active track to a new position.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Seek {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The position in milliseconds to seek to.
        pub position: i64,
    }

    impl Seek {
        /// Create a new seek event.
        pub fn new(guild_id: GuildId, position: i64) -> Self {
            Self::from((guild_id, position))
        }
    }

    impl From<(GuildId, i64)> for Seek {
        fn from((guild_id, position): (GuildId, i64)) -> Self {
            Self {
                op: Opcode::Seek,
                guild_id,
                position,
            }
        }
    }

    /// Set the volume of a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Volume {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The volume of the player from 0 to 1000. 100 is the default.
        pub volume: i64,
    }

    impl Volume {
        /// Create a new volume event.
        pub fn new(guild_id: GuildId, volume: i64) -> Self {
            Self::from((guild_id, volume))
        }
    }

    impl From<(GuildId, i64)> for Volume {
        fn from((guild_id, volume): (GuildId, i64)) -> Self {
            Self {
                op: Opcode::Volume,
                guild_id,
                volume,
            }
        }
    }

    /// Set the filters of a player
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Filters {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The karaoke filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub karaoke: Option<Karaoke>,
        /// The timescale filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timescale: Option<Timescale>,
        /// The tremolo filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tremolo: Option<Tremolo>,
        /// The vibrato filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vibrato: Option<Vibrato>,
        /// The equalizer filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub equalizer: Option<Equalizer>,
    }

    impl Filters {
        /// Create a new filters event.
        pub fn new(
            guild_id: GuildId,
            karaoke: Option<Karaoke>,
            timescale: Option<Timescale>,
            tremolo: Option<Tremolo>,
            vibrato: Option<Vibrato>,
            equalizer: Option<Equalizer>,
        ) -> Self {
            Self::from((guild_id, karaoke, timescale, tremolo, vibrato, equalizer))
        }
    }

    impl
        From<(
            GuildId,
            Option<Karaoke>,
            Option<Timescale>,
            Option<Tremolo>,
            Option<Vibrato>,
            Option<Equalizer>,
        )> for Filters
    {
        fn from(
            (guild_id, karaoke, timescale, tremolo, vibrato, equalizer): (
                GuildId,
                Option<Karaoke>,
                Option<Timescale>,
                Option<Tremolo>,
                Option<Vibrato>,
                Option<Equalizer>,
            ),
        ) -> Self {
            Self {
                op: Opcode::Filters,
                guild_id,
                karaoke,
                timescale,
                tremolo,
                vibrato,
                equalizer,
            }
        }
    }

    /// Karaoke filter.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Karaoke {
        /// The effect level.
        pub level: f64,
        /// The mono effect level.
        pub mono_level: f64,
        /// The filter band.
        pub filter_band: f64,
        /// The filter width.
        pub filter_width: f64,
        /// Whether is enabled, skipped when serializing.
        #[serde(skip_serializing)]
        pub enabled: bool,
    }

    impl Karaoke {
        /// Create a new karaoke filter.
        pub fn new(level: f64, mono_level: f64, filter_band: f64, filter_width: f64) -> Self {
            Self::from((level, mono_level, filter_band, filter_width))
        }
    }

    impl From<f64> for Karaoke {
        fn from(level: f64) -> Self {
            Self::from((level, 1 as f64, 220 as f64, 100 as f64))
        }
    }

    impl From<(f64, f64, f64, f64)> for Karaoke {
        fn from((level, mono_level, filter_band, filter_width): (f64, f64, f64, f64)) -> Self {
            Self {
                level,
                mono_level,
                filter_band,
                filter_width,
                enabled: false,
            }
        }
    }

    /// Timescale filter.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Timescale {
        /// Speed to play at.
        pub speed: f64,
        /// Pitch to play at.
        pub pitch: f64,
        /// Rate to play at.
        pub rate: f64,
        /// Whether is enabled, skipped when serializing.
        #[serde(skip_serializing)]
        pub enabled: bool,
    }

    impl Timescale {
        /// Create a new timescale filter.
        pub fn new(speed: f64, pitch: f64, rate: f64) -> Self {
            Self::from((speed, pitch, rate))
        }
    }

    impl From<(f64, f64, f64)> for Timescale {
        fn from((speed, pitch, rate): (f64, f64, f64)) -> Self {
            Self {
                speed,
                pitch,
                rate,
                enabled: false,
            }
        }
    }

    /// Tremolo filter.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Tremolo {
        /// The filter frequency.
        pub frequency: f64,
        /// The filter depth.
        pub depth: f64,
        /// Whether is enabled, skipped when serializing.
        #[serde(skip_serializing)]
        pub enabled: bool,
    }

    impl Tremolo {
        /// Create a new tremolo filter.
        pub fn new(frequency: f64, depth: f64) -> Self {
            Self::from((frequency, depth))
        }
    }

    impl From<(f64, f64)> for Tremolo {
        fn from((frequency, depth): (f64, f64)) -> Self {
            Self {
                frequency,
                depth,
                enabled: false,
            }
        }
    }

    /// Vibrato filter.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Vibrato {
        /// The filter frequency.
        pub frequency: f64,
        /// The filter depth.
        pub depth: f64,
        /// Whether is enabled, skipped when serializing.
        #[serde(skip_serializing)]
        pub enabled: bool,
    }

    impl Vibrato {
        /// Create a new timescale filter.
        pub fn new(frequency: f64, depth: f64) -> Self {
            Self::from((frequency, depth))
        }
    }

    impl From<(f64, f64)> for Vibrato {
        fn from((frequency, depth): (f64, f64)) -> Self {
            Self {
                frequency,
                depth,
                enabled: false,
            }
        }
    }

    /// Equalize a player.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Equalizer {
        /// The bands to use as part of the equalizer.
        pub bands: Vec<EqualizerBand>,
        /// Whether is enabled, skipped when serializing.
        #[serde(skip_serializing)]
        pub enabled: bool,
    }

    impl Equalizer {
        /// Create a new equalizer filter
        pub fn new(bands: Vec<EqualizerBand>) -> Self {
            Self::from(bands)
        }
    }

    impl From<Vec<EqualizerBand>> for Equalizer {
        fn from(bands: Vec<EqualizerBand>) -> Self {
            Self {
                bands,
                enabled: false,
            }
        }
    }

    /// A band of the equalizer event.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EqualizerBand {
        /// The band.
        pub band: i64,
        /// The gain.
        pub gain: f64,
    }

    impl EqualizerBand {
        /// Create a new equalizer band.
        pub fn new(band: i64, gain: f64) -> Self {
            Self::from((band, gain))
        }
    }

    impl From<(i64, f64)> for EqualizerBand {
        fn from((band, gain): (i64, f64)) -> Self {
            Self { band, gain }
        }
    }

    /// Destroy a player from a node.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Destroy {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
    }

    impl Destroy {
        /// Create a new destroy event.
        pub fn new(guild_id: GuildId) -> Self {
            Self {
                op: Opcode::Destroy,
                guild_id,
            }
        }
    }

    impl From<GuildId> for Destroy {
        fn from(guild_id: GuildId) -> Self {
            Self {
                op: Opcode::Destroy,
                guild_id,
            }
        }
    }
}

pub mod incoming {
    //! Events that Lavalink sends to clients.

    use super::outgoing::{Equalizer, Karaoke, Timescale, Tremolo, Vibrato};
    use super::Opcode;
    use crate::http::Error;
    use serde::{Deserialize, Serialize};
    use twilight_model::id::GuildId;

    /// An incoming event from a Lavalink node.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IncomingEvent {
        /// An update about the information of a player.
        PlayerUpdate(PlayerUpdate),
        /// New statistics about a node and its host.
        Stats(Stats),
        /// A track ended.
        TrackEnd(TrackEnd),
        /// A track started.
        TrackStart(TrackStart),
    }

    impl From<PlayerUpdate> for IncomingEvent {
        fn from(event: PlayerUpdate) -> IncomingEvent {
            Self::PlayerUpdate(event)
        }
    }

    impl From<Stats> for IncomingEvent {
        fn from(event: Stats) -> IncomingEvent {
            Self::Stats(event)
        }
    }

    /// An update about the information of a player.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerUpdate {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The new state of the player.
        pub state: PlayerUpdateState,
    }

    /// New statistics about a node and its host.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerUpdateState {
        /// The new time of the player.
        pub time: i64,
        /// The new position of the player.
        pub position: i64,
        /// Whether the player is paused.
        pub paused: bool,
        /// Volume of the player.
        pub volume: i64,
        /// Filters present.
        pub filters: FiltersState,
        /// Mixer, always None.
        #[serde(skip)]
        pub mixer: Option<()>,
        /// Mixer enabled, always None.
        #[serde(skip)]
        pub mixer_enabled: Option<()>,
        /// Frame loss and success, always None.
        #[serde(skip)]
        pub frame: Option<()>,
    }

    /// List of filters present.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FiltersState {
        /// The karaoke filter.
        pub karaoke: Karaoke,
        /// The timescale filter.
        pub timescale: Timescale,
        /// The tremolo filter.
        pub tremolo: Tremolo,
        /// The vibrato filter.
        pub vibrato: Vibrato,
        /// The equalizer filter.
        pub equalizer: Equalizer,
        /// The volume filter, always None.
        #[serde(skip)]
        pub volume: Option<()>,
    }

    impl FiltersState {
        /// Create a new filters state.
        pub fn new() -> Self {
            Self {
                karaoke: Karaoke {
                    level: 0.0,
                    mono_level: 0.0,
                    filter_band: 0.0,
                    filter_width: 0.0,
                    enabled: false,
                },
                timescale: Timescale {
                    speed: 0.0,
                    pitch: 0.0,
                    rate: 0.0,
                    enabled: false,
                },
                tremolo: Tremolo {
                    frequency: 0.0,
                    depth: 0.0,
                    enabled: false,
                },
                vibrato: Vibrato {
                    frequency: 0.0,
                    depth: 0.0,
                    enabled: false,
                },
                equalizer: Equalizer {
                    bands: vec![],
                    enabled: false,
                },
                volume: None,
            }
        }
    }

    /// Statistics about a node and its host.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Stats {
        /// The opcode of the event.
        pub op: Opcode,
        /// The current number of total players (active and not active) within
        /// the node.
        pub players: u64,
        /// The current number of active players within the node.
        pub playing_players: u64,
        /// The uptime of the Lavalink server in seconds.
        pub uptime: u64,
        /// Memory information about the node's host.
        pub memory: StatsMemory,
        /// CPU information about the node's host.
        pub cpu: StatsCpu,
        /// Statistics about audio frames.
        #[serde(rename = "frameStats", skip_serializing_if = "Option::is_none")]
        pub frames: Option<StatsFrames>,
    }

    /// Memory information about a node and its host.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatsMemory {
        /// The number of bytes allocated.
        pub allocated: u64,
        /// The number of bytes free.
        pub free: u64,
        /// The number of bytes reservable.
        pub reservable: u64,
        /// The number of bytes used.
        pub used: u64,
    }

    /// CPU information about a node and its host.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatsCpu {
        /// The number of CPU cores.
        pub cores: usize,
        /// The load of the Lavalink server.
        pub lavalink_load: f64,
        /// The load of the system as a whole.
        pub system_load: f64,
    }

    /// CPU information about a node and its host.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatsFrames {
        /// The number of CPU cores.
        pub sent: u64,
        /// The load of the Lavalink server.
        pub nulled: u64,
        /// The load of the system as a whole.
        pub deficit: u64,
    }

    /// The type of track event that was received.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub enum TrackEventType {
        /// A track for a player started.
        #[serde(rename = "TrackStartEvent")]
        Start,
        /// A track for a player ended.
        #[serde(rename = "TrackEndEvent")]
        End,
        /// A track for a player met an exception.
        #[serde(rename = "TrackExceptionEvent")]
        Exception,
        /// A track for a player got stuck.
        #[serde(rename = "TrackStuckEvent")]
        Stuck,
        /// The websocket got closed.
        #[serde(rename = "WebSocketClosedEvent")]
        WebsocketClose,
    }

    /// A track started.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TrackStart {
        /// The opcode of the event.
        pub op: Opcode,
        /// The type of track event.
        #[serde(rename = "type")]
        pub kind: TrackEventType,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The user ID affected, always None.
        #[serde(skip)]
        pub user_id: Option<()>,
        /// The base64 track that was affected.
        pub track: String,
    }

    /// A track ended.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TrackEnd {
        /// The opcode of the event.
        pub op: Opcode,
        /// The type of track event.
        #[serde(rename = "type")]
        pub kind: TrackEventType,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The user ID affected, always None.
        #[serde(skip)]
        pub user_id: Option<()>,
        /// The base64 track that was affected.
        pub track: String,
        /// The reason that the track ended.
        pub reason: String,
    }

    /// A track encountered exception.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TrackException {
        /// The opcode of the event.
        pub op: Opcode,
        /// The type of track event.
        #[serde(rename = "type")]
        pub kind: TrackEventType,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The user ID affected, always None.
        #[serde(skip)]
        pub user_id: Option<()>,
        /// The base64 track that was affected.
        pub track: String,
        /// The error that the track encountered exception.
        pub error: String,
        /// The specific error.
        pub exception: Error,
    }

    /// A track got stuck.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TrackStuck {
        /// The opcode of the event.
        pub op: Opcode,
        /// The type of track event.
        #[serde(rename = "type")]
        pub kind: TrackEventType,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The user ID affected, always None.
        #[serde(skip)]
        pub user_id: Option<()>,
        /// The base64 track that was affected.
        pub track: String,
        /// The threshold for track stuck.
        pub threshold_ms: i64,
    }

    /// AThe websocket got closed.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WebsocketClose {
        /// The opcode of the event.
        pub op: Opcode,
        /// The type of track event.
        #[serde(rename = "type")]
        pub kind: TrackEventType,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The reason for the close of websocket.
        pub reason: String,
        /// The code for this websocket close.
        pub code: i64,
        /// Whether it is closed by remote.
        pub by_remote: bool,
    }
}

pub use self::{
    incoming::{
        FiltersState, IncomingEvent, PlayerUpdate, PlayerUpdateState, Stats, StatsCpu, StatsFrames,
        StatsMemory, TrackEnd, TrackEventType, TrackException, TrackStart, TrackStuck,
        WebsocketClose,
    },
    outgoing::{
        Destroy, Equalizer, EqualizerBand, Filters, Karaoke, OutgoingEvent, Pause, Play, Seek,
        SlimVoiceServerUpdate, Stop, Timescale, Tremolo, Vibrato, VoiceUpdate, Volume,
    },
};
