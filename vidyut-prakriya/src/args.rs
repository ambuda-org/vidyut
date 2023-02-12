/*!
Common arguments for the crate's main functions.

Before we begin a prakriya, we must declare certain morphological information up-front, such as our
desired purusha and vacana, the dhatu we wish to use, and so on. To better document the API and to
help users avoid configuration mistakes, we model this information through the enums and structs in
this module.

For extra flexibility, all of the enums here provides `as_str` and `from_str` methods. For details
on which strings are valid arguments in `from_str`, please read the source code directly.
*/
mod macros;

mod dhatu;
mod krt;
mod sup;
mod tin;

pub use dhatu::*;
pub use krt::*;
pub use sup::*;
pub use tin::*;
