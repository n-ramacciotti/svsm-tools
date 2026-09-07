# SVSM tools

Userspace utility to interact with [COCONUT-SVSM](https://github.com/coconut-svsm/svsm) from within a guest of a Confidential Virtual Machine (CVM).

## Observability and Configuration Protocol

A guest operating system (OS), running inside a Confidential Virtual Machine (CVM), does not know the state information, such as log data or memory usage, of the secure module [COCONUT-SVSM](https://github.com/coconut-svsm/svsm), which is responsible for managing the CVM. These sources of information could be useful for the guest OS to understand the environment it is running in.

A new protocol, called Observability and Configuration Protocol (OCP), has been defined to enable information to be retrieved from an SVSM source, and to allow the configuration of sources in SVSM.

## Getting started

### Requirements
In order to implement this protocol, the following parts are required:
  -  The implementation of a SVSM-side handler for the protocol. For now, it is available as a pull request [(#1138)](https://github.com/coconut-svsm/svsm/pull/1138)
  -  A guest-side handler in the form of a Linux device driver. For now, it is available as a pull request [(#20)](https://github.com/coconut-svsm/linux/pull/20)
  -  A user-space tool that uses the Linux device driver to interact with the sources. This repository.

### Setup
- Compile SVSM, following the instructions in [INSTALL.md](https://github.com/coconut-svsm/svsm/blob/main/Documentation/docs/installation/INSTALL.md). Make sure to use a version with OCP support.

- Prepare a guest image with OCP-capable kernel and launch a virtual machine following the instruction in [Launch Script](https://github.com/coconut-svsm/svsm/blob/main/Documentation/docs/installation/INSTALL.md#launch-script). Make sure that the `ocp-svsm` module is loaded. You can now interact with SVSM through `/dev/ocp`. Change the permissions of it so you can interact without `sudo`.

- Once the VM is running, clone this repository inside it.

### How to use

Build the utility with `cargo build` and launch it with `sudo path/to/svsm-tools`. Specify the `-h` or `--help` option to view a brief description of each command.

#### Commands
- `list`: displays a list of all sources, grouped by category.
- `read`: reads a specified source by name.
- `write`: writes a specified source by name.

## License
Licensed under the [MIT License](LICENSE).
