//! The module defines the response from an lsinfo command.

use crate::convert::FromMap;
use crate::error::Error;
use crate::song::Song;
use std::collections::BTreeMap;

#[derive(Debug)]
/// Response form the lsinfo command. Contains either a song, a directory, or a playlist.
pub enum LsInfoResponse {
    /// A song
    Song(Song),
    /// A directory
    Directory {
        /// The path to this directory
        path: String,
        /// Metadata for the directory
        metadata: BTreeMap<String, String>,
    },
    /// A playlist
    Playlist {
        /// Path to the playlist
        path: String,
        /// Metadata for the playlist
        metadata: BTreeMap<String, String>,
    },
}

impl FromMap for LsInfoResponse {
    /// build song from map
    fn from_map(map: BTreeMap<String, String>) -> Result<Self, Error> {
        if let Some(path) = map.get("directory") {
            Ok(LsInfoResponse::Directory {
                path: path.clone(),
                metadata: map,
            })
        } else if let Some(path) = map.get("playlist") {
            Ok(LsInfoResponse::Playlist {
                path: path.clone(),
                metadata: map,
            })
        } else {
            Ok(LsInfoResponse::Song(Song::from_map(map)?))
        }
    }
}
