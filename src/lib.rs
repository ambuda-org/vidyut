#![deny(missing_docs)]

/*!
Vidyut provides best-in-class infrastructure for Sanskrit software.

These docs are meant for those who want to use Vidyut through Rust. So if you are interested in:

- improving Vidyut's core code
- writing native applications
- creating bindings for Vidyut in your preferred language, or
- using Rust directly

then these docs are for you. Otherwise, we strongly recommend using Vidyut through our Python
bindings, which are available in the `vidyut` package on PyPI.


Structure
=========

Vidyut is an umbrella project that contains various fine-grained crates. Each crate focuses on a
specific Sanskrit problem domain. Although these crates are largely independent, they are designed
to work well together.

Our three primary crates are:

- `vidyut-cheda`, which segments and annotates Sanskrit expressions
- `vidyut-kosha`, which compactly stores millions of Sanskrit words
- `vidyut-prakriya`, which generates Sanksrit words by applying Paninian rules

Any other crates here are more minor.


Design principles
=================

All of our crates follow a core set of design principles:

*Speed*. We care deeply about running as quickly as possible. Our heuristic is that all else equal,
a >10% reduction in runtime is always worth pursuing. Examples:

- We generally use SLP1 transliteration for all of our Sanskrit text, since SLP1 represents each
  Sanskrit sound in exactly one byte. Thus strings are smaller, indexing is O(1), and sound tests
  are simpler and faster.

- We generally use `CompactString` to store all Sanskrit text. On 64-bit machines, `CompactString`
  can store 24 bytes in size directly on the stack, which reduces extraneous heap allocations.
  `CompactString` is especially powerful when combined with SLP1.

- We make aggressive use of Rust's enums and structs as opposed to "stringly-typed" data encoded in
  plain text or simple hash maps. This approach is not just more idiomatic in Rust; it is also more
  efficient and much faster.

*Power*. Vidyut is for power users and exposes details that other libraries might hide. That said,
we provide various convenience functions to reduce boilerplate. Examples:

- In `vidyut-prakriya`, our input arguments are elaborate and require extensive use of the builder
  pattern. This is by design: by asking for all requirements up-front, we ensure that arguments are
  always well-structured and greatly reduce undefined behavior.

- On our more expensive structs, we expose a variety of feature flags through the builder pattern
  so that users can load exactly the features they need.

*Trust*. We see Vidyut as core infrastructure for the next era of Sanskrit software. Accordingly,
we strive to make our code as clear, approachable, and trustworthy as we can. Examples:

- As much as possible, we use simple programming idioms that those without a Rust background can
  understand. When more complex Rust code is necessary, we leave extensive comments on what our
  code is doing and why.

- In general, we have documented our code extensively and provided various examples of how to use
  it. This is a work in progress, and we have much more to do here.

- We break Rust conventions when doing so leads to greater clarity. For example, we encode the
  krt-pratyayas in `vidyut-prakriya` in SLP1, even though doing so breaks Rust's `CamelCase`
  convention for enum names.
*/
