---
title: Build from source
description: Clone the repository and run its gate.
---

# Build from source

You need Rust, the [Task runner](https://taskfile.dev/) and the
[ESS](https://beyond10x.github.io/ess/) command line; CI pins ESS 0.52.0.

```bash
git clone https://github.com/beyond10x/commission.git
cd commission
task check
```

`task check` runs the specification gate, the generated-model drift check, the hand-written-model
check, the dependency guard, the documentation drift check, formatting, Clippy and the tests.

## The documentation

The pages under **Reference** are generated from the ESS specification by the `commission-docs`
crate. Regenerate them after changing `ess/`:

```bash
task docs
```

To build the site itself you need Node.js 20 or later:

```bash
npm --prefix website ci
npm --prefix website run build
```
