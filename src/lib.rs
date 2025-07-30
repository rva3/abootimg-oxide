//! A parser for Android boot image headers (e.g. `boot.img` or `vendor_boot.img`).
//!
//! This can be used to extract or patch e.g. the kernel or ramdisk.
//!
//! Byte array fields (`[u8; N]`) can be used as null-terminated strings.
//!
//! [`Header`] denotes the standard `boot.img` boot image's header with the file signature
//! `ANDROID!`. [`VendorHeader`] denotes the `vendor_boot.img` vendor boot image's header with the
//! file signature `VNDRBOOT`.
//!
//! # Examples
//!
//! ```no_run
//! use std::fs::File;
//! use abootimg_oxide::{BufReader, Header};
//!
//! let mut r = BufReader::new(File::open("boot_a.img").unwrap());
//! let hdr = Header::parse(&mut r).unwrap();
//! println!("{hdr:#?}");
//!
//! // Extract the kernel
//! use std::io::{self, BufWriter, Read, Seek, SeekFrom};
//!
//! let mut w = BufWriter::new(File::create("boot_a_kernel").unwrap());
//! let r = r.get_mut();
//! r.seek(SeekFrom::Start(hdr.kernel_position() as u64))
//!     .unwrap();
//! io::copy(&mut r.take(hdr.kernel_size().into()), w.get_mut()).unwrap();
//! ```
//!
//! # Features
//!
//! - `std` (enabled by default): Enables [`binrw`](mod@binrw)'s `std` feature. See its
//!   documentation for more details.
//!
//! # Note about `Seek` requirement
//!
//! [`binrw`](mod@binrw) requires the [`Seek` trait](binrw::io::Seek) to be implemented on readers and writers.
//!
//! Only the read portion of [`Header`] seeks. For other functionality, you can use the
//! [`binrw::io::NoSeek`] adapter to be able to read and write to and from unseekable streams.
#![cfg_attr(not(any(feature = "std", test)), no_std)]

extern crate alloc;

#[doc(no_inline)]
pub use binrw::io::BufReader;

mod standard;
mod vendor;
mod version;

pub use standard::{Header, HeaderV0, HeaderV0Versioned, HeaderV3};
pub use vendor::{VendorHeader, VendorHeaderV4};
pub use version::{OsPatch, OsVersion, OsVersionPatch};
