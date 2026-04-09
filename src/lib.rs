// This file is part of gaps-online-software and published 
// under the GPLv3 license

pub mod prelude;
use crate::prelude::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 
pub const VERSION_MAJ: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const VERSION_MIN: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const VERSION_PCH: &str = env!("CARGO_PKG_VERSION_PATCH");

#[pyfunction]
fn get_version() -> &'static str {
  return VERSION;
}

/// Major version of the package 
/// from the version string
/// (Major.Minor.Patch)
#[pyfunction]
fn get_version_major() -> u8 {
  return VERSION_MAJ.parse().unwrap();
}

/// Minor version of the package 
/// from the version string
/// (Major.Minor.Patch)
#[pyfunction]
fn get_version_minor() -> u8 {
  return VERSION_MIN.parse().unwrap();
}

/// Patch level of the package 
/// from the version string
/// (Major.Minor.Patch)
#[pyfunction]
fn get_version_patch() -> u8 {
  return VERSION_PCH.parse().unwrap();
}

#[pymodule]
#[pyo3(name = "kanidashi")]
fn kanidashi_py<'_py>(m: &Bound<'_py, PyModule>) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(get_version, m)?)?;
  m.add_function(wrap_pyfunction!(get_version_major, m)?)?;
  m.add_function(wrap_pyfunction!(get_version_minor, m)?)?;
  m.add_function(wrap_pyfunction!(get_version_patch, m)?)?;
  Ok(())
}
