# bad-apple-qr
Very weird (but somewhat functional) way to extract `Bad Apple!`'s lyrics.

It does that by scanning every frame of https://www.youtube.com/watch?v=tPYUdjptMpk as a QR code.

Uses QOI to speed up the process of decoding the images.

They are first extracted by `ffmpeg` and cropped by `convert`.

## Dependencies
- ffmpeg
- convert
- yt-dlp
- rust toolchain above version 1.56.0 (because of qoi)

## Usage
Run:

```bash
./extract-frames.sh
./convert-lyrics.sh
```

Example output included in the `bad-apple-lyrics-*.txt` files.
