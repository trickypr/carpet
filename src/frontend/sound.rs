use crate::{sounds::SoundMetadata, ControlThreadCommand, RX};

use super::prelude::*;

#[derive(Props, PartialEq)]
pub struct SoundProps {
    sound: SoundMetadata,
}

#[allow(non_snake_case)]
pub fn Sound(cx: Scope<SoundProps>) -> Element {
    let sound = &cx.props.sound;
    let sound_volume = cx.props.sound.volume as f64 * 100.0;

    let volume_change = move |vol: f64| {
        let rx = unsafe { RX.as_ref().unwrap() };
        rx.send(ControlThreadCommand::ChangeVolume(
            sound.id,
            vol as f32 / 100.0,
        ))
        .unwrap();
    };

    cx.render(rsx! {
        rect {
            width: "100%",
            padding: "8",

            label {
                "{&sound.name}"
            },
            Slider {
                width: 100.0,
                starting_value: sound_volume,
                onmoved: volume_change
            }
        },
    })
}
