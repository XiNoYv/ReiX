// ReiX
// Copyright (C) 2024  XiNoYv
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::Read;
use std::io::Seek;

pub(crate) struct SliceFileReader {
    reader: std::io::BufReader<std::fs::File>,
    start: u64,
    end: u64,
    current: u64,
}

impl SliceFileReader {
    pub(crate) fn new_from_path(
        file_path: String,
        start: Option<i64>,
        end: Option<i64>,
    ) -> SliceFileReader {
        if let Some(file) = std::fs::File::open(&file_path).ok() {
            let file_size = file.metadata().unwrap().len();
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(file_size as i64);
            let start = (if start < 0 {
                file_size as i64 + start
            } else {
                start
            }) as u64;
            let end = (if end < 0 { file_size as i64 + end } else { end }) as u64;
            if start > end {
                panic!("Invalid start {} smaller than end {}", start, end);
            }
            if start > file_size || end > file_size {
                panic!(
                    "Invalid start {} and end {} positions outside the file size {}",
                    start, end, file_size
                );
            }
            SliceFileReader {
                reader: std::io::BufReader::new(file),
                start,
                end,
                current: start,
            }
        } else {
            panic!("Cannot open file '{}'", file_path);
        }
    }

    pub(crate) fn len(&self) -> u64 {
        self.end - self.start
    }

    pub(crate) fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let bytes_to_read = buffer.len() as u64;
        let bytes_left = self.end - self.current;
        if bytes_left == 0 {
            return Ok(0);
        }
        let bytes_to_read = if bytes_to_read > bytes_left {
            bytes_left
        } else {
            bytes_to_read
        };
        self.reader.seek(std::io::SeekFrom::Start(self.current))?;
        let bytes_read = self.reader.read(&mut buffer[..bytes_to_read as usize])?;
        self.current += bytes_read as u64;
        Ok(bytes_read)
    }
}
