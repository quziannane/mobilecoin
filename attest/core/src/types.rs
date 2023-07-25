// Copyright (c) 2018-2022 The MobileCoin Foundation

//! This module contains FFI wrapper types which are used for multiple purposes,
//! or in multiple places.

pub mod basename;
pub mod cpu_svn;
pub mod epid_group_id;
pub mod ext_prod_id;
pub mod family_id;
pub mod key_id;
pub mod mac;
pub mod measurement;
pub mod pib;
pub mod report_body;
pub mod report_data;
pub mod spid;
pub mod update_info;

pub type MiscSelect = u32;
pub type ProductId = u16;
