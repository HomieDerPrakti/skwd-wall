use crate::domain::scene_properties::{SceneProperty, ScenePropertyKind, ScenePropertyValue};

#[derive(Clone, Debug, PartialEq)]
pub enum ScenePropMsg {
    Toggle(String),
    Slide(String, f64),
    Commit(String),
    Choose(String, f64),
    ColourInput(String, String),
    ColourSlide(String, usize, f64),
    ColourCommit(String),
    Fps(Option<u32>),
    FpsSlide(u32),
    FpsCommit,
    Reset,
    Close,
}

pub struct SceneProperties {
    pub we_id: String,
    pub title: String,
    pub rows: Vec<SceneProperty>,
    pub loading: bool,
    pub error: Option<String>,
    pub fps: Option<u32>,
    pub global_fps: Option<u32>,
    fps_revision: u64,
    pub revision: u64,
    accepted_revision: u64,
    edits: std::collections::BTreeMap<String, u64>,
    pub colour_drafts: std::collections::BTreeMap<String, String>,
}

impl SceneProperties {
    #[must_use]
    pub fn opening(we_id: &str, title: &str) -> Self {
        Self {
            we_id: we_id.to_string(),
            title: title.to_string(),
            rows: Vec::new(),
            loading: true,
            error: None,
            fps: None,
            global_fps: None,
            fps_revision: 0,
            revision: 0,
            accepted_revision: 0,
            edits: std::collections::BTreeMap::new(),
            colour_drafts: std::collections::BTreeMap::new(),
        }
    }

    pub fn accept(
        &mut self,
        we_id: &str,
        revision: u64,
        writes: &[String],
        mut rows: Vec<SceneProperty>,
    ) {
        if we_id != self.we_id {
            return;
        }
        if revision < self.accepted_revision {
            self.edits.retain(|name, edited| *edited > revision || !writes.contains(name));
            return;
        }
        for property in &mut rows {
            if let Some(current) = self.row(&property.name) {
                if self
                    .edits
                    .get(&property.name)
                    .is_some_and(|edited| *edited > revision || !writes.contains(&property.name))
                {
                    property.value = current.value.clone();
                    property.overridden = current.overridden;
                }
                if property.kind == ScenePropertyKind::Colour
                    && self.colour_draft(&property.name) == Some(current.value.display().as_str())
                {
                    self.colour_drafts.insert(property.name.clone(), property.value.display());
                }
            } else if property.kind == ScenePropertyKind::Colour {
                self.colour_drafts.insert(property.name.clone(), property.value.display());
            }
        }
        self.edits.retain(|name, edited| *edited > revision || !writes.contains(name));
        self.accepted_revision = revision;
        self.rows = rows;
        self.loading = false;
        self.error = None;
    }

    pub fn accept_fps(
        &mut self,
        we_id: &str,
        revision: u64,
        fps: Option<u32>,
        global: Option<u32>,
    ) {
        if we_id != self.we_id || revision < self.accepted_revision || revision < self.fps_revision
        {
            return;
        }
        self.fps = fps;
        self.global_fps = global;
    }

    pub fn set_fps_local(&mut self, fps: Option<u32>) {
        self.revision += 1;
        self.fps_revision = self.revision;
        self.fps = fps.map(|fps| fps.clamp(1, 240));
        self.error = None;
    }

    pub fn fail(&mut self, error: &str) {
        self.loading = false;
        self.error = Some(error.to_string());
    }

    #[must_use]
    pub fn row(&self, name: &str) -> Option<&SceneProperty> {
        self.rows.iter().find(|row| row.name == name)
    }

    #[must_use]
    pub fn shown_rows(&self) -> Vec<&SceneProperty> {
        self.rows.iter().filter(|row| row.shown(&self.rows)).collect()
    }

    #[must_use]
    pub fn editable_count(&self) -> usize {
        self.rows.iter().filter(|row| row.editable() && row.shown(&self.rows)).count()
            + usize::from(self.global_fps.is_some())
    }

    #[must_use]
    pub fn changed_count(&self) -> usize {
        self.rows.iter().filter(|row| row.changed()).count() + usize::from(self.fps.is_some())
    }

    #[must_use]
    pub fn colour_draft(&self, name: &str) -> Option<&str> {
        self.colour_drafts.get(name).map(String::as_str)
    }

    pub fn set_colour_draft(&mut self, name: &str, text: &str) {
        self.colour_drafts.insert(name.to_string(), text.to_string());
    }

    pub fn reset_local(&mut self) {
        self.set_fps_local(None);
        for row in &mut self.rows {
            row.value = row.default.clone();
            row.overridden = false;
            self.edits.insert(row.name.clone(), self.revision);
            if row.kind == ScenePropertyKind::Colour {
                self.colour_drafts.insert(row.name.clone(), row.value.display());
            }
        }
        self.error = None;
    }

    pub fn set_local(&mut self, name: &str, value: ScenePropertyValue) {
        if let Some(row) = self.rows.iter_mut().find(|row| row.name == name) {
            self.revision += 1;
            self.edits.insert(name.to_string(), self.revision);
            if row.kind == ScenePropertyKind::Colour {
                self.colour_drafts.insert(name.to_string(), value.display());
            }
            row.value = value;
            row.overridden = row.changed();
            self.error = None;
        }
    }
}
