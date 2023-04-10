// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Changed, Color, Commands, DespawnRecursiveExt, Entity, EventWriter, Query, Res, ResMut,
        Transform, With, Without,
    },
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::plugins::{
    audio::{events::AudioStartPlayEvent, AudioClip},
    dialog_box::{
        components::{DialogBoxContent, DialogBoxTitle},
        events::DialogBoxStartDialogEvent,
    },
    game::resources::GameProgress,
    mushroom::components::{Mushroom, MushroomEffect, MushroomEffectActiveTimer},
    player::{
        components::{
            Player, PlayerConsumedMushroom, PlayerDebuffNoDamageAgainstBlobs, PlayerDebuffNoMagic,
            PlayerDebuffSlowWalk, PlayerHealth, PlayerHealthMax,
        },
        states::PlayerState,
    },
};

pub fn player_eat_mushroom(
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            &Transform,
            &mut PlayerHealth,
            &PlayerHealthMax,
            &mut PlayerState,
            &mut TextureAtlasSprite,
        ),
        (
            With<Player>,
            Changed<Transform>,
            Without<PlayerConsumedMushroom>,
        ),
    >,
    mushroom_query: Query<
        (
            Entity,
            &Transform,
            &MushroomEffect,
            &MushroomEffectActiveTimer,
        ),
        With<Mushroom>,
    >,
    mut dialog_box_start_dialog_event_writer: EventWriter<DialogBoxStartDialogEvent>,
    mut audio_start_play_event_writer: EventWriter<AudioStartPlayEvent>,
    mut game_progress: ResMut<GameProgress>,
) {
    if player_query.is_empty() || mushroom_query.is_empty() {
        return;
    }

    let (
        player_entity,
        player_transform,
        mut player_health,
        player_health_max,
        mut player_state,
        mut player_sprite,
    ) = player_query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    for (mushroom_entity, mushroom_transform, mushroom_effect, mushroom_effect_active_timer) in
        mushroom_query.iter()
    {
        if mushroom_transform
            .translation
            .truncate()
            .distance(player_transform.translation.truncate())
            < 8.0
        {
            commands
                .entity(player_entity)
                .insert(PlayerConsumedMushroom {
                    effect: mushroom_effect.clone(),
                    effect_active_timer: mushroom_effect_active_timer.clone(),
                });

            player_sprite.color = Color::ORANGE_RED.with_a(player_sprite.color.a());

            commands.entity(mushroom_entity).despawn_recursive();

            let mut dialog_box_message: String;
            match mushroom_effect {
                MushroomEffect::LowHealth => {
                    player_health.0 /= 2.0;
                    dialog_box_message =
                        "The mushroom made you sick. You lose half your lifepoints.".into();
                }
                MushroomEffect::HealthBonus => {
                    player_health.0 = player_health_max.0;
                    dialog_box_message = "Yummy! You get your lifepoints back.".into();
                }
                MushroomEffect::SlowWalk => {
                    commands.entity(player_entity).insert(PlayerDebuffSlowWalk);
                    dialog_box_message = "The mushroom stole your energy. You can't run.".into();
                }
                MushroomEffect::NoMagic => {
                    commands.entity(player_entity).insert(PlayerDebuffNoMagic);
                    dialog_box_message =
                        "The mushroom stole your magical powers. You can't cast spells.".into();
                }
                MushroomEffect::NoDamageAgainstBlobs => {
                    commands
                        .entity(player_entity)
                        .insert(PlayerDebuffNoDamageAgainstBlobs);
                    dialog_box_message =
                        "The mushroom stole your strength. You can't deal damage with melee."
                            .into();
                }
            };

            if game_progress.mushroom_tutorial_finished == false {
                dialog_box_message = 
                    "It looks like you ate your first mushroom. You can only attack the nests of the\ncreatures while the effect lasts.\n".to_owned()
                    + dialog_box_message.as_str();
                game_progress.mushroom_tutorial_finished = true;
            }

            *player_state = PlayerState::Talk;
            audio_start_play_event_writer.send(AudioStartPlayEvent(AudioClip::EatMushroom));
            dialog_box_start_dialog_event_writer.send(DialogBoxStartDialogEvent {
                title: DialogBoxTitle("Mushroom consumed!".into()),
                content: DialogBoxContent(
                    dialog_box_message + "\nPress [Space] To Continue.",
                ),
            });
        }
    }
}

pub fn player_update_mushroom_effect(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut PlayerConsumedMushroom, &mut TextureAtlasSprite)>,
    delta: Res<Time>,
) {
    if player_query.is_empty() {
        return;
    }

    let (player_entity, mut player_consumed_mushroom, mut player_sprite) = player_query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    player_consumed_mushroom
        .effect_active_timer
        .0
        .tick(delta.delta());

    if player_consumed_mushroom
        .effect_active_timer
        .0
        .just_finished()
    {
        player_sprite.color = Color::WHITE.with_a(player_sprite.color.a());

        commands.entity(player_entity).remove::<(
            PlayerConsumedMushroom,
            PlayerDebuffSlowWalk,
            PlayerDebuffNoMagic,
            PlayerDebuffNoDamageAgainstBlobs,
        )>();
    }
}
