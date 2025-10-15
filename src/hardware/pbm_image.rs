use defmt::info;

/// Simple PBM (Portable Bitmap) image format support
/// Supports both P1 (ASCII) and P4 (binary) formats
pub struct PBMImage {
    width: u16,
    height: u16,
    pixels: heapless::Vec<bool, 8192>,
}

impl PBMImage {
    /// Create a new PBM image from static data
    pub fn new(data: &'static [u8]) -> Result<Self, PBMError> {
        let mut pos = 0;

        info!("Starting PBM parsing, data length: {}", data.len());

        // Skip initial whitespace
        pos = skip_whitespace_and_comments(data, pos);

        // Check for magic number (P1 or P4)
        if pos + 1 >= data.len() || data[pos] != b'P' {
            info!("Invalid PBM magic number - no P found at position {}", pos);
            if pos < data.len() {
                info!("Found character: '{}'", data[pos] as char);
            }
            return Err(PBMError::InvalidFormat);
        }
        pos += 1;

        let format = data[pos];
        if format != b'1' && format != b'4' {
            info!(
                "Invalid PBM format - must be P1 or P4, got P{}",
                format as char
            );
            return Err(PBMError::InvalidFormat);
        }
        pos += 1;

        info!("Found PBM format P{}", format as char);

        // Skip whitespace and comments after magic number
        pos = skip_whitespace_and_comments(data, pos);

        // Parse width
        let (width, new_pos) = parse_number(data, pos)?;
        pos = new_pos;
        info!("Parsed width: {}", width);

        // Skip whitespace and comments
        pos = skip_whitespace_and_comments(data, pos);

        // Parse height
        let (height, new_pos) = parse_number(data, pos)?;
        pos = new_pos;
        info!("Parsed height: {}", height);

        // Skip whitespace and comments before data
        pos = skip_whitespace_and_comments(data, pos);

        info!(
            "Parsed PBM image: {}x{}, data starts at byte {}",
            width, height, pos
        );

        // Validate image size
        let total_pixels = (width as usize) * (height as usize);
        if total_pixels > 8192 {
            info!("Image too large: {} pixels (max 8192)", total_pixels);
            return Err(PBMError::BufferFull);
        }

        // Parse pixel data based on format
        let mut pixels = heapless::Vec::new();

        if format == b'1' {
            // P1 format - ASCII
            Self::parse_p1_data(data, pos, width, height, &mut pixels)?;
        } else {
            // P4 format - binary
            Self::parse_p4_data(data, pos, width, height, &mut pixels)?;
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    /// Parse P1 (ASCII) format data
    fn parse_p1_data(
        data: &[u8],
        mut pos: usize,
        width: u16,
        height: u16,
        pixels: &mut heapless::Vec<bool, 8192>,
    ) -> Result<(), PBMError> {
        info!("Parsing P1 ASCII data starting at position {}", pos);

        let total_pixels = (width as usize) * (height as usize);

        for pixel_idx in 0..total_pixels {
            // Skip whitespace and comments
            pos = skip_whitespace_and_comments(data, pos);

            if pos >= data.len() {
                info!("Unexpected end of data in P1 format at pixel {}", pixel_idx);
                return Err(PBMError::InvalidFormat);
            }

            // Read pixel value
            let pixel_char = data[pos];
            let pixel_value = match pixel_char {
                b'0' => false,
                b'1' => true,
                b'2' => true, // Treat '2' as '1' for compatibility with some generators
                _ => {
                    info!(
                        "Invalid pixel value in P1 format: '{}' at position {}",
                        pixel_char as char, pos
                    );
                    return Err(PBMError::InvalidFormat);
                }
            };

            pixels.push(pixel_value).map_err(|_| PBMError::BufferFull)?;
            pos += 1;
        }

        info!("Successfully parsed {} pixels in P1 format", pixels.len());
        Ok(())
    }

    /// Parse P4 (binary) format data
    fn parse_p4_data(
        data: &[u8],
        pos: usize,
        width: u16,
        height: u16,
        pixels: &mut heapless::Vec<bool, 8192>,
    ) -> Result<(), PBMError> {
        info!("Parsing P4 binary data starting at position {}", pos);

        // Calculate bytes per row (each row is padded to byte boundary)
        let bits_per_row = width as usize;
        let bytes_per_row = (bits_per_row + 7) / 8;

        info!(
            "Bits per row: {}, bytes per row: {}",
            bits_per_row, bytes_per_row
        );

        for y in 0..height {
            for x in 0..width {
                let bit_index_in_row = x as usize;
                let byte_index_in_row = bit_index_in_row / 8;
                let bit_offset = 7 - (bit_index_in_row % 8);

                let byte_pos = pos + (y as usize * bytes_per_row) + byte_index_in_row;

                if byte_pos >= data.len() {
                    info!(
                        "Unexpected end of data in P4 format at byte {} (row {}, col {})",
                        byte_pos, y, x
                    );
                    info!(
                        "Data length: {}, expected at least: {}",
                        data.len(),
                        byte_pos + 1
                    );
                    return Err(PBMError::InvalidFormat);
                }

                let pixel_value = (data[byte_pos] >> bit_offset) & 1 == 1;
                pixels.push(pixel_value).map_err(|_| PBMError::BufferFull)?;
            }
        }

        info!("Successfully parsed {} pixels in P4 format", pixels.len());
        Ok(())
    }

    /// Get image width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get image height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get pixel value at specific coordinates (true = black/on, false = white/off)
    pub fn get_pixel(&self, x: u16, y: u16) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let index = (y as usize * self.width as usize) + x as usize;
        self.pixels.get(index).copied().unwrap_or(false)
    }

    /// Convert image to display buffer format (for SSD1306)
    /// Returns bytes organized in pages (8 rows per page)
    pub fn to_display_buffer(
        &self,
        target_width: u16,
        target_height: u16,
    ) -> Result<heapless::Vec<u8, 1024>, PBMError> {
        info!(
            "Converting {}x{} image to {}x{} display buffer",
            self.width, self.height, target_width, target_height
        );

        let mut buffer = heapless::Vec::new();

        // SSD1306 organizes pixels in pages of 8 rows each
        let pages = (target_height + 7) / 8;

        for page in 0..pages {
            for col in 0..target_width {
                let mut page_byte = 0u8;

                // Process 8 rows for this page
                for row_in_page in 0..8 {
                    let y = page * 8 + row_in_page;
                    if y < target_height {
                        // Scale coordinates to source image
                        let src_x = if self.width > 0 {
                            (col * self.width) / target_width
                        } else {
                            0
                        };
                        let src_y = if self.height > 0 {
                            (y * self.height) / target_height
                        } else {
                            0
                        };

                        if self.get_pixel(src_x, src_y) {
                            page_byte |= 1 << row_in_page;
                        }
                    }
                }

                buffer.push(page_byte).map_err(|_| PBMError::BufferFull)?;
            }
        }

        info!("Generated display buffer with {} bytes", buffer.len());
        Ok(buffer)
    }
}

/// Helper function to check if a byte is whitespace
fn is_whitespace(byte: u8) -> bool {
    byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r'
}

/// Skip whitespace and comments (lines starting with #)
fn skip_whitespace_and_comments(data: &[u8], mut pos: usize) -> usize {
    while pos < data.len() {
        if is_whitespace(data[pos]) {
            pos += 1;
        } else if data[pos] == b'#' {
            // Skip comment line
            while pos < data.len() && data[pos] != b'\n' && data[pos] != b'\r' {
                pos += 1;
            }
            // Skip the newline character(s)
            while pos < data.len() && (data[pos] == b'\n' || data[pos] == b'\r') {
                pos += 1;
            }
        } else {
            break;
        }
    }
    pos
}

/// Parse a number from the data starting at pos
fn parse_number(data: &[u8], pos: usize) -> Result<(u16, usize), PBMError> {
    let start_pos = pos;
    let mut end_pos = pos;

    // Find the end of the number
    while end_pos < data.len() && data[end_pos] >= b'0' && data[end_pos] <= b'9' {
        end_pos += 1;
    }

    if end_pos == start_pos {
        info!("No number found at position {}", pos);
        if pos < data.len() {
            info!("Found character: '{}'", data[pos] as char);
        }
        return Err(PBMError::InvalidFormat);
    }

    // Convert to string and parse
    let number_str = core::str::from_utf8(&data[start_pos..end_pos]).map_err(|_| {
        info!("Invalid UTF-8 in number at position {}", start_pos);
        PBMError::InvalidFormat
    })?;

    let number = number_str.parse::<u16>().map_err(|_| {
        info!(
            "Failed to parse number '{}' at position {}",
            number_str, start_pos
        );
        PBMError::InvalidFormat
    })?;

    info!("Parsed number: {} (from '{}')", number, number_str);
    Ok((number, end_pos))
}

/// PBM parsing errors
#[derive(Debug, Clone, Copy)]
pub enum PBMError {
    InvalidFormat,
    BufferFull,
}
