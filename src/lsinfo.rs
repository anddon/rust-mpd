//! The module defines the response from an lsinfo command.

use crate::convert::FromIter;
use crate::error::Error;
use crate::song::Song;

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
        metadata: Vec<(String, String)>,
    },
    /// A playlist
    Playlist {
        /// Path to the playlist
        path: String,
        /// Metadata for the playlist
        metadata: Vec<(String, String)>,
    },
}

impl FromIter for LsInfoResponse {
    /// build song from map
    fn from_iter<I: Iterator<Item = Result<(String, String), Error>>>(iter: I) -> Result<Self, Error> {
        let items = iter.collect::<Result<Vec<_>, _>>()?;
        if let Some((_, path)) = items.iter().find(|(k, _)| k == "directory") {
            Ok(LsInfoResponse::Directory {
                path: path.clone(),
                metadata: items,
            })
        } else if let Some((_, path)) = items.iter().find(|(k, _)| k == "playlist") {
            Ok(LsInfoResponse::Playlist {
                path: path.clone(),
                metadata: items,
            })
        } else {
            Ok(LsInfoResponse::Song(Song::from_iter(items.into_iter().map(Ok))?))
        }
    }
}
