//! The [os2](https://docs.microsoft.com/en-us/typography/opentype/spec/os2) table

use font_types::Tag;

/// 'OS/2'
pub const TAG: Tag = Tag::new(b"OS/2");

include!("../../generated/generated_os2.rs");

#[cfg(test)]
mod tests {
    use crate::test_data;

    #[test]
    fn read_sample() {
        let table = test_data::os2::sample();
        assert_eq!(table.version(), 4);
    }
}
