mod gcast_controller;
mod inhibit_controller;
mod mini_controller;
#[cfg(unix)]
mod mpris_controller;
mod sidebar_controller;
mod toolbar_controller;

pub use gcast_controller::GCastController;
pub use inhibit_controller::InhibitController;
pub use mini_controller::MiniController;
#[cfg(unix)]
pub use mpris_controller::MprisController;
pub use sidebar_controller::SidebarController;
pub use toolbar_controller::ToolbarController;

use crate::api::SwStation;
use crate::audio::PlaybackState;

pub trait Controller {
    fn set_station(&self, station: SwStation);
    fn set_playback_state(&self, playback_state: &PlaybackState);
    fn set_volume(&self, volume: f64);
    fn set_song_title(&self, title: &str);
    fn handle_output_device_change(&self);
}
