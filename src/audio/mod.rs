mod backend;
mod controller;

pub use controller::{Controller, GCastController};

mod gcast_discoverer;
mod player;
mod song;

pub use gcast_discoverer::{GCastDevice, GCastDiscoverer, GCastDiscovererMessage};
pub use player::{PlaybackState, Player};
pub use song::Song;

impl Player {
    pub fn handle_output_device_change(&self) {
        self.backend.lock().unwrap().gstreamer.handle_output_device_change();
    }
}
