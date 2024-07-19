# metadata-provider-interface
A CLI template for tools to provide system metadata to installers and environment managers

## Overview

Metadata providers are CLI tools that provide metadata about the system they are running on. This metadata can be used
by installers and environment managers to make decisions about which packages to choose from.

The CLI was chosen as the lowest common denominator so that installer tools in different languages can all use the same
metadata providers [(as suggested by Paul Moore)](https://discuss.python.org/t/selecting-variant-wheels-according-to-a-semi-static-specification/53446/89). This "interface" project provides a template for metadata providers to follow, and defines a minimal
set of functionality that metadata providers should implement.

This project provides reference implementations of a metadata providers. Metadata providers should have no
runtime dependencies, other than libraries that they may load for determining the system state. For example, a python
script would not be a good metadata provider, because it would require the python runtime (and possibly dependencies).

## Metadata Provider Interface

Metadata providers must provide the following commands:

* `names` - List the names of the variants that the metadata provider provides
* `state` - List the state of the environment with respect to the metadata that the provider provides

### `names`

The `names` command should output a list of names of the variants that the metadata provider provides. The names should
be separated by newlines.

```shell
$ metadata-provider names
some-variant-key
another-variant-key
```

### `state`

The `state` command should output the state of the environment with respect to the metadata that the provider provides.
The state should be a JSON object, to minimize parsing complexity for the installer tools.

```shell
$ metadata-provider state
{
  "some-variant-key": "some-value",  
  "another-variant-key": "another-value"
}
```
