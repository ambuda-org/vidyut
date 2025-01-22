/// Implements various boilerplate for enums representing Sanskrit linguistic data.
macro_rules! sanskrit_enum {
    ($Enum:ident, { $( $variant:ident => $str:literal ),* $(,)? }) => {
        impl $Enum {
            /// Returns a string label of this enum variant.
            ///
            /// The string representation makes the following guarantees:
            ///
            /// - The label is a valid Sanskrit string in SLP1 encoding.
            /// - Each variant's string label is unique.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $Enum::$variant => $str,
                    )*
                }
            }

            /// Iterates over all values of this enum in order.
            #[allow(dead_code)]
            pub fn iter() -> impl Iterator<Item = $Enum> {
                // In Rust, `const` items are created at compile time.
                const ITEMS: &[$Enum] = &[
                    $(
                        $Enum::$variant,
                    )*
                ];
                ITEMS.iter().copied()
            }
        }

        impl std::str::FromStr for $Enum {
            type Err = Error;
            fn from_str(value: &str) -> $crate::core::errors::Result<Self> {
                let ret = match value {
                    $(
                        $str => $Enum::$variant,
                    )*
                    _ => return Err($crate::core::errors::Error::enum_parse_error(value))
                };
                Ok(ret)
            }
        }

        impl core::fmt::Display for $Enum {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    }
}

pub(crate) use sanskrit_enum;
