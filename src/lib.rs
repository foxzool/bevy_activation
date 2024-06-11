#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use bevy_app::{App, Plugin, PostUpdate};
use bevy_derive::Deref;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_time::{Stopwatch, Time};
use std::time::Duration;

/// Bevy Entity Activation Plugin;
pub struct ActivationPlugin;

impl Plugin for ActivationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ActiveState>()
            .add_event::<TimeoutEvent>()
            .add_systems(PostUpdate, check_timeout);
    }
}

/// Timeout Event
#[derive(Deref, Debug, Event)]
pub struct TimeoutEvent(pub Entity);

/// Activation State Component
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ActiveState {
    /// is active
    active: bool,
    /// stop watch
    watch: Option<Stopwatch>,
    /// Time to Interactive (TTI)
    time_to_idle: Duration,
}

impl Default for ActiveState {
    fn default() -> Self {
        Self::always()
    }
}

impl ActiveState {
    /// no timeout
    pub fn always() -> ActiveState {
        ActiveState {
            active: true,
            watch: None,
            time_to_idle: Duration::MAX,
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.active
    }

    #[inline]
    pub fn is_idle(&self) -> bool {
        !self.is_active()
    }

    /// set active
    #[inline]
    pub fn active_mut(&mut self) -> &mut bool {
        &mut self.active
    }

    /// setup timeout time
    #[inline]
    pub fn time_to_idle_mut(&mut self) -> &mut Duration {
        &mut self.time_to_idle
    }

    /// new active state with timeout
    pub fn new(timeout: Duration) -> ActiveState {
        ActiveState {
            active: true,
            watch: Some(Stopwatch::new()),
            time_to_idle: timeout,
        }
    }

    /// set timeout time
    pub fn set_tti_time(&mut self, timeout: Duration) -> &mut ActiveState {
        self.time_to_idle = timeout;
        if self.watch.is_none() {
            self.watch = Some(Stopwatch::new());
        }
        self
    }

    /// toggle active state
    pub fn toggle(&mut self) {
        self.active = true;
        if let Some(watch) = &mut self.watch {
            watch.reset();
        }
    }
}

/// set timeout to not active and send timeout event
fn check_timeout(
    time: Res<Time>,
    mut active_state_query: Query<(Entity, &mut ActiveState)>,
    mut timeout_events: EventWriter<TimeoutEvent>,
) {
    for (entity, mut active_state) in active_state_query.iter_mut() {
        if active_state.is_idle() {
            continue;
        }
        let timeout = active_state.time_to_idle;
        if let Some(watch) = &mut active_state.watch {
            watch.tick(time.delta());

            if watch.elapsed() >= timeout {
                timeout_events.send(TimeoutEvent(entity));
                watch.reset();
                active_state.active = false;
            }
        }
    }
}
