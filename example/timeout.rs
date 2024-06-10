use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_activation::{ActivationPlugin, ActiveState, TimeoutEvent};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 60.0),
            )),
        )
        .add_plugins(LogPlugin::default())
        .add_plugins(ActivationPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                check_active,
                timeout_event,
                reactive_idle.run_if(time_passed(5.0)),
            ),
        )
        .run();
}

#[derive(Component)]
struct TestAlive(pub &'static str);

fn setup(mut commands: Commands) {
    // this entity always active
    commands.spawn((TestAlive("always alive component"), ActiveState::default()));
    // this entity will be active for 2 seconds
    commands.spawn((
        TestAlive("ttl 2 secs component"),
        ActiveState::new(Duration::from_secs(2)),
    ));
}

fn check_active(q: Query<(&TestAlive, &ActiveState)>) {
    for (TestAlive(name), active_state) in q.iter() {
        info!("'{}'  active: {}", name, active_state.is_active());
    }
}

fn timeout_event(mut timeout_event: EventReader<TimeoutEvent>) {
    for timeout_ev in timeout_event.read() {
        warn!("entity {:?} is idle", timeout_ev.0);
    }
}

fn reactive_idle(mut q_idle: Query<&mut ActiveState>) {
    for mut active_state in q_idle.iter_mut() {
        if active_state.is_idle() {
            active_state.toggle();
        }
    }
}

fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut timer: Local<f32>, time: Res<Time>| {
        // Tick the timer
        *timer += time.delta_seconds();
        // Return true if the timer has passed the time
        *timer >= t
    }
}
