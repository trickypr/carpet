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
        container {
            width: "100%",
            padding: "32",

            label {
                font_size: "30",
                height: "30",
                "{&category}"
            },
            category_sounds
        }
    })
}
