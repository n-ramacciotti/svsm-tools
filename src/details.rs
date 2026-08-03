// SPDX-License-Identifier: MIT
//
// Copyright (C) 2026 Nicola Ramacciotti
//
// Author: Nicola Ramacciotti <niko.ramak@gmail.com>

use bitfield_struct::bitfield;
use core::mem;
use std::error::Error;
use std::ffi::CStr;
use zerocopy::{Immutable, IntoBytes};

const OCP_SOURCE_NAME_LEN: usize = 112;
pub const OCP_SOURCE_DETAILS_SIZE: usize = 128;
pub const OCP_OBJECT_DETAILS_SIZE: usize = 12;

#[bitfield(u32)]
#[derive(IntoBytes, Immutable)]
pub struct OcpSourceFlags {
    writable: bool,
    #[bits(31)]
    _rsvd_31_1: u32,
}

#[repr(u32)]
#[derive(Debug, IntoBytes, Immutable, Clone, Copy)]
/// Type of data the OCP source contains.
pub enum OcpSourceType {
    StaticString = 0,
    Integer = 1,
    String = 2,
}

/// OCP source entry structure.
#[repr(C)]
#[derive(Debug, IntoBytes, Immutable)]
pub struct OcpSourceDetails {
    /// Super index of the source
    sup_index: u32,
    /// Sub index of the source
    sub_index: u32,
    /// Type of the source.
    kind: OcpSourceType,
    /// Source flags.
    flags: OcpSourceFlags,
    /// Name of the source encoded as UTF-8.
    name: [u8; OCP_SOURCE_NAME_LEN],
}

impl OcpSourceDetails {
    pub fn sup_index(&self) -> u32 {
        self.sup_index
    }

    pub fn sub_index(&self) -> u32 {
        self.sub_index
    }

    pub fn kind(&self) -> OcpSourceType {
        self.kind
    }

    pub fn flags(&self) -> OcpSourceFlags {
        self.flags
    }

    pub fn name(&self) -> Result<&str, Box<dyn Error>> {
        CStr::from_bytes_until_nul(&self.name)?
            .to_str()
            .map_err(|e| e.into())
    }

    pub fn writable(&self) -> bool {
        self.flags.writable()
    }
}

#[repr(u32)]
#[derive(Debug, IntoBytes, Immutable, Clone, Copy)]
/// Type of objects the SVSM contains.
pub enum OcpObjectType {
    Svsm = 0,
}

#[repr(C)]
#[derive(Debug, IntoBytes, Immutable)]
pub struct OcpObjectDetails {
    sup_index: u32,
    category: OcpObjectType,
    count: u32,
}

impl OcpObjectDetails {
    pub fn sup_index(&self) -> u32 {
        self.sup_index
    }

    pub fn category(&self) -> OcpObjectType {
        self.category
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

const _: () = assert!(
    mem::offset_of!(OcpSourceDetails, sup_index) == 0x00
        && mem::offset_of!(OcpSourceDetails, sub_index) == 0x04
        && mem::offset_of!(OcpSourceDetails, kind) == 0x08
        && mem::offset_of!(OcpSourceDetails, flags) == 0x0C
        && mem::offset_of!(OcpSourceDetails, name) == 0x10
        && mem::size_of::<OcpSourceDetails>() == OCP_SOURCE_DETAILS_SIZE
);

const _: () = assert!(
    mem::offset_of!(OcpObjectDetails, sup_index) == 0x00
        && mem::offset_of!(OcpObjectDetails, category) == 0x04
        && mem::offset_of!(OcpObjectDetails, count) == 0x08
        && mem::size_of::<OcpObjectDetails>() == OCP_OBJECT_DETAILS_SIZE
);
