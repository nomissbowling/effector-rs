//! core
//!

use std::{result::Result, error::Error};

use pyo3::prelude::*;
use pyo3::types::{PyNone, PyDict};

/// wx_test
pub fn wx_test(t: &str) -> Result<(), Box<dyn Error>> {
  Python::attach(|py| {
    let wx = py.import("wx")?;
    let kw = PyDict::new(py);
    let app = wx.getattr("App")?.call((), Some(&kw))?;
    let kw = PyDict::new(py);
    kw.set_item("title", t)?;
    kw.set_item("size", (800, 600))?;
    let frm = wx.getattr("Frame")?.call((PyNone::get(py), -1), Some(&kw))?;
    frm.call_method0("Show")?;
    app.call_method0("MainLoop")?;
    Ok(())
  })
}

/// lib_test
pub fn lib_test() -> Result<(), Box<dyn Error>> {
  assert_eq!(wx_test("test lib")?, ());
  Ok(())
}
