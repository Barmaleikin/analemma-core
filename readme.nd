# analemma — architecture-neutral balanced-ternary core

Low-level library of balanced-ternary (−1, 0, +1) arithmetic and data
structures for binary machines. It is the numerical foundation of the
Setun-70 emulator project, but carries **no architecture knowledge**:
no instruction sets, no punched-tape frames, no charsets. Those live in
architecture crates built on top of this core.

## Word width

Chosen at build time via `ANALEMMA_TRITS` (default `6`):

| Width | Values | Range | Storage |
|-------|--------|-------|---------|
| 3  | 27       | ±13          | `static table::TABLE` |
| 6  | 729      | ±364         | `static table::TABLE` |
| 9  | 19 683   | ±9 841       | `static table::TABLE` |
| 18 | 387 420 489 | ±193 710 244 | composite over `table::table6`, no static table |

```bash
cargo build                       # 6 trits (default)
ANALEMMA_TRITS=9 cargo build      # 9 trits
ANALEMMA_TRITS=18 cargo build     # 18 trits (composite, tryte-based)
```

For 3/6/9 the whole substitution table is generated at compile time.
For 18 a static table is impossible (387M entries), so `table::entry_by_value`
composes three lookups into the internal 6-trit (tryte) table — the same
way the historical machine assembled a word of three trytes.

## String representations (debugging)

Every `table::Entry` (3/6/9 trits) always carries pre-rendered
`&'static str` fields — the primary debugging instrument:

| Field | Example (−159, 6 trits) |
|-------|--------------------------|
| `str_trits` | `"ZPZZPN"` (least significant first) |
| `str_nonary` | `"Y03"` (most significant digit first) |
| `str_abs_binary` | `"10011111"` |
| `str_signed_binary` | `"11111111111111111111111101100001"` (32-bit two's complement) |
| `str_abs_decimal` / `str_signed_decimal` | `"159"` / `"-159"` |
| `str_abs_hex` | `"9F"` |
| `str_signed_hex` | `"FFFFFF61"` (32-bit two's complement) |

For 18 trits no static table exists, so `Entry` implements `Display`
composed on demand, and per-tryte strings are reachable via
`entry.trytes.{0,1,2}`:

```text
Entry18(value=-159, trits=NPZZPZ, nonary=000000Y03, hex=FFFFFF61)
```

## API sketch

```rust
use analemma::table;

let e = table::entry_by_value(-159).unwrap();
assert_eq!(e.trits.map(|t| t.to_i8()), [0, 1, 0, 0, 1, -1]); // "ZPZZPN"
assert_eq!(e.nonary_digits, [-2, 0, 3]);                      // X, 0, 3
assert_eq!(e.str_nonary, "Y03");
assert_eq!(e.str_signed_hex, "FFFFFF61");
let back = table::entry_by_trits(&e.trits).unwrap();
assert_eq!(back.value, -159);
```

Plain numeric types (`Trit`, `Tryte` = 6 trits, `Word` = 18 trits) and
sparse `TernaryArray`/`TernaryArray2D`/`3D`/`4D` with negative indices
are available independently of the table width.

## Migration from 0.1.x

- `tryte_table` is deprecated; use `table` (`Entry`, `entry_by_value`,
  `entry_by_trits`, `TABLE`, `TABLE_SIZE`, `OFFSET`, `TRITS`).
  The old module remains as a façade and will be removed in a later release.
- Numeric fields are `i32`/`u32` now (was `i16`/`u16`) — 9-trit range.
- `TryteEntry::mnemonic/op_code/op_type/is_op_syllable/punch_frame` are
  gone from the core. ISA decoding, tape and charset support are being
  developed as architecture crates (`analemma-setun70`, `analemma-setun58`).
- `ISA_NAME` is a compatibility stub equal to `"none"`.
- Signed string representations are 32-bit two's complement (was 16-bit).

## Migration from 0.2.x

- The `fmt` feature is gone: string fields are generated unconditionally
  for 3/6/9 (they are the main debugging tool); 18-trit entries expose
  strings via `Display` and `entry.trytes.*`.

## Roadmap

- [ ] Packed `u16` tryte storage option (`code = value + 364`).
- [ ] `no_std` support (sparse arrays behind `std` feature).
- [ ] `analemma-setun70`: 81-op decoder (27 bas + 27 spec + macro), 7-track tape.
- [ ] `analemma-setun58`: 9-trit commands, 5-track tape, shift-register charset.
- [ ] Carry/addition lookup tables generated alongside the value table.
