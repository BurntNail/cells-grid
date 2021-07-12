use crate::states::CellState;
use find_folder::Search::ParentsThenKids;
use piston_window::{Flip, G2dTexture, PistonWindow, Texture, TextureSettings};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Cacher {
    path: PathBuf,
    assets: HashMap<String, G2dTexture>,
}

impl Cacher {
    pub fn new() -> Self {
        let path = ParentsThenKids(3, 3)
            .for_folder("art")
            .expect("assets folder not found");
        Cacher {
            path,
            assets: HashMap::new(),
        }
    }

    pub fn get(&self, p: String) -> Option<&G2dTexture> {
        self.assets.get(p.as_str())
    }
    fn insert(&mut self, p: String, win: &mut PistonWindow) {
        let path = self.path.join(p.clone());
        let tex: G2dTexture = Texture::from_path(
            &mut win.create_texture_context(),
            &path,
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Error getting img");

        self.assets.insert(p, tex);
    }
    pub fn populate(&mut self, win: &mut PistonWindow) {
        for state in CellState::get_all_states() {
            self.insert(state.get_sprite_name(), win);
        }
    }
}
