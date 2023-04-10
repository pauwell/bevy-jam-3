// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{prelude::Component, time::Timer};

#[derive(Component, Default, Debug, Clone)]
pub struct Mushroom;

#[derive(Component, Default, Debug, Clone)]
pub struct MushroomCollected;

#[derive(Component, Default, Debug, Clone)]
pub struct MushroomConsumed;

#[derive(Component, Default, Debug, Clone)]
pub struct MushroomDespawnTimer(pub Timer);

#[derive(Component, Default, Debug, Clone)]
pub struct MushroomEffectActiveTimer(pub Timer);

#[derive(Component, Debug, Clone)]
pub enum MushroomEffect {
    LowHealth,
    SlowWalk,
    NoMagic,
    NoDamageAgainstBlobs,
    HealthBonus,
}
