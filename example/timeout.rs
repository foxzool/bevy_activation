use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_activation::{ActivationPlugin, ActiveState, TimeoutEvent};
use std::time::Duration;
use bevy_time::common_conditions::on_timer;

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
                reactive_idle.run_if(on_timer(Duration::from_secs(5))),
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
    )).observe(on_timeout);
}

fn check_active(q: Query<(&TestAlive, &ActiveState)>) {
    for (TestAlive(name), active_state) in q.iter() {
        info!("'{}' active: {}", name, active_state.is_active());
    }
}

/// observe timeout event
fn on_timeout(trigger: Trigger<TimeoutEvent>) {
    warn!("entity {:?} timeout", trigger.target());
}

/// reactive idle component to active
fn reactive_idle(mut q_idle: Query<&mut ActiveState>) {
    for mut active_state in q_idle.iter_mut() {
        if active_state.is_idle() {
            active_state.toggle();
        }
    }
}
