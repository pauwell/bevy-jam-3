// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Component, Vec2},
    reflect::Reflect,
};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Npc;

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct NpcName(pub String);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct NpcSize(pub Vec2);

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct NpcDialog(pub String);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum NpcProfession {
    #[default]
    None,
    Healer,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum NpcState {
    #[default]
    Idle,
    Talk,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum NpcDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}
