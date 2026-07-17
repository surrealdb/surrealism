# surrealism-color

Color conversion, manipulation, and WCAG accessibility functions for Surrealism.
Zero dependencies beyond the Surrealism SDK itself.

```surql
DEFINE MODULE mod::color AS f"bucket:/color.surli";

RETURN mod::color::hex_to_rgb("#3366ff");        -- [51, 102, 255]
RETURN mod::color::lighten("#3366ff", 20.0);     -- '#99b3ff'
RETURN mod::color::contrast_ratio("#000", "#fff"); -- 21f
```

| Function | Signature | Description |
|---|---|---|
| `hex_to_rgb` | `(hex: string) -> [int, int, int]` | Parses a hex color into `(r, g, b)` |
| `rgb_to_hex` | `(r: int, g: int, b: int) -> string` | Formats channels as `#rrggbb` |
| `rgb_to_hsl` | `(r: int, g: int, b: int) -> [float, float, float]` | RGB to `(h, s, l)` |
| `hsl_to_rgb` | `(h: float, s: float, l: float) -> [int, int, int]` | HSL to `(r, g, b)` |
| `lighten` | `(hex: string, amount: float) -> string` | Adds `amount` to lightness |
| `darken` | `(hex: string, amount: float) -> string` | Subtracts `amount` from lightness |
| `luminance` | `(hex: string) -> float` | WCAG 2.x relative luminance |
| `contrast_ratio` | `(hex_a: string, hex_b: string) -> float` | WCAG 2.x contrast ratio |
| `is_light` | `(hex: string) -> bool` | Whether the color reads as "light" |
| `invert` | `(hex: string) -> string` | Inverts each channel |
| `complementary` | `(hex: string) -> string` | Hue rotated 180 degrees |

Note: `lighten`/`darken`/`amount` and HSL values are `float` arguments — pass e.g.
`20.0`, not `20`, from SurrealQL/the CLI.
