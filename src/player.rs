//! Players containing information about active playing state within guilds and
//! allowing you to send events to connected nodes.
//!
//! Use the [`PlayerManager`] to retrieve existing [players] for guilds and
//! use those players to do things like [send events] or [read the position] of
//! the active audio.
//!
//! [`PlayerManager`]: struct.PlayerManager.html
//! [players]: struct.Player.html
//! [send events]: struct.Player.html#method.send
//! [read the position]: struct.Player.html#method.position

use crate::{model::*, node::Node};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use futures_channel::mpsc::TrySendError;
use std::{fmt::Debug, sync::Arc};
use twilight_model::id::GuildId;

/// Retrieve and create players for guilds.
///
/// The player manager contains all of the players for all guilds over all
/// nodes, and can be used to read player information and send events to nodes.
#[derive(Clone, Debug, Default)]
pub struct PlayerManager {
    pub(crate) players: Arc<DashMap<GuildId, Player>>,
}

impl PlayerManager {
    /// Create a new player manager.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Return an immutable reference to a player by guild ID.
    pub fn get(&self, guild_id: &GuildId) -> Option<Ref<'_, GuildId, Player>> {
        self.players.get(guild_id)
    }

    /// Return a mutable reference to a player by guild ID.
    pub(crate) fn get_mut(&self, guild_id: &GuildId) -> Option<RefMut<'_, GuildId, Player>> {
        self.players.get_mut(guild_id)
    }

    /// Return a mutable reference to a player by guild ID or insert a new
    /// player linked to a given node.
    pub fn get_or_insert(&self, guild_id: GuildId, node: Node) -> RefMut<'_, GuildId, Player> {
        self.players
            .entry(guild_id)
            .or_insert_with(|| Player::new(guild_id, node))
    }

    /// Remove a player by guild ID.
    pub fn remove(&self, guild_id: &GuildId) -> Option<(GuildId, Player)> {
        self.players.remove(guild_id)
    }
}

/// A player for a guild connected to a node.
///
/// This can be used to send events over a node and to read the details of a
/// player for a guild.
#[derive(Debug)]
pub struct Player {
    guild_id: GuildId,
    node: Node,
    time: i64,
    position: Option<i64>,
    paused: bool,
    volume: i64,
    filters: FiltersState,
}

impl Player {
    pub(crate) fn new(guild_id: GuildId, node: Node) -> Self {
        Self {
            guild_id,
            node,
            time: 0,
            position: None,
            paused: false,
            volume: 0,
            filters: FiltersState::new(),
        }
    }

    /// Send an event to the player's node.
    ///
    /// Returns a `futures_channel` `TrySendError` if the node has been removed.
    ///
    /// # Examples
    ///
    /// Send a [`Play`] and [`Pause`] event:
    ///
    /// ```
    /// use twilight_lavalink::{model::{Play, Pause}, Lavalink};
    /// # use twilight_model::id::{GuildId, UserId};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (guild_id, user_id) = (GuildId(1), UserId(2));
    /// # let track = String::new();
    ///
    /// let lavalink = Lavalink::new(user_id, 10);
    /// let players = lavalink.players();
    ///
    /// if let Some(player) = players.get(&guild_id) {
    ///     player.send(Play::from((guild_id, track)))?;
    ///     player.send(Pause::from((guild_id, true)))?;
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// [`Pause`]: ../model/outgoing/struct.Pause.html
    /// [`Play`]: ../model/outgoing/struct.Play.html
    pub fn send(&self, event: impl Into<OutgoingEvent>) -> Result<(), TrySendError<OutgoingEvent>> {
        self._send(event.into())
    }

    fn _send(&self, event: OutgoingEvent) -> Result<(), TrySendError<OutgoingEvent>> {
        tracing::debug!(
            "sending event on guild player {}: {:?}",
            self.guild_id,
            event
        );

        self.node.send(event)
    }

    /// Return an immutable reference to the node linked to the player.
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Return a copy of the player's guild ID.
    pub fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    /// Return a copy of the player's time.
    pub fn time(&self) -> i64 {
        self.time
    }

    /// Return a mutable reference to the player's time.
    pub(crate) fn time_mut(&mut self) -> &mut i64 {
        &mut self.time
    }

    /// Return a copy of the player's position.
    pub fn position(&self) -> Option<i64> {
        self.position
    }

    /// Return a mutable reference to the player's position.
    pub(crate) fn position_mut(&mut self) -> &mut Option<i64> {
        &mut self.position
    }

    /// Return a copy of whether the player is paused.
    pub fn paused(&self) -> bool {
        self.paused
    }

    /// Return a mutable copy of whether the player is paused.
    pub(crate) fn paused_mut(&mut self) -> &mut bool {
        &mut self.paused
    }

    /// Return a copy of the player's volume.
    pub fn volume(&self) -> i64 {
        self.volume
    }

    /// Return a mutable reference to the player's volume.
    pub(crate) fn volume_mut(&mut self) -> &mut i64 {
        &mut self.volume
    }

    /// Return a copy of the player's filters.
    pub fn filters(&self) -> FiltersState {
        self.filters.clone()
    }

    /// Return a mutable copy of the player's filters.
    pub(crate) fn filters_mut(&mut self) -> &mut FiltersState {
        &mut self.filters
    }
}
