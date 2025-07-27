#![allow(unsafe_op_in_unsafe_fn)]

const BASE: u64 = 62;
const BASE_TO_10: u128 =
    (BASE * BASE * BASE * BASE * BASE * BASE * BASE * BASE * BASE * BASE) as u128;

const BASE62_LEN: usize = 22;

struct Base62Tables {
    // standard: [u8; 62],
    // alternative: [u8; 62],
    // decode_standard: [u8; 256],
    // decode_alternative: [u8; 256],
}

impl Base62Tables {
    // Standard encoding table (0-9A-Za-z)
    const STANDARD: [u8; 62] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E',
        b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
        b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
        b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
        b'y', b'z',
    ];

    // Alternative encoding table (0-9a-zA-Z)
    // const ALTERNATIVE: [u8; 62] = [
    //     b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e',
    //     b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
    //     b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I',
    //     b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
    //     b'Y', b'Z',
    // ];

    // const fn new() -> Self {
    //     let mut decode_standard = [255u8; 256];
    //     let mut decode_alternative = [255u8; 256];

    //     // Populate standard decoding table
    //     let mut i = 0u8;
    //     while i < 10 {
    //         decode_standard[(b'0' + i) as usize] = i;
    //         i += 1;
    //     }
    //     let mut i = 0u8;
    //     while i < 26 {
    //         decode_standard[(b'A' + i) as usize] = i + 10;
    //         i += 1;
    //     }
    //     let mut i = 0u8;
    //     while i < 26 {
    //         decode_standard[(b'a' + i) as usize] = i + 36;
    //         i += 1;
    //     }

    //     // Populate alternative decoding table
    //     let mut i = 0u8;
    //     while i < 10 {
    //         decode_alternative[(b'0' + i) as usize] = i;
    //         i += 1;
    //     }
    //     let mut i = 0u8;
    //     while i < 26 {
    //         decode_alternative[(b'a' + i) as usize] = i + 10;
    //         i += 1;
    //     }
    //     let mut i = 0u8;
    //     while i < 26 {
    //         decode_alternative[(b'A' + i) as usize] = i + 36;
    //         i += 1;
    //     }

    //     Self {
    //         standard: Self::STANDARD,
    //         alternative: Self::ALTERNATIVE,
    //         decode_standard,
    //         decode_alternative,
    //     }
    // }
}

// static TABLES: Base62Tables = Base62Tables::new();

// Common encoding function
#[inline(always)]
unsafe fn encode_impl(mut num: u128, buf: &mut [u8; BASE62_LEN], encode_table: &[u8; 62]) {
    let mut write_idx = BASE62_LEN;
    let mut digit_index = 0_usize;
    let mut u64_num = (num % BASE_TO_10) as u64;
    num /= BASE_TO_10;

    while digit_index < BASE62_LEN {
        write_idx = write_idx.wrapping_sub(1);
        *buf.get_unchecked_mut(write_idx) = *encode_table.get_unchecked((u64_num % BASE) as usize);

        digit_index = digit_index.wrapping_add(1);
        match digit_index {
            10 => {
                u64_num = (num % BASE_TO_10) as u64;
                num /= BASE_TO_10;
            }
            20 => u64_num = num as u64,
            _ => u64_num /= BASE,
        }
    }
}

#[inline(always)]
unsafe fn _encode_buf(num: u128, buf: &mut [u8; BASE62_LEN]) {
    encode_impl(num, buf, &Base62Tables::STANDARD)
}

pub fn encode_bytes(num: u128, buf: &mut [u8; BASE62_LEN]) { unsafe { _encode_buf(num, buf) } }
