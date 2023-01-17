pub mod airplay;
pub mod chromecast;
pub mod datpiff;
pub mod deezer;
pub mod genius;
pub mod kodi;
pub mod local;
pub mod myvazo;
pub mod tononkira;

use anyhow::Error;
use async_trait::async_trait;
use music_player_types::types::{Album, Artist, Device, Playlist, Track};

pub trait Addon {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn author(&self) -> &str;
    fn description(&self) -> &str;
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
}

pub trait StreamingAddon {
    fn stream(&self, url: &str) -> Result<(), Error>;
}

pub trait LyricsAddon {
    fn get_lyrics(&self, artist: &str, title: &str) -> Option<String>;
}

#[async_trait]
pub trait Browseable {
    async fn albums(&mut self, offset: i32, limit: i32) -> Result<Vec<Album>, Error>;
    async fn artists(&mut self, offset: i32, limit: i32) -> Result<Vec<Artist>, Error>;
    async fn tracks(&mut self, offset: i32, limit: i32) -> Result<Vec<Track>, Error>;
    async fn playlists(&mut self, offset: i32, limit: i32) -> Result<Vec<Playlist>, Error>;
    async fn album(&mut self, id: &str) -> Result<Album, Error>;
    async fn artist(&mut self, id: &str) -> Result<Artist, Error>;
    async fn track(&mut self, id: &str) -> Result<Track, Error>;
    async fn playlist(&mut self, id: &str) -> Result<Playlist, Error>;
}

#[async_trait]
pub trait Player {
    async fn play(&mut self) -> Result<(), Error>;
    async fn pause(&mut self) -> Result<(), Error>;
    async fn stop(&mut self) -> Result<(), Error>;
    async fn next(&mut self) -> Result<(), Error>;
    async fn previous(&mut self) -> Result<(), Error>;
    async fn seek(&mut self, position: u32) -> Result<(), Error>;
    async fn load_tracks(&mut self, tracks: Vec<Track>) -> Result<(), Error>;
}

pub struct CurrentDevice {
    pub source: Option<Box<dyn Browseable + Send>>,
    pub receiver: Option<Box<dyn Player + Send>>,
    pub source_device: Option<Device>,
    pub receiver_device: Option<Device>,
}

impl CurrentDevice {
    pub fn new() -> Self {
        Self {
            source: None,
            receiver: None,
            source_device: None,
            receiver_device: None,
        }
    }

    pub fn set_source(&mut self, source: Box<dyn Browseable + Send>) {
        self.source = Some(source);
    }

    pub fn set_source_device(&mut self, device: Device) {
        self.source_device = Some(device);
    }

    pub fn clear_source(&mut self) -> Option<Device> {
        self.source = None;
        match self.source_device.take() {
            Some(device) => Some(device),
            None => None,
        }
    }

    pub fn set_receiver(&mut self, receiver: Box<dyn Player + Send>) {
        self.receiver = Some(receiver);
    }

    pub fn set_receiver_device(&mut self, device: Device) {
        self.receiver_device = Some(device);
    }

    pub fn clear_receiver(&mut self) -> Option<Device> {
        self.receiver = None;
        match self.receiver_device.take() {
            Some(device) => Some(device),
            None => None,
        }
    }

    pub fn get_source_device(&self) -> Option<Device> {
        match &self.source_device {
            Some(device) => Some(device.clone()),
            None => None,
        }
    }
}
