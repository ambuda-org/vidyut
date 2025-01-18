Architecture
============

This document describes the high-level architecture of `vidyut-prakriya`. We
have written this doc for anyone who wants to better understand how we designed
our system.

This document assumes some familiarity with basic Rust concepts like structs,
enums, closures, and lifetimes.


Goals and values
----------------

Our goal is to create a reference implementation of the Ashtadhyayi that is
pragmatic and efficient while remaining faithful to the tradition and flexible
enough for new use cases.

`vidyut-prakriya` is a complex program because traditional grammar is
inherently complex. To avoid overwhelming ourselves and future maintainers, we
have pursed simplicity in our overall program design, which takes the following
forms:

- We use a procedural control flow that applies rules in an obvious order. A
  user should be able to know exactly what order we apply rules in just by
  reading our source code. This means that we don't implement any kind of
  dynamic rule selection.

- We use closures to reduce boilerplate and ensure that the code for each rule
  is concise, clear, and obvious in its results.

- We separate the Ashtadhyayi's rules into modules that broadly follow its
  overall architecture, and to the extent that we can, we define rules that are
  close together in the text to be close together in our code.

With all that said, we do use small hacks judiciously if we don't have a clear
enough vision of the overall system to decide how to handle a specific edge
case. In our view, it is better to make *some* kind of forward progress rather
than wait for perfect clarity. Fortunately, we have found that as the program
has grown and matured, we have become more and more able to remove hacks and
solve problems in a more fundamental way.


Core data types
---------------

The three most important types are `Term`, `Prakriya` and `Vyakarana`.

- `Term` is essentially a string that has various metadata that helps when
  creating derivations. `Term` provides a rich API for testing different
  properties and mutating internal state.

- `Prakriya` manages the derivation itself in a `Vec<Term>`, along with a few
  other minor fields. It also contains a list of any optional rules that were
  encountered during the derivation, which helps the system explore alternative
  paths that make different choices against these optional rules. `Prakriya`
  can optionally record a history of each rule in the derivation and the result
  it produced.

- `Vyakarana` is the main entrypoint to the system. It is a lightweight struct
  that lets the user configure how the derivation should be run. Given some
  user arguments, `Vyakarana` returns a `Vec<Prakriya>`.

In addition to these three types, we recommend exploring the types in the
`args` module. This module defines several types that `Vyakarana` accepts as
part of its API. Our hope is that callers can lean on Rust's type system to
define meaningful requests and receive correct derivations.


Core modules
------------

We start our description here with the high-level `vyakarana` module then work
our way into specific implementation details.


### `vyakarana`

This defines the public API. Given certain input conditions, the method here
return all `Prakriya`s compatible with those conditions. `vyakarana` is a thin
wrapper over the `ashtadhyayi` module, which we describe below.


### `args`

This defines the argument types that `Vyakarana` accepts. The types here follow
Paninian categories as closely as possible, so we have types like `Dhatu`,
`Krdanta`, `Subanta`, and so on. Likewise, the type system enforces that
arguments are well formed. For example, a `Krdanta` must have a `Dhatu` and a
`Krt`.



### `ashtadhyayi`

This defines the core rules of the Ashtadhayi. Given the input constraints
defined by `vyakarana`, the functions here return a `Result<Prakriya>`, where
the `Ok` condition means that the `Prakriya` was completed successfully.

`ashtadhyayi` invokes several other modules, most of which correspond to specific
portions of the Ashtadhyayi. A representative sample includes:

- `angasya` for rules under the scope of rule rule 6.4.1 (*aṅgasya*)
- `asiddhavat` for rules under the scope of rule 6.4.22 (*asiddhavadatrābhāt*)
- `krt` for rules under the scope of rule 3.1.91 (*kṛd atiṅ*)
- `taddhita` for rules under the scope of rule 4.1.76 (*taddhitāḥ*)
- `tripadi` for all rules from 8.2, 8.3, and 8.4, which are traditionally
  called the *tripādī*.

`ashtadhyayi` also manages access to various secondary texts, which exist in
their own modules. Some examples:

- `dhatupatha`, which provides a simple API for our `dhatupatha.tsv` file.
  These rules enter the Ashtadhyayi through rule 1.3.1 (*bhūvādayo dhātavaḥ*).

- `linganushasanam`, which defines the rules of the Paniniya-Linganushasanam.

- `unadipatha`, which defines the rules of the Unadipatha. These rules enter
  the Ashtadhyayi through rule 3.3.1 (*uṇādayo bahulam*)


### `prakriya`, `terms`, and `tags`

These define the `Prakriya` and `Term` types, as well as some useful secondary
types.

`tags` contains the `Tag` enum, which contains over a hundred variants. `Tag`
generalizes the *saṃjñā* concept and contains both traditional *saṃjñā*s and a
variety of ad-hoc flags.

Since `Term` and `Tag` are not stable, we do not expose them in our public API.


### `prakriya_stack`

This module defines utilities for exploring different paths of optional rules.
The core type here is `PrakriyaStack`, which manages a stack of rule paths.
(`RulePathStack` might be a clearer name, but it doesn't quite roll off the
tongue!)


### `sounds`

This defines various functions for testing and modifying Sanskrit sounds. The
core data structure here is `Set`, which stores sounds in a simple array.


Code style
----------

Here's a simplified example of what most of our rule code looks like:

```rust
if condition_1 {
    p.run_at("rule_1", i, |t| t.do_something_1());
} else if condition_2 {
    p.run_at("rule_2", i, |t| t.do_something_2());
} else if condition_3 {
    p.run_at("rule_3", i, |t| t.do_something_3());
}
```

Our code routinely uses short variable names to reduce visual noise and make
the logic of each rule more obvious. Here, `p` is a `Prakriya`, `i` is the
index of some `Term` within the `Prakriya`, and `|t| ...` is a closure (inline
function) that accepts a `Term`.


### Closures

`run_at`, like many of the methods on `Prakriya`, accepts a closure.
`vidyut-prakriya` leans heavily on closures for two important reasons:

1. We wish to record every rule we apply in the grammar. With closures, we can
   place this logic within the `run_at` call itself and ensure that it always
   takes effect. Otherwise, it's easy to forget to implement some of this
   bookkeeping logic, especially when we're refactoring the program.

2. Rust's [borrow checker][rust-borrow] enforces a "shared `xor` mutable" rule
   that prevents our code from holding multiple mutable references to the parts
   of a `Prakriya`. This minor snag becomes a major annoyance when implementing
   thousands of rules. Closures provide a neatly scoped way to mutate
   structures without extensive boilerplate.

To support this coding style, we also make heavy uses of indexes into the
`Vec<Term>` defined on `Prakriya`. In the code sample above, `i` is an index
for `p.terms[i]`. `Prakriya` also defines many methods for working with
    indices, such as `find_first_where`, `find_last_where`, `find_next_where`,
    and so on.


### Naming conventions

Since we have so many rules to write, we use short variable names for common
concepts. Some examples:

- `p` is our `Prakriya`.
- `i` is the index of a `Term` in the prakriya.
- `t` is a `Term`.
- `?` is a [Rust operator][rust-q] that roughly means "return if not found."
- `T` is an alias for `Tag`.

[rust-borrow]: https://users.rust-lang.org/t/newbie-mut-with-nested-structs/84755
[rust-q]: https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html


Control flow
------------

`vidyut-prakriya` generally uses a simple, procedural control flow. Here's an
example:

```rust
if condition_1 {
    p.run_at("rule_1", i, |t| t.do_something_1());
}
if condition_2 {
    p.run_at("rule_2", i, |t| t.do_something_2());
}
```

In this example, note that `rule_1` is an *apavāda* (exception) that blocks the
*utsarga* (general case) in `rule_2`. Our humble `if`-`else` structure thus
directly encodes a critical principle of the grammar.

The sections below extend the example above and illustrate the various kinds of
control flow we use, ordered from least to most complex.


### Rule sequences

The control flow in the example above is appropriate for simple sequences of
rules:

```rust
if condition_1 {
    p.run_at("rule_1", i, |t| t.do_something_1());
}
if condition_2 {
    p.run_at("rule_2", i, |t| t.do_something_2());
}
```


### Simple rule blocking

A natural extension is to use `if`-`else` chains to enforce that only one rule
can apply:

```rust
if condition_1 {
    p.run_at("rule_1", i, |t| t.do_something_1());
} else if condition_2 {
    p.run_at("rule_2", i, |t| t.do_something_2());
} else if condition_3 {
    p.run_at("rule_3", i, |t| t.do_something_2());
}
```


### Falling through

If a rule is optional, we might wish to "fall through" and consider other
options if the rule is rejected:

```rust
if condition_1 {
    p.optional_run_at("rule_1", i, |t| t.do_something_1());
}

if condition_2 {
    p.run_at("rule_2", i, |t| t.do_something_2());
} else if condition_3 {
    p.run_at("rule_3", i, |t| t.do_something_2());
}
```

Here, `rule_2` is accessible even if we reject `rule_1`.


### Simple locking

If we fall through in a simple way, we could end up running both `rule_1` and
`rule_2`. If we wish to prevent that, one option is to record whether `rule_1`
has applied then explore other rules only if it hasn't:

```rust
let mut done = false;
if condition_1 {
    // `optional_run_at` returns whether the rule was applied.
    done = p.optional_run_at("rule_1", i, |t| t.do_something_1());
}

if done {
    // do nothing.
} else if condition_2 {
    p.run_at("rule_2", i, |t| t.do_something_2());
} else if condition_3 {
    p.run_at("rule_3", i, |t| t.do_something_2());
}
```


### Extended locking

Sometimes, we might wish to implement rule locking over a very large section of
rules. For example, rules 7.2.8 - 7.2.78 all explain whether or not we should
add *iṭ-āgama*. In these cases, using boolean flags to manage locking is
tedious and clumsy.

One pattern we like is to wrap `Prakriya` in a struct that records whether a
rule has been applied or not. Here's a toy example:

```rust
struct LockingPrakriya<'a> {
    p: &'a mut Prakriya,
    done: bool,
}

impl<'a> LockingPrakriya<'a> {
    fn do_something_1(&mut self, rule: Rule, i: usize) {
        if !this.done {
            p.run_at(rule, i, |t| t.do_something_1());
            this.done = true;
        }
    }

    fn do_something_2(&mut self, rule: Rule, i: usize) {
        if !this.done {
            p.run_at(rule, i, |t| t.do_something_2());
            this.done = true;
        }
    }
}
```

Then in our code, we can construct a `LockingPrakriya` directly:

```rust
let mut lp = LockingPrakriya { p };
```

If we use the new `do_something_1` and `do_something_2` methods here,
`LockingPrakriya` ensures that we will apply at most one rule. If we still need
the original `Prakriya` struct, we can access it through `lp.p`. Once the
lifetime of `lp` has ended, we can continue using `p` as before.


### Context-aware locking

Suppose we wish to derive a *taddhitānta* that uses a specific
taddhita-pratyaya only if available in a specific meaning context. In this
case, we can extend the `LockingPrakriya` pattern above to record other useful
metadata, such as the meaning condition we wish to derive (if any). For
examples of this pattern, see `KrtPrakriya` and `TaddhitaPrakriya`.

