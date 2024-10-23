# bevy_activation

[![Crates.io](https://img.shields.io/crates/v/bevy_activation)](https://crates.io/crates/bevy_activation)
[![Downloads](https://img.shields.io/crates/d/bevy_activation)](https://crates.io/crates/bevy_activation)
[![Documentation](https://docs.rs/bevy_activation/badge.svg)](https://docs.rs/bevy_activation)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)

A simple HTTP client Bevy Plugin for both native and WASM.

## Example

```rust
use std::time::Duration;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_activation::{ActivationPlugin, ActiveState, TimeoutEvent};

fn main() {
    App::new().add_plugins(MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
        Duration::from_secs_f64(1.0 / 60.0),
    )))
        .add_plugins(LogPlugin::default())
        .add_plugins(ActivationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (check_active, timeout_event, reactive_idle.run_if(time_passed(5.0))))
        .run();
}


#[derive(Component)]
struct TestAlive(pub &'static str);

fn setup(mut commands: Commands) {
    // this entity always active
    commands.spawn((TestAlive("always alive component"), ActiveState::default()));
    // this entity will be active for 2 seconds
    commands.spawn((TestAlive("ttl 2 secs component"), ActiveState::new(Duration::from_secs(2))));
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
```

## Supported Versions

| bevy | bevy_activation |
|------|-----------------|
| 0.15 | 0.3             |
| 0.14 | 0.2             |
| 0.13 | 0.1             |

## License

Dual-licensed under either:

- [`MIT`](LICENSE-MIT): [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
- [`Apache 2.0`](LICENSE-APACHE): [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means that when using this crate in your game, you may choose which license to use.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.