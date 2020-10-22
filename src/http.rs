//! Models to deserialize responses into and functions to create `http` crate
//! requests.

use http::{
    header::{HeaderValue, AUTHORIZATION},
    Error as HttpError, Request,
};
use percent_encoding::NON_ALPHANUMERIC;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// The type of search result given.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoadType {
    /// Loading the results failed.
    LoadFailed,
    /// There were no matches.
    NoMatches,
    /// A playlist was found.
    PlaylistLoaded,
    /// Some results were found.
    SearchResult,
    /// A single track was found.
    TrackLoaded,
}

/// A track within a search result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// Details about a track, such as the author and title.
    pub info: TrackInfo,
    /// The base64 track string that you use in the [`Play`] event.
    ///
    /// [`Play`]: ../model/outgoing/struct.Play.html
    pub track: String,
}

/// Additional information about a track, such as the author.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackInfo {
    /// Class name of the lavaplayer track.
    pub class: String,
    /// The title.
    pub title: String,
    /// The name of the author.
    pub author: String,
    /// The length of the audio in milliseconds.
    pub length: u64,
    /// The identifier of the source of the track.
    pub identifier: String,
    /// The source URI of the track.
    pub uri: String,
    /// Whether the source is a stream.
    pub is_stream: bool,
    /// Whether the source is seekable.
    pub is_seekable: bool,
    /// The position of the audio.
    pub position: u64,
}

/// Information about a playlist from a search result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    /// The name of the playlist, if available.
    pub name: String,
    /// The selected track, if one was selected.
    pub selected_track: Option<u64>,
}

/// Possible track results for a query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedTracks {
    /// The type of search result, such as a list of tracks or a playlist.
    pub load_type: LoadType,
    /// The list of tracks returned for the search query.
    pub tracks: Option<Vec<Track>>,
    /// Information about the playlist, if provided.
    pub playlist_info: Option<PlaylistInfo>,
    /// Error that happened while loading track.
    pub cause: Option<Error>,
    /// Severity of the error.
    pub severity: Option<String>
}

/// Error information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// Class of the error.
    pub class: String,
    /// Message of the error.
    pub message: Option<String>,
    /// Stack trace of the error, always None.
    #[serde(skip)]
    pub stack: Option<String>,
    /// Cause of the error, always None.
    #[serde(skip)]
    pub cause: Option<String>,
    /// Suppressed errors, always None.
    #[serde(skip)]
    pub suppressed: Option<String>,
}

/// Get a list of tracks that match an identifier.
///
/// The response will include a body which can be deserialized into a
/// [`LoadedTracks`].
///
/// [`LoadedTracks`]: struct.LoadedTracks.html
pub fn load_track(
    address: SocketAddr,
    identifier: impl AsRef<str>,
    authorization: impl AsRef<str>,
) -> Result<Request<&'static [u8]>, HttpError> {
    let identifier =
        percent_encoding::percent_encode(identifier.as_ref().as_bytes(), NON_ALPHANUMERIC);
    let url = format!("http://{}/loadtracks?identifier={}", address, identifier);

    let mut req = Request::get(url);

    let auth_value = HeaderValue::from_str(authorization.as_ref())?;
    req = req.header(AUTHORIZATION, auth_value);

    req.body(b"")
}
