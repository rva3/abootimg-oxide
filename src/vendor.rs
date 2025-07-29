use binrw::{binrw, BinRead, BinWrite};

// TODO: pretty section layout using asciiflow.com

/// Android vendor boot image header version 3 and 4
#[binrw]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[brw(little, magic = b"VNDRBOOT")]
#[br(assert(header_size == self.header_size()))]
pub struct VendorHeader {
    #[br(temp)]
    #[bw(calc = self.header_version())]
    header_version: u32,
    /// Page size in bytes
    pub page_size: u32,
    /// Kernel physical load address
    pub kernel_addr: u32,
    /// Ramdisk physical load address
    pub ramdisk_addr: u32,
    /// Vendor ramdisk size
    pub vendor_ramdisk_size: u32,
    /// Kernel command line
    pub cmdline: Box<[u8; 2048]>,
    /// Kernel tags physical address
    pub tags_addr: u32,
    /// Board or product name
    pub board_name: [u8; 16],
    #[br(temp)]
    #[bw(calc = self.header_size())]
    header_size: u32,
    /// DTB size
    pub dtb_size: u32,
    /// DTB physical load address
    pub dtb_addr: u64,
    /// V4-specific fields.
    ///
    /// This is only present in version 4 and the version will be inferred from this field.
    #[br(if(header_version == 4))]
    pub v4: Option<VendorHeaderV4>,
}
impl VendorHeader {
    /// Returns the vendor boot image header's version number.
    #[must_use]
    pub const fn header_version(&self) -> u32 {
        if self.v4.is_some() {
            4
        } else {
            3
        }
    }
    const fn header_size(&self) -> u32 {
        if self.v4.is_some() {
            2128
        } else {
            2112
        }
    }

    // TODO: functions to return positions
}

/// V4-specific fields of the Android vendor boot image header
#[derive(BinRead, BinWrite, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VendorHeaderV4 {
    /// Vendor ramdisk table size
    pub vendor_ramdisk_table_size: u32,
    /// Vendor ramdisk entry number
    pub vendor_ramdisk_table_entry_num: u32,
    /// Vendor ramdisk entry size
    pub vendor_ramdisk_table_entry_size: u32,
    /// Bootconfig size
    pub bootconfig_size: u32,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::io::NoSeek;
    use expect_test_bytes::expect_file;

    use super::*;

    #[test]
    fn simple_write_read() {
        fn pad_slice_to_array<const N: usize>(slice: &[u8]) -> [u8; N] {
            let mut arr = [0u8; N];
            let len = slice.len().min(N);
            arr[..len].copy_from_slice(&slice[..len]);
            arr
        }
        let expected_header = VendorHeader {
            page_size: 0x7357_0001,
            kernel_addr: 0x7357_0002,
            ramdisk_addr: 0x7357_0003,
            vendor_ramdisk_size: 0x7357_0004,
            cmdline: Box::new(pad_slice_to_array(b"example")),
            tags_addr: 0x7357_0005,
            board_name: *b"example\0\0\0\0\0\0\0\0\0",
            dtb_size: 0x7357_0006,
            dtb_addr: 0x7357_7357_7357_0007,
            v4: Some(VendorHeaderV4 { vendor_ramdisk_table_size: 0x7357_0007, vendor_ramdisk_table_entry_num: 0x7357_0008, vendor_ramdisk_table_entry_size: 0x7357_0009, bootconfig_size: 0x7357_000a }),

        };

        let mut actual_bytes = Vec::new();
        expected_header
            .write(&mut NoSeek::new(&mut actual_bytes))
            .unwrap();

        expect_file!["test_data/vendor/simple_write_read"].assert_eq(&actual_bytes);

        let actual_header = VendorHeader::read(&mut Cursor::new(&actual_bytes)).unwrap();

        assert_eq!(expected_header, actual_header);
    }
}
