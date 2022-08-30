use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;

#[derive(PartialEq, Props)]
pub struct SliderProps {
    pub min: f32,
    pub max: f32,
    pub val: f32,
}

pub fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element {
    let min = cx.props.min;
    let max = cx.props.max;
    let val = cx.props.val;

    let width = (val - min) / (max - min) * 100.0;

    let onscroll = |ev: UiEvent<MouseData>| println!("Scroll!");

    cx.render(rsx! {
        view {
            height: "32",
            width: "auto",
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
