# Sbt Plugin

[![ci](https://github.com/fluentci-io/sbt-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/sbt-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [sbt](https://sbt.apache.org/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm sbt setup
```

## Functions

| Name          | Description                                 |
| ------------- | ------------------------------------------- |
| setup         | Installs a specific version of sbt.         |
| compile       | Compiles sources                            |
| clean         | Deletes files produced by the build, such as generated sources, compiled classes, and task caches. |
| doc           | Generates API documentation.                |
| package       | Produces the main artifact, such as a binary jar.  This is typically an alias for the task that actually does the packaging. |
| package_bin   |  Produces a main artifact, such as a binary jar. |
| package_doc   | Produces a documentation artifact, such as a jar containing API documentation. |
| package_src   | Produces a source artifact, such as a jar containing sources and resources. |
| publish       | Publishes artifacts to a repository. |
| publish_local | Publishes artifacts to the local Ivy repository. |
| publish_m2    | Publishes artifacts to the local Maven repository. |
| test          | Executes all tests. |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/sbt@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: sbt
    args: |
      setup
- name: Show sbt version
  run: |
    type sbt
```
