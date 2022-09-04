use crate::sounds::{Category, SoundMetadata};

use super::prelude::*;
use super::sound::Sound;

#[derive(Props, PartialEq)]
pub struct CategoryProps {
    category: Category,
    sounds: Vec<SoundMetadata>,
}

pub fn category(cx: Scope<CategoryProps>) -> Element {
    let category = cx.props.category;
    let sounds = &cx.props.sounds;

    let category_sounds = sounds
        .iter()
        .filter(|sound| sound.category == category)
        .map(|sound| {
            cx.render(rsx!(Sound {
                sound: sound.clone()
            }))
        });

    cx.render(rsx! {
        view {
            height: "auto",
            width: "100%",
            padding: "60",
            background: "black",
            text {
                "{&category}"
            },
            category_sounds
        }
    })
}
