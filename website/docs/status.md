---
title: 'Status: bootstrap'
sidebar_label: Status
description: What the Commission repository holds today, and what is planned.
lede: The model, the ports, the local runtime loop and the checks a run needs are on main. The runtime executes no effect yet, and there is no release.
source: website/data/status.json, a b10x-status/1 document; the landing page reads the same file
source_url: https://github.com/beyond10x/commission/blob/main/website/data/status.json
---

import status from '@site/data/status.json';

Shipped means code on main that runs today. Decided means recorded in an architecture decision, not
built. Planned means on the plan only. The responsibility model is described on the
[domain model](./reference/domain-model.md) page, and effect invocation in
[A run](./concepts/a-run.md#effect-invocation-decided-design-not-shipped-code).

<StatusTable data={status} />

Commission ships no command-line tool. The repository's two command lines, `commission-xtask` and
`commission-docs`, are unpublished repository tools.
