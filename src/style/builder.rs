#![allow(missing_docs)]
//! Defines fluent builder for styles.

use bevy::{
    asset::AssetPath,
    prelude::*,
    ui::{self, ZIndex},
};

pub struct StyleBuilder<'a, 'w> {
    pub(crate) target: &'a mut EntityWorldMut<'w>,
    pub(crate) style: ui::Style,
    pub(crate) style_changed: bool,
}

impl<'a, 'w> StyleBuilder<'a, 'w> {
    pub fn load_asset<A: Asset>(&mut self, path: AssetPath<'_>) -> Handle<A> {
        self.target.world_scope(|world| {
            let server = world.get_resource::<AssetServer>().unwrap();
            server.load(path)
        })
    }
}

// LineBreak(BreakLineOn),

// fn pointer_events(&mut self, pe: PointerEvents) -> &mut Self {
//     self.props.push(StyleProp::PointerEvents(pe));
//     self
// }

// fn scale_x(&mut self, scale: f32) -> &mut Self {
//     self.props.push(StyleProp::ScaleX(scale));
//     self
// }

// fn scale_y(&mut self, scale: f32) -> &mut Self {
//     self.props.push(StyleProp::ScaleY(scale));
//     self
// }

// fn scale(&mut self, scale: f32) -> &mut Self {
//     self.props.push(StyleProp::Scale(scale));
//     self
// }

// fn rotation(&mut self, rot: f32) -> &mut Self {
//     self.props.push(StyleProp::Rotation(rot));
//     self
// }

// fn translation(&mut self, trans: Vec3) -> &mut Self {
//     self.props.push(StyleProp::Translation(trans));
//     self
// }

// fn transition(&mut self, transition: &[Transition]) -> &mut Self {
//     self.props
//         .push(StyleProp::Transition(Vec::from(transition)));
//     self
// }

/// Trait that represents a CSS color
pub trait ColorParam {
    fn to_val(self) -> Option<Color>;
}

impl ColorParam for Option<Color> {
    fn to_val(self) -> Option<Color> {
        self
    }
}

impl ColorParam for Color {
    fn to_val(self) -> Option<Color> {
        Some(self)
    }
}

impl ColorParam for &str {
    fn to_val(self) -> Option<Color> {
        Some(Color::hex(self).unwrap())
    }
}

/// Trait that represents a CSS "length"
pub trait LengthParam {
    fn to_val(self) -> ui::Val;
}

impl LengthParam for ui::Val {
    fn to_val(self) -> ui::Val {
        self
    }
}

impl LengthParam for f32 {
    fn to_val(self) -> ui::Val {
        ui::Val::Px(self)
    }
}

impl LengthParam for i32 {
    fn to_val(self) -> ui::Val {
        ui::Val::Px(self as f32)
    }
}

/// Trait that represents a CSS Z-index
pub trait ZIndexParam {
    fn to_val(self) -> ZIndex;
}

impl ZIndexParam for ZIndex {
    fn to_val(self) -> ZIndex {
        self
    }
}

impl ZIndexParam for i32 {
    fn to_val(self) -> ZIndex {
        ZIndex::Local(self)
    }
}

/// Trait that represents CSS edge widths (margin, padding, etc.)
pub trait UiRectParam {
    fn to_uirect(self) -> ui::UiRect;
}

impl UiRectParam for ui::UiRect {
    fn to_uirect(self) -> ui::UiRect {
        self
    }
}

impl UiRectParam for ui::Val {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(self)
    }
}

impl UiRectParam for f32 {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self))
    }
}

impl UiRectParam for i32 {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self as f32))
    }
}

impl<H: LengthParam, V: LengthParam> UiRectParam for (H, V) {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::axes(self.0.to_val(), self.1.to_val())
    }
}

/// Trait that represents an optional float
pub trait OptFloatParam {
    fn to_val(self) -> Option<f32>;
}

impl OptFloatParam for Option<f32> {
    fn to_val(self) -> Option<f32> {
        self
    }
}

impl OptFloatParam for f32 {
    fn to_val(self) -> Option<f32> {
        Some(self)
    }
}

impl OptFloatParam for i32 {
    fn to_val(self) -> Option<f32> {
        Some(self as f32)
    }
}

/// Trait that represents an optional float
pub trait AssetPathParam<'a> {
    fn to_path(self) -> Option<AssetPath<'a>>;
}

impl<'a> AssetPathParam<'a> for Option<AssetPath<'a>> {
    fn to_path(self) -> Option<AssetPath<'a>> {
        self
    }
}

impl<'a> AssetPathParam<'a> for AssetPath<'a> {
    fn to_path(self) -> Option<AssetPath<'a>> {
        Some(self)
    }
}

impl<'a> AssetPathParam<'a> for &'a str {
    fn to_path(self) -> Option<AssetPath<'a>> {
        Some(AssetPath::parse(self))
    }
}