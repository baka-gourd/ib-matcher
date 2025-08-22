//! C-compatible FFI bindings for ib-matcher

use std::ffi::{c_char, CStr};
use std::mem::ManuallyDrop;
use std::ptr;

#[cfg(feature = "romaji")]
use ib_matcher::matcher::RomajiMatchConfig;
use ib_matcher::matcher::{IbMatcher, Match, PinyinMatchConfig};
use ib_matcher::pinyin::PinyinNotation;
use widestring::U16Str;

// Opaque handle for IbMatcher
#[repr(C)]
pub struct IbMatcherHandle(*mut ManuallyDrop<IbMatcher<'static>>);
impl IbMatcherHandle {
    fn from_matcher(matcher: IbMatcher<'static>) -> Self {
        let boxed = Box::new(ManuallyDrop::new(matcher));
        Self(Box::into_raw(boxed))
    }

    fn as_ref(&self) -> Option<&IbMatcher<'static>> {
        if self.0.is_null() {
            None
        } else {
            unsafe { Some(&**self.0) }
        }
    }
}

// Match result structure for C FFI
#[repr(C)]
pub struct MatchResult {
    pub start: usize,
    pub end: usize,
    pub is_pattern_partial: bool,
    pub found: bool,
}

impl From<Option<Match>> for MatchResult {
    fn from(m: Option<Match>) -> Self {
        match m {
            Some(m) => MatchResult {
                start: m.start(),
                end: m.end(),
                is_pattern_partial: m.is_pattern_partial(),
                found: true,
            },
            None => MatchResult {
                start: 0,
                end: 0,
                is_pattern_partial: false,
                found: false,
            },
        }
    }
}

// Configuration struct for matcher creation
#[repr(C)]
#[derive(Clone)]
pub struct MatcherConfig {
    pub analyze: bool,
    pub is_pattern_partial: bool,
    pub starts_with: bool,
    pub ends_with: bool,
    pub case_insensitive: bool,
    pub mix_lang: bool,
    pub enable_pinyin: bool,
    pub pinyin_notations: u32,
    pub pinyin_case_insensitive: bool,
    pub enable_romaji: bool,
    pub romaji_case_insensitive: bool,
}

impl Default for MatcherConfig {
    fn default() -> Self {
        MatcherConfig {
            analyze: false,
            is_pattern_partial: false,
            starts_with: false,
            ends_with: false,
            case_insensitive: true,
            mix_lang: false,
            enable_pinyin: false,
            pinyin_notations: (PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter).bits(),
            pinyin_case_insensitive: true,
            enable_romaji: false,
            romaji_case_insensitive: true,
        }
    }
}

/// Create a new IbMatcher with UTF-8 pattern string
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_new(
    pattern: *const c_char,
    config: *const MatcherConfig,
) -> IbMatcherHandle {
    let pattern_str = if pattern.is_null() {
        ""
    } else {
        match CStr::from_ptr(pattern).to_str() {
            Ok(s) => s,
            Err(_) => return IbMatcherHandle(ptr::null_mut()),
        }
    };

    let config = if config.is_null() {
        MatcherConfig::default()
    } else {
        (*config).clone()
    };

    let pattern_str_static: &'static str = std::mem::transmute(pattern_str);

    // Static lifetime conversion is unsafe but necessary for FFI.
    // The matcher will be dropped properly through ib_matcher_free.
    create_matcher(pattern_str_static, &config)
}

/// Create a new IbMatcher with UTF-16 pattern string
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_new_utf16(
    pattern: *const u16,
    pattern_len: usize,
    config: *const MatcherConfig,
) -> IbMatcherHandle {
    if pattern.is_null() {
        return IbMatcherHandle(ptr::null_mut());
    }

    let pattern_str = match U16Str::from_ptr(pattern, pattern_len).to_string() {
        Ok(s) => s,
        Err(_) => return IbMatcherHandle(ptr::null_mut()),
    };

    let config = if config.is_null() {
        MatcherConfig::default()
    } else {
        (*config).clone()
    };

    // Static lifetime conversion is unsafe but necessary for FFI.
    // The matcher will be dropped properly through ib_matcher_free.
    let pattern_str_static: &'static str = std::mem::transmute(pattern_str.as_str());
    create_matcher(pattern_str_static, &config)
}

/// Helper function to create a matcher with the given config
fn create_matcher(pattern: &'static str, config: &MatcherConfig) -> IbMatcherHandle {
    // We need to build up the matcher in stages to avoid type issues with the builder pattern
    let builder = IbMatcher::builder(pattern)
        .analyze(config.analyze)
        .is_pattern_partial(config.is_pattern_partial)
        .starts_with(config.starts_with)
        .ends_with(config.ends_with)
        .case_insensitive(config.case_insensitive)
        .mix_lang(config.mix_lang);

    // Create the final matcher with the appropriate configuration
    let matcher = if config.enable_pinyin && config.enable_romaji {
        #[cfg(feature = "romaji")]
        {
            let pinyin_config = PinyinMatchConfig::builder(PinyinNotation::from_bits_truncate(
                config.pinyin_notations,
            ))
            .case_insensitive(config.pinyin_case_insensitive)
            .build();

            let romaji_config = RomajiMatchConfig::builder()
                .case_insensitive(config.romaji_case_insensitive)
                .build();

            builder.pinyin(pinyin_config).romaji(romaji_config).build()
        }
        #[cfg(not(feature = "romaji"))]
        {
            let pinyin_config = PinyinMatchConfig::builder(PinyinNotation::from_bits_truncate(
                config.pinyin_notations,
            ))
            .case_insensitive(config.pinyin_case_insensitive)
            .build();

            builder.pinyin(pinyin_config).build()
        }
    } else if config.enable_pinyin {
        let pinyin_config =
            PinyinMatchConfig::builder(PinyinNotation::from_bits_truncate(config.pinyin_notations))
                .case_insensitive(config.pinyin_case_insensitive)
                .build();

        builder.pinyin(pinyin_config).build()
    } else if config.enable_romaji {
        #[cfg(feature = "romaji")]
        {
            let romaji_config = RomajiMatchConfig::builder()
                .case_insensitive(config.romaji_case_insensitive)
                .build();

            builder.romaji(romaji_config).build()
        }
        #[cfg(not(feature = "romaji"))]
        {
            builder.build()
        }
    } else {
        builder.build()
    };

    IbMatcherHandle::from_matcher(matcher)
}

/// Free the IbMatcher instance
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_free(handle: IbMatcherHandle) {
    if !handle.0.is_null() {
        let _ = Box::from_raw(handle.0);
    }
}

/// Check if the pattern matches anywhere in the haystack (UTF-8)
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_is_match(
    handle: IbMatcherHandle,
    haystack: *const c_char,
) -> bool {
    let matcher = match handle.as_ref() {
        Some(m) => m,
        None => return false,
    };

    if haystack.is_null() {
        return false;
    }

    let haystack_str = match CStr::from_ptr(haystack).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    matcher.is_match(haystack_str)
}

/// Check if the pattern matches anywhere in the haystack (UTF-16)
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_is_match_utf16(
    handle: IbMatcherHandle,
    haystack: *const u16,
    haystack_len: usize,
) -> bool {
    let matcher = match handle.as_ref() {
        Some(m) => m,
        None => return false,
    };

    if haystack.is_null() {
        return false;
    }

    let haystack_str = match U16Str::from_ptr(haystack, haystack_len).to_string() {
        Ok(s) => s,
        Err(_) => return false,
    };

    matcher.is_match(haystack_str.as_str())
}

/// Find the first match in the haystack (UTF-8)
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_find(
    handle: IbMatcherHandle,
    haystack: *const c_char,
) -> MatchResult {
    let matcher = match handle.as_ref() {
        Some(m) => m,
        None => return MatchResult::from(None),
    };

    if haystack.is_null() {
        return MatchResult::from(None);
    }

    let haystack_str = match CStr::from_ptr(haystack).to_str() {
        Ok(s) => s,
        Err(_) => return MatchResult::from(None),
    };

    MatchResult::from(matcher.find(haystack_str))
}

/// Find the first match in the haystack (UTF-16)
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_find_utf16(
    handle: IbMatcherHandle,
    haystack: *const u16,
    haystack_len: usize,
) -> MatchResult {
    let matcher = match handle.as_ref() {
        Some(m) => m,
        None => return MatchResult::from(None),
    };

    if haystack.is_null() {
        return MatchResult::from(None);
    }

    let haystack_str = match U16Str::from_ptr(haystack, haystack_len).to_string() {
        Ok(s) => s,
        Err(_) => return MatchResult::from(None),
    };

    MatchResult::from(matcher.find(haystack_str.as_str()))
}

/// Test if the pattern matches at the start of the haystack (UTF-8)
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_test(
    handle: IbMatcherHandle,
    haystack: *const c_char,
) -> MatchResult {
    let matcher = match handle.as_ref() {
        Some(m) => m,
        None => return MatchResult::from(None),
    };

    if haystack.is_null() {
        return MatchResult::from(None);
    }

    let haystack_str = match CStr::from_ptr(haystack).to_str() {
        Ok(s) => s,
        Err(_) => return MatchResult::from(None),
    };

    MatchResult::from(matcher.test(haystack_str))
}

/// Test if the pattern matches at the start of the haystack (UTF-16)
#[no_mangle]
pub unsafe extern "C" fn ib_matcher_test_utf16(
    handle: IbMatcherHandle,
    haystack: *const u16,
    haystack_len: usize,
) -> MatchResult {
    let matcher = match handle.as_ref() {
        Some(m) => m,
        None => return MatchResult::from(None),
    };

    if haystack.is_null() {
        return MatchResult::from(None);
    }

    let haystack_str = match U16Str::from_ptr(haystack, haystack_len).to_string() {
        Ok(s) => s,
        Err(_) => return MatchResult::from(None),
    };

    MatchResult::from(matcher.test(haystack_str.as_str()))
}
