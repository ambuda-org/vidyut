/*!
This library defines our Rust-Python bridge.

Almost all of the heavy lifting is done by PyO3, which generates a native Python module with the
help of a few macros. Then, Maturin combines the code here with our Python code to create our final
package.


Structure
---------

The `vidyut` function below defines our native module. It uses symbols from the `cheda`, `kosha`,
and `prakriya` Rust modules, which define bindings for their corresponding Rust crates.

To handle name collisions, we create submodules with PyO3's `wrap_pymodule!` macro. One restriction
of these native submodules is that we cannot `import` from them:

```python
# This will fail
from foo.bar import *
```

However, we can still access symbols by attribute:

```python
# This will fail
from foo import bar
X = bar.X
```

We use this pattern extensively. For an example, see `vidyut/kosha.py`.


Conventions
-----------

`__repr__`
~~~~~~~~~~

Prefer returning a string that could be `eval`-ed:

> For many types, this function makes an attempt to return **a string that would yield an object with
the same value when passed to eval()**; otherwise, the representation is a string enclosed in angle
brackets that contains the name of the type of the object together with additional information
often including the name and address of the object.

-- Python docs (emphasis added)

Comments
~~~~~~~~

Rustdoc comments are used as-is to document their Python counterparts [1]. If a comment is on an
item wrapped in a PyO3 macro, prefer an imperative style ("Return" vs. "Returns"). Otherwise, use
the normal Rust style ("Returns" vs. "Return").

[1]: https://pyo3.rs/v0.18.0/module.html#documentation

*/

#![deny(clippy::panic)]
#![warn(clippy::unwrap_used)]

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod chandas;
mod cheda;
mod kosha;
mod lipi;
mod prakriya;
mod sandhi;
mod utils;

/// Defines the `vidyut.chandas` native module.
///
/// For usage examples, see `vidyut/chandas.py`.
#[pymodule]
#[pyo3(name = "chandas")]
fn py_chandas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<chandas::PyAkshara>()?;
    m.add_class::<chandas::PyChandas>()?;
    m.add_class::<chandas::PyJati>()?;
    m.add_class::<chandas::PyMatch>()?;
    m.add_class::<chandas::PyVrtta>()?;
    m.add_class::<chandas::PyVrttaPada>()?;
    Ok(())
}

/// Defines the `vidyut.cheda` native module.
///
/// For usage examples, see `vidyut/cheda.py`.
#[pymodule]
#[pyo3(name = "cheda")]
fn py_cheda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<cheda::PyChedaka>()?;
    m.add_class::<cheda::PyToken>()?;
    m.add_class::<cheda::PyModel>()?;
    m.add_class::<cheda::PyModelBuilder>()?;

    Ok(())
}

/// Defines the `vidyut.kosha` native module.
///
/// For usage examples, see `vidyut/kosha.py`.
#[pymodule]
#[pyo3(name = "kosha")]
fn py_kosha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<kosha::PyKosha>()?;
    m.add_class::<kosha::PyBuilder>()?;

    m.add_class::<kosha::entries::PyDhatuEntry>()?;
    m.add_class::<kosha::entries::PyPratipadikaEntry>()?;
    m.add_class::<kosha::entries::PyPadaEntry>()?;

    Ok(())
}

/// Defines the `vidyut.lipi` native module.
///
/// For usage examples, see `vidyut/lipi.py`.
#[pymodule]
#[pyo3(name = "lipi")]
fn py_lipi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<lipi::PyScheme>()?;
    m.add_function(wrap_pyfunction!(lipi::detect, m)?)?;
    m.add_function(wrap_pyfunction!(lipi::transliterate, m)?)?;

    Ok(())
}

/// Defines the `vidyut.prakriya` native module.
///
/// For usage examples, see `vidyut/prakriya.py`.
#[pymodule]
#[pyo3(name = "prakriya")]
fn py_prakriya(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // API
    m.add_class::<prakriya::PyVyakarana>()?;

    // Data types
    m.add_class::<prakriya::PyData>()?;
    m.add_class::<prakriya::PyDhatupathaEntry>()?;
    m.add_class::<prakriya::PySutra>()?;
    m.add_class::<prakriya::PySource>()?;

    // Prakriya types
    m.add_class::<prakriya::PyPrakriya>()?;
    m.add_class::<prakriya::PyStep>()?;

    // Argument types.
    // For details on these symbols, see the comments in `py_kosha`.
    m.add_class::<prakriya::args::PyDhatu>()?;
    m.add_class::<prakriya::args::PyPratipadika>()?;
    m.add_class::<prakriya::args::PyAntargana>()?;
    m.add_class::<prakriya::args::PyGana>()?;
    m.add_class::<prakriya::args::PyKrt>()?;
    m.add_class::<prakriya::args::PyLakara>()?;
    m.add_class::<prakriya::args::PyLinga>()?;
    m.add_class::<prakriya::args::PyPrayoga>()?;
    m.add_class::<prakriya::args::PyPurusha>()?;
    m.add_class::<prakriya::args::PySanadi>()?;
    m.add_class::<prakriya::args::PyTaddhita>()?;
    m.add_class::<prakriya::args::PyVacana>()?;
    m.add_class::<prakriya::args::PyVibhakti>()?;
    m.add_class::<prakriya::args::PyPada>()?;

    Ok(())
}

/// Defines the `vidyut.sandhi` native module.
///
/// For usage examples, see `vidyut/sandhi.py`.
#[pymodule]
#[pyo3(name = "sandhi")]
fn py_sandhi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<sandhi::PySplitter>()?;
    m.add_class::<sandhi::PySplit>()?;

    Ok(())
}

/// Defines the `vidyut` native module.
///
/// In our Python code, this module is made available as `vidyut.vidyut`. For usage examples, see
/// any of the files in the `vidyut` Python package, e.g. `vidyut/kosha.py`.
#[pymodule]
fn vidyut(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_wrapped(wrap_pymodule!(py_chandas))?;
    m.add_wrapped(wrap_pymodule!(py_cheda))?;
    m.add_wrapped(wrap_pymodule!(py_kosha))?;
    m.add_wrapped(wrap_pymodule!(py_lipi))?;
    m.add_wrapped(wrap_pymodule!(py_prakriya))?;
    m.add_wrapped(wrap_pymodule!(py_sandhi))?;

    Ok(())
}
