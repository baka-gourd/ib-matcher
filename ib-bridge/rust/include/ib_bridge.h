#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct ManuallyDrop;

struct IbMatcherHandle {
  ManuallyDrop<IbMatcher> *_0;
};

struct MatcherConfig {
  bool analyze;
  bool is_pattern_partial;
  bool starts_with;
  bool ends_with;
  bool case_insensitive;
  bool mix_lang;
  bool enable_pinyin;
  uint32_t pinyin_notations;
  bool pinyin_case_insensitive;
  bool enable_romaji;
  bool romaji_case_insensitive;
};

struct MatchResult {
  uintptr_t start;
  uintptr_t end;
  bool is_pattern_partial;
  bool found;
};

extern "C" {

/// Create a new IbMatcher with UTF-8 pattern string
IbMatcherHandle ib_matcher_new(const char *pattern, const MatcherConfig *config);

/// Create a new IbMatcher with UTF-16 pattern string
IbMatcherHandle ib_matcher_new_utf16(const uint16_t *pattern,
                                     uintptr_t pattern_len,
                                     const MatcherConfig *config);

/// Free the IbMatcher instance
void ib_matcher_free(IbMatcherHandle handle);

/// Check if the pattern matches anywhere in the haystack (UTF-8)
bool ib_matcher_is_match(IbMatcherHandle handle, const char *haystack);

/// Check if the pattern matches anywhere in the haystack (UTF-16)
bool ib_matcher_is_match_utf16(IbMatcherHandle handle,
                               const uint16_t *haystack,
                               uintptr_t haystack_len);

/// Find the first match in the haystack (UTF-8)
MatchResult ib_matcher_find(IbMatcherHandle handle, const char *haystack);

/// Find the first match in the haystack (UTF-16)
MatchResult ib_matcher_find_utf16(IbMatcherHandle handle,
                                  const uint16_t *haystack,
                                  uintptr_t haystack_len);

/// Test if the pattern matches at the start of the haystack (UTF-8)
MatchResult ib_matcher_test(IbMatcherHandle handle, const char *haystack);

/// Test if the pattern matches at the start of the haystack (UTF-16)
MatchResult ib_matcher_test_utf16(IbMatcherHandle handle,
                                  const uint16_t *haystack,
                                  uintptr_t haystack_len);

} // extern "C"
