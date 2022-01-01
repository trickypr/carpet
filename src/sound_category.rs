use orbtk::prelude::*;

use crate::{sound, MainState, SoundCategory};

pub fn sound_display(
    ctx: &mut BuildContext,
    sound: &sound::Sound,
    parent_id: Entity,
    category_id: usize,
) -> Entity {
    let sound_id = sound.id.clone();

    Stack::new()
        .child(TextBlock::new().text(sound.name.clone()).build(ctx))
        .child(
            Slider::new()
                .id(format!("sound_{}", sound.id.clone()))
                .min(0.0)
                .val(sound.volume.clone())
                .max(1.0)
                .min_width(100)
                .on_changed("val", move |states, widget_id| {
                    states
                        .get_mut::<MainState>(parent_id)
                        .change_volume(((category_id, sound_id), widget_id));
                })
                .build(ctx),
        )
        .build(ctx)
}

pub fn display(
    ctx: &mut BuildContext,
    category: &SoundCategory,
    parent_id: Entity,
    category_id: usize,
) -> Entity {
    let mut stack = Stack::new()
        .child(
            TextBlock::new()
                .text(category.name.clone())
                .style("header")
                .build(ctx),
        )
        .spacing(8);

    for sound in category.sounds.iter() {
        stack = stack.child(sound_display(ctx, sound, parent_id, category_id));
    }

    stack.build(ctx)
}
