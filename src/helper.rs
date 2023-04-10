// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Convert to rounded percentage string.
pub fn get_percentage_rounded(x: f32, y: f32) -> f32 {
    let result = (x * 100.0) / y;
    result.round()
}
