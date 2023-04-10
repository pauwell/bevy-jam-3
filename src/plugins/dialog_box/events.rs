// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::components::{DialogBoxContent, DialogBoxTitle};

pub struct DialogBoxStartDialogEvent {
    pub title: DialogBoxTitle,
    pub content: DialogBoxContent,
}

pub struct DialogBoxCloseEvent;
