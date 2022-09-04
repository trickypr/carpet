use dioxus::{core::UiEvent, events::MouseData};

use super::prelude::*;

#[derive(Props)]
pub struct SliderProps<'a> {
    pub min: f32,
    pub max: f32,
    pub val: f32,
    pub onchange: EventHandler<'a, f32>,
}

pub fn Slider<'a>(cx: Scope<'a, SliderProps<'a>>) -> Element {
    let min = cx.props.min;
    let max = cx.props.max;
    let val = use_state(&cx, || cx.props.val);

    let width = (val - min) / (max - min) * 100.0;
    let width = width.round() as u32;

    let onscroll = move |ev: UiEvent<MouseData>| {
        let page = ev.coordinates().page();

        let mut scroll = *val.get();

        scroll += (page.y as f32) / 50.0;
        scroll = scroll.min(max).max(min);

        val.set(scroll);

        cx.props.onchange.call(scroll);
    };

    cx.render(rsx! {
        view {
            height: "32",
            width: "100%",
            padding: "16",
            onscroll: onscroll,
            view {
                width: "{width}%",
                height: "stretch",
                background: "blue",
            }
        }
    })
}
