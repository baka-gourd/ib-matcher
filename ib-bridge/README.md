# IbBridge

C# bindings for [ib-matcher](https://github.com/Chaoses-Ib/ib-matcher) - a multilingual and fast string matcher, supporting Chinese pinyin and Japanese romaji matching.

## Features

- Full support for all ib-matcher capabilities
- .NET Framework 4.8 and .NET 6.0+ compatibility
- Proper memory management with IDisposable pattern
- Thread-safe operations
- Native performance through P/Invoke
- Customizable native DLL loading path
- Comprehensive pinyin notation support including:
  - Standard ASCII pinyin (全拼)
  - ASCII first letter (简拼)
  - ASCII with tone numbers
  - Unicode with tone marks
  - Multiple double-pinyin systems (双拼):
    - 智能 ABC 双拼
    - 拼音加加双拼
    - 微软双拼
    - 华宇双拼（紫光双拼）
    - 小鹤双拼
    - 自然码双拼

## Usage

### Basic Pinyin Matching

```csharp
// Create a matcher with pinyin matching enabled
using (var matcher = IbMatcher.CreatePinyinMatcher("pysousuoeve"))
{
    string haystack = "拼音搜索Everything";
    bool isMatch = matcher.IsMatch(haystack);
    Console.WriteLine($"Match result: {isMatch}"); // True
    
    var match = matcher.Find(haystack);
    if (match.HasValue)
    {
        Console.WriteLine($"Match position: [{match.Value.Start}..{match.Value.End}]");
        Console.WriteLine($"Matched text: \"{match.Value.GetMatchedText(haystack)}\"");
    }
}
```

### Romaji Matching with Partial Pattern

```csharp
var config = IbMatcherConfig.WithRomaji();
config.IsPatternPartial = true;

using (var matcher = new IbMatcher("konosuba", config))
{
    string haystack = "この素晴らしい世界に祝福を";
    bool isMatch = matcher.IsMatch(haystack);
    Console.WriteLine($"Match result: {isMatch}"); // True
    
    var match = matcher.Find(haystack);
    if (match.HasValue)
    {
        Console.WriteLine($"Matched text: \"{match.Value.GetMatchedText(haystack)}\"");
        Console.WriteLine($"Is pattern partial: {match.Value.IsPatternPartial}");
    }
}
```

### Testing Matches at Start of String

```csharp
using (var matcher = IbMatcher.CreatePinyinMatcher("pin"))
{
    string haystack = "拼音输入法";
    var match = matcher.Test(haystack);
    if (match.HasValue)
    {
        Console.WriteLine($"Match found at the start");
        Console.WriteLine($"Matched text: \"{match.Value.GetMatchedText(haystack)}\"");
    }
}
```

### Using StartsWith and EndsWith Constraints

```csharp
var config = new IbMatcherConfig
{
    EnablePinyin = true,
    PinyinNotations = PinyinNotation.Common,
    StartsWith = true  // Only match if the haystack starts with the pattern
};

using (var matcher = new IbMatcher("pin", config))
{
    bool isMatch1 = matcher.IsMatch("拼音输入法"); // True
    bool isMatch2 = matcher.IsMatch("输入拼音");   // False
}
```

### Mixed Language Matching

```csharp
using (var matcher = IbMatcher.CreateMultilingualMatcher("hatsuneodxyy"))
{
    string haystack = "初音殴打喜羊羊";
    var match = matcher.Find(haystack);
    if (match.HasValue)
    {
        Console.WriteLine($"Matched text: \"{match.Value.GetMatchedText(haystack)}\"");
    }
}
```

## Installation

Reference the IbBridge library in your project and ensure the native ib_bridge.dll is available in your output directory.

### Custom DLL Path

You can specify a custom path to the native library:

```csharp
// Set this before creating any IbMatcher instances
IbBridgeConfig.DllPath = @"C:\path\to\your\ib_bridge.dll";
```

## Building from Source

### Requirements

- .NET SDK 6.0 or later
- Rust toolchain (for building the native library)

### Build Steps

1. Build the native library:
   ```
   cd ib-bridge/rust
   cargo build --release
   ```

2. Build the C# library:
   ```
   cd ib-bridge/csharp/IbBridge
   dotnet build
   ```

## License

This project is licensed under the same license as [ib-matcher](https://github.com/Chaoses-Ib/ib-matcher).
