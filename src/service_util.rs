// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::result::Result;

pub use util::core::GError;
pub use util::core::GResult;


pub trait MessageReader {
    fn read_next(&mut self) -> Result<String, GError>;
}

pub trait MessageWriter {
    fn write_message(&mut self, msg: &str) -> Result<(), GError>;
}
