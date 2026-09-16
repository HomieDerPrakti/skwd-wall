use iced::alignment::{Horizontal, Vertical};
use iced::widget::Row;
use iced::widget::row::Wrapping;
use iced::{Element, Length, Padding};

pub fn rtl() -> bool {
    crate::i18n::is_rtl()
}

pub struct Flow<'a, Message> {
    children: Vec<Element<'a, Message>>,
    spacing: f32,
    padding: Padding,
    width: Option<Length>,
    height: Option<Length>,
    align_y: Vertical,
    clip: bool,
}

impl<'a, Message: 'a> Flow<'a, Message> {
    pub fn new(children: Vec<Element<'a, Message>>) -> Self {
        Self {
            children,
            spacing: 0.0,
            padding: Padding::ZERO,
            width: None,
            height: None,
            align_y: Vertical::Top,
            clip: false,
        }
    }

    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn push_maybe(self, child: Option<impl Into<Element<'a, Message>>>) -> Self {
        match child {
            Some(child) => self.push(child),
            None => self,
        }
    }

    pub fn extend(mut self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn spacing(mut self, spacing: impl Into<iced::Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn align_y(mut self, align: impl Into<Vertical>) -> Self {
        self.align_y = align.into();
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn wrap(self) -> Wrapping<'a, Message> {
        self.row().wrap()
    }

    pub fn row(self) -> Row<'a, Message> {
        let mut row = Row::with_children(ordered(self.children, rtl()))
            .spacing(self.spacing)
            .padding(self.padding)
            .align_y(self.align_y)
            .clip(self.clip);
        if let Some(width) = self.width {
            row = row.width(width);
        }
        if let Some(height) = self.height {
            row = row.height(height);
        }
        row
    }
}

impl<'a, Message: 'a> From<Flow<'a, Message>> for Element<'a, Message> {
    fn from(flow: Flow<'a, Message>) -> Self {
        flow.row().into()
    }
}

macro_rules! row {
    () => {
        $crate::frontend::ui::Flow::new(Vec::new())
    };
    ($($x:expr),+ $(,)?) => {
        $crate::frontend::ui::Flow::new(vec![$(::iced::Element::from($x)),+])
    };
}
pub(crate) use row;

pub fn ordered<T>(mut items: Vec<T>, rtl: bool) -> Vec<T> {
    if rtl {
        items.reverse();
    }
    items
}

pub fn start() -> Horizontal {
    start_for(rtl())
}

pub fn end() -> Horizontal {
    end_for(rtl())
}

pub fn start_for(rtl: bool) -> Horizontal {
    if rtl { Horizontal::Right } else { Horizontal::Left }
}

pub fn end_for(rtl: bool) -> Horizontal {
    if rtl { Horizontal::Left } else { Horizontal::Right }
}

pub fn logical_padding(top: f32, end: f32, bottom: f32, start: f32) -> Padding {
    logical_padding_for(rtl(), top, end, bottom, start)
}

pub fn logical_padding_for(rtl: bool, top: f32, end: f32, bottom: f32, start: f32) -> Padding {
    if rtl {
        Padding { top, right: start, bottom, left: end }
    } else {
        Padding { top, right: end, bottom, left: start }
    }
}

pub fn mirror_x(x: f32, width: f32, extent: f32) -> f32 {
    mirror_x_for(rtl(), x, width, extent)
}

pub fn mirror_x_for(rtl: bool, x: f32, width: f32, extent: f32) -> f32 {
    if rtl { extent - x - width } else { x }
}
