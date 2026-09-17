mod chrome;
mod composition;
mod panels;
mod settings;
mod tags;
mod transient;

#[cfg(test)]
pub(super) use chrome::overview_set;
pub(crate) use chrome::{bar_show, filter_bar_footprint};
#[cfg(test)]
pub(crate) use composition::tag_cloud_visible;
pub(crate) use composition::view_count;
pub use composition::{view, view_single};
#[cfg(test)]
pub(crate) use tags::{cloud_fit_height, cloud_fit_width};
