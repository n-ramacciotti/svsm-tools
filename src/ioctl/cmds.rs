// SPDX-License-Identifier: MIT
//
// Copyright (C) 2026 Nicola Ramacciotti
//
// Author: Nicola Ramacciotti <niko.ramak@gmail.com>

use super::bindings::OCP_SVSM_IOCTL_TYPE;
use super::bindings::{ocp_svsm_list_objects, ocp_svsm_list_sources, ocp_svsm_read_write_source};
use nix::{ioctl_readwrite, ioctl_write_ptr};

const OCP_SVSM_IOCTL_TYPE_CMD: u8 = OCP_SVSM_IOCTL_TYPE as u8;
const OCP_SVSM_LIST_OBJECTS_TYPE: u8 = 1;
const OCP_SVSM_LIST_SOURCES_TYPE: u8 = 2;
const OCP_SVSM_READ_SOURCE: u8 = 3;
const OCP_SVSM_WRITE_SOURCE: u8 = 4;

ioctl_readwrite!(
    list_objects,
    OCP_SVSM_IOCTL_TYPE_CMD,
    OCP_SVSM_LIST_OBJECTS_TYPE,
    ocp_svsm_list_objects
);
ioctl_readwrite!(
    list_sources,
    OCP_SVSM_IOCTL_TYPE_CMD,
    OCP_SVSM_LIST_SOURCES_TYPE,
    ocp_svsm_list_sources
);
ioctl_readwrite!(
    read_source,
    OCP_SVSM_IOCTL_TYPE_CMD,
    OCP_SVSM_READ_SOURCE,
    ocp_svsm_read_write_source
);
ioctl_write_ptr!(
    write_source,
    OCP_SVSM_IOCTL_TYPE_CMD,
    OCP_SVSM_WRITE_SOURCE,
    ocp_svsm_read_write_source
);
