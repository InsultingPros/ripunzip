// Copyright 2022 Google LLC

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RipunzipErrors {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ZipErrorr(#[from] zip::result::ZipError),
    #[error(transparent)]
    ReqwestErrorr(#[from] reqwest::Error),
}

mod unzip;

pub use unzip::FilenameFilter;
pub use unzip::NullProgressReporter;
pub use unzip::UnzipEngine;
pub use unzip::UnzipOptions;
pub use unzip::UnzipProgressReporter;
