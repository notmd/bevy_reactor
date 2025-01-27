use bevy::{prelude::*, ui};
use bevy_color::{Alpha, Luminance};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{ListenerInput, On},
};
use bevy_reactor::*;

use crate::{
    animation::{AnimatedBackgroundColor, AnimatedScale, AnimatedTransition},
    colors,
    focus::{KeyPressEvent, TabGroup},
    hooks::{BistableTransitionState, CreateBistableTransition},
    typography::text_default,
};

// Dialog background overlay
fn style_dialog_overlay(ss: &mut StyleBuilder) {
    ss.position(PositionType::Absolute)
        .display(ui::Display::Flex)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .left(0)
        .top(0)
        .right(0)
        .bottom(0)
        .z_index(100)
        .background_color(colors::U2.with_alpha(0.0));
}

fn style_dialog(ss: &mut StyleBuilder) {
    ss.background_color(colors::U2)
        .position(PositionType::Relative)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Stretch)
        .border_color(colors::U1)
        .width(400)
        .border(3);
    // .scale(0.5)
    // .transition(&[Transition {
    //     property: TransitionProperty::Transform,
    //     duration: 0.3,
    //     timing: timing::EASE_IN_OUT,
    //     ..default()
    // }])
    // .selector(".entering > &,.entered > &", |ss| ss.scale(1.));
}

const TRANSITION_DURATION: f32 = 0.3;

/// Displays a modal dialog box. This will display the dialog frame and the backdrop overlay.
/// Use the dialog header/body/footer controls to get the standard layout.
#[derive(Default)]
pub struct Dialog {
    /// The width of the dialog, one of several standard widths.
    pub width: ui::Val,

    /// Signal that controls whether the dialog is open. Note that when this becomes false,
    /// the dialog will still remain visible until it completes its closing animation.
    pub open: Signal<bool>,

    /// The content of the dialog.
    pub children: ViewHandle,

    /// Callback called when the dialog's close button is clicked.
    pub on_close: Option<Callback>,

    /// Callback called when the dialog has completed it's closing animation.
    pub on_exited: Option<Callback>,
}

impl ViewFactory for Dialog {
    fn create(&self, cx: &mut Cx) -> impl View + Send + Sync + 'static {
        let on_close = self.on_close;
        let on_exited = self.on_exited;
        let state = cx.create_bistable_transition(self.open, TRANSITION_DURATION);
        let children = self.children.clone();
        let width = self.width;

        cx.create_effect(move |ve| {
            let state = state.get(ve);
            if state == BistableTransitionState::Exited {
                if let Some(on_exited) = on_exited {
                    ve.run_callback(on_exited, ());
                }
            }
        });

        Cond::new(
            move |cx| state.get(cx) != BistableTransitionState::Exited,
            move || {
                Portal::new(
                    Element::<NodeBundle>::new()
                        .with_styles(style_dialog_overlay)
                        .insert((
                            // Click on backdrop sends close signal.
                            On::<Pointer<Click>>::run(move |world: &mut World| {
                                if let Some(on_close) = on_close {
                                    world.run_callback(on_close, ());
                                }
                            }),
                            On::<KeyPressEvent>::run({
                                move |world: &mut World| {
                                    let mut event = world
                                        .get_resource_mut::<ListenerInput<KeyPressEvent>>()
                                        .unwrap();
                                    if !event.repeat && event.key_code == KeyCode::Escape {
                                        event.stop_propagation();
                                        if let Some(on_close) = on_close {
                                            world.run_callback(on_close, ());
                                        }
                                    }
                                }
                            }),
                        ))
                        .create_effect(move |cx, ent| {
                            let state = state.get(cx);
                            let mut entt = cx.world_mut().entity_mut(ent);
                            let target = match state {
                                BistableTransitionState::Entering
                                | BistableTransitionState::Entered
                                | BistableTransitionState::ExitStart => colors::U2.with_alpha(0.7),
                                BistableTransitionState::EnterStart
                                | BistableTransitionState::Exiting
                                | BistableTransitionState::Exited => colors::U2.with_alpha(0.0),
                            };
                            AnimatedTransition::<AnimatedBackgroundColor>::start(
                                &mut entt,
                                target,
                                TRANSITION_DURATION,
                            );
                        })
                        .with_children(
                            Element::<NodeBundle>::new()
                                .insert(TabGroup {
                                    order: 0,
                                    modal: true,
                                })
                                .with_styles((
                                    text_default,
                                    style_dialog,
                                    move |ss: &mut StyleBuilder| {
                                        ss.width(width);
                                    },
                                ))
                                .create_effect(move |cx, ent| {
                                    let state = state.get(cx);
                                    let mut entt = cx.world_mut().entity_mut(ent);
                                    let target = match state {
                                        BistableTransitionState::EnterStart
                                        | BistableTransitionState::Exiting
                                        | BistableTransitionState::Exited => Vec3::splat(0.0),
                                        BistableTransitionState::Entering
                                        | BistableTransitionState::Entered
                                        | BistableTransitionState::ExitStart => Vec3::splat(1.0),
                                    };
                                    AnimatedTransition::<AnimatedScale>::start(
                                        &mut entt,
                                        target,
                                        TRANSITION_DURATION,
                                    );
                                })
                                .with_child(&children),
                        ),
                )
            },
            || (),
        )
    }
}

fn style_dialog_header(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::SpaceBetween)
        .font_size(18)
        .border_color(colors::U2.darker(0.01))
        .border_bottom(1)
        .padding((12, 6));
}

/// Displays a standard dialog header.
#[derive(Clone, Default)]
pub struct DialogHeader {
    /// The content of the dialog header.
    pub children: ViewHandle,
}

impl ViewFactory for DialogHeader {
    fn create(&self, _cx: &mut Cx) -> impl View + Send + Sync + 'static {
        Element::<NodeBundle>::new()
            .with_styles(style_dialog_header)
            .with_child(&self.children)
    }
}

fn style_dialog_body(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch)
        .justify_content(ui::JustifyContent::FlexStart)
        .padding((12, 6))
        .min_height(200);
}

/// Displays a standard dialog body.
#[derive(Clone, Default)]
pub struct DialogBody {
    /// The content of the dialog header.
    pub children: ViewHandle,
}

impl ViewFactory for DialogBody {
    fn create(&self, _cx: &mut Cx) -> impl View + Send + Sync + 'static {
        Element::<NodeBundle>::new()
            .with_styles(style_dialog_body)
            .with_child(&self.children)
    }
}

fn style_dialog_footer(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexEnd)
        .align_items(ui::AlignItems::Center)
        .border_color(colors::U2.darker(0.01))
        .border_top(1)
        .column_gap(4)
        .padding((8, 6));
}

/// Displays a standard dialog footer.
#[derive(Clone, Default)]
pub struct DialogFooter {
    /// The content of the dialog header.
    pub children: ViewHandle,
}

impl ViewFactory for DialogFooter {
    fn create(&self, _cx: &mut Cx) -> impl View + Send + Sync + 'static {
        Element::<NodeBundle>::new()
            .with_styles(style_dialog_footer)
            .with_child(&self.children)
    }
}
