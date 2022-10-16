Vidyut
======

> मा भूदेवं क्षणमपि च ते विद्युता विप्रयोगः ॥ 

**Vidyut** is an experimental Sanskrit parser that aims to be accurate,
lightweight, and lightning fast. But right now, Vidyut is none of these things.

Our current code is a proof of concept that parses most Sanskrit text with
middling accuracy at a rate of about 4 shlokas a second. Now that we have a
basic scaffold in place, we can iterate on speed, efficiency, and quality.

[![Build status](https://github.com/ambuda-org/vidyut/workflows/ci/badge.svg)](https://github.com/ambuda-org/vidyut/actions)


Usage
-----

Build the code and fetch our linguistic data:

    make install

Run a simple evaluation script:

    make eval


Development
-----------

Run unit tests:

    cargo test

Profile overall runtime and memory usage:

    make profile-general

Profile runtime per function:

    make target=time profile-target-osx

Profile memory allocations:

    make target=alloc profile-target-osx
