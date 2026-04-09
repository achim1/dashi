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

/// Check if the version is at least the 
/// required, given string following the 
/// format MAJOR.MINOR.PATCH 
#[pyfunction]
fn version_at_least(version_string : String) -> bool {
  let parts: Vec<u32> = version_string.split('.')
    .map(|s| s.parse().expect("Input is not a valid version string. Please follow the MAJOR.MINOR.PATCH format!"))
    .collect();
  let major : u8 = parts[0] as u8;
  let minor : u8 = parts[1] as u8;
  let patch : u8 = parts[2] as u8;
  if major < get_version_major() {
    return true;
  }
  if major > get_version_major() {
    println!("Required major version of {} is larger than current version {}", major, VERSION);
    return false;
  }
  if major == get_version_major() {
    if minor < get_version_minor() {
      return true;
    }
    if minor > get_version_minor() {
      println!("Required minor version of {} is larger than current version {}", minor, VERSION);
      return false;
    }
    if minor == get_version_minor() {
      if patch > get_version_patch() {
        println!("Required patch version of {} is larger than current version {}", patch, VERSION);
        return false 
      }
    }
  } 
  true
}

#[pymodule]
#[pyo3(name = "kanidashi_core")]
fn kanidashi_core_py<'_py>(m: &Bound<'_py, PyModule>) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(get_version, m)?)?;
  m.add_function(wrap_pyfunction!(get_version_major, m)?)?;
  m.add_function(wrap_pyfunction!(get_version_minor, m)?)?;
  m.add_function(wrap_pyfunction!(get_version_patch, m)?)?;
  m.add_function(wrap_pyfunction!(version_at_least, m)?)?;
  Ok(())
}
