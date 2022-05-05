// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

use font_types::*;

/// <https://docs.microsoft.com/en-us/typography/opentype/spec/head>
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct Head {
    /// Major version number of the font header table — set to 1.
    pub major_version: BigEndian<u16>,
    /// Minor version number of the font header table — set to 0.
    pub minor_version: BigEndian<u16>,
    /// Set by font manufacturer.
    pub font_revision: BigEndian<Fixed>,
    /// To compute: set it to 0, sum the entire font as uint32, then
    /// store 0xB1B0AFBA - sum. If the font is used as a component in a
    /// font collection file, the value of this field will be
    /// invalidated by changes to the file structure and font table
    /// directory, and must be ignored.
    pub checksum_adjustment: BigEndian<u32>,
    /// Set to 0x5F0F3CF5.
    pub magic_number: BigEndian<u32>,
    /// See the flags enum
    pub flags: BigEndian<u16>,
    /// Set to a value from 16 to 16384. Any value in this range is
    /// valid. In fonts that have TrueType outlines, a power of 2 is
    /// recommended as this allows performance optimizations in some
    /// rasterizers.
    pub units_per_em: BigEndian<u16>,
    /// Number of seconds since 12:00 midnight that started January 1st
    /// 1904 in GMT/UTC time zone.
    pub created: BigEndian<LongDateTime>,
    /// Number of seconds since 12:00 midnight that started January 1st
    /// 1904 in GMT/UTC time zone.
    pub modified: BigEndian<LongDateTime>,
    /// Minimum x coordinate across all glyph bounding boxes.
    pub x_min: BigEndian<i16>,
    /// Minimum y coordinate across all glyph bounding boxes.
    pub y_min: BigEndian<i16>,
    /// Maximum x coordinate across all glyph bounding boxes.
    pub x_max: BigEndian<i16>,
    /// Maximum y coordinate across all glyph bounding boxes.
    pub y_max: BigEndian<i16>,
    /// see somewhere else
    pub mac_style: BigEndian<u16>,
    /// Smallest readable size in pixels.
    pub lowest_rec_ppem: BigEndian<u16>,
    /// Deprecated (Set to 2).
    pub font_direction_hint: BigEndian<i16>,
    /// 0 for short offsets (Offset16), 1 for long (Offset32).
    pub index_to_loc_format: BigEndian<i16>,
    /// 0 for current format.
    pub glyph_data_format: BigEndian<i16>,
}

impl Head {
    /// Major version number of the font header table — set to 1.
    pub fn major_version(&self) -> u16 {
        self.major_version.get()
    }

    /// Minor version number of the font header table — set to 0.
    pub fn minor_version(&self) -> u16 {
        self.minor_version.get()
    }

    /// Set by font manufacturer.
    pub fn font_revision(&self) -> Fixed {
        self.font_revision.get()
    }

    /// To compute: set it to 0, sum the entire font as uint32, then
    /// store 0xB1B0AFBA - sum. If the font is used as a component in a
    /// font collection file, the value of this field will be
    /// invalidated by changes to the file structure and font table
    /// directory, and must be ignored.
    pub fn checksum_adjustment(&self) -> u32 {
        self.checksum_adjustment.get()
    }

    /// Set to 0x5F0F3CF5.
    pub fn magic_number(&self) -> u32 {
        self.magic_number.get()
    }

    /// See the flags enum
    pub fn flags(&self) -> u16 {
        self.flags.get()
    }

    /// Set to a value from 16 to 16384. Any value in this range is
    /// valid. In fonts that have TrueType outlines, a power of 2 is
    /// recommended as this allows performance optimizations in some
    /// rasterizers.
    pub fn units_per_em(&self) -> u16 {
        self.units_per_em.get()
    }

    /// Number of seconds since 12:00 midnight that started January 1st
    /// 1904 in GMT/UTC time zone.
    pub fn created(&self) -> LongDateTime {
        self.created.get()
    }

    /// Number of seconds since 12:00 midnight that started January 1st
    /// 1904 in GMT/UTC time zone.
    pub fn modified(&self) -> LongDateTime {
        self.modified.get()
    }

    /// Minimum x coordinate across all glyph bounding boxes.
    pub fn x_min(&self) -> i16 {
        self.x_min.get()
    }

    /// Minimum y coordinate across all glyph bounding boxes.
    pub fn y_min(&self) -> i16 {
        self.y_min.get()
    }

    /// Maximum x coordinate across all glyph bounding boxes.
    pub fn x_max(&self) -> i16 {
        self.x_max.get()
    }

    /// Maximum y coordinate across all glyph bounding boxes.
    pub fn y_max(&self) -> i16 {
        self.y_max.get()
    }

    /// see somewhere else
    pub fn mac_style(&self) -> u16 {
        self.mac_style.get()
    }

    /// Smallest readable size in pixels.
    pub fn lowest_rec_ppem(&self) -> u16 {
        self.lowest_rec_ppem.get()
    }

    /// Deprecated (Set to 2).
    pub fn font_direction_hint(&self) -> i16 {
        self.font_direction_hint.get()
    }

    /// 0 for short offsets (Offset16), 1 for long (Offset32).
    pub fn index_to_loc_format(&self) -> i16 {
        self.index_to_loc_format.get()
    }

    /// 0 for current format.
    pub fn glyph_data_format(&self) -> i16 {
        self.glyph_data_format.get()
    }
}
