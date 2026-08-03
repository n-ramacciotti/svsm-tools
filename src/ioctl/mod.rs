// SPDX-License-Identifier: MIT
//
// Copyright (C) 2026 Nicola Ramacciotti
//
// Author: Nicola Ramacciotti <niko.ramak@gmail.com>

#[allow(non_camel_case_types)]
#[allow(unused)]
mod bindings;
mod cmds;

pub use bindings::{
    OCP_MAX_BUFFER_SIZE, OCP_MAX_SOURCE_NAME_SIZE, OCP_SOURCE_DETAIL_SIZE, ocp_svsm_list_objects,
    ocp_svsm_list_sources, ocp_svsm_read_write_source,
};
pub use cmds::{list_objects, list_sources, read_source, write_source};
