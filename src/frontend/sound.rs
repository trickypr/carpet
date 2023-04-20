use crate::{sounds::SoundMetadata, ControlThreadCommand, RX};

use freya::prelude::*;

#[derive(Props, PartialEq)]
pub struct SoundProps {
    sound: SoundMetadata,
}

#[allow(non_snake_case)]
pub fn Sound(cx: Scope<SoundProps>) -> Element {
    let sound = &cx.props.sound;
    let sound_volume_state = use_state(&cx, || sound.volume as f64 * 100.0);

    let volume_change = move |vol: f64| {
        sound_volume_state.set(vol);
        let rx = unsafe { RX.as_ref().unwrap() };
        rx.send(ControlThreadCommand::ChangeVolume(
            sound.id,
            vol as f32 / 100.0,
        ))
        .unwrap();
    };

    render!(
        rect {
            width: "100%",
            padding: "8",

            label {
                "{&sound.name}"
            },
            Slider {
                width: 100.0,
                value: *sound_volume_state.get(),
                onmoved: volume_change
            }
        },
    )
}
