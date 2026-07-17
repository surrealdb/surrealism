# surrealism-image

Image manipulation functions for Surrealism. Images are passed and returned as raw
`bytes`. Built on the [`image`](https://docs.rs/image) crate (pure Rust codecs: PNG,
JPEG, GIF, BMP, ICO, TIFF, WebP-decode).

```surql
DEFINE MODULE mod::image AS f"bucket:/image.surli";

LET $thumb = mod::image::thumbnail($photo, 200);
LET $bw = mod::image::grayscale($photo);
LET $jpg = mod::image::convert($photo, "jpeg");
```

| Function | Signature | Description |
|---|---|---|
| `resize` | `(data: bytes, width: int, height: int) -> bytes` | Resizes to exactly `width` x `height` |
| `thumbnail` | `(data: bytes, max_size: int) -> bytes` | Fits within `max_size` x `max_size`, preserving aspect ratio |
| `grayscale` | `(data: bytes) -> bytes` | Converts to black and white |
| `convert` | `(data: bytes, format: string) -> bytes` | Re-encodes to another format (`"png"`, `"jpeg"`, `"gif"`, `"bmp"`, `"ico"`, `"tiff"`, `"webp"`) |
| `saturate` | `(data: bytes, amount: float) -> bytes` | Scales color saturation (`1.0` = unchanged, `0.0` = grayscale) |
| `rotate` | `(data: bytes, degrees: int) -> bytes` | Rotates clockwise by 0/90/180/270 |
| `flip_horizontal` / `flip_vertical` | `(data: bytes) -> bytes` | Mirrors the image |
| `blur` | `(data: bytes, sigma: float) -> bytes` | Gaussian blur |
| `crop` | `(data: bytes, x: int, y: int, width: int, height: int) -> bytes` | Crops a region |
| `invert` | `(data: bytes) -> bytes` | Inverts colors |
| `brightness` | `(data: bytes, value: int) -> bytes` | Adjusts brightness by `value` (-255..=255) |
| `contrast` | `(data: bytes, value: float) -> bytes` | Adjusts contrast |
| `dimensions` | `(data: bytes) -> [int, int]` | Returns `(width, height)` in pixels |

All functions except `convert` preserve the input image's original format.
