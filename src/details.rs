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

use crate::ioctl::OCP_MAX_SOURCE_NAME_SIZE;
pub const OCP_SOURCE_DETAILS_SIZE: usize = 128;

#[bitfield(u32)]
#[derive(IntoBytes, Immutable)]
pub struct OcpSourceFlags {
    writable: bool,
    #[bits(31)]
    _rsvd_31_1: u32,
}

#[repr(u16)]
#[derive(Debug, IntoBytes, Immutable, Clone, Copy)]
/// Type of data the OCP source contains.
pub enum OcpSourceType {
    Object = 0,
    Bytes = 1,
    SInteger8Bit = 2,
    SInteger16Bit = 3,
    SInteger32Bit = 4,
    SInteger64Bit = 5,
}

/// OCP source entry structure.
#[repr(C)]
#[derive(Debug, IntoBytes, Immutable)]
pub struct OcpSource {
    /// Source flags.
    flags: OcpSourceFlags,
    /// Type of the source.
    kind: OcpSourceType,
    /// Reserved field
    _rsvd: u16,
    /// Name of the source encoded as UTF-8.
    name: [u8; OCP_MAX_SOURCE_NAME_SIZE as usize],
}

impl OcpSource {
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

const _: () = assert!(
    mem::offset_of!(OcpSource, flags) == 0x00
        && mem::offset_of!(OcpSource, kind) == 0x04
        && mem::offset_of!(OcpSource, _rsvd) == 0x06
        && mem::offset_of!(OcpSource, name) == 0x08
        && mem::size_of::<OcpSource>() == OCP_SOURCE_DETAILS_SIZE
);
