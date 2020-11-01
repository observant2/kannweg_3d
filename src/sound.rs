use crate::player::WalkState;
use crate::player::WalkState::Running;
use rand::{thread_rng, Rng};
use rg3d::engine::resource_manager::{ResourceManager, SharedSoundBuffer};
use rg3d::sound::buffer::{DataSource, SoundBuffer};
use rg3d::sound::context::Context;
use rg3d::sound::pool::Handle;
use rg3d::sound::source::generic::GenericSourceBuilder;
use rg3d::sound::source::{SoundSource, Status};

pub fn start_ambient_sound() {
    // Initialize new sound context with default output device.
    let context = Context::new().unwrap();

    // Load sound buffer.
    let humming_buffer =
        SoundBuffer::new_streaming(DataSource::from_file("assets/humming.ogg").unwrap()).unwrap();

    // Create flat source (without spatial effects) using that buffer.
    let source = GenericSourceBuilder::new(humming_buffer)
        .with_status(Status::Playing)
        .with_looping(true)
        .with_gain(0.1)
        .build_source()
        .unwrap();

    // Each sound sound must be added to context, context takes ownership on source
    // and returns pool handle to it by which it can be accessed later on if needed.
    let _source_handle: Handle<SoundSource> = context.lock().unwrap().add_source(source);
}

pub async fn load_footstep_sounds(resource_manager: &mut ResourceManager) -> SharedSoundBuffer {
    resource_manager
        .request_sound_buffer("assets/footstep.ogg", false)
        .await
        .unwrap()
}

pub fn play_footstep(ctx: &mut Context, foot_step: SharedSoundBuffer, walk_state: &WalkState) {
    let gain = if *walk_state == Running { 0.15 } else { 0.07 };
    ctx.add_source(
        GenericSourceBuilder::new(foot_step.into())
            .with_play_once(true)
            .with_gain(gain)
            .with_pitch(thread_rng().gen_range(0.85, 1.0))
            .with_status(Status::Playing)
            .build_source()
            .unwrap(),
    );
}
