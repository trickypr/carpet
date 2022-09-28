use std::sync::mpsc::Receiver;

use crate::{config::Config, get_holder, ControlThreadCommand};

pub fn control(tx: Receiver<ControlThreadCommand>, mut config: Config) {
    loop {
        if let Ok(command) = tx.recv() {
            let mut holder = get_holder();

            match command {
                crate::ControlThreadCommand::ChangeVolume(id, volume) => {
                    let sound = holder.get_sound_mut(id);

                    if let Some(sound) = sound {
                        sound.volume = volume;
                        config.sound_volume.insert(sound.name.clone(), volume);
                        holder.correct_volume();
                    }
                }
                crate::ControlThreadCommand::SetPlaying(local_is_playing) => {
                    holder.is_playing = local_is_playing;
                    holder.correct_volume();

                    config.is_playing = Some(local_is_playing);
                }
            }

            config.save();
        }
    }
}
