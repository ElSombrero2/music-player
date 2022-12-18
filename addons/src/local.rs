use super::{Addon, Browseable, Player, StreamingAddon};
use anyhow::Error;
use async_trait::async_trait;
use music_player_client::{
    library::LibraryClient, playback::PlaybackClient, playlist::PlaylistClient,
    tracklist::TracklistClient,
};
use music_player_types::types::{Album, Artist, Device, Track};

pub struct Client {
    pub library: LibraryClient,
    pub playback: PlaybackClient,
    pub playlist: PlaylistClient,
    pub tracklist: TracklistClient,
}

pub struct Local {
    name: String,
    version: String,
    author: String,
    description: String,
    enabled: bool,
    client: Option<Client>,
    host: String,
    port: u16,
}

impl Local {
    pub fn new() -> Self {
        Self {
            name: "Local".to_string(),
            version: "0.1.0".to_string(),
            author: "Tsiry Sandratraina".to_string(),
            description: "Local addon".to_string(),
            enabled: true,
            client: None,
            host: "localhost".to_string(),
            port: 5051,
        }
    }
}

impl Addon for Local {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn author(&self) -> &str {
        &self.author
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl StreamingAddon for Local {
    fn stream(&self, url: &str) -> Result<(), Error> {
        todo!("Implement Local::stream");
    }
}

#[async_trait]
impl Browseable for Local {
    async fn albums(&mut self) -> Result<Vec<Album>, Error> {
        self.client.as_mut().unwrap().library.albums().await?;
        todo!()
    }

    async fn artists(&mut self) -> Result<Vec<Artist>, Error> {
        self.client.as_mut().unwrap().library.artists().await?;
        todo!()
    }

    async fn tracks(&mut self) -> Result<Vec<Track>, Error> {
        self.client.as_mut().unwrap().library.songs().await?;
        todo!()
    }

    async fn album(&mut self, id: &str) -> Result<Album, Error> {
        self.client.as_mut().unwrap().library.album(id).await?;
        todo!()
    }

    async fn artist(&mut self, id: &str) -> Result<Artist, Error> {
        self.client.as_mut().unwrap().library.artist(id).await?;
        todo!()
    }

    async fn track(&mut self, id: &str) -> Result<Track, Error> {
        todo!()
    }
}

#[async_trait]
impl Player for Local {
    async fn play(&mut self) -> Result<(), Error> {
        self.client.as_mut().unwrap().playback.play().await?;
        todo!()
    }

    async fn pause(&mut self) -> Result<(), Error> {
        self.client.as_mut().unwrap().playback.pause().await?;
        todo!()
    }

    async fn stop(&mut self) -> Result<(), Error> {
        self.client.as_mut().unwrap().playback.stop().await?;
        todo!()
    }

    async fn next(&mut self) -> Result<(), Error> {
        self.client.as_mut().unwrap().playback.next().await?;
        todo!()
    }

    async fn previous(&mut self) -> Result<(), Error> {
        self.client.as_mut().unwrap().playback.prev().await?;
        todo!()
    }

    async fn seek(&mut self, position: u32) -> Result<(), Error> {
        todo!()
    }
}

impl From<Device> for Local {
    fn from(device: Device) -> Self {
        Self {
            host: device.host,
            port: device.port,
            ..Local::new()
        }
    }
}

impl Local {
    pub async fn connect(&mut self) -> Result<(), Error> {
        let client = Client {
            library: LibraryClient::new(self.host.clone(), self.port).await?,
            playback: PlaybackClient::new(self.host.clone(), self.port).await?,
            playlist: PlaylistClient::new(self.host.clone(), self.port).await?,
            tracklist: TracklistClient::new(self.host.clone(), self.port).await?,
        };

        self.client = Some(client);

        Ok(())
    }
}
