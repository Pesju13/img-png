use crate::error::Error;

use crate::chunk::FromBytes;

/// # Header Chunk
/// - Chunk Type: IHDR
/// - Description: Contains basic information about the image, such as `width`, `height`, `bit depth`,
/// `color type`, `compression method`, `filter method`, and `interlace method`.
///
/// - ## Width (4 bytes):
///     - Represents the image's width in pixels. Stored as a 32-bit unsigned integer in network byte order (Big-Endian).
/// - ## Height (4 bytes):
///     - Indicates the image's height in pixels. Also stored as a 32-bit unsigned integer in network byte order.
/// - ## Bit Depth (1 byte):
///     - Specifies the number of bits used for each pixel's color value. Possible values are 1, 2, 4, 8, and 16.
/// - ## Color Type (1 byte):
///     - Defines the color model of the image, indicating the combination of color channels. Possible values include:
///         - 0: Grayscale image with grayscale values.
///         - 2: Truecolor image with red, green, and blue channels.
///         - 3: Indexed color image using a palette.
///         - 4: Grayscale image with transparency.
///         - 6: Truecolor image with transparency.
/// - ## Compression Method (1 byte):
///     - Specifies the compression method for image data. Currently defined as 0, indicating deflate/inflate compression.
/// - ## Filter Method (1 byte):
///     - Indicates the filtering method applied to image data. Currently defined as 0, representing adaptive filtering.
/// - ## Interlace Method (1 byte):
///     - Describes the interlacing method for image data, determining the scanning order. Possible values are 0 (non-interlaced) and 1 (Adam7 interlacing).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IHDR {
    /// Represents the image's width in pixels. Stored as a 32-bit unsigned integer in network byte order (Big-Endian).
    pub width: u32,
    /// Indicates the image's height in pixels. Also stored as a 32-bit unsigned integer in network byte order.
    pub height: u32,
    /// Specifies the number of bits used for each pixel's color value. Possible values are 1, 2, 4, 8, and 16.
    pub bit_depth: u8,
    /// Defines the color model of the image, indicating the combination of color channels. Possible values include:
    /// - 0: Grayscale image with grayscale values.
    /// - 2: Truecolor image with red, green, and blue channels.
    /// - 3: Indexed color image using a palette.
    /// - 4: Grayscale image with transparency.
    /// - 6: Truecolor image with transparency.
    pub color_type: u8,
    ///     Specifies the compression method for image data. Currently defined as 0, indicating deflate/inflate compression.
    pub compression_method: u8,
    /// Indicates the filtering method applied to image data. Currently defined as 0, representing adaptive filtering.
    pub filter_method: u8,
    ///  Describes the interlacing method for image data, determining the scanning order.
    /// Possible values are 0 (non-interlaced) and 1 (Adam7 interlacing).
    pub interlace_method: u8,
}
impl FromBytes for IHDR {
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        Self {
            width: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            height: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            bit_depth: bytes[8],
            color_type: bytes[9],
            compression_method: bytes[10],
            filter_method: bytes[11],
            interlace_method: bytes[12],
        }
    }

    fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        if bytes.len() != 13 {
            return Err(Error::InvalidLength);
        }
        Ok(unsafe { Self::from_bytes_unchecked(bytes) })
    }
}
