#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputScope {
    Everywhere,
    Picker,
    Fields,
    Search,
    Downloads,
}

impl InputScope {
    pub const fn overlaps(self, other: Self) -> bool {
        !matches!(
            (self, other),
            (Self::Picker, Self::Fields | Self::Search | Self::Downloads)
                | (Self::Fields | Self::Search | Self::Downloads, Self::Picker)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ActiveScopes {
    pub fields: bool,
    pub search: bool,
    pub downloads: bool,
}

impl ActiveScopes {
    pub const fn contains(self, scope: InputScope) -> bool {
        match scope {
            InputScope::Everywhere => true,
            InputScope::Picker => !self.fields,
            InputScope::Fields => self.fields,
            InputScope::Search => self.search,
            InputScope::Downloads => self.downloads,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    Select,
    Apply,
    Flip,
    Favourite,
    Effects,
    Studio,
    SceneProperties,
    Playlists,
    Settings,
    Help,
    ThemePanel,
    TagCloud,
    TagMode,
    FilterBar,
    FolderPrev,
    FolderNext,
    FolderToggle,
    HiddenFolders,
    ColorPrev,
    ColorNext,
    NavLeft,
    NavRight,
    NavUp,
    NavDown,
    Autocomplete,
    TypePrev,
    TypeNext,
    SortPrev,
    SortNext,
    RandomRotate,
    Downloads,
    SearchMode,
    SourceWallhaven,
    SourceSteam,
    SourceUnsplash,
    SourcePexels,
    SourceYoutube,
    SourceBing,
}

impl InputAction {
    pub const ALL: [Self; 38] = [
        Self::Select,
        Self::Apply,
        Self::Flip,
        Self::Favourite,
        Self::Effects,
        Self::Studio,
        Self::SceneProperties,
        Self::Playlists,
        Self::Settings,
        Self::Help,
        Self::ThemePanel,
        Self::TagCloud,
        Self::TagMode,
        Self::FilterBar,
        Self::FolderPrev,
        Self::FolderNext,
        Self::FolderToggle,
        Self::HiddenFolders,
        Self::ColorPrev,
        Self::ColorNext,
        Self::NavLeft,
        Self::NavRight,
        Self::NavUp,
        Self::NavDown,
        Self::Autocomplete,
        Self::TypePrev,
        Self::TypeNext,
        Self::SortPrev,
        Self::SortNext,
        Self::RandomRotate,
        Self::Downloads,
        Self::SearchMode,
        Self::SourceWallhaven,
        Self::SourceSteam,
        Self::SourceUnsplash,
        Self::SourcePexels,
        Self::SourceYoutube,
        Self::SourceBing,
    ];

    pub const fn default_binding(self) -> &'static str {
        match self {
            Self::Select => "click",
            Self::Apply => "enter",
            Self::Flip => "right-click",
            Self::Favourite => "f",
            Self::Effects => "ctrl+click",
            Self::Studio => "shift+right-click",
            Self::SceneProperties => "shift+w",
            Self::Playlists => "p",
            Self::Settings => "shift+s",
            Self::Help => "?",
            Self::ThemePanel => "c",
            Self::TagCloud => "shift+down",
            Self::TagMode => "t",
            Self::FilterBar => "shift+up",
            Self::FolderPrev => "ctrl+left",
            Self::FolderNext => "ctrl+right",
            Self::FolderToggle => "ctrl+m",
            Self::HiddenFolders => "ctrl+h",
            Self::ColorPrev => "shift+left",
            Self::ColorNext => "shift+right",
            Self::NavLeft => "left",
            Self::NavRight => "right",
            Self::NavUp => "up",
            Self::NavDown => "down",
            Self::Autocomplete | Self::TypeNext => "tab",
            Self::TypePrev => "shift+tab",
            Self::SortPrev => "alt+left",
            Self::SortNext => "alt+right",
            Self::RandomRotate => "ctrl+r",
            Self::Downloads => "ctrl+d",
            Self::SearchMode => "ctrl+tab",
            Self::SourceWallhaven => "1",
            Self::SourceSteam => "2",
            Self::SourceUnsplash => "3",
            Self::SourcePexels => "4",
            Self::SourceYoutube => "5",
            Self::SourceBing => "6",
        }
    }

    pub const fn scope(self) -> InputScope {
        match self {
            Self::TypePrev | Self::TypeNext => InputScope::Picker,
            Self::Autocomplete => InputScope::Fields,
            Self::SearchMode => InputScope::Search,
            Self::SourceWallhaven
            | Self::SourceSteam
            | Self::SourceUnsplash
            | Self::SourcePexels
            | Self::SourceYoutube
            | Self::SourceBing => InputScope::Downloads,
            _ => InputScope::Everywhere,
        }
    }

    pub const fn targets_card(self) -> bool {
        matches!(
            self,
            Self::Select
                | Self::Apply
                | Self::Flip
                | Self::Favourite
                | Self::Effects
                | Self::Studio
                | Self::SceneProperties
        )
    }
}
