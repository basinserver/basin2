use std::sync::{Arc, RwLock};

pub struct BaseWorld {
    
}

pub trait WorldTrait {
    
}

pub type World = Arc<RwLock<Box<dyn WorldTrait + Send + Sync>>>;
