/// Generates the following boilerplate methods:
///
/// - `__new__`
/// - `__format__`
/// - `__hash__`
/// - `__repr__`
/// - `__str__`
/// - `From<RustEnum> for PyEnum`
/// - `From<PyEnum> for RustEnum`
///
/// And the following boilerplate attributes:
/// - `name`
/// - `value`
/// - `_name_`
/// - `_value_`
///
/// `__iter__` is not possible since it requires accessing the metaclass.
/// See https://github.com/PyO3/pyo3/issues/906 and related issues.
///
/// Requirements:
/// - Enum must derive `Hash`
///
/// For enum compat, see: https://github.com/PyO3/pyo3/issues/2887
macro_rules! py_enum {
    ($Py:ident, $Rust:ident, [$( $variant:ident ),*]) => {
        impl From<$Rust> for $Py {
            fn from(val: $Rust) -> Self {
                match val {
                    $(
                        $Rust::$variant => $Py::$variant,
                    )*
                }
            }
        }

        impl From<$Py> for $Rust {
            fn from(val: $Py) -> Self {
                match val {
                    $(
                        $Py::$variant => $Rust::$variant,
                    )*
                }
            }
        }

        impl std::fmt::Display for $Py {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $Rust::from(*self).as_str())
            }
        }

        #[pymethods]
        impl $Py {
            #[new]
            fn __new__(val: String) -> PyResult<$Py> {
                // O(n), but this is a rare method not on a hot path, so it's fine for now.
                for choice in Self::choices() {
                    if val == choice.__str__() {
                        return Ok(choice)
                    }
                }
                Err(pyo3::exceptions::PyValueError::new_err(
                    format!("{:?} is not a valid {}", val, stringify!($Rust))))
            }

            /*
            fn __format__(&self, spec: String) -> String {
                format!("{{{}:{}}}",
                    $Rust::from(*self).as_str(),
                    spec
                )
            }
            */

            fn __getitem__(&self, val: String) -> PyResult<$Py> {
                // O(n), but this is a rare method not on a hot path, so it's fine for now.
                for choice in Self::choices() {
                    if val == choice.name() {
                        return Ok(choice)
                    }
                }
                Err(pyo3::exceptions::PyKeyError::new_err(String::new()))
            }

            fn __hash__(&self) -> u64 {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }

            pub(crate) fn __repr__(&self) -> String {
                format!("{}.{}", stringify!{$Rust}, self.name())
            }

            /// The string representation of this enum.
            ///
            /// `__str__` returns a valid SLP1 string. This string might include
            /// accent marks (``\\``, ``^``) or nasality markers (``~``).
            fn __str__(&self) -> String {
                self.value()
            }

            // Enum built-ins
            // -------------------

            /// The name used to define the `Enum` member.
            #[getter]
            fn name(&self) -> String {
                let ret = match self {
                    $(
                        $Py::$variant => stringify!($variant),
                    )*
                };
                ret.to_string()
            }

            /// The name used to define the `Enum` member. This is identical
            /// to `name`.
            ///
            /// (For compatibility with Python `Enum`).
            #[getter]
            fn _name_(&self) -> String {
                self.name()
            }

            /// The name used to define the `Enum` member.
            ///
            /// The returned value is guaranteed to be identical to the output of
            /// `__str__`.
            #[getter]
            fn value(&self) -> String {
                $Rust::from(*self).as_str().to_string()
            }

            /// The value associated with this variant. This is identical
            /// to `value`.
            ///
            /// (For compatibility with Python `Enum`).
            #[getter]
            fn _value_(&self) -> String {
                self.value()
            }

            /// Return all possible values for this enum.
            #[staticmethod]
            fn choices() -> Vec<$Py> {
                const CHOICES: &[$Py] = &[
                    $(
                        $Py::$variant,
                    )*
                ];
                CHOICES.to_vec()
            }

            /// DEPRECATED -- prefer `__new__` instead.
            ///
            /// Create an enum value from the given string.
            ///
            /// This is the inverse of `__str__`. Performance is currently linear
            /// in the number of enum options.
            #[staticmethod]
            fn from_string(val: &str) -> PyResult<Self> {
                // O(n), but this is a rare method not on a hot path, so it's fine for now.
                for choice in Self::choices() {
                    if val == choice.__str__() {
                        return Ok(choice)
                    }
                }
                Err(pyo3::exceptions::PyValueError::new_err(format!("Could not parse {val}")))
            }

        }
    }
}

/// Helper macro for pratyayas (Krt, Taddhita, Sanadi)
///
/// Generates the following boilerplate methods:
///
/// - `From<RustEnum> for PyEnum`
/// - `From<PyEnum> for RustEnum`
/// - `__new__`
/// - `__format__`
/// - `__hash__`
/// - `__repr__`
/// - `__str__`
/// - `choices`
///
/// And the following boilerplate attributes:
/// - `drshya`
/// - `anubandhas`
/// - `name`
/// - `value`
/// - `_name_`
/// - `_value_`
///
/// `__iter__` is not possible since it requires accessing the metaclass.
/// See https://github.com/PyO3/pyo3/issues/906 and related issues.
///
/// Requirements:
/// - Enum must derive `Hash`
///
/// For enum compat, see: https://github.com/PyO3/pyo3/issues/2887
macro_rules! py_pratyaya {
    ($Py:ident, $Rust:ident, [$( $variant:ident ),*]) => {
        impl From<$Rust> for $Py {
            fn from(val: $Rust) -> Self {
                match val {
                    $(
                        $Rust::$variant => $Py::$variant,
                    )*
                }
            }
        }

        impl From<$Py> for $Rust {
            fn from(val: $Py) -> Self {
                match val {
                    $(
                        $Py::$variant => $Rust::$variant,
                    )*
                }
            }
        }

        impl std::fmt::Display for $Py {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $Rust::from(*self).as_str())
            }
        }

        #[pymethods]
        impl $Py {
            #[new]
            fn __new__(val: String) -> PyResult<$Py> {
                // O(n), but this is a rare method not on a hot path, so it's fine for now.
                for choice in Self::choices() {
                    if val == choice.__str__() {
                        return Ok(choice)
                    }
                }
                Err(pyo3::exceptions::PyValueError::new_err(
                    format!("{:?} is not a valid {}", val, stringify!($Rust))))
            }

            /*
            fn __format__(&self, spec: String) -> String {
                format!("{{{}:{}}}",
                    $Rust::from(*self).as_str(),
                    spec
                )
            }
            */

            fn __getitem__(&self, val: String) -> PyResult<$Py> {
                // O(n), but this is a rare method not on a hot path, so it's fine for now.
                for choice in Self::choices() {
                    if val == choice.name() {
                        return Ok(choice)
                    }
                }
                Err(pyo3::exceptions::PyKeyError::new_err(String::new()))
            }

            fn __hash__(&self) -> u64 {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }

            pub(crate) fn __repr__(&self) -> String {
                format!("{}.{}", stringify!{$Rust}, self.name())
            }

            /// The string representation of this enum.
            ///
            /// `__str__` returns a valid SLP1 string. This string might include
            /// accent marks (``\\``, ``^``) or nasality markers (``~``).
            fn __str__(&self) -> String {
                self.value()
            }

            /// The name used to define the `Enum` member.
            #[getter]
            fn name(&self) -> String {
                let ret = match self {
                    $(
                        $Py::$variant => stringify!($variant),
                    )*
                };
                ret.to_string()
            }

            /// The name used to define the `Enum` member. This is identical
            /// to `name`.
            ///
            /// (For compatibility with Python `Enum`).
            #[getter]
            fn _name_(&self) -> String {
                self.name()
            }

            /// The name used to define the `Enum` member.
            ///
            /// The returned value is guaranteed to be identical to the output of
            /// `__str__`.
            #[getter]
            fn value(&self) -> String {
                $Rust::from(*self).as_str().to_string()
            }

            /// The value associated with this variant. This is identical
            /// to `value`.
            ///
            /// (For compatibility with Python `Enum`).
            #[getter]
            fn _value_(&self) -> String {
                self.value()
            }

            /// Return all possible values for this enum.
            #[staticmethod]
            fn choices() -> Vec<$Py> {
                const CHOICES: &[$Py] = &[
                    $(
                        $Py::$variant,
                    )*
                ];
                CHOICES.to_vec()
            }

            /// DEPRECATED -- prefer `__new__` instead.
            ///
            /// Create an enum value from the given string.
            ///
            /// This is the inverse of `__str__`. Performance is currently linear
            /// in the number of enum options.
            #[staticmethod]
            fn from_string(val: &str) -> PyResult<Self> {
                // O(n), but this is a rare method not on a hot path, so it's fine for now.
                for choice in Self::choices() {
                    if val == choice.__str__() {
                        return Ok(choice)
                    }
                }
                Err(pyo3::exceptions::PyValueError::new_err(format!("Could not parse {val}")))
            }

            // Pratyaya methods
            // ----------------

            fn drshya(&self) -> String {
                let rust = $Rust::from(*self);
                rust.drshya().to_string()
            }

            fn anubandhas(&self) -> Vec<PyAnubandha> {
                let rust = $Rust::from(*self);
                rust.anubandhas().iter().map(|x| (*x).into()).collect()
            }
        }
    }
}

macro_rules! py_only_enum {
    ($Py:ident, $Name:ident, { $( $variant:ident => $str:literal ),* } ) => {
        impl std::fmt::Display for $Py {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        #[pymethods]
        impl $Py {
            #[new]
            fn new(val: &str) -> PyResult<Self> {
                match val {
                    $(
                        $str => Ok($Py::$variant),
                    )*
                    _ =>  Err(pyo3::exceptions::PyValueError::new_err(
                        format!("{:?} is not a valid {}", val, stringify!($Rust))))
                }
            }

            fn __format__(&self, spec: String) -> String {
                format!("{{{}:{}}}", self.name(), spec)
            }

            fn __str__(&self) -> String {
                match self {
                    $(
                        $Py::$variant => $str.to_string(),
                    )*
                }
            }

            fn __repr__(&self) -> String {
                format!("{}.{}", stringify!{$Name}, self.name())
            }

            /// The name used to define the `Enum` member.
            ///
            /// (Defined for compatibility with Python enums.)
            #[getter]
            fn name(&self) -> String {
                let ret = match self {
                    $(
                        $Py::$variant => stringify!($variant),
                    )*
                };
                ret.to_string()
            }

            /// The name used to define the `Enum` member. This is identical
            /// to `name`.
            ///
            /// (For compatibility with Python `Enum`).
            #[getter]
            fn _name_(&self) -> String {
                self.name()
            }

            /// The value used to define the `Enum` member.
            ///
            /// (Defined for compatibility with Python enums.)
            #[getter]
            fn value(&self) -> String {
                self.__str__()
            }

            /// The value associated with this variant. This is identical
            /// to `value`.
            ///
            /// (For compatibility with Python `Enum`).
            #[getter]
            fn _value_(&self) -> String {
                self.value()
            }

            /// Return all possible values for this enum.
            #[staticmethod]
            fn choices() -> Vec<$Py> {
                const CHOICES: &[$Py] = &[
                    $(
                        $Py::$variant,
                    )*
                ];
                CHOICES.to_vec()
            }

            /// DEPRECATED -- prefer `__new__` instead.
            ///
            /// Create an enum value from the given string.
            ///
            /// This is the inverse of `__str__`.
            #[staticmethod]
            fn from_string(val: &str) -> PyResult<Self> {
                match val {
                    $(
                        $str => Ok($Py::$variant),
                    )*
                    _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Could not parse {val}"))),
                }
            }
        }
    }
}

pub(crate) use py_enum;
pub(crate) use py_only_enum;
pub(crate) use py_pratyaya;
