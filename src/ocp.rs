// SPDX-License-Identifier: MIT
//
// Copyright (C) 2026 Nicola Ramacciotti
//
// Author: Nicola Ramacciotti <niko.ramak@gmail.com>

use super::details::*;
use super::ioctl::*;
use std::cmp;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::os::fd::{AsRawFd, RawFd};
use std::vec::Vec;

/// Userspace handler for the SVSM OCP kernel module.
pub struct OcpHandler {
    ocp: File,
}

impl OcpHandler {
    /// Open "/dev/ocp_svsm" in read/write mode.
    /// If the file cannot be opened, this function will panic.
    pub fn new() -> Self {
        let ocp_svsm_path = "/dev/ocp";
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(ocp_svsm_path)
            .unwrap();
        Self { ocp: file }
    }

    pub fn list_objects(
        &self,
        first_entry: u32,
        buf_size: u32,
    ) -> Result<Vec<OcpObjectDetails>, Box<dyn Error>> {
        if buf_size > OCP_MAX_BUFFER_SIZE
            || !(buf_size as usize).is_multiple_of(OCP_OBJECT_DETAILS_SIZE)
        {
            return Err("Buffer is too large".into());
        }

        let num_entries = buf_size as usize / OCP_OBJECT_DETAILS_SIZE;

        let mut objects = Vec::with_capacity(num_entries);

        let mut request = ocp_svsm_list_objects {
            buf_ptr: objects.as_mut_ptr() as u64,
            first_entry,
            buf_size,
        };
        // SAFETY: We trust the kernel on the validity of the number of bytes
        // returned
        unsafe {
            let bytes_returned = list_objects(self.ocp.as_raw_fd(), &mut request)? as usize;
            let entries_returned = bytes_returned / OCP_OBJECT_DETAILS_SIZE;
            objects.set_len(entries_returned);
        };

        Ok(objects)
    }

    pub fn list_all_objects(&self) -> Result<Vec<OcpObjectDetails>, Box<dyn Error>> {
        let mut final_objects = Vec::new();

        let mut index = 0;
        const OBJECTS_PER_CALL: u32 = 5;

        loop {
            let tmp_objs =
                self.list_objects(index, OBJECTS_PER_CALL * OCP_OBJECT_DETAILS_SIZE as u32)?;
            let tmp_len = tmp_objs.len() as u32;
            index += tmp_len;
            final_objects.extend(tmp_objs);
            if tmp_len < OBJECTS_PER_CALL {
                break;
            }
        }

        Ok(final_objects)
    }

    pub fn list_sources(
        &self,
        object_index: u32,
        first_entry: u32,
        buf_size: u32,
    ) -> Result<Vec<OcpSourceDetails>, Box<dyn Error>> {
        if buf_size > OCP_MAX_BUFFER_SIZE
            || !(buf_size as usize).is_multiple_of(OCP_SOURCE_DETAILS_SIZE)
        {
            return Err("Buffer is too large".into());
        }

        let num_entries = buf_size as usize / OCP_SOURCE_DETAILS_SIZE;

        let mut objects = Vec::with_capacity(num_entries);

        let mut request = ocp_svsm_list_object_sources {
            buf_ptr: objects.as_mut_ptr() as u64,
            object_index,
            first_entry,
            buf_size,
        };
        // SAFETY: We trust the kernel on the validity of the number of entries
        // returned
        unsafe {
            let bytes_returned = list_sources(self.ocp.as_raw_fd(), &mut request)? as usize;
            let entries_returned = bytes_returned / OCP_SOURCE_DETAILS_SIZE;
            objects.set_len(entries_returned);
        };

        Ok(objects)
    }

    pub fn list_all_sources(
        &self,
        object_index: u32,
    ) -> Result<Vec<OcpSourceDetails>, Box<dyn Error>> {
        let mut final_objects = Vec::new();

        let mut index = 0;
        const SOURCES_PER_CALL: u32 = 5;

        loop {
            let tmp_objs = self.list_sources(
                object_index,
                index,
                SOURCES_PER_CALL * OCP_SOURCE_DETAILS_SIZE as u32,
            )?;
            let tmp_len = tmp_objs.len() as u32;
            index += tmp_len;
            final_objects.extend(tmp_objs);
            if tmp_len < SOURCES_PER_CALL {
                break;
            }
        }

        Ok(final_objects)
    }

    pub fn read_source(
        &self,
        object_index: u32,
        source_index: u32,
        offset: u32,
        bytes_to_read: u32,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut bytes = Vec::with_capacity(bytes_to_read as usize);

        let mut request = ocp_svsm_read_source {
            buf_ptr: bytes.as_mut_ptr() as u64,
            object_index,
            source_index,
            offset,
            bytes_to_read,
        };
        // SAFETY: We trust the kernel on the validity of the number of entries
        // returned
        unsafe {
            let bytes_read = read_source(self.ocp.as_raw_fd(), &mut request)?;
            bytes.set_len(bytes_read as usize);
        };

        Ok(bytes)
    }

    pub fn read_entire_source(
        &self,
        object_index: u32,
        source_index: u32,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut bytes = Vec::new();
        let mut offset = 0;
        const BYTES_PER_CALL: u32 = 1024;

        loop {
            let tmp_bytes = self.read_source(object_index, source_index, offset, BYTES_PER_CALL)?;
            let tmp_len = tmp_bytes.len() as u32;
            offset += tmp_len;
            bytes.extend(tmp_bytes);
            if tmp_len < BYTES_PER_CALL {
                break;
            }
        }

        Ok(bytes)
    }

    pub fn write_source(
        &self,
        object_index: u32,
        source_index: u32,
        offset: u32,
        bytes_to_write: u32,
        buf: &[u8],
    ) -> Result<u32, Box<dyn Error>> {
        let request = ocp_svsm_write_source {
            buf_ptr: buf.as_ptr() as u64,
            object_index,
            source_index,
            offset,
            bytes_to_write,
        };
        // SAFETY: ...
        let bytes_written = unsafe { write_source(self.ocp.as_raw_fd(), &request)? };

        Ok(bytes_written as u32)
    }

    pub fn write_entire_source(
        &self,
        object_index: u32,
        source_index: u32,
        buf: &[u8],
    ) -> Result<u32, Box<dyn Error>> {
        const BYTES_PER_CALL: u32 = 1024;
        let mut offset = 0;
        let mut total_bytes_written = 0;

        if buf.len() >= u32::MAX as usize {
            return Err("Buffer is too large".into());
        }

        let size = buf.len() as u32;

        while offset < size {
            let bytes_to_write = cmp::min(BYTES_PER_CALL, size - offset);
            let bytes_written = self.write_source(
                object_index,
                source_index,
                offset,
                bytes_to_write,
                &buf[offset as usize..(offset + bytes_to_write) as usize],
            )?;
            total_bytes_written += bytes_written;
            offset += bytes_written;
            if bytes_written < bytes_to_write {
                break;
            }
        }

        Ok(total_bytes_written)
    }

    pub fn find_source_by_name(
        &self,
        source_name: &str,
    ) -> Result<OcpSourceDetails, Box<dyn Error>> {
        let objects = self.list_all_objects()?;
        for obj in objects {
            let sources = self.list_all_sources(obj.sup_index())?;
            for src in sources {
                if src.name()? == source_name {
                    return Ok(src);
                }
            }
        }
        Err("Source not found".into())
    }
}

impl AsRawFd for OcpHandler {
    fn as_raw_fd(&self) -> RawFd {
        self.ocp.as_raw_fd()
    }
}

impl Default for OcpHandler {
    fn default() -> Self {
        Self::new()
    }
}
