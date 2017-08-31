// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
/// Whether a tool can be compiled, tested or neither
pub enum ToolState {
    /// The tool compiles successfully, but the test suite fails
    Compiling,
    /// The tool compiles successfully and its test suite passes
    Testing,
    /// The tool can't even be compiled
    Broken,
}

impl ToolState {
    pub fn is_compiling(self) -> bool {
        match self {
            ToolState::Compiling |
            ToolState::Testing => true,
            ToolState::Broken => false,
        }
    }

    pub fn is_test_suite_passing(self) -> bool {
        match self {
            ToolState::Testing => true,
            ToolState::Compiling |
            ToolState::Broken => false,
        }
    }
}

impl Default for ToolState {
    fn default() -> Self {
        // err on the safe side
        ToolState::Broken
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Default)]
/// Used to check `toolstate.toml` for tools which should not
/// be compiled or tested
pub struct ToolStates {
    pub miri: ToolState,
}
