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

mod slice_file_reader;
use slice_file_reader::SliceFileReader;

fn main() {
    let mut file_path: String = String::new();
    let mut range_str: String = String::new();

    let (display_help, display_line_numbers, display_ascii, display_inline) =
        parse_args(std::env::args(), &mut file_path, &mut range_str);

    let (start, end) = parse_range(&range_str);
    if display_help || file_path.is_empty() {
        help();
        return;
    }

    let reader = SliceFileReader::new_from_path(file_path, start, end);
    ReiX(reader, display_line_numbers, display_ascii, display_inline);
}

fn parse_args(
    env_args: std::env::Args,
    file_path: &mut String,
    range: &mut String,
) -> (bool, bool, bool, bool) {
    let mut display_help = false;
    let mut display_line_numbers = true;
    let mut display_ascii = true;
    let mut display_inline = false;
    let mut options_flag = true;
    let mut arg_iter = env_args.skip(1);
    while let Some(argument) = arg_iter.next() {
        if options_flag && argument.starts_with('-') {
            let options: Vec<char> = argument.chars().collect();
            if options.contains(&'h') {
                display_help = true;
            }
            if options.contains(&'n') {
                display_line_numbers = false;
            }
            if options.contains(&'c') {
                display_ascii = false;
            }
            if options.contains(&'i') {
                display_inline = true;
            }
            continue;
        }
        if file_path.is_empty() {
            *file_path = argument;
            options_flag = false;
        } else {
            *range = argument;
        }
    }
    (
        display_help,
        display_line_numbers,
        display_ascii,
        display_inline,
    )
}

fn help() {
    println!("ReiX  Copyright (C) 2024  XiNoYv");
    println!("This program comes with ABSOLUTELY NO WARRANTY; for details type `show w`.");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions; type `show c` for details.");
    println!();
    println!("Usage: ReiX_cli [options] <file> [start:end | pos]");
    println!();
    println!("Options:");
    println!("  -h  Show help message");
    println!("  -n  Don't display line numbers");
    println!("  -c  Don't display ASCII characters");
    println!("  -i  Display hex bytes in one line (will set -n and -c to true)");
    println!();
    println!("Arguments:");
    println!("  <file>  The path to the file to be viewed.");
    println!("  [start:end]  The range of bytes to be displayed, end is exclusive.");
    println!("    > Both `start` and `end` can be empty or negative.");
    println!("      > - If `start` is empty, it will be set to 0.");
    println!("      > - If `end` is empty, it will be set to the end of the file.");
    println!("      > - If `start` or `end` is negative, it will be counted from the end of the file.");
    println!("  [pos]  The position of the byte to be displayed.");
}

fn parse_range(range: &str) -> (Option<i64>, Option<i64>) {
    return if range.is_empty() {
        (None, None)
    } else {
        return if !range.contains(':') {
            let tmp: i64 = range.parse().unwrap();
            if tmp == -1 {
                return (Some(tmp), None);
            }
            return (Some(tmp), Some(tmp + 1));
        } else {
            let parts: Vec<&str> = range.split(':').collect();
            let start: Option<i64> = if parts[0].is_empty() {
                None
            } else {
                parts[0].parse().ok()
            };
            let end: Option<i64> = if parts[1].is_empty() {
                None
            } else {
                parts[1].parse().ok()
            };
            (start, end)
        };
    };
}

#[allow(non_snake_case)]
fn ReiX(
    mut reader: SliceFileReader,
    display_line_numbers: bool,
    display_ascii: bool,
    display_inline: bool,
) {
    if display_inline {
        ReiX_inline(reader);
        return;
    }

    const LINE_LENGTH: u8 = 16;
    const CHUNK_SIZE: u64 = 16;
    let mut buffer = [0; LINE_LENGTH as usize];
    let mut line_number: u64 = 0;
    let mut output = String::new();
    let file_length = reader.len();
    let max_line_number = file_length / LINE_LENGTH as u64;
    let line_number_width = format!("{:x}", max_line_number).len();
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if display_line_numbers {
            output += &format!("0x{:01$X}  ", line_number, line_number_width);
        }

        output += &buffer[0..bytes_read]
            .iter()
            .map(|&byte| format!("{:02X} ", byte))
            .collect::<String>();

        if bytes_read < LINE_LENGTH as usize {
            let spaces = "   ".repeat(LINE_LENGTH as usize - bytes_read);
            output += &spaces;
        }

        if display_ascii {
            output += " ";

            for i in 0..bytes_read {
                if buffer[i].is_ascii() && !buffer[i].is_ascii_control() {
                    output += &format!("{}", char::from(buffer[i]));
                } else {
                    output += ".";
                }
            }
        }

        output += "\n";

        line_number += 1;

        if line_number % CHUNK_SIZE == 0 {
            print!("{}", output);
            output.clear();
        }
    }
    print!("{}", output);
}

#[allow(non_snake_case)]
fn ReiX_inline(mut reader: SliceFileReader) {
    const CHUNK_SIZE: u64 = 1024;
    let mut buffer = [0; CHUNK_SIZE as usize];
    let mut output = String::new();
    let mut total_bytes_read = 0;
    let length = reader.len();

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        total_bytes_read += bytes_read as u64;
        if total_bytes_read >= length {
            let bytes_to_read = length - (total_bytes_read - bytes_read as u64);
            output += &buffer[0..bytes_to_read as usize]
                .iter()
                .map(|&byte| format!("{:02X}", byte))
                .collect::<String>();
            print!("{}", output);
            break;
        }
        output += &buffer[0..bytes_read]
            .iter()
            .map(|&byte| format!("{:02X}", byte))
            .collect::<String>();
        print!("{}", output);
        output.clear();
    }
}
