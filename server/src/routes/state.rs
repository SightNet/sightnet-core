use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use lazy_static::lazy_static;

use sightnet_core::collection::Collection;

pub struct State {
    pub(crate) collections: HashMap<String, Arc<Mutex<Collection>>>,
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State {
        collections: HashMap::new()
    });
}
