---

event_types: ["log"]
issues_url: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22transform%3A+sampler%22
sidebar_label: "sampler|[\"log\"]"
source_url: https://github.com/timberio/vector/tree/master/src/transforms/sampler.rs
status: "beta"
title: "sampler transform" 
---

The `sampler` transform accepts [`log`][docs.data-model#log] events and allows you to sample events with a configurable rate.

## Configuration

import CodeHeader from '@site/src/components/CodeHeader';

<CodeHeader fileName="vector.toml" learnMoreUrl="/docs/setup/configuration"/ >

```toml
[transforms.my_transform_id]
  # REQUIRED
  type = "sampler" # example, must be: "sampler"
  inputs = ["my-source-id"] # example
  rate = 10 # example
  
  # OPTIONAL
  pass_list = ["[error]", "field2"] # example, no default
```

## Options

import Fields from '@site/src/components/Fields';

import Field from '@site/src/components/Field';

<Fields filters={true}>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[["[error]","field2"]]}
  name={"pass_list"}
  nullable={true}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"[string]"}
  unit={null}
  >

### pass_list

A list of regular expression patterns to exclude events from sampling. If an event's `"message"` key matches _any_ of these patterns it will _not_ be sampled.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[10]}
  name={"rate"}
  nullable={false}
  path={null}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"int"}
  unit={null}
  >

### rate

The rate at which events will be forwarded, expressed as 1/N. For example, `rate = 10` means 1 out of every 10 events will be forwarded and the rest will be dropped.


</Field>


</Fields>

## How It Works

### Environment Variables

Environment variables are supported through all of Vector's configuration.
Simply add `${MY_ENV_VAR}` in your Vector configuration file and the variable
will be replaced before being evaluated.

You can learn more in the [Environment Variables][docs.configuration#environment-variables]
section.


[docs.configuration#environment-variables]: /docs/setup/configuration#environment-variables
[docs.data-model#log]: /docs/about/data-model#log