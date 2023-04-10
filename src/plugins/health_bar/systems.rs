// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Added, BuildChildren, Changed, Commands, Entity, EventReader, Query, Vec2, Without},
    sprite::Sprite,
};

use crate::plugins::health_bar::bundles::HealthBarBundle;

use super::{
    components::{HealthBar, HealthBarPercentFilled, HealthBarReady, HealthBarTargetEntity},
    events::HealthBarFillPercentEvent,
};

pub(super) fn health_bar_spawn(
    mut commands: Commands,
    query: Query<Entity, (Added<HealthBar>, Without<HealthBarReady>)>,
) {
    for health_bar_entity in query.iter() {
        commands.entity(health_bar_entity).remove::<HealthBar>();

        let health_bar_bundle = commands
            .spawn(HealthBarBundle::new(
                health_bar_entity,
                Vec2::new(0.0, 15.0),
                Vec2::new(20.0, 3.0),
                20.0,
            ))
            .insert(HealthBarReady)
            .id();

        commands
            .entity(health_bar_entity)
            .add_child(health_bar_bundle);
    }
}

pub(super) fn health_bar_handle_percent_filled_event(
    mut query: Query<(&HealthBarTargetEntity, &mut HealthBarPercentFilled)>,
    mut percent_filled_event_reader: EventReader<HealthBarFillPercentEvent>,
) {
    for percent_filled_event in percent_filled_event_reader.iter() {
        for (health_bar_target_entity, mut health_bar_percent_filled) in query.iter_mut() {
            if health_bar_target_entity.0 == percent_filled_event.target {
                health_bar_percent_filled.0 = percent_filled_event.percent_filled;
            }
        }
    }
}

pub(super) fn health_bar_update_percent_filled(
    mut query: Query<(&HealthBarPercentFilled, &mut Sprite), Changed<HealthBarPercentFilled>>,
) {
    for (health_bar_percent_filled, mut health_bar_sprite) in query.iter_mut() {
        health_bar_sprite.custom_size = Some(Vec2::new(0.16 * health_bar_percent_filled.0, 3.0));
    }
}
