using System;
using System.IO;

namespace IbBridge
{
    /// <summary>
    /// Configuration settings for the IbBridge library.
    /// </summary>
    public static class IbBridgeConfig
    {
        private static string? _dllPath;

        /// <summary>
        /// Gets or sets the path to the native ib_bridge.dll.
        /// If not set, the library will look for the DLL in the default locations.
        /// </summary>
        /// <remarks>
        /// Must be set before any IbMatcher instances are created.
        /// Setting this path after any IbMatcher has been created will have no effect.
        /// </remarks>
        public static string? DllPath
        {
            get { return _dllPath; }
            set
            {
                if (value != null && !File.Exists(value))
                {
                    throw new FileNotFoundException($"The specified DLL file does not exist: {value}");
                }
                _dllPath = value;
            }
        }
    }
}
