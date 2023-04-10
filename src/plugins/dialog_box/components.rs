// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct DialogBox;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct DialogBoxIsActive;

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct DialogBoxTitle(pub String);

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct DialogBoxContent(pub String);
