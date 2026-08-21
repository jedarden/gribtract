# Buffer Underrun Code Path Analysis

## Summary

Located the exact buffer underrun code path in `decode.rs` that produces the "need 4, got 0" error when validating the GRIB2 "7777" end marker.

## Exact Location

**File:** `crates/gribtract-core/src/decode.rs`  
**Function:** `decode_message` (starting at line 234)  
**Error Line:** 436  

### The Failing Code

```rust
// Line 436 - Buffer underrun occurs here
let end = &msg[body_end..body_end + 4];

// Line 438 - Misleading error message (hardcoded got: 0)
if end != b"7777" {
    return Err(Error::TooShort { needed: 4, got: 0 });
}
```

## Call Stack Path

```
Entry Point:
  decode_bytes (line 212)
    ├─> Loop: while pos < bytes.len()
    │    └─> decode_message(&bytes[pos..], &mut fields) (line 225)
    │         ├─> Parse Section 0 (Indicator) - get total_len
    │         ├─> Parse Sections 1-7 (iterate until body_end)
    │         └─> body_end = total_len - 4 (line 274)
    │              └─> BUFFER UNDERRUN: &msg[body_end..body_end + 4] (line 436)
    │                   └─> Return Error::TooShort { needed: 4, got: 0 } (line 438)
```

## Why "need 4, got 0"?

The error message is **misleading by design**:

1. **Line 436** creates a slice `msg[body_end..body_end + 4]` without bounds checking
2. If `body_end + 4 > msg.len()`, this slice goes **out of bounds**
3. **Line 438** hardcodes `got: 0` instead of reporting the actual available bytes

The actual available bytes would be `msg.len() - body_end`, not 0.

## Root Cause

**Missing bounds validation** before attempting the slice operation.

### Current Code (UNSAFE)
```rust
// No check that body_end + 4 <= msg.len()
let end = &msg[body_end..body_end + 4];
```

### Should Be
```rust
if body_end + 4 > msg.len() {
    return Err(Error::TooShort { 
        needed: body_end + 4, 
        got: msg.len() 
    });
}
let end = &msg[body_end..body_end + 4];
```

## Duplicate Bug

The **same bug exists** in `decode_lazy_message` function:

**Location:** Lines 2116-2119  
**Same pattern:**
```rust
let end = &msg[body_end..body_end + 4];
if end != b"7777" {
    return Err(Error::TooShort { needed: 4, got: 0 });
}
```

## Buffer Read Operation Details

### What the code is trying to do:
Validate the GRIB2 message ends with the 4-byte end marker "7777" (ASCII 0x37 0x37 0x37 0x37).

### When the underrun occurs:
- After parsing all sections 1-7
- At position `body_end = total_len - 4` (4 bytes before message end)
- When `body_end + 4 > msg.len()` (message is shorter than declared)

### Why the earlier validation doesn't catch this:
Line 263-268 validates `total_len <= msg.len()`:
```rust
if msg.len() < total_len {
    return Err(Error::TooShort {
        needed: total_len,
        got: msg.len(),
    });
}
```

However, if section parsing errors cause `buf.pos` to be incorrect, or if `total_len` itself is corrupted, the later `body_end` calculation can still be wrong.

## Related Code

The `Buf::need` method (lines 34-49) properly checks bounds:
```rust
fn need(&self, n: usize) -> Result<()> {
    if self.remaining() < n {
        Err(Error::TooShort {
            needed: n,
            got: self.remaining(),  // ← Reports ACTUAL remaining bytes
        })
    } else {
        Ok(())
    }
}
```

This shows the correct pattern: report the **actual** remaining bytes, not a hardcoded 0.

## Impact

- **Affects:** Any GRIB2 file where the end marker is missing or the message is truncated
- **Two locations:** Both `decode_message` and `decode_lazy_message` have the bug
- **Error quality:** Misleading error message obscures the real problem (out-of-bounds access)

## Next Steps

1. Fix both locations with proper bounds checking
2. Report actual available bytes instead of hardcoded 0
3. Consider adding debug assertions to catch unsafe slices in debug builds
