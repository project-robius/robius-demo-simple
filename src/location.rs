//! Functions for querying the device's current location.

use std::time::SystemTime;

use makepad_widgets::{Cx, error, log};
use robius_location::{Access, Accuracy, Coordinates, Location, Manager};

/// The action emitted upon every location update.
#[derive(Copy, Clone, Debug)]
pub enum LocationAction {
    /// The location handler received a new location update.
    Update(LocationUpdate),
    /// The location handler encountered an error.
    Error(robius_location::Error),
    None
}

/// An updated location sample, including coordinates and a system timestamp.
#[derive(Copy, Clone, Debug)]
pub struct LocationUpdate {
    pub coordinates: Coordinates,
    pub time: Option<SystemTime>,
}


struct LocationHandler;

impl robius_location::Handler for LocationHandler {
    fn handle(&self, location: Location<'_>) {
        let coords = location.coordinates();
        log!("Received location update: {coords:?}");
        match coords {
            Ok(coords) => {
                let update = LocationUpdate {
                    coordinates: coords,
                    time: location.time().ok(),
                };
                Cx::post_action(LocationAction::Update(update));
            }
            Err(e) => {
                error!("Error getting coordinates from location update: {e:?}");
                Cx::post_action(LocationAction::Error(e));
            }
        }
    }

    fn error(&self, e: robius_location::Error) {
        error!("Got error in location handler: {e:?}");
        Cx::post_action(LocationAction::Error(e));
    }
}


/// Initializes the location manager.
///
/// Requests permission to access the device's fine-grained location
/// in the foreground only, and then requests an initial location update.
pub fn init(_cx: &mut Cx) -> Result<Manager, robius_location::Error> {
    let manager = Manager::new(LocationHandler)?;
    manager.request_authorization(Access::Foreground, Accuracy::Precise)?;
    let _ = manager.update_once();
    Ok(manager)
}
