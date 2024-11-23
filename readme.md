# org - Simple File Organizer

`org` is a lightweight, fast file organizer written in Rust that helps you keep your folders tidy by automatically organizing files based on their types.

## Features

- Quick and efficient file organization
- Zero configuration required
- Preserves original files
- Organizes files into meaningful categories
- Written in Rust for maximum performance and safety

## Installation

Make sure you have Rust installed on your system. Then:

```bash
git clone https://github.com/ash2228/org
cd org
cargo install --path .
```

## Usage

```bash
org [source_folder] [destination_folder]
```

### Examples

```bash
# Organize Downloads folder into a new organized directory
org ~/Downloads ~/Organized

# Organize current directory into a new 'sorted' directory
org . ./sorted
```

## File Organization Structure

Files are automatically sorted into the following directories:

- 📸 Images: .jpg, .jpeg, .png, .gif, .bmp, .svg, .webp
- 📄 Documents: .pdf, .doc, .docx, .txt, .rtf, .odt
- 🎵 Audio: .mp3, .wav, .flac, .m4a, .aac
- 🎬 Video: .mp4, .avi, .mkv, .mov, .wmv
- 📦 Archives: .zip, .rar, .7z, .tar, .gz
- 💻 Code: .rs, .js, .py, .cpp, .h, .java, .html, .css
- 📊 Data: .csv, .xlsx, .json, .xml
- Other: Everything else

## Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/org
```

2. Navigate to the project directory:
```bash
cd org
```

3. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/org`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Rust community for providing excellent documentation and crates
- All contributors who help improve this tool

## Note on File Safety

`org` never deletes your original files. It creates organized copies in the destination directory, ensuring your data remains safe.

## Roadmap

- [ ] Add custom organization rules
- [ ] Implement duplicate file detection
- [ ] Add undo operation
- [ ] Create interactive mode