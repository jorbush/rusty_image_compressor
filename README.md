# Rusty Image Compressor

This is a simple image compressor written in Rust.

## Usage

To compress an image, use the following command:

```bash
cargo run --release -- --input <input> -- format <format>
```

Where:
```
<input> - the path to the input image
<format> - the format of the output image (png, jpg, webp)
```

## Testing

To run the tests, use the following command:

```bash
cargo test -- --test-threads=1
```

## Formatting

To format the code, use the following command:

```bash
cargo fmt
```
