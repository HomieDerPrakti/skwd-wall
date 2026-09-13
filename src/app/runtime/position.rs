use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use skwd_config::keys::selector::{LAST_APPLIED_KEY, LAST_BROWSE_POSITION, START_POSITION};

use crate::app::App;

#[derive(Serialize, Deserialize)]
struct BrowsePosition {
    selection: String,
    anchor: String,
    layout: String,
    camera: f32,
}

impl App {
    fn position_layout(&self) -> String {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        format!(
            "{:?}",
            (self.scene.mode, self.scene.sp, self.scene.gp, self.scene.hp, self.scene.xp)
        )
        .hash(&mut hash);
        for &index in &self.library_session.filtered {
            let item = &self.library_session.library.catalog().items[index as usize];
            (&item.key, item.width, item.height).hash(&mut hash);
        }
        format!("{:016x}", hash.finish())
    }

    pub(in crate::app) fn save_browse_position(&mut self) {
        if self.config.str_path(START_POSITION) != "browsing" || self.runtime_state.demo.is_some() {
            return;
        }
        let Some(&index) = self.library_session.filtered.get(self.scene.current) else {
            return;
        };
        let selection = self.library_session.library.catalog().items[index as usize].key.clone();
        let (width, height) = self.scene.viewport;
        let anchor = self
            .scene
            .render
            .hits
            .iter()
            .min_by(|left, right| {
                let distance = |hit: &crate::frontend::scene::layout::Hit| {
                    (hit.cx - width * 0.5).hypot(hit.cy - height * 0.5)
                };
                distance(left).total_cmp(&distance(right))
            })
            .and_then(|hit| self.library_session.filtered.get(hit.index))
            .map_or_else(
                || selection.clone(),
                |&index| self.library_session.library.catalog().items[index as usize].key.clone(),
            );
        let position = BrowsePosition {
            selection,
            anchor,
            layout: self.position_layout(),
            camera: self.scene.camera_target(),
        };
        if let Ok(value) = serde_json::to_value(position) {
            self.config.save_key(LAST_BROWSE_POSITION, value);
        }
    }

    pub(in crate::app) fn restore_start_position(&mut self) {
        if self.library_session.library.catalog().items.is_empty()
            || self.library_session.selection_restored
        {
            return;
        }
        self.library_session.selection_restored = true;
        if self.runtime_state.demo.is_some() {
            return;
        }
        let (key, camera) = match self.config.str_path(START_POSITION).as_str() {
            "applied" => (self.config.str_path(LAST_APPLIED_KEY), None),
            "browsing" => {
                let Some(position) = skwd_config::get(self.config.root(), LAST_BROWSE_POSITION)
                    .and_then(|value| serde_json::from_value::<BrowsePosition>(value.clone()).ok())
                else {
                    return;
                };
                if position.layout == self.position_layout() {
                    (position.selection, Some(position.camera))
                } else {
                    (position.anchor, None)
                }
            }
            _ => return,
        };
        if let Some(index) = self.library_session.filtered.iter().position(|&index| {
            !key.is_empty()
                && self.library_session.library.catalog().items[index as usize].key == key
        }) {
            self.scene.restore_selection(index, self.library_session.filtered.len(), camera);
        }
    }
}
