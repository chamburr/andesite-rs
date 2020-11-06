//! Client to manage nodes and players.

use crate::{
    model::{IncomingEvent, OutgoingEvent},
    node::{Node, NodeConfig, NodeError, Resume},
    player::{Player, PlayerManager},
};
use dashmap::{mapref::one::Ref, DashMap};
use futures_channel::mpsc::{TrySendError, UnboundedReceiver};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    net::SocketAddr,
    sync::Arc,
};
use twilight_model::id::{GuildId, UserId};

/// An error that can occur while interacting with the client.
#[derive(Clone, Debug, PartialEq)]
pub enum ClientError {
    /// A node isn't configured, so the operation isn't possible to fulfill.
    NodesUnconfigured,
    /// Sending a voice update event to the node failed because the node's
    /// connection was shutdown.
    SendingVoiceUpdate {
        /// The source of the error.
        source: TrySendError<OutgoingEvent>,
    },
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NodesUnconfigured => f.write_str("no node has been configured"),
            Self::SendingVoiceUpdate { .. } => f.write_str("couldn't send voice update to node"),
        }
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NodesUnconfigured => None,
            Self::SendingVoiceUpdate { source } => Some(source),
        }
    }
}

#[derive(Debug, Default)]
struct LavalinkRef {
    guilds: DashMap<GuildId, SocketAddr>,
    nodes: DashMap<SocketAddr, Node>,
    players: PlayerManager,
    user_id: UserId,
}

/// The lavalink client that manages nodes, players, and processes events from
/// Discord to tie it all together.
///
/// **Note**: You must call the [`process`] method with every Voice State Update
/// and Voice Server Update event you receive from Discord. It will
/// automatically forward these events to Lavalink. See its documentation for
/// more information.
///
/// You can retrieve players using the [`player`] method. Players contain
/// information about the active playing information of a guild and allows you to send events to the
/// connected node, such as [`Play`] events.
///
/// [`Play`]: ../model/outgoing/struct.Play.html
/// [`player`]: #method.player
/// [`process`]: #method.process
#[derive(Clone, Debug)]
pub struct Lavalink(Arc<LavalinkRef>);

impl Lavalink {
    /// Create a new Lavalink client instance.
    ///
    /// The user ID and number of shards provided may not be modified during
    /// runtime, and the client must be re-created. These parameters are
    /// automatically passed to new nodes created via [`add`].
    ///
    /// [`add`]: #method.add
    pub fn new(user_id: UserId) -> Self {
        Self(Arc::new(LavalinkRef {
            guilds: DashMap::new(),
            nodes: DashMap::new(),
            players: PlayerManager::new(),
            user_id,
        }))
    }

    /// Add a new node to be managed by the Lavalink client.
    ///
    /// If a node already exists with the provided address, then it will be
    /// replaced.
    pub async fn add(
        &self,
        address: SocketAddr,
        authorization: impl Into<String>,
    ) -> Result<(Node, UnboundedReceiver<IncomingEvent>), NodeError> {
        self.add_with_resume(address, authorization, None).await
    }

    /// Similar to [`add`], but allows you to specify resume capability.
    ///
    /// [`add`]: #method.add
    pub async fn add_with_resume(
        &self,
        address: SocketAddr,
        authorization: impl Into<String>,
        resume: impl Into<Option<Resume>>,
    ) -> Result<(Node, UnboundedReceiver<IncomingEvent>), NodeError> {
        let config = NodeConfig {
            address,
            authorization: authorization.into(),
            resume: resume.into(),
            user_id: self.0.user_id,
        };

        let (node, rx) = Node::connect(config, self.0.players.clone()).await?;
        self.0.nodes.insert(address, node.clone());

        Ok((node, rx))
    }

    /// Get a node with the socket address.
    pub fn get(&self, address: SocketAddr) -> Option<Node> {
        if let Some(node) = self.0.nodes.get(&address) {
            Some(node.clone())
        } else {
            None
        }
    }

    /// Remove a node from the list of nodes being managed by the Lavalink
    /// client.
    ///
    /// The node is returned if it existed.
    pub fn remove(&self, address: SocketAddr) -> Option<(SocketAddr, Node)> {
        self.0.nodes.remove(&address)
    }

    /// Determine the "best" node for new players according to available nodes'
    /// penalty scores.
    ///
    /// Refer to [`Node::penalty`] for how this is calculated.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::NodesUnconfigured`] if there are no configured
    /// nodes available in the client.
    ///
    /// [`ClientError::NodesUnconfigured`]: enum.ClientError.html#variant.NodesUnconfigured
    /// [`Node::penalty`]: ../node/struct.Node.html#method.penalty
    pub async fn best(&self) -> Result<Node, ClientError> {
        let mut lowest = i32::MAX;
        let mut best = None;

        for node in self.0.nodes.iter() {
            let penalty = node.value().penalty().await;

            if penalty < lowest {
                lowest = penalty;
                best.replace(node.clone());
            }
        }

        best.ok_or(ClientError::NodesUnconfigured)
    }

    /// Retrieve an immutable reference to the player manager.
    pub fn players(&self) -> &PlayerManager {
        &self.0.players
    }

    /// Retrieve a player for the guild.
    ///
    /// Creates a player configured to use the best available node if a player
    /// for the guild doesn't already exist. Use [`PlayerManager::get`] to only
    /// retrieve and not create.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::NodesUnconfigured`] if no node has been
    /// configured via [`add`].
    ///
    /// [`ClientError::NodesUnconfigured`]: enum.ClientError.html#variant.NodesUnconfigured
    /// [`PlayerManager::get`]: ../player/struct.PlayerManager.html#method.get
    /// [`add`]: #method.add
    pub async fn player(&self, guild_id: GuildId) -> Result<Ref<'_, GuildId, Player>, ClientError> {
        if let Some(player) = self.players().get(&guild_id) {
            return Ok(player);
        }

        let node = self.best().await?;

        Ok(self.players().get_or_insert(guild_id, node).downgrade())
    }
}
