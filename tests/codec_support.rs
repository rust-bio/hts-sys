//! Checks that the optional compression codecs selected by cargo features are compiled into the
//! bundled htslib, and that codecs whose features are disabled are not.

use hts_sys::{hts_features, HTS_FEATURE_BZIP2, HTS_FEATURE_LZMA};

/// Returns true if htslib reports `feature` (one of the `HTS_FEATURE_*` bits) as compiled in.
fn htslib_has_feature(feature: u32) -> bool {
    unsafe { hts_features() & feature != 0 }
}

#[test]
#[cfg(feature = "bzip2")]
fn bzip2_support_is_compiled_in_when_feature_enabled() {
    assert!(htslib_has_feature(HTS_FEATURE_BZIP2));
}

#[test]
#[cfg(not(feature = "bzip2"))]
fn bzip2_support_is_absent_when_feature_disabled() {
    assert!(!htslib_has_feature(HTS_FEATURE_BZIP2));
}

#[test]
#[cfg(feature = "lzma")]
fn lzma_support_is_compiled_in_when_feature_enabled() {
    assert!(htslib_has_feature(HTS_FEATURE_LZMA));
}

#[test]
#[cfg(not(feature = "lzma"))]
fn lzma_support_is_absent_when_feature_disabled() {
    assert!(!htslib_has_feature(HTS_FEATURE_LZMA));
}

/// htslib can only identify the contents of an xz-compressed file by decompressing the start of
/// it with liblzma, so this exercises a real lzma decode rather than just the feature flag.
#[test]
#[cfg(feature = "lzma")]
fn xz_compressed_sam_is_detected_as_sam() {
    use hts_sys::{
        hclose, hopen, htsCompression_xz_compression, htsExactFormat_sam, htsFormat,
        hts_detect_format2,
    };
    use std::ffi::CString;

    let sam_header = b"@HD\tVN:1.6\tSO:unsorted\n@SQ\tSN:chr1\tLN:100\n";
    let path = std::env::temp_dir().join(format!(
        "hts-sys-codec-support-{}.sam.xz",
        std::process::id()
    ));
    std::fs::write(&path, xz_compress(sam_header)).unwrap();

    let c_path = CString::new(path.to_str().unwrap()).unwrap();
    let mode = CString::new("r").unwrap();
    let mut format: htsFormat = unsafe { std::mem::zeroed() };
    let detect_result = unsafe {
        let fp = hopen(c_path.as_ptr(), mode.as_ptr());
        assert!(!fp.is_null(), "failed to open {}", path.display());
        let detect_result = hts_detect_format2(fp, c_path.as_ptr(), &mut format);
        hclose(fp);
        detect_result
    };
    std::fs::remove_file(&path).unwrap();

    assert_eq!(detect_result, 0);
    assert_eq!(format.compression, htsCompression_xz_compression);
    assert_eq!(format.format, htsExactFormat_sam);
}

/// Compresses `data` into a complete xz stream using liblzma from lzma-sys.
#[cfg(feature = "lzma")]
fn xz_compress(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; data.len() + 1024];
    let mut out_pos = 0;
    let result = unsafe {
        lzma_sys::lzma_easy_buffer_encode(
            6,
            lzma_sys::LZMA_CHECK_CRC32,
            std::ptr::null(),
            data.as_ptr(),
            data.len(),
            out.as_mut_ptr(),
            &mut out_pos,
            out.len(),
        )
    };
    assert_eq!(result, lzma_sys::LZMA_OK);
    out.truncate(out_pos);
    out
}
