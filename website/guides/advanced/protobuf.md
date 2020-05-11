---
last_modified_on: "2020-05-11"
$schema: "/.meta/.schemas/guides.json"
title: Decoding Protobufs using WASM
description: Plug an experimental protobuf plugin into Vector
author_github: https://github.com/hoverbear
tags: ["type: guide", "domain: config"]
---

import Alert from '@site/src/components/Alert';
import Assumptions from '@site/src/components/Assumptions';

<Assumptions name="guide">

* You need to decode a specific, static protobuf.
* You understand the [basic Vector concepts][docs.about.concepts] and understand [how to set up a pipeline][guides.getting-started.your-first-pipeline].
* You are comfortable setting up a development toolchain on your machine, editing code, compiling code, and shipping the artifacts.
* You are comfortable with the basics of Git and Rust.

</Assumptions>

Today, we'll use Vector's experimental WASM plugin support to resolve the gnarly issue of handling protobufs in Vector.

[Protocol Buffers (or protobufs)](https://developers.google.com/protocol-buffers) are an efficient, language neutral data format used commonly in place of JSON. Unfortunately, protobuf messages don't describe themselves fully, so in order to be useful, Vector needs a protobuf definition.

Rust, the language Vector is written in, does not currently have ecosystem support for dynamically generating protobuf decoders. Based on discussions with library maintainers, our team at Vector decided that supporting the more general idea of WASM plugins would offer broader, and more performant, support for our users.

So let's cross this chasm by writing a protobuf plugin to Vector!

# Your Toolchain

<Alert type="warning">

While you can build a WASM plugin from any platform (Linux, FreeBSD, Windows, even OS X), you'll need to run it a host supported by Lucet, our WASM runtime. Currently this is limited to currently only supports a x86, Glibc based, Linux kernel 4.0+ based host. This guide presumes you are writing, building, and running the WASM plugin on the same host.

So if you don't use Linux, you'll need to deviate a bit below.

</Alert>

First, you'll need a Rust toolchain. Since we're going to be **cross compiling** to the `wasm32-wasi` you will need to use [Rustup](https://rustup.rs/). For most users, this shall suffice:

```bash
curl https://sh.rustup.rs -sSf | sh
rustup target add wasm32-wasi
```

# Get the template

Vector's official repository stores a template protobuf plugin used to test WASM support at `tests/data/wasm/protobuf/`. To use it, first note your Vector version. Run `vector --version` and make sure it's version 0.10.0 or higher.

```bash
git clone https://github.com/timberio/vector.git
git checkout v0.x.x # Put your version here!
cp vector/tests/data/wasm/protobuf/ my-protobuf-plugin
cd my-protobuf-plugin
```

# Using your protobuf

The template you downloaded can take a defined protobuf file then build it into a plaintext `wat` file. You
can then include the output `wat` in your configuration management, dropping it into the `/etc/vector/plugins/` folder.

You can follow the `README.md` file included for specific steps, but for most users, the following will suffice:

```bash
rm -r protos
cp -r $YOUR_PROTOS protos
```

Then, edit the `src/lib.rs` file's includes. Near the top you'll see `include!(...)` statements that look similar to the
[`prost` documentation](https://docs.rs/prost-build/0.6.1/prost_build/). You'll need to match the structure of the
protos you copied in:

```rust
type DecodingTarget = crate::example::subexample::Example;

pub mod example {
    pub mod subexample {
        include!(concat!(env!("OUT_DIR"), "/example.subexample.rs"));
    }
}

pub mod other_example {
    include!(concat!(env!("OUT_DIR"), "/other_example.rs"));
}
```

Next, you can load


[docs.about.concepts]: /docs/about/concepts/
[guides.getting-started.your-first-pipeline]: /guides/getting-started/your-first-pipeline/