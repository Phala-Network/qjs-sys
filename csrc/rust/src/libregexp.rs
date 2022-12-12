use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn pstrcpy(buf: *mut libc::c_char, buf_size: libc::c_int, str: *const libc::c_char);
    fn dbuf_init2(
        s: *mut DynBuf,
        opaque: *mut libc::c_void,
        realloc_func: Option::<DynBufReallocFunc>,
    );
    fn dbuf_realloc(s: *mut DynBuf, new_size: size_t) -> libc::c_int;
    fn dbuf_put(s: *mut DynBuf, data: *const uint8_t, len: size_t) -> libc::c_int;
    fn dbuf_put_self(s: *mut DynBuf, offset: size_t, len: size_t) -> libc::c_int;
    fn dbuf_putc(s: *mut DynBuf, c: uint8_t) -> libc::c_int;
    fn dbuf_free(s: *mut DynBuf);
    fn unicode_to_utf8(buf: *mut uint8_t, c: libc::c_uint) -> libc::c_int;
    fn unicode_from_utf8(
        p: *const uint8_t,
        max_len: libc::c_int,
        pp: *mut *const uint8_t,
    ) -> libc::c_int;
    fn lre_case_conv(
        res: *mut uint32_t,
        c: uint32_t,
        conv_type: libc::c_int,
    ) -> libc::c_int;
    fn cr_union1(
        cr: *mut CharRange,
        b_pt: *const uint32_t,
        b_len: libc::c_int,
    ) -> libc::c_int;
    fn cr_op(
        cr: *mut CharRange,
        a_pt: *const uint32_t,
        a_len: libc::c_int,
        b_pt: *const uint32_t,
        b_len: libc::c_int,
        op: libc::c_int,
    ) -> libc::c_int;
    fn cr_invert(cr: *mut CharRange) -> libc::c_int;
    fn lre_is_id_start(c: uint32_t) -> libc::c_int;
    fn lre_is_id_continue(c: uint32_t) -> libc::c_int;
    fn unicode_script(
        cr: *mut CharRange,
        script_name: *const libc::c_char,
        is_ext: libc::c_int,
    ) -> libc::c_int;
    fn unicode_general_category(
        cr: *mut CharRange,
        gc_name: *const libc::c_char,
    ) -> libc::c_int;
    fn unicode_prop(cr: *mut CharRange, prop_name: *const libc::c_char) -> libc::c_int;
    fn cr_init(
        cr: *mut CharRange,
        mem_opaque: *mut libc::c_void,
        realloc_func: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_void,
                size_t,
            ) -> *mut libc::c_void,
        >,
    );
    fn cr_free(cr: *mut CharRange);
    fn cr_realloc(cr: *mut CharRange, size: libc::c_int) -> libc::c_int;
    fn lre_realloc(
        opaque: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn lre_check_stack_overflow(
        opaque: *mut libc::c_void,
        alloca_size: size_t,
    ) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: __builtin_va_list,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn abort() -> !;
    fn alloca(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type __builtin_va_list = *mut libc::c_void;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type uintptr_t = libc::c_uint;
pub type intptr_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type BOOL = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct packed_u32 {
    pub v: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct packed_u16 {
    pub v: uint16_t,
}
pub type DynBufReallocFunc = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynBuf {
    pub buf: *mut uint8_t,
    pub size: size_t,
    pub allocated_size: size_t,
    pub error: BOOL,
    pub realloc_func: Option::<DynBufReallocFunc>,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharRange {
    pub len: libc::c_int,
    pub size: libc::c_int,
    pub points: *mut uint32_t,
    pub mem_opaque: *mut libc::c_void,
    pub realloc_func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const CR_OP_XOR: C2RustUnnamed_0 = 2;
pub const CR_OP_INTER: C2RustUnnamed_0 = 1;
pub const CR_OP_UNION: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REParseState {
    pub byte_code: DynBuf,
    pub buf_ptr: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub buf_start: *const uint8_t,
    pub re_flags: libc::c_int,
    pub is_utf16: BOOL,
    pub ignore_case: BOOL,
    pub dotall: BOOL,
    pub capture_count: libc::c_int,
    pub total_capture_count: libc::c_int,
    pub has_named_captures: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub group_names: DynBuf,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub error_msg: [libc::c_char; 128],
    pub tmp_buf: [libc::c_char; 128],
}
pub const REOP_range32: C2RustUnnamed_2 = 22;
pub const REOP_range: C2RustUnnamed_2 = 21;
pub const REOP_bne_char_pos: C2RustUnnamed_2 = 26;
pub const REOP_drop: C2RustUnnamed_2 = 16;
pub const REOP_push_char_pos: C2RustUnnamed_2 = 25;
pub const REOP_push_i32: C2RustUnnamed_2 = 15;
pub const REOP_COUNT: C2RustUnnamed_2 = 29;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REOpCode {
    pub size: uint8_t,
}
pub const REOP_match: C2RustUnnamed_2 = 10;
pub const REOP_save_end: C2RustUnnamed_2 = 12;
pub const REOP_loop: C2RustUnnamed_2 = 14;
pub const REOP_split_goto_first: C2RustUnnamed_2 = 8;
pub const REOP_goto: C2RustUnnamed_2 = 7;
pub const REOP_split_next_first: C2RustUnnamed_2 = 9;
pub const REOP_save_reset: C2RustUnnamed_2 = 13;
pub const REOP_backward_back_reference: C2RustUnnamed_2 = 20;
pub const REOP_back_reference: C2RustUnnamed_2 = 19;
pub const REOP_save_start: C2RustUnnamed_2 = 11;
pub const REOP_prev: C2RustUnnamed_2 = 27;
pub const REOP_not_word_boundary: C2RustUnnamed_2 = 18;
pub const REOP_word_boundary: C2RustUnnamed_2 = 17;
pub const REOP_line_end: C2RustUnnamed_2 = 6;
pub const REOP_line_start: C2RustUnnamed_2 = 5;
pub const REOP_any: C2RustUnnamed_2 = 4;
pub const REOP_dot: C2RustUnnamed_2 = 3;
pub const REOP_char32: C2RustUnnamed_2 = 2;
pub const REOP_char: C2RustUnnamed_2 = 1;
pub const REOP_simple_greedy_quant: C2RustUnnamed_2 = 28;
pub const CHAR_RANGE_W: C2RustUnnamed_3 = 5;
pub const CHAR_RANGE_w: C2RustUnnamed_3 = 4;
pub const CHAR_RANGE_S: C2RustUnnamed_3 = 3;
pub const CHAR_RANGE_s: C2RustUnnamed_3 = 2;
pub const CHAR_RANGE_D: C2RustUnnamed_3 = 1;
pub const CHAR_RANGE_d: C2RustUnnamed_3 = 0;
pub const REOP_lookahead: C2RustUnnamed_2 = 23;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REExecContext {
    pub cbuf: *const uint8_t,
    pub cbuf_end: *const uint8_t,
    pub cbuf_type: libc::c_int,
    pub capture_count: libc::c_int,
    pub stack_size_max: libc::c_int,
    pub multi_line: BOOL,
    pub ignore_case: BOOL,
    pub is_utf16: BOOL,
    pub opaque: *mut libc::c_void,
    pub state_size: size_t,
    pub state_stack: *mut uint8_t,
    pub state_stack_size: size_t,
    pub state_stack_len: size_t,
}
pub type StackInt = uintptr_t;
pub type REExecStateEnum = libc::c_uint;
pub const RE_EXEC_STATE_GREEDY_QUANT: REExecStateEnum = 3;
pub const RE_EXEC_STATE_NEGATIVE_LOOKAHEAD: REExecStateEnum = 2;
pub const RE_EXEC_STATE_LOOKAHEAD: REExecStateEnum = 1;
pub const RE_EXEC_STATE_SPLIT: REExecStateEnum = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct REExecState {
    #[bitfield(name = "type_0", ty = "REExecStateEnum", bits = "0..=7")]
    pub type_0: [u8; 1],
    pub stack_len: uint8_t,
    pub count: size_t,
    pub cptr: *const uint8_t,
    pub pc: *const uint8_t,
    pub buf: [*mut libc::c_void; 0],
}
pub const REOP_negative_lookahead: C2RustUnnamed_2 = 24;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const REOP_invalid: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
#[inline]
unsafe extern "C" fn get_u32(mut tab: *const uint8_t) -> uint32_t {
    return (*(tab as *const packed_u32)).v;
}
#[inline]
unsafe extern "C" fn put_u32(mut tab: *mut uint8_t, mut val: uint32_t) {
    (*(tab as *mut packed_u32)).v = val;
}
#[inline]
unsafe extern "C" fn get_u16(mut tab: *const uint8_t) -> uint32_t {
    return (*(tab as *const packed_u16)).v as uint32_t;
}
#[inline]
unsafe extern "C" fn dbuf_put_u16(mut s: *mut DynBuf, mut val: uint16_t) -> libc::c_int {
    return dbuf_put(
        s,
        &mut val as *mut uint16_t as *mut uint8_t,
        2 as libc::c_int as size_t,
    );
}
#[inline]
unsafe extern "C" fn dbuf_put_u32(mut s: *mut DynBuf, mut val: uint32_t) -> libc::c_int {
    return dbuf_put(
        s,
        &mut val as *mut uint32_t as *mut uint8_t,
        4 as libc::c_int as size_t,
    );
}
#[inline]
unsafe extern "C" fn dbuf_error(mut s: *mut DynBuf) -> BOOL {
    return (*s).error;
}
#[inline]
unsafe extern "C" fn from_hex(mut c: libc::c_int) -> libc::c_int {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32
    } else if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as libc::c_int
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn cr_union_interval(
    mut cr: *mut CharRange,
    mut c1: uint32_t,
    mut c2: uint32_t,
) -> libc::c_int {
    let mut b_pt: [uint32_t; 2] = [0; 2];
    b_pt[0 as libc::c_int as usize] = c1;
    b_pt[1 as libc::c_int as usize] = c2.wrapping_add(1 as libc::c_int as libc::c_uint);
    return cr_union1(cr, b_pt.as_mut_ptr(), 2 as libc::c_int);
}
#[inline]
unsafe extern "C" fn cr_add_point(
    mut cr: *mut CharRange,
    mut v: uint32_t,
) -> libc::c_int {
    if (*cr).len >= (*cr).size {
        if cr_realloc(cr, (*cr).len + 1 as libc::c_int) != 0 {
            return -(1 as libc::c_int);
        }
    }
    let ref mut fresh0 = (*cr).len;
    let fresh1 = *fresh0;
    *fresh0 = *fresh0 + 1;
    *((*cr).points).offset(fresh1 as isize) = v;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn lre_js_is_ident_first(mut c: libc::c_int) -> libc::c_int {
    if (c as uint32_t) < 128 as libc::c_int as libc::c_uint {
        return (lre_id_start_table_ascii[(c >> 5 as libc::c_int) as usize]
            >> (c & 31 as libc::c_int) & 1 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        return lre_is_id_start(c as uint32_t)
    };
}
#[inline]
unsafe extern "C" fn lre_js_is_ident_next(mut c: libc::c_int) -> libc::c_int {
    if (c as uint32_t) < 128 as libc::c_int as libc::c_uint {
        return (lre_id_continue_table_ascii[(c >> 5 as libc::c_int) as usize]
            >> (c & 31 as libc::c_int) & 1 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        return (lre_is_id_continue(c as uint32_t) != 0 || c == 0x200c as libc::c_int
            || c == 0x200d as libc::c_int) as libc::c_int
    };
}
static mut reopcode_info: [REOpCode; 29] = [
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 17 as libc::c_int as uint8_t,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn is_digit(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as libc::c_int;
}
unsafe extern "C" fn dbuf_insert(
    mut s: *mut DynBuf,
    mut pos: libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int {
    if dbuf_realloc(s, ((*s).size).wrapping_add(len as libc::c_ulong)) != 0 {
        return -(1 as libc::c_int);
    }
    memmove(
        ((*s).buf).offset(pos as isize).offset(len as isize) as *mut libc::c_void,
        ((*s).buf).offset(pos as isize) as *const libc::c_void,
        ((*s).size).wrapping_sub(pos as libc::c_ulong),
    );
    let ref mut fresh2 = (*s).size;
    *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(len as libc::c_ulong) as size_t
        as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn lre_canonicalize(mut c: uint32_t, mut is_utf16: BOOL) -> uint32_t {
    let mut res: [uint32_t; 3] = [0; 3];
    let mut len: libc::c_int = 0;
    if is_utf16 != 0 {
        if (c < 128 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long != 0 {
            if c >= 'A' as i32 as libc::c_uint && c <= 'Z' as i32 as libc::c_uint {
                c = c
                    .wrapping_sub('A' as i32 as libc::c_uint)
                    .wrapping_add('a' as i32 as libc::c_uint);
            }
        } else {
            lre_case_conv(res.as_mut_ptr(), c, 2 as libc::c_int);
            c = res[0 as libc::c_int as usize];
        }
    } else if (c < 128 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long
        != 0
    {
        if c >= 'a' as i32 as libc::c_uint && c <= 'z' as i32 as libc::c_uint {
            c = c
                .wrapping_sub('a' as i32 as libc::c_uint)
                .wrapping_add('A' as i32 as libc::c_uint);
        }
    } else {
        len = lre_case_conv(res.as_mut_ptr(), c, FALSE as libc::c_int);
        if len == 1 as libc::c_int
            && res[0 as libc::c_int as usize] >= 128 as libc::c_int as libc::c_uint
        {
            c = res[0 as libc::c_int as usize];
        }
    }
    return c;
}
static mut char_range_d: [uint16_t; 3] = [
    1 as libc::c_int as uint16_t,
    0x30 as libc::c_int as uint16_t,
    (0x39 as libc::c_int + 1 as libc::c_int) as uint16_t,
];
static mut char_range_s: [uint16_t; 21] = [
    10 as libc::c_int as uint16_t,
    0x9 as libc::c_int as uint16_t,
    (0xd as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x20 as libc::c_int as uint16_t,
    (0x20 as libc::c_int + 1 as libc::c_int) as uint16_t,
    0xa0 as libc::c_int as uint16_t,
    (0xa0 as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x1680 as libc::c_int as uint16_t,
    (0x1680 as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x2000 as libc::c_int as uint16_t,
    (0x200a as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x2028 as libc::c_int as uint16_t,
    (0x2029 as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x202f as libc::c_int as uint16_t,
    (0x202f as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x205f as libc::c_int as uint16_t,
    (0x205f as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x3000 as libc::c_int as uint16_t,
    (0x3000 as libc::c_int + 1 as libc::c_int) as uint16_t,
    0xfeff as libc::c_int as uint16_t,
    (0xfeff as libc::c_int + 1 as libc::c_int) as uint16_t,
];
#[no_mangle]
pub unsafe extern "C" fn lre_is_space(mut c: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    n = (::core::mem::size_of::<[uint16_t; 21]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<uint16_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        low = char_range_s[(2 as libc::c_int * i + 1 as libc::c_int) as usize]
            as libc::c_int;
        if c < low {
            return FALSE as libc::c_int;
        }
        high = char_range_s[(2 as libc::c_int * i + 2 as libc::c_int) as usize]
            as libc::c_int;
        if c < high {
            return TRUE as libc::c_int;
        }
        i += 1;
    }
    return FALSE as libc::c_int;
}
#[no_mangle]
pub static mut lre_id_start_table_ascii: [uint32_t; 4] = [
    0 as libc::c_int as uint32_t,
    0x10 as libc::c_int as uint32_t,
    0x87fffffe as libc::c_uint,
    0x7fffffe as libc::c_int as uint32_t,
];
#[no_mangle]
pub static mut lre_id_continue_table_ascii: [uint32_t; 4] = [
    0 as libc::c_int as uint32_t,
    0x3ff0010 as libc::c_int as uint32_t,
    0x87fffffe as libc::c_uint,
    0x7fffffe as libc::c_int as uint32_t,
];
static mut char_range_w: [uint16_t; 9] = [
    4 as libc::c_int as uint16_t,
    0x30 as libc::c_int as uint16_t,
    (0x39 as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x41 as libc::c_int as uint16_t,
    (0x5a as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x5f as libc::c_int as uint16_t,
    (0x5f as libc::c_int + 1 as libc::c_int) as uint16_t,
    0x61 as libc::c_int as uint16_t,
    (0x7a as libc::c_int + 1 as libc::c_int) as uint16_t,
];
static mut char_range_table: [*const uint16_t; 3] = unsafe {
    [char_range_d.as_ptr(), char_range_s.as_ptr(), char_range_w.as_ptr()]
};
unsafe extern "C" fn cr_init_char_range(
    mut s: *mut REParseState,
    mut cr: *mut CharRange,
    mut c: uint32_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut invert: BOOL = 0;
    let mut c_pt: *const uint16_t = 0 as *const uint16_t;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    invert = (c & 1 as libc::c_int as libc::c_uint) as BOOL;
    c_pt = char_range_table[(c >> 1 as libc::c_int) as usize];
    let fresh3 = c_pt;
    c_pt = c_pt.offset(1);
    len = *fresh3 as libc::c_int;
    cr_init(
        cr,
        (*s).opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                ) -> *mut libc::c_void,
        ),
    );
    i = 0 as libc::c_int;
    loop {
        if !(i < len * 2 as libc::c_int) {
            current_block = 13513818773234778473;
            break;
        }
        if cr_add_point(cr, *c_pt.offset(i as isize) as uint32_t) != 0 {
            current_block = 17615007574301030357;
            break;
        }
        i += 1;
    }
    match current_block {
        13513818773234778473 => {
            if invert != 0 {
                if cr_invert(cr) != 0 {
                    current_block = 17615007574301030357;
                } else {
                    current_block = 2968425633554183086;
                }
            } else {
                current_block = 2968425633554183086;
            }
            match current_block {
                17615007574301030357 => {}
                _ => return 0 as libc::c_int,
            }
        }
        _ => {}
    }
    cr_free(cr);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cr_canonicalize(mut cr: *mut CharRange) -> libc::c_int {
    let mut a: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut uint32_t,
        mem_opaque: 0 as *mut libc::c_void,
        realloc_func: None,
    };
    let mut pt: [uint32_t; 2] = [0; 2];
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    cr_init(
        &mut a,
        (*cr).mem_opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                ) -> *mut libc::c_void,
        ),
    );
    pt[0 as libc::c_int as usize] = 'a' as i32 as uint32_t;
    pt[1 as libc::c_int as usize] = ('z' as i32 + 1 as libc::c_int) as uint32_t;
    ret = cr_op(
        &mut a,
        (*cr).points,
        (*cr).len,
        pt.as_mut_ptr(),
        2 as libc::c_int,
        CR_OP_INTER as libc::c_int,
    );
    if !(ret != 0) {
        i = 0 as libc::c_int;
        while i < a.len {
            let ref mut fresh4 = *(a.points).offset(i as isize);
            *fresh4 = (*fresh4 as libc::c_uint)
                .wrapping_add(('A' as i32 - 'a' as i32) as libc::c_uint) as uint32_t
                as uint32_t;
            i += 1;
        }
        ret = cr_union1(cr, a.points, a.len);
    }
    cr_free(&mut a);
    return ret;
}
unsafe extern "C" fn re_emit_op(mut s: *mut REParseState, mut op: libc::c_int) {
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
}
unsafe extern "C" fn re_emit_op_u32(
    mut s: *mut REParseState,
    mut op: libc::c_int,
    mut val: uint32_t,
) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    pos = (*s).byte_code.size as libc::c_int;
    dbuf_put_u32(&mut (*s).byte_code, val);
    return pos;
}
unsafe extern "C" fn re_emit_goto(
    mut s: *mut REParseState,
    mut op: libc::c_int,
    mut val: uint32_t,
) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    pos = (*s).byte_code.size as libc::c_int;
    dbuf_put_u32(
        &mut (*s).byte_code,
        val.wrapping_sub((pos + 4 as libc::c_int) as libc::c_uint),
    );
    return pos;
}
unsafe extern "C" fn re_emit_op_u8(
    mut s: *mut REParseState,
    mut op: libc::c_int,
    mut val: uint32_t,
) {
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    dbuf_putc(&mut (*s).byte_code, val as uint8_t);
}
unsafe extern "C" fn re_emit_op_u16(
    mut s: *mut REParseState,
    mut op: libc::c_int,
    mut val: uint32_t,
) {
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    dbuf_put_u16(&mut (*s).byte_code, val as uint16_t);
}
unsafe extern "C" fn re_parse_error(
    mut s: *mut REParseState,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: va_list = 0 as *mut libc::c_void;
    ap = args.clone();
    vsnprintf(
        ((*s).u.error_msg).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        fmt,
        ap,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn re_parse_out_of_memory(mut s: *mut REParseState) -> libc::c_int {
    return re_parse_error(s, b"out of memory\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn parse_digits(
    mut pp: *mut *const uint8_t,
    mut allow_overflow: BOOL,
) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut v: uint64_t = 0;
    let mut c: libc::c_int = 0;
    p = *pp;
    v = 0 as libc::c_int as uint64_t;
    loop {
        c = *p as libc::c_int;
        if c < '0' as i32 || c > '9' as i32 {
            break;
        }
        v = v
            .wrapping_mul(10 as libc::c_int as libc::c_ulonglong)
            .wrapping_add(c as libc::c_ulonglong)
            .wrapping_sub('0' as i32 as libc::c_ulonglong);
        if v >= 0x7fffffff as libc::c_int as libc::c_ulonglong {
            if allow_overflow != 0 {
                v = 0x7fffffff as libc::c_int as uint64_t;
            } else {
                return -(1 as libc::c_int)
            }
        }
        p = p.offset(1);
    }
    *pp = p;
    return v as libc::c_int;
}
unsafe extern "C" fn re_parse_expect(
    mut s: *mut REParseState,
    mut pp: *mut *const uint8_t,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p = *pp;
    if *p as libc::c_int != c {
        return re_parse_error(
            s,
            b"expecting '%c'\0" as *const u8 as *const libc::c_char,
            c,
        );
    }
    p = p.offset(1);
    *pp = p;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lre_parse_escape(
    mut pp: *mut *const uint8_t,
    mut allow_utf16: libc::c_int,
) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    p = *pp;
    let fresh5 = p;
    p = p.offset(1);
    c = *fresh5 as uint32_t;
    match c {
        98 => {
            c = '\u{8}' as i32 as uint32_t;
        }
        102 => {
            c = '\u{c}' as i32 as uint32_t;
        }
        110 => {
            c = '\n' as i32 as uint32_t;
        }
        114 => {
            c = '\r' as i32 as uint32_t;
        }
        116 => {
            c = '\t' as i32 as uint32_t;
        }
        118 => {
            c = '\u{b}' as i32 as uint32_t;
        }
        120 | 117 => {
            let mut h: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut c1: uint32_t = 0;
            if *p as libc::c_int == '{' as i32 && allow_utf16 != 0 {
                p = p.offset(1);
                c = 0 as libc::c_int as uint32_t;
                loop {
                    let fresh6 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh6 as libc::c_int);
                    if h < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    c = c << 4 as libc::c_int | h as libc::c_uint;
                    if c > 0x10ffff as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int);
                    }
                    if *p as libc::c_int == '}' as i32 {
                        break;
                    }
                }
                p = p.offset(1);
            } else {
                if c == 'x' as i32 as libc::c_uint {
                    n = 2 as libc::c_int;
                } else {
                    n = 4 as libc::c_int;
                }
                c = 0 as libc::c_int as uint32_t;
                i = 0 as libc::c_int;
                while i < n {
                    let fresh7 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh7 as libc::c_int);
                    if h < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    c = c << 4 as libc::c_int | h as libc::c_uint;
                    i += 1;
                }
                if c >= 0xd800 as libc::c_int as libc::c_uint
                    && c < 0xdc00 as libc::c_int as libc::c_uint
                    && allow_utf16 == 2 as libc::c_int
                    && *p.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'u' as i32
                {
                    c1 = 0 as libc::c_int as uint32_t;
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        h = from_hex(
                            *p.offset((2 as libc::c_int + i) as isize) as libc::c_int,
                        );
                        if h < 0 as libc::c_int {
                            break;
                        }
                        c1 = c1 << 4 as libc::c_int | h as libc::c_uint;
                        i += 1;
                    }
                    if i == 4 as libc::c_int
                        && c1 >= 0xdc00 as libc::c_int as libc::c_uint
                        && c1 < 0xe000 as libc::c_int as libc::c_uint
                    {
                        p = p.offset(6 as libc::c_int as isize);
                        c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                            << 10 as libc::c_int
                            | c1 & 0x3ff as libc::c_int as libc::c_uint)
                            .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                    }
                }
            }
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            c = (c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint) as uint32_t
                as uint32_t;
            if allow_utf16 == 2 as libc::c_int {
                if c != 0 as libc::c_int as libc::c_uint
                    || is_digit(*p as libc::c_int) != 0
                {
                    return -(1 as libc::c_int);
                }
            } else {
                let mut v: uint32_t = 0;
                v = (*p as libc::c_int - '0' as i32) as uint32_t;
                if !(v > 7 as libc::c_int as libc::c_uint) {
                    c = c << 3 as libc::c_int | v;
                    p = p.offset(1);
                    if !(c >= 32 as libc::c_int as libc::c_uint) {
                        v = (*p as libc::c_int - '0' as i32) as uint32_t;
                        if !(v > 7 as libc::c_int as libc::c_uint) {
                            c = c << 3 as libc::c_int | v;
                            p = p.offset(1);
                        }
                    }
                }
            }
        }
        _ => return -(2 as libc::c_int),
    }
    *pp = p;
    return c as libc::c_int;
}
unsafe extern "C" fn is_unicode_char(mut c: libc::c_int) -> BOOL {
    return (c >= '0' as i32 && c <= '9' as i32 || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= 'a' as i32 && c <= 'z' as i32 || c == '_' as i32) as libc::c_int;
}
unsafe extern "C" fn parse_unicode_property(
    mut s: *mut REParseState,
    mut cr: *mut CharRange,
    mut pp: *mut *const uint8_t,
    mut is_inv: BOOL,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut value: [libc::c_char; 64] = [0; 64];
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut script_ext: BOOL = 0;
    let mut ret: libc::c_int = 0;
    p = *pp;
    if *p as libc::c_int != '{' as i32 {
        return re_parse_error(
            s,
            b"expecting '{' after \\p\0" as *const u8 as *const libc::c_char,
        );
    }
    p = p.offset(1);
    q = name.as_mut_ptr();
    loop {
        if !(is_unicode_char(*p as libc::c_int) != 0) {
            current_block = 2868539653012386629;
            break;
        }
        if q.offset_from(name.as_mut_ptr()) as libc::c_long as libc::c_ulong
            >= (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            current_block = 13479925467177283591;
            break;
        }
        let fresh8 = p;
        p = p.offset(1);
        let fresh9 = q;
        q = q.offset(1);
        *fresh9 = *fresh8 as libc::c_char;
    }
    match current_block {
        2868539653012386629 => {
            *q = '\0' as i32 as libc::c_char;
            q = value.as_mut_ptr();
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1);
                while is_unicode_char(*p as libc::c_int) != 0 {
                    if q.offset_from(value.as_mut_ptr()) as libc::c_long as libc::c_ulong
                        >= (::core::mem::size_of::<[libc::c_char; 64]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        return re_parse_error(
                            s,
                            b"unknown unicode property value\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    let fresh10 = p;
                    p = p.offset(1);
                    let fresh11 = q;
                    q = q.offset(1);
                    *fresh11 = *fresh10 as libc::c_char;
                }
            }
            *q = '\0' as i32 as libc::c_char;
            if *p as libc::c_int != '}' as i32 {
                return re_parse_error(
                    s,
                    b"expecting '}'\0" as *const u8 as *const libc::c_char,
                );
            }
            p = p.offset(1);
            if strcmp(name.as_mut_ptr(), b"Script\0" as *const u8 as *const libc::c_char)
                == 0
                || strcmp(name.as_mut_ptr(), b"sc\0" as *const u8 as *const libc::c_char)
                    == 0
            {
                script_ext = FALSE as libc::c_int;
                current_block = 3042168899176905925;
            } else if strcmp(
                name.as_mut_ptr(),
                b"Script_Extensions\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    name.as_mut_ptr(),
                    b"scx\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                script_ext = TRUE as libc::c_int;
                current_block = 3042168899176905925;
            } else if strcmp(
                name.as_mut_ptr(),
                b"General_Category\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(name.as_mut_ptr(), b"gc\0" as *const u8 as *const libc::c_char)
                    == 0
            {
                cr_init(
                    cr,
                    (*s).opaque,
                    Some(
                        lre_realloc
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                size_t,
                            ) -> *mut libc::c_void,
                    ),
                );
                ret = unicode_general_category(cr, value.as_mut_ptr());
                if ret != 0 {
                    cr_free(cr);
                    if ret == -(2 as libc::c_int) {
                        return re_parse_error(
                            s,
                            b"unknown unicode general category\0" as *const u8
                                as *const libc::c_char,
                        )
                    } else {
                        current_block = 14623436174320709670;
                    }
                } else {
                    current_block = 9353995356876505083;
                }
            } else if value[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                cr_init(
                    cr,
                    (*s).opaque,
                    Some(
                        lre_realloc
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                size_t,
                            ) -> *mut libc::c_void,
                    ),
                );
                ret = unicode_general_category(cr, name.as_mut_ptr());
                if ret == -(1 as libc::c_int) {
                    cr_free(cr);
                    current_block = 14623436174320709670;
                } else if ret < 0 as libc::c_int {
                    ret = unicode_prop(cr, name.as_mut_ptr());
                    if ret != 0 {
                        cr_free(cr);
                        if ret == -(2 as libc::c_int) {
                            current_block = 13479925467177283591;
                        } else {
                            current_block = 14623436174320709670;
                        }
                    } else {
                        current_block = 9353995356876505083;
                    }
                } else {
                    current_block = 9353995356876505083;
                }
            } else {
                current_block = 13479925467177283591;
            }
            match current_block {
                13479925467177283591 => {}
                _ => {
                    match current_block {
                        3042168899176905925 => {
                            cr_init(
                                cr,
                                (*s).opaque,
                                Some(
                                    lre_realloc
                                        as unsafe extern "C" fn(
                                            *mut libc::c_void,
                                            *mut libc::c_void,
                                            size_t,
                                        ) -> *mut libc::c_void,
                                ),
                            );
                            ret = unicode_script(cr, value.as_mut_ptr(), script_ext);
                            if ret != 0 {
                                cr_free(cr);
                                if ret == -(2 as libc::c_int) {
                                    return re_parse_error(
                                        s,
                                        b"unknown unicode script\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                } else {
                                    current_block = 14623436174320709670;
                                }
                            } else {
                                current_block = 9353995356876505083;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        14623436174320709670 => return re_parse_out_of_memory(s),
                        _ => {
                            if is_inv != 0 {
                                if cr_invert(cr) != 0 {
                                    cr_free(cr);
                                    return -(1 as libc::c_int);
                                }
                            }
                            *pp = p;
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return re_parse_error(
        s,
        b"unknown unicode property name\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn get_class_atom(
    mut s: *mut REParseState,
    mut cr: *mut CharRange,
    mut pp: *mut *const uint8_t,
    mut inclass: BOOL,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    let mut ret: libc::c_int = 0;
    p = *pp;
    c = *p as uint32_t;
    match c {
        92 => {
            p = p.offset(1);
            if p >= (*s).buf_end {
                current_block = 11369301397065209419;
            } else {
                let fresh12 = p;
                p = p.offset(1);
                c = *fresh12 as uint32_t;
                match c {
                    100 => {
                        c = CHAR_RANGE_d as libc::c_int as uint32_t;
                        current_block = 2195997158480489718;
                    }
                    68 => {
                        c = CHAR_RANGE_D as libc::c_int as uint32_t;
                        current_block = 2195997158480489718;
                    }
                    115 => {
                        c = CHAR_RANGE_s as libc::c_int as uint32_t;
                        current_block = 2195997158480489718;
                    }
                    83 => {
                        c = CHAR_RANGE_S as libc::c_int as uint32_t;
                        current_block = 2195997158480489718;
                    }
                    119 => {
                        c = CHAR_RANGE_w as libc::c_int as uint32_t;
                        current_block = 2195997158480489718;
                    }
                    87 => {
                        c = CHAR_RANGE_W as libc::c_int as uint32_t;
                        current_block = 2195997158480489718;
                    }
                    99 => {
                        c = *p as uint32_t;
                        if c >= 'a' as i32 as libc::c_uint
                            && c <= 'z' as i32 as libc::c_uint
                            || c >= 'A' as i32 as libc::c_uint
                                && c <= 'Z' as i32 as libc::c_uint
                            || (c >= '0' as i32 as libc::c_uint
                                && c <= '9' as i32 as libc::c_uint
                                || c == '_' as i32 as libc::c_uint) && inclass != 0
                                && (*s).is_utf16 == 0
                        {
                            c &= 0x1f as libc::c_int as libc::c_uint;
                            p = p.offset(1);
                            current_block = 5159818223158340697;
                        } else if (*s).is_utf16 != 0 {
                            current_block = 1503728797416693332;
                        } else {
                            p = p.offset(-1);
                            c = '\\' as i32 as uint32_t;
                            current_block = 5159818223158340697;
                        }
                    }
                    112 | 80 => {
                        if (*s).is_utf16 != 0 {
                            if parse_unicode_property(
                                s,
                                cr,
                                &mut p,
                                (c == 'P' as i32 as libc::c_uint) as libc::c_int,
                            ) != 0
                            {
                                return -(1 as libc::c_int);
                            }
                            c = 0x40000000 as libc::c_int as uint32_t;
                            current_block = 5159818223158340697;
                        } else {
                            current_block = 4469401719621032066;
                        }
                    }
                    _ => {
                        current_block = 4469401719621032066;
                    }
                }
                match current_block {
                    5159818223158340697 => {}
                    _ => {
                        match current_block {
                            4469401719621032066 => {
                                p = p.offset(-1);
                                ret = lre_parse_escape(
                                    &mut p,
                                    (*s).is_utf16 * 2 as libc::c_int,
                                );
                                if ret >= 0 as libc::c_int {
                                    c = ret as uint32_t;
                                    current_block = 5159818223158340697;
                                } else if ret == -(2 as libc::c_int)
                                    && *p as libc::c_int != '\0' as i32
                                    && !(strchr(
                                        b"^$\\.*+?()[]{}|/\0" as *const u8 as *const libc::c_char,
                                        *p as libc::c_int,
                                    ))
                                        .is_null()
                                {
                                    current_block = 4245027872499580016;
                                } else if (*s).is_utf16 != 0 {
                                    current_block = 1503728797416693332;
                                } else {
                                    current_block = 4245027872499580016;
                                }
                            }
                            2195997158480489718 => {
                                if cr_init_char_range(s, cr, c) != 0 {
                                    return -(1 as libc::c_int);
                                }
                                c = 0x40000000 as libc::c_int as uint32_t;
                                current_block = 5159818223158340697;
                            }
                            _ => {}
                        }
                        match current_block {
                            4245027872499580016 => {}
                            5159818223158340697 => {}
                            _ => {
                                return re_parse_error(
                                    s,
                                    b"invalid escape sequence in regular expression\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
            }
        }
        0 => {
            if p >= (*s).buf_end {
                current_block = 11369301397065209419;
            } else {
                current_block = 4245027872499580016;
            }
        }
        _ => {
            current_block = 4245027872499580016;
        }
    }
    match current_block {
        4245027872499580016 => {
            if c >= 128 as libc::c_int as libc::c_uint {
                c = unicode_from_utf8(p, 6 as libc::c_int, &mut p) as uint32_t;
                if c > 0xffff as libc::c_int as libc::c_uint && (*s).is_utf16 == 0 {
                    return re_parse_error(
                        s,
                        b"malformed unicode char\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                p = p.offset(1);
            }
        }
        11369301397065209419 => {
            return re_parse_error(
                s,
                b"unexpected end\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    *pp = p;
    return c as libc::c_int;
}
unsafe extern "C" fn re_emit_range(
    mut s: *mut REParseState,
    mut cr: *const CharRange,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut high: uint32_t = 0;
    len = ((*cr).len as libc::c_uint).wrapping_div(2 as libc::c_int as libc::c_uint)
        as libc::c_int;
    if len >= 65535 as libc::c_int {
        return re_parse_error(
            s,
            b"too many ranges\0" as *const u8 as *const libc::c_char,
        );
    }
    if len == 0 as libc::c_int {
        re_emit_op_u32(s, REOP_char32 as libc::c_int, -(1 as libc::c_int) as uint32_t);
    } else {
        high = *((*cr).points).offset(((*cr).len - 1 as libc::c_int) as isize);
        if high == 0xffffffff as libc::c_uint {
            high = *((*cr).points).offset(((*cr).len - 2 as libc::c_int) as isize);
        }
        if high <= 0xffff as libc::c_int as libc::c_uint {
            re_emit_op_u16(s, REOP_range as libc::c_int, len as uint32_t);
            i = 0 as libc::c_int;
            while i < (*cr).len {
                dbuf_put_u16(
                    &mut (*s).byte_code,
                    *((*cr).points).offset(i as isize) as uint16_t,
                );
                high = (*((*cr).points).offset((i + 1 as libc::c_int) as isize))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
                if high
                    == (0xffffffff as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    high = 0xffff as libc::c_int as uint32_t;
                }
                dbuf_put_u16(&mut (*s).byte_code, high as uint16_t);
                i += 2 as libc::c_int;
            }
        } else {
            re_emit_op_u16(s, REOP_range32 as libc::c_int, len as uint32_t);
            i = 0 as libc::c_int;
            while i < (*cr).len {
                dbuf_put_u32(&mut (*s).byte_code, *((*cr).points).offset(i as isize));
                dbuf_put_u32(
                    &mut (*s).byte_code,
                    (*((*cr).points).offset((i + 1 as libc::c_int) as isize))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                i += 2 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn re_parse_char_class(
    mut s: *mut REParseState,
    mut pp: *mut *const uint8_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c1: uint32_t = 0;
    let mut c2: uint32_t = 0;
    let mut cr_s: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut uint32_t,
        mem_opaque: 0 as *mut libc::c_void,
        realloc_func: None,
    };
    let mut cr: *mut CharRange = &mut cr_s;
    let mut cr1_s: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut uint32_t,
        mem_opaque: 0 as *mut libc::c_void,
        realloc_func: None,
    };
    let mut cr1: *mut CharRange = &mut cr1_s;
    let mut invert: BOOL = 0;
    cr_init(
        cr,
        (*s).opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                ) -> *mut libc::c_void,
        ),
    );
    p = *pp;
    p = p.offset(1);
    invert = FALSE as libc::c_int;
    if *p as libc::c_int == '^' as i32 {
        p = p.offset(1);
        invert = TRUE as libc::c_int;
    }
    loop {
        if *p as libc::c_int == ']' as i32 {
            current_block = 572715077006366937;
            break;
        }
        c1 = get_class_atom(s, cr1, &mut p, TRUE as libc::c_int) as uint32_t;
        if (c1 as libc::c_int) < 0 as libc::c_int {
            current_block = 15092484466962266423;
            break;
        }
        if *p as libc::c_int == '-' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int != ']' as i32
        {
            let mut p0: *const uint8_t = p.offset(1 as libc::c_int as isize);
            if c1 >= 0x40000000 as libc::c_int as libc::c_uint {
                if (*s).is_utf16 != 0 {
                    cr_free(cr1);
                    current_block = 15028968826697170054;
                } else {
                    current_block = 14058106991300340185;
                }
            } else {
                c2 = get_class_atom(s, cr1, &mut p0, TRUE as libc::c_int) as uint32_t;
                if (c2 as libc::c_int) < 0 as libc::c_int {
                    current_block = 15092484466962266423;
                    break;
                }
                if c2 >= 0x40000000 as libc::c_int as libc::c_uint {
                    cr_free(cr1);
                    if (*s).is_utf16 != 0 {
                        current_block = 15028968826697170054;
                    } else {
                        current_block = 14058106991300340185;
                    }
                } else {
                    p = p0;
                    if c2 < c1 {
                        current_block = 15028968826697170054;
                    } else if cr_union_interval(cr, c1, c2) != 0 {
                        current_block = 14955198446389028284;
                        break;
                    } else {
                        continue;
                    }
                }
            }
            match current_block {
                14058106991300340185 => {}
                _ => {
                    re_parse_error(
                        s,
                        b"invalid class range\0" as *const u8 as *const libc::c_char,
                    );
                    current_block = 15092484466962266423;
                    break;
                }
            }
        }
        if c1 >= 0x40000000 as libc::c_int as libc::c_uint {
            let mut ret: libc::c_int = 0;
            ret = cr_union1(cr, (*cr1).points, (*cr1).len);
            cr_free(cr1);
            if ret != 0 {
                current_block = 14955198446389028284;
                break;
            }
        } else if cr_union_interval(cr, c1, c1) != 0 {
            current_block = 14955198446389028284;
            break;
        }
    }
    match current_block {
        572715077006366937 => {
            if (*s).ignore_case != 0 {
                if cr_canonicalize(cr) != 0 {
                    current_block = 14955198446389028284;
                } else {
                    current_block = 1847472278776910194;
                }
            } else {
                current_block = 1847472278776910194;
            }
            match current_block {
                14955198446389028284 => {}
                _ => {
                    if invert != 0 {
                        if cr_invert(cr) != 0 {
                            current_block = 14955198446389028284;
                        } else {
                            current_block = 3160140712158701372;
                        }
                    } else {
                        current_block = 3160140712158701372;
                    }
                    match current_block {
                        14955198446389028284 => {}
                        _ => {
                            if re_emit_range(s, cr) != 0 {
                                current_block = 15092484466962266423;
                            } else {
                                cr_free(cr);
                                p = p.offset(1);
                                *pp = p;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        14955198446389028284 => {
            re_parse_out_of_memory(s);
        }
        _ => {}
    }
    cr_free(cr);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn re_check_advance(
    mut bc_buf: *const uint8_t,
    mut bc_buf_len: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pos: libc::c_int = 0;
    let mut opcode: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut val: uint32_t = 0;
    let mut last: uint32_t = 0;
    let mut has_back_reference: BOOL = 0;
    let mut capture_bitmap: [uint8_t; 255] = [0; 255];
    ret = -(2 as libc::c_int);
    pos = 0 as libc::c_int;
    has_back_reference = FALSE as libc::c_int;
    memset(
        capture_bitmap.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 255]>() as libc::c_ulong,
    );
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as libc::c_int;
        len = reopcode_info[opcode as usize].size as libc::c_int;
        match opcode {
            21 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as libc::c_int as isize),
                );
                len = (len as libc::c_uint)
                    .wrapping_add(val.wrapping_mul(4 as libc::c_int as libc::c_uint))
                    as libc::c_int as libc::c_int;
                current_block = 12214832785714998086;
            }
            22 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as libc::c_int as isize),
                );
                len = (len as libc::c_uint)
                    .wrapping_add(val.wrapping_mul(8 as libc::c_int as libc::c_uint))
                    as libc::c_int as libc::c_int;
                current_block = 12214832785714998086;
            }
            1 | 2 | 3 | 4 => {
                current_block = 12214832785714998086;
            }
            5 | 6 | 15 | 25 | 16 | 17 | 18 | 27 => {
                current_block = 9520865839495247062;
            }
            11 | 12 => {
                val = *bc_buf.offset((pos + 1 as libc::c_int) as isize) as uint32_t;
                capture_bitmap[val
                    as usize] = (capture_bitmap[val as usize] as libc::c_int
                    | 1 as libc::c_int) as uint8_t;
                current_block = 9520865839495247062;
            }
            13 => {
                val = *bc_buf.offset((pos + 1 as libc::c_int) as isize) as uint32_t;
                last = *bc_buf.offset((pos + 2 as libc::c_int) as isize) as uint32_t;
                while val < last {
                    let fresh13 = val;
                    val = val.wrapping_add(1);
                    capture_bitmap[fresh13
                        as usize] = (capture_bitmap[fresh13 as usize] as libc::c_int
                        | 1 as libc::c_int) as uint8_t;
                }
                current_block = 9520865839495247062;
            }
            19 | 20 => {
                val = *bc_buf.offset((pos + 1 as libc::c_int) as isize) as uint32_t;
                capture_bitmap[val
                    as usize] = (capture_bitmap[val as usize] as libc::c_int
                    | 2 as libc::c_int) as uint8_t;
                has_back_reference = TRUE as libc::c_int;
                current_block = 9520865839495247062;
            }
            _ => {
                if ret == -(2 as libc::c_int) {
                    ret = 0 as libc::c_int;
                }
                current_block = 9520865839495247062;
            }
        }
        match current_block {
            12214832785714998086 => {
                if ret == -(2 as libc::c_int) {
                    ret = 1 as libc::c_int;
                }
            }
            _ => {}
        }
        pos += len;
    }
    if has_back_reference != 0 {
        i = 0 as libc::c_int;
        while i < 255 as libc::c_int {
            if capture_bitmap[i as usize] as libc::c_int == 3 as libc::c_int {
                return -(1 as libc::c_int);
            }
            i += 1;
        }
    }
    if ret == -(2 as libc::c_int) {
        ret = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn re_is_simple_quantifier(
    mut bc_buf: *const uint8_t,
    mut bc_buf_len: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pos: libc::c_int = 0;
    let mut opcode: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut val: uint32_t = 0;
    count = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as libc::c_int;
        len = reopcode_info[opcode as usize].size as libc::c_int;
        match opcode {
            21 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as libc::c_int as isize),
                );
                len = (len as libc::c_uint)
                    .wrapping_add(val.wrapping_mul(4 as libc::c_int as libc::c_uint))
                    as libc::c_int as libc::c_int;
                current_block = 16319297390544596957;
            }
            22 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as libc::c_int as isize),
                );
                len = (len as libc::c_uint)
                    .wrapping_add(val.wrapping_mul(8 as libc::c_int as libc::c_uint))
                    as libc::c_int as libc::c_int;
                current_block = 16319297390544596957;
            }
            1 | 2 | 3 | 4 => {
                current_block = 16319297390544596957;
            }
            5 | 6 | 17 | 18 => {
                current_block = 7149356873433890176;
            }
            _ => return -(1 as libc::c_int),
        }
        match current_block {
            16319297390544596957 => {
                count += 1;
            }
            _ => {}
        }
        pos += len;
    }
    return count;
}
unsafe extern "C" fn re_parse_group_name(
    mut buf: *mut libc::c_char,
    mut buf_size: libc::c_int,
    mut pp: *mut *const uint8_t,
    mut is_utf16: BOOL,
) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    p = *pp;
    q = buf;
    loop {
        c = *p as uint32_t;
        if c == '\\' as i32 as libc::c_uint {
            p = p.offset(1);
            if *p as libc::c_int != 'u' as i32 {
                return -(1 as libc::c_int);
            }
            c = lre_parse_escape(&mut p, is_utf16 * 2 as libc::c_int) as uint32_t;
        } else {
            if c == '>' as i32 as libc::c_uint {
                break;
            }
            if c >= 128 as libc::c_int as libc::c_uint {
                c = unicode_from_utf8(p, 6 as libc::c_int, &mut p) as uint32_t;
            } else {
                p = p.offset(1);
            }
        }
        if c > 0x10ffff as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        if q == buf {
            if lre_js_is_ident_first(c as libc::c_int) == 0 {
                return -(1 as libc::c_int);
            }
        } else if lre_js_is_ident_next(c as libc::c_int) == 0 {
            return -(1 as libc::c_int)
        }
        if q.offset_from(buf) as libc::c_long + 6 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long > buf_size as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        if c < 128 as libc::c_int as libc::c_uint {
            let fresh14 = q;
            q = q.offset(1);
            *fresh14 = c as libc::c_char;
        } else {
            q = q.offset(unicode_to_utf8(q as *mut uint8_t, c) as isize);
        }
    }
    if q == buf {
        return -(1 as libc::c_int);
    }
    *q = '\0' as i32 as libc::c_char;
    p = p.offset(1);
    *pp = p;
    return 0 as libc::c_int;
}
unsafe extern "C" fn re_parse_captures(
    mut s: *mut REParseState,
    mut phas_named_captures: *mut libc::c_int,
    mut capture_name: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut capture_index: libc::c_int = 0;
    let mut name: [libc::c_char; 128] = [0; 128];
    capture_index = 1 as libc::c_int;
    *phas_named_captures = 0 as libc::c_int;
    p = (*s).buf_start;
    while p < (*s).buf_end {
        match *p as libc::c_int {
            40 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '?' as i32 {
                    if *p.offset(2 as libc::c_int as isize) as libc::c_int == '<' as i32
                        && *p.offset(3 as libc::c_int as isize) as libc::c_int
                            != '=' as i32
                        && *p.offset(3 as libc::c_int as isize) as libc::c_int
                            != '!' as i32
                    {
                        *phas_named_captures = 1 as libc::c_int;
                        if !capture_name.is_null() {
                            p = p.offset(3 as libc::c_int as isize);
                            if re_parse_group_name(
                                name.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_int,
                                &mut p,
                                (*s).is_utf16,
                            ) == 0 as libc::c_int
                            {
                                if strcmp(name.as_mut_ptr(), capture_name) == 0 {
                                    return capture_index;
                                }
                            }
                        }
                        capture_index += 1;
                        if capture_index >= 255 as libc::c_int {
                            break;
                        }
                    }
                } else {
                    capture_index += 1;
                    if capture_index >= 255 as libc::c_int {
                        break;
                    }
                }
            }
            92 => {
                p = p.offset(1);
            }
            91 => {
                p = p
                    .offset(
                        (1 as libc::c_int
                            + (*p as libc::c_int == ']' as i32) as libc::c_int) as isize,
                    );
                while p < (*s).buf_end && *p as libc::c_int != ']' as i32 {
                    if *p as libc::c_int == '\\' as i32 {
                        p = p.offset(1);
                    }
                    p = p.offset(1);
                }
            }
            _ => {}
        }
        p = p.offset(1);
    }
    if !capture_name.is_null() {
        return -(1 as libc::c_int)
    } else {
        return capture_index
    };
}
unsafe extern "C" fn re_count_captures(mut s: *mut REParseState) -> libc::c_int {
    if (*s).total_capture_count < 0 as libc::c_int {
        (*s)
            .total_capture_count = re_parse_captures(
            s,
            &mut (*s).has_named_captures,
            0 as *const libc::c_char,
        );
    }
    return (*s).total_capture_count;
}
unsafe extern "C" fn re_has_named_captures(mut s: *mut REParseState) -> BOOL {
    if (*s).has_named_captures < 0 as libc::c_int {
        re_count_captures(s);
    }
    return (*s).has_named_captures;
}
unsafe extern "C" fn find_group_name(
    mut s: *mut REParseState,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut name_len: size_t = 0;
    let mut capture_index: libc::c_int = 0;
    name_len = strlen(name);
    p = (*s).group_names.buf as *mut libc::c_char;
    buf_end = ((*s).group_names.buf as *mut libc::c_char)
        .offset((*s).group_names.size as isize);
    capture_index = 1 as libc::c_int;
    while p < buf_end {
        len = strlen(p);
        if len == name_len
            && memcmp(name as *const libc::c_void, p as *const libc::c_void, name_len)
                == 0 as libc::c_int
        {
            return capture_index;
        }
        p = p.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        capture_index += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn re_parse_term(
    mut s: *mut REParseState,
    mut is_backward_dir: BOOL,
) -> libc::c_int {
    let mut q: *const uint8_t = 0 as *const uint8_t;
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: libc::c_int = 0;
    let mut last_atom_start: libc::c_int = 0;
    let mut quant_min: libc::c_int = 0;
    let mut quant_max: libc::c_int = 0;
    let mut last_capture_count: libc::c_int = 0;
    let mut greedy: BOOL = 0;
    let mut add_zero_advance_check: BOOL = 0;
    let mut is_neg: BOOL = 0;
    let mut is_backward_lookahead: BOOL = 0;
    let mut cr_s: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut uint32_t,
        mem_opaque: 0 as *mut libc::c_void,
        realloc_func: None,
    };
    let mut cr: *mut CharRange = &mut cr_s;
    last_atom_start = -(1 as libc::c_int);
    last_capture_count = 0 as libc::c_int;
    p = (*s).buf_ptr;
    c = *p as libc::c_int;
    match c {
        94 => {
            p = p.offset(1);
            re_emit_op(s, REOP_line_start as libc::c_int);
            current_block = 12151070351325546249;
        }
        36 => {
            p = p.offset(1);
            re_emit_op(s, REOP_line_end as libc::c_int);
            current_block = 12151070351325546249;
        }
        46 => {
            p = p.offset(1);
            last_atom_start = (*s).byte_code.size as libc::c_int;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as libc::c_int);
            }
            re_emit_op(
                s,
                if (*s).dotall != 0 {
                    REOP_any as libc::c_int
                } else {
                    REOP_dot as libc::c_int
                },
            );
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as libc::c_int);
            }
            current_block = 12151070351325546249;
        }
        123 => {
            if (*s).is_utf16 != 0 {
                return re_parse_error(
                    s,
                    b"syntax error\0" as *const u8 as *const libc::c_char,
                )
            } else if is_digit(*p.offset(1 as libc::c_int as isize) as libc::c_int) == 0
            {
                current_block = 16980022268469227725;
            } else {
                let mut p1: *const uint8_t = p.offset(1 as libc::c_int as isize);
                parse_digits(&mut p1, TRUE as libc::c_int);
                if *p1 as libc::c_int == ',' as i32 {
                    p1 = p1.offset(1);
                    if is_digit(*p1 as libc::c_int) != 0 {
                        parse_digits(&mut p1, TRUE as libc::c_int);
                    }
                }
                if *p1 as libc::c_int != '}' as i32 {
                    current_block = 16980022268469227725;
                } else {
                    current_block = 13554468014478421449;
                }
            }
        }
        42 | 43 | 63 => {
            current_block = 13554468014478421449;
        }
        40 => {
            let mut pos: libc::c_int = 0;
            let mut capture_index: libc::c_int = 0;
            let mut current_block_82: u64;
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '?' as i32 {
                if *p.offset(2 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                    p = p.offset(3 as libc::c_int as isize);
                    last_atom_start = (*s).byte_code.size as libc::c_int;
                    last_capture_count = (*s).capture_count;
                    let ref mut fresh15 = (*s).buf_ptr;
                    *fresh15 = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as libc::c_int);
                    }
                    p = (*s).buf_ptr;
                    if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                        return -(1 as libc::c_int);
                    }
                    current_block_82 = 12070711452894729854;
                } else {
                    if *p.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32
                        || *p.offset(2 as libc::c_int as isize) as libc::c_int
                            == '!' as i32
                    {
                        is_neg = (*p.offset(2 as libc::c_int as isize) as libc::c_int
                            == '!' as i32) as libc::c_int;
                        is_backward_lookahead = FALSE as libc::c_int;
                        p = p.offset(3 as libc::c_int as isize);
                        current_block_82 = 7084033976171894212;
                    } else if *p.offset(2 as libc::c_int as isize) as libc::c_int
                        == '<' as i32
                        && (*p.offset(3 as libc::c_int as isize) as libc::c_int
                            == '=' as i32
                            || *p.offset(3 as libc::c_int as isize) as libc::c_int
                                == '!' as i32)
                    {
                        pos = 0;
                        is_neg = (*p.offset(3 as libc::c_int as isize) as libc::c_int
                            == '!' as i32) as libc::c_int;
                        is_backward_lookahead = TRUE as libc::c_int;
                        p = p.offset(4 as libc::c_int as isize);
                        current_block_82 = 7084033976171894212;
                    } else {
                        if *p.offset(2 as libc::c_int as isize) as libc::c_int
                            == '<' as i32
                        {
                            p = p.offset(3 as libc::c_int as isize);
                            if re_parse_group_name(
                                ((*s).u.tmp_buf).as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_int,
                                &mut p,
                                (*s).is_utf16,
                            ) != 0
                            {
                                return re_parse_error(
                                    s,
                                    b"invalid group name\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if find_group_name(s, ((*s).u.tmp_buf).as_mut_ptr())
                                > 0 as libc::c_int
                            {
                                return re_parse_error(
                                    s,
                                    b"duplicate group name\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            dbuf_put(
                                &mut (*s).group_names,
                                ((*s).u.tmp_buf).as_mut_ptr() as *mut uint8_t,
                                (strlen(((*s).u.tmp_buf).as_mut_ptr()))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            (*s).has_named_captures = 1 as libc::c_int;
                        } else {
                            return re_parse_error(
                                s,
                                b"invalid group\0" as *const u8 as *const libc::c_char,
                            )
                        }
                        current_block_82 = 451133658865815704;
                    }
                    match current_block_82 {
                        451133658865815704 => {}
                        _ => {
                            if (*s).is_utf16 == 0 && is_backward_lookahead == 0 {
                                last_atom_start = (*s).byte_code.size as libc::c_int;
                                last_capture_count = (*s).capture_count;
                            }
                            pos = re_emit_op_u32(
                                s,
                                REOP_lookahead as libc::c_int + is_neg,
                                0 as libc::c_int as uint32_t,
                            );
                            let ref mut fresh16 = (*s).buf_ptr;
                            *fresh16 = p;
                            if re_parse_disjunction(s, is_backward_lookahead) != 0 {
                                return -(1 as libc::c_int);
                            }
                            p = (*s).buf_ptr;
                            if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                                return -(1 as libc::c_int);
                            }
                            re_emit_op(s, REOP_match as libc::c_int);
                            if dbuf_error(&mut (*s).byte_code) != 0 {
                                return -(1 as libc::c_int);
                            }
                            put_u32(
                                ((*s).byte_code.buf).offset(pos as isize),
                                ((*s).byte_code.size)
                                    .wrapping_sub((pos + 4 as libc::c_int) as libc::c_ulong)
                                    as uint32_t,
                            );
                            current_block_82 = 12070711452894729854;
                        }
                    }
                }
            } else {
                capture_index = 0;
                p = p.offset(1);
                dbuf_putc(&mut (*s).group_names, 0 as libc::c_int as uint8_t);
                current_block_82 = 451133658865815704;
            }
            match current_block_82 {
                451133658865815704 => {
                    if (*s).capture_count >= 255 as libc::c_int {
                        return re_parse_error(
                            s,
                            b"too many captures\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    last_atom_start = (*s).byte_code.size as libc::c_int;
                    last_capture_count = (*s).capture_count;
                    let ref mut fresh17 = (*s).capture_count;
                    let fresh18 = *fresh17;
                    *fresh17 = *fresh17 + 1;
                    capture_index = fresh18;
                    re_emit_op_u8(
                        s,
                        REOP_save_start as libc::c_int + is_backward_dir,
                        capture_index as uint32_t,
                    );
                    let ref mut fresh19 = (*s).buf_ptr;
                    *fresh19 = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as libc::c_int);
                    }
                    p = (*s).buf_ptr;
                    re_emit_op_u8(
                        s,
                        REOP_save_start as libc::c_int + 1 as libc::c_int
                            - is_backward_dir,
                        capture_index as uint32_t,
                    );
                    if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                        return -(1 as libc::c_int);
                    }
                }
                _ => {}
            }
            current_block = 12151070351325546249;
        }
        92 => {
            match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                98 | 66 => {
                    current_block = 5575789995229184048;
                    match current_block {
                        5575789995229184048 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as libc::c_int
                                    + (*p.offset(1 as libc::c_int as isize) as libc::c_int
                                        != 'b' as i32) as libc::c_int,
                            );
                            p = p.offset(2 as libc::c_int as isize);
                            current_block = 12151070351325546249;
                        }
                        10841866045112274702 => {
                            p = p.offset(2 as libc::c_int as isize);
                            c = 0 as libc::c_int;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as libc::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                            } else if *p as libc::c_int >= '0' as i32
                                && *p as libc::c_int <= '7' as i32
                            {
                                let fresh20 = p;
                                p = p.offset(1);
                                c = *fresh20 as libc::c_int - '0' as i32;
                                if *p as libc::c_int >= '0' as i32
                                    && *p as libc::c_int <= '7' as i32
                                {
                                    let fresh21 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as libc::c_int) + *fresh21 as libc::c_int
                                        - '0' as i32;
                                }
                            }
                            current_block = 10029414263193446915;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: libc::c_int = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as libc::c_int as isize) as libc::c_int
                                != '<' as i32
                            {
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                } else {
                                    current_block = 16980022268469227725;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as libc::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 128]>()
                                        as libc::c_ulong as libc::c_int,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8 as *const libc::c_char,
                                        )
                                    } else {
                                        current_block = 16980022268469227725;
                                    }
                                } else {
                                    c = find_group_name(s, ((*s).u.tmp_buf).as_mut_ptr());
                                    if c < 0 as libc::c_int {
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                        );
                                        if c < 0 as libc::c_int {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const libc::c_char,
                                                )
                                            } else {
                                                current_block = 16980022268469227725;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        16980022268469227725 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8659235703914667890;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as libc::c_int);
                            if c < 0 as libc::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    p = q;
                                    if *p as libc::c_int <= '7' as i32 {
                                        c = 0 as libc::c_int;
                                        if *p as libc::c_int <= '3' as i32 {
                                            let fresh22 = p;
                                            p = p.offset(1);
                                            c = *fresh22 as libc::c_int - '0' as i32;
                                        }
                                        if *p as libc::c_int >= '0' as i32
                                            && *p as libc::c_int <= '7' as i32
                                        {
                                            let fresh23 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as libc::c_int) + *fresh23 as libc::c_int
                                                - '0' as i32;
                                            if *p as libc::c_int >= '0' as i32
                                                && *p as libc::c_int <= '7' as i32
                                            {
                                                let fresh24 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as libc::c_int) + *fresh24 as libc::c_int
                                                    - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh25 = p;
                                        p = p.offset(1);
                                        c = *fresh25 as libc::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                }
                                current_block = 10029414263193446915;
                            } else {
                                current_block = 8659235703914667890;
                            }
                        }
                    }
                    match current_block {
                        10029414263193446915 => {}
                        16980022268469227725 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as libc::c_int;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as libc::c_int + is_backward_dir,
                                c as uint32_t,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                107 => {
                    current_block = 17395932908762866334;
                    match current_block {
                        5575789995229184048 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as libc::c_int
                                    + (*p.offset(1 as libc::c_int as isize) as libc::c_int
                                        != 'b' as i32) as libc::c_int,
                            );
                            p = p.offset(2 as libc::c_int as isize);
                            current_block = 12151070351325546249;
                        }
                        10841866045112274702 => {
                            p = p.offset(2 as libc::c_int as isize);
                            c = 0 as libc::c_int;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as libc::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                            } else if *p as libc::c_int >= '0' as i32
                                && *p as libc::c_int <= '7' as i32
                            {
                                let fresh20 = p;
                                p = p.offset(1);
                                c = *fresh20 as libc::c_int - '0' as i32;
                                if *p as libc::c_int >= '0' as i32
                                    && *p as libc::c_int <= '7' as i32
                                {
                                    let fresh21 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as libc::c_int) + *fresh21 as libc::c_int
                                        - '0' as i32;
                                }
                            }
                            current_block = 10029414263193446915;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: libc::c_int = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as libc::c_int as isize) as libc::c_int
                                != '<' as i32
                            {
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                } else {
                                    current_block = 16980022268469227725;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as libc::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 128]>()
                                        as libc::c_ulong as libc::c_int,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8 as *const libc::c_char,
                                        )
                                    } else {
                                        current_block = 16980022268469227725;
                                    }
                                } else {
                                    c = find_group_name(s, ((*s).u.tmp_buf).as_mut_ptr());
                                    if c < 0 as libc::c_int {
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                        );
                                        if c < 0 as libc::c_int {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const libc::c_char,
                                                )
                                            } else {
                                                current_block = 16980022268469227725;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        16980022268469227725 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8659235703914667890;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as libc::c_int);
                            if c < 0 as libc::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    p = q;
                                    if *p as libc::c_int <= '7' as i32 {
                                        c = 0 as libc::c_int;
                                        if *p as libc::c_int <= '3' as i32 {
                                            let fresh22 = p;
                                            p = p.offset(1);
                                            c = *fresh22 as libc::c_int - '0' as i32;
                                        }
                                        if *p as libc::c_int >= '0' as i32
                                            && *p as libc::c_int <= '7' as i32
                                        {
                                            let fresh23 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as libc::c_int) + *fresh23 as libc::c_int
                                                - '0' as i32;
                                            if *p as libc::c_int >= '0' as i32
                                                && *p as libc::c_int <= '7' as i32
                                            {
                                                let fresh24 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as libc::c_int) + *fresh24 as libc::c_int
                                                    - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh25 = p;
                                        p = p.offset(1);
                                        c = *fresh25 as libc::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                }
                                current_block = 10029414263193446915;
                            } else {
                                current_block = 8659235703914667890;
                            }
                        }
                    }
                    match current_block {
                        10029414263193446915 => {}
                        16980022268469227725 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as libc::c_int;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as libc::c_int + is_backward_dir,
                                c as uint32_t,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                48 => {
                    current_block = 10841866045112274702;
                    match current_block {
                        5575789995229184048 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as libc::c_int
                                    + (*p.offset(1 as libc::c_int as isize) as libc::c_int
                                        != 'b' as i32) as libc::c_int,
                            );
                            p = p.offset(2 as libc::c_int as isize);
                            current_block = 12151070351325546249;
                        }
                        10841866045112274702 => {
                            p = p.offset(2 as libc::c_int as isize);
                            c = 0 as libc::c_int;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as libc::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                            } else if *p as libc::c_int >= '0' as i32
                                && *p as libc::c_int <= '7' as i32
                            {
                                let fresh20 = p;
                                p = p.offset(1);
                                c = *fresh20 as libc::c_int - '0' as i32;
                                if *p as libc::c_int >= '0' as i32
                                    && *p as libc::c_int <= '7' as i32
                                {
                                    let fresh21 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as libc::c_int) + *fresh21 as libc::c_int
                                        - '0' as i32;
                                }
                            }
                            current_block = 10029414263193446915;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: libc::c_int = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as libc::c_int as isize) as libc::c_int
                                != '<' as i32
                            {
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                } else {
                                    current_block = 16980022268469227725;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as libc::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 128]>()
                                        as libc::c_ulong as libc::c_int,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8 as *const libc::c_char,
                                        )
                                    } else {
                                        current_block = 16980022268469227725;
                                    }
                                } else {
                                    c = find_group_name(s, ((*s).u.tmp_buf).as_mut_ptr());
                                    if c < 0 as libc::c_int {
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                        );
                                        if c < 0 as libc::c_int {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const libc::c_char,
                                                )
                                            } else {
                                                current_block = 16980022268469227725;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        16980022268469227725 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8659235703914667890;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as libc::c_int);
                            if c < 0 as libc::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    p = q;
                                    if *p as libc::c_int <= '7' as i32 {
                                        c = 0 as libc::c_int;
                                        if *p as libc::c_int <= '3' as i32 {
                                            let fresh22 = p;
                                            p = p.offset(1);
                                            c = *fresh22 as libc::c_int - '0' as i32;
                                        }
                                        if *p as libc::c_int >= '0' as i32
                                            && *p as libc::c_int <= '7' as i32
                                        {
                                            let fresh23 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as libc::c_int) + *fresh23 as libc::c_int
                                                - '0' as i32;
                                            if *p as libc::c_int >= '0' as i32
                                                && *p as libc::c_int <= '7' as i32
                                            {
                                                let fresh24 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as libc::c_int) + *fresh24 as libc::c_int
                                                    - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh25 = p;
                                        p = p.offset(1);
                                        c = *fresh25 as libc::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                }
                                current_block = 10029414263193446915;
                            } else {
                                current_block = 8659235703914667890;
                            }
                        }
                    }
                    match current_block {
                        10029414263193446915 => {}
                        16980022268469227725 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as libc::c_int;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as libc::c_int + is_backward_dir,
                                c as uint32_t,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    current_block = 7337917895049117968;
                    match current_block {
                        5575789995229184048 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as libc::c_int
                                    + (*p.offset(1 as libc::c_int as isize) as libc::c_int
                                        != 'b' as i32) as libc::c_int,
                            );
                            p = p.offset(2 as libc::c_int as isize);
                            current_block = 12151070351325546249;
                        }
                        10841866045112274702 => {
                            p = p.offset(2 as libc::c_int as isize);
                            c = 0 as libc::c_int;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as libc::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                            } else if *p as libc::c_int >= '0' as i32
                                && *p as libc::c_int <= '7' as i32
                            {
                                let fresh20 = p;
                                p = p.offset(1);
                                c = *fresh20 as libc::c_int - '0' as i32;
                                if *p as libc::c_int >= '0' as i32
                                    && *p as libc::c_int <= '7' as i32
                                {
                                    let fresh21 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as libc::c_int) + *fresh21 as libc::c_int
                                        - '0' as i32;
                                }
                            }
                            current_block = 10029414263193446915;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: libc::c_int = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as libc::c_int as isize) as libc::c_int
                                != '<' as i32
                            {
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                } else {
                                    current_block = 16980022268469227725;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as libc::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 128]>()
                                        as libc::c_ulong as libc::c_int,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8 as *const libc::c_char,
                                        )
                                    } else {
                                        current_block = 16980022268469227725;
                                    }
                                } else {
                                    c = find_group_name(s, ((*s).u.tmp_buf).as_mut_ptr());
                                    if c < 0 as libc::c_int {
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                        );
                                        if c < 0 as libc::c_int {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const libc::c_char,
                                                )
                                            } else {
                                                current_block = 16980022268469227725;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        16980022268469227725 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8659235703914667890;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as libc::c_int);
                            if c < 0 as libc::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    p = q;
                                    if *p as libc::c_int <= '7' as i32 {
                                        c = 0 as libc::c_int;
                                        if *p as libc::c_int <= '3' as i32 {
                                            let fresh22 = p;
                                            p = p.offset(1);
                                            c = *fresh22 as libc::c_int - '0' as i32;
                                        }
                                        if *p as libc::c_int >= '0' as i32
                                            && *p as libc::c_int <= '7' as i32
                                        {
                                            let fresh23 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as libc::c_int) + *fresh23 as libc::c_int
                                                - '0' as i32;
                                            if *p as libc::c_int >= '0' as i32
                                                && *p as libc::c_int <= '7' as i32
                                            {
                                                let fresh24 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as libc::c_int) + *fresh24 as libc::c_int
                                                    - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh25 = p;
                                        p = p.offset(1);
                                        c = *fresh25 as libc::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                }
                                current_block = 10029414263193446915;
                            } else {
                                current_block = 8659235703914667890;
                            }
                        }
                    }
                    match current_block {
                        10029414263193446915 => {}
                        16980022268469227725 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as libc::c_int;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as libc::c_int + is_backward_dir,
                                c as uint32_t,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                _ => {
                    current_block = 16980022268469227725;
                }
            }
        }
        91 => {
            last_atom_start = (*s).byte_code.size as libc::c_int;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as libc::c_int);
            }
            if re_parse_char_class(s, &mut p) != 0 {
                return -(1 as libc::c_int);
            }
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as libc::c_int);
            }
            current_block = 12151070351325546249;
        }
        93 | 125 => {
            if (*s).is_utf16 != 0 {
                return re_parse_error(
                    s,
                    b"syntax error\0" as *const u8 as *const libc::c_char,
                );
            }
            current_block = 16980022268469227725;
        }
        _ => {
            current_block = 16980022268469227725;
        }
    }
    match current_block {
        16980022268469227725 => {
            c = get_class_atom(s, cr, &mut p, FALSE as libc::c_int);
            if c < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            current_block = 10029414263193446915;
        }
        13554468014478421449 => {
            return re_parse_error(
                s,
                b"nothing to repeat\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    match current_block {
        10029414263193446915 => {
            last_atom_start = (*s).byte_code.size as libc::c_int;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as libc::c_int);
            }
            if c >= 0x40000000 as libc::c_int {
                let mut ret: libc::c_int = 0;
                ret = re_emit_range(s, cr);
                cr_free(cr);
                if ret != 0 {
                    return -(1 as libc::c_int);
                }
            } else {
                if (*s).ignore_case != 0 {
                    c = lre_canonicalize(c as uint32_t, (*s).is_utf16) as libc::c_int;
                }
                if c <= 0xffff as libc::c_int {
                    re_emit_op_u16(s, REOP_char as libc::c_int, c as uint32_t);
                } else {
                    re_emit_op_u32(s, REOP_char32 as libc::c_int, c as uint32_t);
                }
            }
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as libc::c_int);
            }
        }
        _ => {}
    }
    if last_atom_start >= 0 as libc::c_int {
        c = *p as libc::c_int;
        match c {
            42 => {
                current_block = 21158313410989312;
                match current_block {
                    21158313410989312 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    8784857954897819670 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 1 as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    15819515763622934372 => {
                        p = p.offset(1);
                        quant_min = 1 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                            == 0
                        {
                            if (*s).is_utf16 != 0 {
                                current_block = 12853172943934786507;
                            } else {
                                current_block = 18432964712698998993;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as libc::c_int);
                            quant_max = quant_min;
                            if *p as libc::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as libc::c_int) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as libc::c_int);
                                    if quant_max < quant_min {
                                        current_block = 12853172943934786507;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 0x7fffffff as libc::c_int;
                                    current_block = 6530401058219605690;
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                12853172943934786507 => {}
                                _ => {
                                    if *p as libc::c_int != '}' as i32 && (*s).is_utf16 == 0 {
                                        p = p1_1;
                                        current_block = 18432964712698998993;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as libc::c_int);
                                        }
                                        current_block = 3186158166477254038;
                                    }
                                }
                            }
                        }
                        match current_block {
                            3186158166477254038 => {}
                            18432964712698998993 => {}
                            _ => {
                                return re_parse_error(
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    18432964712698998993 => {}
                    _ => {
                        greedy = TRUE as libc::c_int;
                        if *p as libc::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as libc::c_int;
                        }
                        if last_atom_start < 0 as libc::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if greedy != 0 {
                            let mut len: libc::c_int = 0;
                            let mut pos_0: libc::c_int = 0;
                            if quant_max > 0 as libc::c_int {
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 13534615676823707907;
                                } else {
                                    len = re_is_simple_quantifier(
                                        ((*s).byte_code.buf).offset(last_atom_start as isize),
                                        ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int,
                                    );
                                    if len > 0 as libc::c_int {
                                        re_emit_op(s, REOP_match as libc::c_int);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as libc::c_int,
                                        ) != 0
                                        {
                                            current_block = 13534615676823707907;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh26 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf)
                                                .offset(
                                                    fresh26 as isize,
                                                ) = REOP_simple_greedy_quant as libc::c_int as uint8_t;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                ((*s).byte_code.size)
                                                    .wrapping_sub(last_atom_start as libc::c_ulong)
                                                    .wrapping_sub(17 as libc::c_int as libc::c_ulong)
                                                    as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_min as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_max as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                len as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            current_block = 18432964712698998993;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                13534615676823707907 => {}
                                18432964712698998993 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 13534615676823707907;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                                            ((*s).byte_code.size)
                                                .wrapping_sub(last_atom_start as libc::c_ulong)
                                                as libc::c_int,
                                        ) == 0 as libc::c_int) as libc::c_int;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as libc::c_int;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            18432964712698998993 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: libc::c_int = 0;
                                        let mut pos_1: libc::c_int = 0;
                                        len_0 = ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int;
                                        if quant_min == 0 as libc::c_int {
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as libc::c_int,
                                                ) != 0
                                                {
                                                    current_block = 13534615676823707907;
                                                } else {
                                                    let fresh27 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh27 as isize,
                                                        ) = REOP_save_reset as libc::c_int as uint8_t;
                                                    let fresh28 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(fresh28 as isize) = last_capture_count as uint8_t;
                                                    let fresh29 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh29 as isize,
                                                        ) = ((*s).capture_count - 1 as libc::c_int) as uint8_t;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0 as libc::c_int {
                                                        (*s).byte_code.size = last_atom_start as size_t;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                len_0 as uint32_t,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 0x7fffffff as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                    as uint32_t,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                *((*s).byte_code.buf)
                                                                    .offset(
                                                                        (last_atom_start + 1 as libc::c_int + 4 as libc::c_int)
                                                                            as isize,
                                                                    ) = REOP_push_char_pos as libc::c_int as uint8_t;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as libc::c_int,
                                                    ) != 0
                                                    {
                                                        current_block = 13534615676823707907;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh30 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh30 as isize,
                                                            ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            quant_max as uint32_t,
                                                        );
                                                        pos_1 += 4 as libc::c_int;
                                                        let fresh31 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh31 as isize,
                                                            ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                            as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            (last_atom_start + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as libc::c_int
                                            && quant_max == 0x7fffffff as libc::c_int
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as libc::c_int - greedy,
                                                last_atom_start as uint32_t,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as libc::c_int {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as libc::c_int,
                                            ) != 0
                                            {
                                                current_block = 13534615676823707907;
                                            } else {
                                                *((*s).byte_code.buf)
                                                    .offset(
                                                        last_atom_start as isize,
                                                    ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf)
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as libc::c_int as isize),
                                                    quant_min as uint32_t,
                                                );
                                                last_atom_start += 5 as libc::c_int;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as libc::c_int,
                                                    last_atom_start as uint32_t,
                                                );
                                                re_emit_op(s, REOP_drop as libc::c_int);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0x7fffffff as libc::c_int {
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                as uint32_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(s, REOP_push_char_pos as libc::c_int);
                                                        }
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as libc::c_int,
                                                            (quant_max - quant_min) as uint32_t,
                                                        );
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            pos_1 as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            13534615676823707907 => {}
                                            _ => {
                                                last_atom_start = -(1 as libc::c_int);
                                                current_block = 18432964712698998993;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    18432964712698998993 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            43 => {
                current_block = 15819515763622934372;
                match current_block {
                    21158313410989312 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    8784857954897819670 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 1 as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    15819515763622934372 => {
                        p = p.offset(1);
                        quant_min = 1 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                            == 0
                        {
                            if (*s).is_utf16 != 0 {
                                current_block = 12853172943934786507;
                            } else {
                                current_block = 18432964712698998993;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as libc::c_int);
                            quant_max = quant_min;
                            if *p as libc::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as libc::c_int) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as libc::c_int);
                                    if quant_max < quant_min {
                                        current_block = 12853172943934786507;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 0x7fffffff as libc::c_int;
                                    current_block = 6530401058219605690;
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                12853172943934786507 => {}
                                _ => {
                                    if *p as libc::c_int != '}' as i32 && (*s).is_utf16 == 0 {
                                        p = p1_1;
                                        current_block = 18432964712698998993;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as libc::c_int);
                                        }
                                        current_block = 3186158166477254038;
                                    }
                                }
                            }
                        }
                        match current_block {
                            3186158166477254038 => {}
                            18432964712698998993 => {}
                            _ => {
                                return re_parse_error(
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    18432964712698998993 => {}
                    _ => {
                        greedy = TRUE as libc::c_int;
                        if *p as libc::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as libc::c_int;
                        }
                        if last_atom_start < 0 as libc::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if greedy != 0 {
                            let mut len: libc::c_int = 0;
                            let mut pos_0: libc::c_int = 0;
                            if quant_max > 0 as libc::c_int {
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 13534615676823707907;
                                } else {
                                    len = re_is_simple_quantifier(
                                        ((*s).byte_code.buf).offset(last_atom_start as isize),
                                        ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int,
                                    );
                                    if len > 0 as libc::c_int {
                                        re_emit_op(s, REOP_match as libc::c_int);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as libc::c_int,
                                        ) != 0
                                        {
                                            current_block = 13534615676823707907;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh26 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf)
                                                .offset(
                                                    fresh26 as isize,
                                                ) = REOP_simple_greedy_quant as libc::c_int as uint8_t;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                ((*s).byte_code.size)
                                                    .wrapping_sub(last_atom_start as libc::c_ulong)
                                                    .wrapping_sub(17 as libc::c_int as libc::c_ulong)
                                                    as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_min as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_max as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                len as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            current_block = 18432964712698998993;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                13534615676823707907 => {}
                                18432964712698998993 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 13534615676823707907;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                                            ((*s).byte_code.size)
                                                .wrapping_sub(last_atom_start as libc::c_ulong)
                                                as libc::c_int,
                                        ) == 0 as libc::c_int) as libc::c_int;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as libc::c_int;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            18432964712698998993 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: libc::c_int = 0;
                                        let mut pos_1: libc::c_int = 0;
                                        len_0 = ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int;
                                        if quant_min == 0 as libc::c_int {
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as libc::c_int,
                                                ) != 0
                                                {
                                                    current_block = 13534615676823707907;
                                                } else {
                                                    let fresh27 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh27 as isize,
                                                        ) = REOP_save_reset as libc::c_int as uint8_t;
                                                    let fresh28 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(fresh28 as isize) = last_capture_count as uint8_t;
                                                    let fresh29 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh29 as isize,
                                                        ) = ((*s).capture_count - 1 as libc::c_int) as uint8_t;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0 as libc::c_int {
                                                        (*s).byte_code.size = last_atom_start as size_t;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                len_0 as uint32_t,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 0x7fffffff as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                    as uint32_t,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                *((*s).byte_code.buf)
                                                                    .offset(
                                                                        (last_atom_start + 1 as libc::c_int + 4 as libc::c_int)
                                                                            as isize,
                                                                    ) = REOP_push_char_pos as libc::c_int as uint8_t;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as libc::c_int,
                                                    ) != 0
                                                    {
                                                        current_block = 13534615676823707907;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh30 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh30 as isize,
                                                            ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            quant_max as uint32_t,
                                                        );
                                                        pos_1 += 4 as libc::c_int;
                                                        let fresh31 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh31 as isize,
                                                            ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                            as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            (last_atom_start + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as libc::c_int
                                            && quant_max == 0x7fffffff as libc::c_int
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as libc::c_int - greedy,
                                                last_atom_start as uint32_t,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as libc::c_int {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as libc::c_int,
                                            ) != 0
                                            {
                                                current_block = 13534615676823707907;
                                            } else {
                                                *((*s).byte_code.buf)
                                                    .offset(
                                                        last_atom_start as isize,
                                                    ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf)
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as libc::c_int as isize),
                                                    quant_min as uint32_t,
                                                );
                                                last_atom_start += 5 as libc::c_int;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as libc::c_int,
                                                    last_atom_start as uint32_t,
                                                );
                                                re_emit_op(s, REOP_drop as libc::c_int);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0x7fffffff as libc::c_int {
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                as uint32_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(s, REOP_push_char_pos as libc::c_int);
                                                        }
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as libc::c_int,
                                                            (quant_max - quant_min) as uint32_t,
                                                        );
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            pos_1 as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            13534615676823707907 => {}
                                            _ => {
                                                last_atom_start = -(1 as libc::c_int);
                                                current_block = 18432964712698998993;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    18432964712698998993 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            63 => {
                current_block = 8784857954897819670;
                match current_block {
                    21158313410989312 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    8784857954897819670 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 1 as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    15819515763622934372 => {
                        p = p.offset(1);
                        quant_min = 1 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                            == 0
                        {
                            if (*s).is_utf16 != 0 {
                                current_block = 12853172943934786507;
                            } else {
                                current_block = 18432964712698998993;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as libc::c_int);
                            quant_max = quant_min;
                            if *p as libc::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as libc::c_int) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as libc::c_int);
                                    if quant_max < quant_min {
                                        current_block = 12853172943934786507;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 0x7fffffff as libc::c_int;
                                    current_block = 6530401058219605690;
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                12853172943934786507 => {}
                                _ => {
                                    if *p as libc::c_int != '}' as i32 && (*s).is_utf16 == 0 {
                                        p = p1_1;
                                        current_block = 18432964712698998993;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as libc::c_int);
                                        }
                                        current_block = 3186158166477254038;
                                    }
                                }
                            }
                        }
                        match current_block {
                            3186158166477254038 => {}
                            18432964712698998993 => {}
                            _ => {
                                return re_parse_error(
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    18432964712698998993 => {}
                    _ => {
                        greedy = TRUE as libc::c_int;
                        if *p as libc::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as libc::c_int;
                        }
                        if last_atom_start < 0 as libc::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if greedy != 0 {
                            let mut len: libc::c_int = 0;
                            let mut pos_0: libc::c_int = 0;
                            if quant_max > 0 as libc::c_int {
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 13534615676823707907;
                                } else {
                                    len = re_is_simple_quantifier(
                                        ((*s).byte_code.buf).offset(last_atom_start as isize),
                                        ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int,
                                    );
                                    if len > 0 as libc::c_int {
                                        re_emit_op(s, REOP_match as libc::c_int);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as libc::c_int,
                                        ) != 0
                                        {
                                            current_block = 13534615676823707907;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh26 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf)
                                                .offset(
                                                    fresh26 as isize,
                                                ) = REOP_simple_greedy_quant as libc::c_int as uint8_t;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                ((*s).byte_code.size)
                                                    .wrapping_sub(last_atom_start as libc::c_ulong)
                                                    .wrapping_sub(17 as libc::c_int as libc::c_ulong)
                                                    as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_min as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_max as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                len as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            current_block = 18432964712698998993;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                13534615676823707907 => {}
                                18432964712698998993 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 13534615676823707907;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                                            ((*s).byte_code.size)
                                                .wrapping_sub(last_atom_start as libc::c_ulong)
                                                as libc::c_int,
                                        ) == 0 as libc::c_int) as libc::c_int;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as libc::c_int;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            18432964712698998993 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: libc::c_int = 0;
                                        let mut pos_1: libc::c_int = 0;
                                        len_0 = ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int;
                                        if quant_min == 0 as libc::c_int {
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as libc::c_int,
                                                ) != 0
                                                {
                                                    current_block = 13534615676823707907;
                                                } else {
                                                    let fresh27 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh27 as isize,
                                                        ) = REOP_save_reset as libc::c_int as uint8_t;
                                                    let fresh28 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(fresh28 as isize) = last_capture_count as uint8_t;
                                                    let fresh29 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh29 as isize,
                                                        ) = ((*s).capture_count - 1 as libc::c_int) as uint8_t;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0 as libc::c_int {
                                                        (*s).byte_code.size = last_atom_start as size_t;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                len_0 as uint32_t,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 0x7fffffff as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                    as uint32_t,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                *((*s).byte_code.buf)
                                                                    .offset(
                                                                        (last_atom_start + 1 as libc::c_int + 4 as libc::c_int)
                                                                            as isize,
                                                                    ) = REOP_push_char_pos as libc::c_int as uint8_t;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as libc::c_int,
                                                    ) != 0
                                                    {
                                                        current_block = 13534615676823707907;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh30 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh30 as isize,
                                                            ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            quant_max as uint32_t,
                                                        );
                                                        pos_1 += 4 as libc::c_int;
                                                        let fresh31 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh31 as isize,
                                                            ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                            as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            (last_atom_start + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as libc::c_int
                                            && quant_max == 0x7fffffff as libc::c_int
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as libc::c_int - greedy,
                                                last_atom_start as uint32_t,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as libc::c_int {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as libc::c_int,
                                            ) != 0
                                            {
                                                current_block = 13534615676823707907;
                                            } else {
                                                *((*s).byte_code.buf)
                                                    .offset(
                                                        last_atom_start as isize,
                                                    ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf)
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as libc::c_int as isize),
                                                    quant_min as uint32_t,
                                                );
                                                last_atom_start += 5 as libc::c_int;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as libc::c_int,
                                                    last_atom_start as uint32_t,
                                                );
                                                re_emit_op(s, REOP_drop as libc::c_int);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0x7fffffff as libc::c_int {
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                as uint32_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(s, REOP_push_char_pos as libc::c_int);
                                                        }
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as libc::c_int,
                                                            (quant_max - quant_min) as uint32_t,
                                                        );
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            pos_1 as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            13534615676823707907 => {}
                                            _ => {
                                                last_atom_start = -(1 as libc::c_int);
                                                current_block = 18432964712698998993;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    18432964712698998993 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            123 => {
                current_block = 12227374774078719326;
                match current_block {
                    21158313410989312 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    8784857954897819670 => {
                        p = p.offset(1);
                        quant_min = 0 as libc::c_int;
                        quant_max = 1 as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    15819515763622934372 => {
                        p = p.offset(1);
                        quant_min = 1 as libc::c_int;
                        quant_max = 0x7fffffff as libc::c_int;
                        current_block = 3186158166477254038;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                            == 0
                        {
                            if (*s).is_utf16 != 0 {
                                current_block = 12853172943934786507;
                            } else {
                                current_block = 18432964712698998993;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as libc::c_int);
                            quant_max = quant_min;
                            if *p as libc::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as libc::c_int) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as libc::c_int);
                                    if quant_max < quant_min {
                                        current_block = 12853172943934786507;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 0x7fffffff as libc::c_int;
                                    current_block = 6530401058219605690;
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                12853172943934786507 => {}
                                _ => {
                                    if *p as libc::c_int != '}' as i32 && (*s).is_utf16 == 0 {
                                        p = p1_1;
                                        current_block = 18432964712698998993;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as libc::c_int);
                                        }
                                        current_block = 3186158166477254038;
                                    }
                                }
                            }
                        }
                        match current_block {
                            3186158166477254038 => {}
                            18432964712698998993 => {}
                            _ => {
                                return re_parse_error(
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    18432964712698998993 => {}
                    _ => {
                        greedy = TRUE as libc::c_int;
                        if *p as libc::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as libc::c_int;
                        }
                        if last_atom_start < 0 as libc::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if greedy != 0 {
                            let mut len: libc::c_int = 0;
                            let mut pos_0: libc::c_int = 0;
                            if quant_max > 0 as libc::c_int {
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 13534615676823707907;
                                } else {
                                    len = re_is_simple_quantifier(
                                        ((*s).byte_code.buf).offset(last_atom_start as isize),
                                        ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int,
                                    );
                                    if len > 0 as libc::c_int {
                                        re_emit_op(s, REOP_match as libc::c_int);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as libc::c_int,
                                        ) != 0
                                        {
                                            current_block = 13534615676823707907;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh26 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf)
                                                .offset(
                                                    fresh26 as isize,
                                                ) = REOP_simple_greedy_quant as libc::c_int as uint8_t;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                ((*s).byte_code.size)
                                                    .wrapping_sub(last_atom_start as libc::c_ulong)
                                                    .wrapping_sub(17 as libc::c_int as libc::c_ulong)
                                                    as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_min as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                quant_max as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            put_u32(
                                                &mut *((*s).byte_code.buf).offset(pos_0 as isize),
                                                len as uint32_t,
                                            );
                                            pos_0 += 4 as libc::c_int;
                                            current_block = 18432964712698998993;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                13534615676823707907 => {}
                                18432964712698998993 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 13534615676823707907;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                                            ((*s).byte_code.size)
                                                .wrapping_sub(last_atom_start as libc::c_ulong)
                                                as libc::c_int,
                                        ) == 0 as libc::c_int) as libc::c_int;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as libc::c_int;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            18432964712698998993 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: libc::c_int = 0;
                                        let mut pos_1: libc::c_int = 0;
                                        len_0 = ((*s).byte_code.size)
                                            .wrapping_sub(last_atom_start as libc::c_ulong)
                                            as libc::c_int;
                                        if quant_min == 0 as libc::c_int {
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as libc::c_int,
                                                ) != 0
                                                {
                                                    current_block = 13534615676823707907;
                                                } else {
                                                    let fresh27 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh27 as isize,
                                                        ) = REOP_save_reset as libc::c_int as uint8_t;
                                                    let fresh28 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(fresh28 as isize) = last_capture_count as uint8_t;
                                                    let fresh29 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *((*s).byte_code.buf)
                                                        .offset(
                                                            fresh29 as isize,
                                                        ) = ((*s).capture_count - 1 as libc::c_int) as uint8_t;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0 as libc::c_int {
                                                        (*s).byte_code.size = last_atom_start as size_t;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                len_0 as uint32_t,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 0x7fffffff as libc::c_int {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as libc::c_int + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 13534615676823707907;
                                                        } else {
                                                            *((*s).byte_code.buf)
                                                                .offset(
                                                                    last_atom_start as isize,
                                                                ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                                as uint8_t;
                                                            put_u32(
                                                                ((*s).byte_code.buf)
                                                                    .offset(last_atom_start as isize)
                                                                    .offset(1 as libc::c_int as isize),
                                                                (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                    as uint32_t,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                *((*s).byte_code.buf)
                                                                    .offset(
                                                                        (last_atom_start + 1 as libc::c_int + 4 as libc::c_int)
                                                                            as isize,
                                                                    ) = REOP_push_char_pos as libc::c_int as uint8_t;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as libc::c_int,
                                                                    last_atom_start as uint32_t,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as libc::c_int,
                                                    ) != 0
                                                    {
                                                        current_block = 13534615676823707907;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh30 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh30 as isize,
                                                            ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            quant_max as uint32_t,
                                                        );
                                                        pos_1 += 4 as libc::c_int;
                                                        let fresh31 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                fresh31 as isize,
                                                            ) = (REOP_split_goto_first as libc::c_int + greedy)
                                                            as uint8_t;
                                                        put_u32(
                                                            ((*s).byte_code.buf).offset(pos_1 as isize),
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            (last_atom_start + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as libc::c_int
                                            && quant_max == 0x7fffffff as libc::c_int
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as libc::c_int - greedy,
                                                last_atom_start as uint32_t,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as libc::c_int {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as libc::c_int,
                                            ) != 0
                                            {
                                                current_block = 13534615676823707907;
                                            } else {
                                                *((*s).byte_code.buf)
                                                    .offset(
                                                        last_atom_start as isize,
                                                    ) = REOP_push_i32 as libc::c_int as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf)
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as libc::c_int as isize),
                                                    quant_min as uint32_t,
                                                );
                                                last_atom_start += 5 as libc::c_int;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as libc::c_int,
                                                    last_atom_start as uint32_t,
                                                );
                                                re_emit_op(s, REOP_drop as libc::c_int);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                13534615676823707907 => {}
                                                _ => {
                                                    if quant_max == 0x7fffffff as libc::c_int {
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int + add_zero_advance_check)
                                                                as uint32_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(s, REOP_push_char_pos as libc::c_int);
                                                        }
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as libc::c_int,
                                                                pos_1 as uint32_t,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as libc::c_int,
                                                            (quant_max - quant_min) as uint32_t,
                                                        );
                                                        pos_1 = (*s).byte_code.size as libc::c_int;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as libc::c_int + greedy,
                                                            (len_0 + 5 as libc::c_int) as uint32_t,
                                                        );
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as size_t,
                                                            len_0 as size_t,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as libc::c_int,
                                                            pos_1 as uint32_t,
                                                        );
                                                        re_emit_op(s, REOP_drop as libc::c_int);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            13534615676823707907 => {}
                                            _ => {
                                                last_atom_start = -(1 as libc::c_int);
                                                current_block = 18432964712698998993;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    18432964712698998993 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let ref mut fresh32 = (*s).buf_ptr;
    *fresh32 = p;
    return 0 as libc::c_int;
}
unsafe extern "C" fn re_parse_alternative(
    mut s: *mut REParseState,
    mut is_backward_dir: BOOL,
) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut ret: libc::c_int = 0;
    let mut start: size_t = 0;
    let mut term_start: size_t = 0;
    let mut end: size_t = 0;
    let mut term_size: size_t = 0;
    start = (*s).byte_code.size;
    loop {
        p = (*s).buf_ptr;
        if p >= (*s).buf_end {
            break;
        }
        if *p as libc::c_int == '|' as i32 || *p as libc::c_int == ')' as i32 {
            break;
        }
        term_start = (*s).byte_code.size;
        ret = re_parse_term(s, is_backward_dir);
        if ret != 0 {
            return ret;
        }
        if is_backward_dir != 0 {
            end = (*s).byte_code.size;
            term_size = end.wrapping_sub(term_start);
            if dbuf_realloc(&mut (*s).byte_code, end.wrapping_add(term_size)) != 0 {
                return -(1 as libc::c_int);
            }
            memmove(
                ((*s).byte_code.buf).offset(start as isize).offset(term_size as isize)
                    as *mut libc::c_void,
                ((*s).byte_code.buf).offset(start as isize) as *const libc::c_void,
                end.wrapping_sub(start),
            );
            memcpy(
                ((*s).byte_code.buf).offset(start as isize) as *mut libc::c_void,
                ((*s).byte_code.buf).offset(end as isize) as *const libc::c_void,
                term_size,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn re_parse_disjunction(
    mut s: *mut REParseState,
    mut is_backward_dir: BOOL,
) -> libc::c_int {
    let mut start: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    if lre_check_stack_overflow((*s).opaque, 0 as libc::c_int as size_t) != 0 {
        return re_parse_error(
            s,
            b"stack overflow\0" as *const u8 as *const libc::c_char,
        );
    }
    start = (*s).byte_code.size as libc::c_int;
    if re_parse_alternative(s, is_backward_dir) != 0 {
        return -(1 as libc::c_int);
    }
    while *(*s).buf_ptr as libc::c_int == '|' as i32 {
        let ref mut fresh33 = (*s).buf_ptr;
        *fresh33 = (*fresh33).offset(1);
        len = ((*s).byte_code.size).wrapping_sub(start as libc::c_ulong) as libc::c_int;
        if dbuf_insert(&mut (*s).byte_code, start, 5 as libc::c_int) != 0 {
            return re_parse_out_of_memory(s);
        }
        *((*s).byte_code.buf)
            .offset(start as isize) = REOP_split_next_first as libc::c_int as uint8_t;
        put_u32(
            ((*s).byte_code.buf)
                .offset(start as isize)
                .offset(1 as libc::c_int as isize),
            (len + 5 as libc::c_int) as uint32_t,
        );
        pos = re_emit_op_u32(s, REOP_goto as libc::c_int, 0 as libc::c_int as uint32_t);
        if re_parse_alternative(s, is_backward_dir) != 0 {
            return -(1 as libc::c_int);
        }
        len = ((*s).byte_code.size)
            .wrapping_sub((pos + 4 as libc::c_int) as libc::c_ulong) as libc::c_int;
        put_u32(((*s).byte_code.buf).offset(pos as isize), len as uint32_t);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compute_stack_size(
    mut bc_buf: *const uint8_t,
    mut bc_buf_len: libc::c_int,
) -> libc::c_int {
    let mut stack_size: libc::c_int = 0;
    let mut stack_size_max: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut opcode: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut val: uint32_t = 0;
    stack_size = 0 as libc::c_int;
    stack_size_max = 0 as libc::c_int;
    bc_buf = bc_buf.offset(7 as libc::c_int as isize);
    bc_buf_len -= 7 as libc::c_int;
    pos = 0 as libc::c_int;
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as libc::c_int;
        len = reopcode_info[opcode as usize].size as libc::c_int;
        assert((opcode < REOP_COUNT as libc::c_int) as libc::c_int);
        assert((pos + len <= bc_buf_len) as libc::c_int);
        match opcode {
            15 | 25 => {
                stack_size += 1;
                if stack_size > stack_size_max {
                    if stack_size > 255 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    stack_size_max = stack_size;
                }
            }
            16 | 26 => {
                assert((stack_size > 0 as libc::c_int) as libc::c_int);
                stack_size -= 1;
            }
            21 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as libc::c_int as isize),
                );
                len = (len as libc::c_uint)
                    .wrapping_add(val.wrapping_mul(4 as libc::c_int as libc::c_uint))
                    as libc::c_int as libc::c_int;
            }
            22 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as libc::c_int as isize),
                );
                len = (len as libc::c_uint)
                    .wrapping_add(val.wrapping_mul(8 as libc::c_int as libc::c_uint))
                    as libc::c_int as libc::c_int;
            }
            _ => {}
        }
        pos += len;
    }
    return stack_size_max;
}
#[no_mangle]
pub unsafe extern "C" fn lre_compile(
    mut plen: *mut libc::c_int,
    mut error_msg: *mut libc::c_char,
    mut error_msg_size: libc::c_int,
    mut buf: *const libc::c_char,
    mut buf_len: size_t,
    mut re_flags: libc::c_int,
    mut opaque: *mut libc::c_void,
) -> *mut uint8_t {
    let mut s_s: REParseState = REParseState {
        byte_code: DynBuf {
            buf: 0 as *mut uint8_t,
            size: 0,
            allocated_size: 0,
            error: 0,
            realloc_func: None,
            opaque: 0 as *mut libc::c_void,
        },
        buf_ptr: 0 as *const uint8_t,
        buf_end: 0 as *const uint8_t,
        buf_start: 0 as *const uint8_t,
        re_flags: 0,
        is_utf16: 0,
        ignore_case: 0,
        dotall: 0,
        capture_count: 0,
        total_capture_count: 0,
        has_named_captures: 0,
        opaque: 0 as *mut libc::c_void,
        group_names: DynBuf {
            buf: 0 as *mut uint8_t,
            size: 0,
            allocated_size: 0,
            error: 0,
            realloc_func: None,
            opaque: 0 as *mut libc::c_void,
        },
        u: C2RustUnnamed_1 {
            error_msg: [0; 128],
        },
    };
    let mut s: *mut REParseState = &mut s_s;
    let mut stack_size: libc::c_int = 0;
    let mut is_sticky: BOOL = 0;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<REParseState>() as libc::c_ulong,
    );
    let ref mut fresh34 = (*s).opaque;
    *fresh34 = opaque;
    let ref mut fresh35 = (*s).buf_ptr;
    *fresh35 = buf as *const uint8_t;
    let ref mut fresh36 = (*s).buf_end;
    *fresh36 = ((*s).buf_ptr).offset(buf_len as isize);
    let ref mut fresh37 = (*s).buf_start;
    *fresh37 = (*s).buf_ptr;
    (*s).re_flags = re_flags;
    (*s)
        .is_utf16 = (re_flags & (1 as libc::c_int) << 4 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    is_sticky = (re_flags & (1 as libc::c_int) << 5 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
    (*s)
        .ignore_case = (re_flags & (1 as libc::c_int) << 1 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    (*s)
        .dotall = (re_flags & (1 as libc::c_int) << 3 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
    (*s).capture_count = 1 as libc::c_int;
    (*s).total_capture_count = -(1 as libc::c_int);
    (*s).has_named_captures = -(1 as libc::c_int);
    dbuf_init2(
        &mut (*s).byte_code,
        opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                ) -> *mut libc::c_void,
        ),
    );
    dbuf_init2(
        &mut (*s).group_names,
        opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                ) -> *mut libc::c_void,
        ),
    );
    dbuf_putc(&mut (*s).byte_code, re_flags as uint8_t);
    dbuf_putc(&mut (*s).byte_code, 0 as libc::c_int as uint8_t);
    dbuf_putc(&mut (*s).byte_code, 0 as libc::c_int as uint8_t);
    dbuf_put_u32(&mut (*s).byte_code, 0 as libc::c_int as uint32_t);
    if is_sticky == 0 {
        re_emit_op_u32(
            s,
            REOP_split_goto_first as libc::c_int,
            (1 as libc::c_int + 5 as libc::c_int) as uint32_t,
        );
        re_emit_op(s, REOP_any as libc::c_int);
        re_emit_op_u32(
            s,
            REOP_goto as libc::c_int,
            -(5 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int) as uint32_t,
        );
    }
    re_emit_op_u8(s, REOP_save_start as libc::c_int, 0 as libc::c_int as uint32_t);
    if !(re_parse_disjunction(s, FALSE as libc::c_int) != 0) {
        re_emit_op_u8(s, REOP_save_end as libc::c_int, 0 as libc::c_int as uint32_t);
        re_emit_op(s, REOP_match as libc::c_int);
        if *(*s).buf_ptr as libc::c_int != '\0' as i32 {
            re_parse_error(
                s,
                b"extraneous characters at the end\0" as *const u8 as *const libc::c_char,
            );
        } else if dbuf_error(&mut (*s).byte_code) != 0 {
            re_parse_out_of_memory(s);
        } else {
            stack_size = compute_stack_size(
                (*s).byte_code.buf,
                (*s).byte_code.size as libc::c_int,
            );
            if stack_size < 0 as libc::c_int {
                re_parse_error(
                    s,
                    b"too many imbricated quantifiers\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                *((*s).byte_code.buf)
                    .offset(1 as libc::c_int as isize) = (*s).capture_count as uint8_t;
                *((*s).byte_code.buf)
                    .offset(2 as libc::c_int as isize) = stack_size as uint8_t;
                put_u32(
                    ((*s).byte_code.buf).offset(3 as libc::c_int as isize),
                    ((*s).byte_code.size).wrapping_sub(7 as libc::c_int as libc::c_ulong)
                        as uint32_t,
                );
                if (*s).group_names.size
                    > ((*s).capture_count - 1 as libc::c_int) as libc::c_ulong
                {
                    dbuf_put(
                        &mut (*s).byte_code,
                        (*s).group_names.buf,
                        (*s).group_names.size,
                    );
                    let ref mut fresh38 = *((*s).byte_code.buf)
                        .offset(0 as libc::c_int as isize);
                    *fresh38 = (*fresh38 as libc::c_int
                        | (1 as libc::c_int) << 7 as libc::c_int) as uint8_t;
                }
                dbuf_free(&mut (*s).group_names);
                *error_msg
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                *plen = (*s).byte_code.size as libc::c_int;
                return (*s).byte_code.buf;
            }
        }
    }
    dbuf_free(&mut (*s).byte_code);
    dbuf_free(&mut (*s).group_names);
    pstrcpy(error_msg, error_msg_size, ((*s).u.error_msg).as_mut_ptr());
    *plen = 0 as libc::c_int;
    return 0 as *mut uint8_t;
}
unsafe extern "C" fn is_line_terminator(mut c: uint32_t) -> BOOL {
    return (c == '\n' as i32 as libc::c_uint || c == '\r' as i32 as libc::c_uint
        || c == 0x2028 as libc::c_int as libc::c_uint
        || c == 0x2029 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn is_word_char(mut c: uint32_t) -> BOOL {
    return (c >= '0' as i32 as libc::c_uint && c <= '9' as i32 as libc::c_uint
        || c >= 'a' as i32 as libc::c_uint && c <= 'z' as i32 as libc::c_uint
        || c >= 'A' as i32 as libc::c_uint && c <= 'Z' as i32 as libc::c_uint
        || c == '_' as i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn push_state(
    mut s: *mut REExecContext,
    mut capture: *mut *mut uint8_t,
    mut stack: *mut StackInt,
    mut stack_len: size_t,
    mut pc: *const uint8_t,
    mut cptr: *const uint8_t,
    mut type_0: REExecStateEnum,
    mut count: size_t,
) -> libc::c_int {
    let mut rs: *mut REExecState = 0 as *mut REExecState;
    let mut new_stack: *mut uint8_t = 0 as *mut uint8_t;
    let mut new_size: size_t = 0;
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut stack_buf: *mut StackInt = 0 as *mut StackInt;
    if (((*s).state_stack_len).wrapping_add(1 as libc::c_int as libc::c_ulong)
        > (*s).state_stack_size) as libc::c_int as libc::c_long != 0
    {
        new_size = ((*s).state_stack_size)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if new_size < 8 as libc::c_int as libc::c_ulong {
            new_size = 8 as libc::c_int as size_t;
        }
        new_stack = lre_realloc(
            (*s).opaque,
            (*s).state_stack as *mut libc::c_void,
            new_size.wrapping_mul((*s).state_size),
        ) as *mut uint8_t;
        if new_stack.is_null() {
            return -(1 as libc::c_int);
        }
        (*s).state_stack_size = new_size;
        let ref mut fresh39 = (*s).state_stack;
        *fresh39 = new_stack;
    }
    rs = ((*s).state_stack)
        .offset(((*s).state_stack_len).wrapping_mul((*s).state_size) as isize)
        as *mut REExecState;
    let ref mut fresh40 = (*s).state_stack_len;
    *fresh40 = (*fresh40).wrapping_add(1);
    (*rs).set_type_0(type_0);
    (*rs).count = count;
    (*rs).stack_len = stack_len as uint8_t;
    let ref mut fresh41 = (*rs).cptr;
    *fresh41 = cptr;
    let ref mut fresh42 = (*rs).pc;
    *fresh42 = pc;
    n = (2 as libc::c_int * (*s).capture_count) as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let ref mut fresh43 = *((*rs).buf).as_mut_ptr().offset(i as isize);
        *fresh43 = *capture.offset(i as isize) as *mut libc::c_void;
        i = i.wrapping_add(1);
    }
    stack_buf = ((*rs).buf).as_mut_ptr().offset(n as isize) as *mut StackInt;
    i = 0 as libc::c_int as size_t;
    while i < stack_len {
        *stack_buf.offset(i as isize) = *stack.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lre_exec_backtrack(
    mut s: *mut REExecContext,
    mut capture: *mut *mut uint8_t,
    mut stack: *mut StackInt,
    mut stack_len: libc::c_int,
    mut pc: *const uint8_t,
    mut cptr: *const uint8_t,
    mut no_recurse: BOOL,
) -> intptr_t {
    let mut rs: *mut REExecState = 0 as *mut REExecState;
    let mut current_block: u64;
    let mut opcode: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut cbuf_type: libc::c_int = 0;
    let mut val: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut cbuf_end: *const uint8_t = 0 as *const uint8_t;
    cbuf_type = (*s).cbuf_type;
    cbuf_end = (*s).cbuf_end;
    's_27: loop {
        let fresh44 = pc;
        pc = pc.offset(1);
        opcode = *fresh44 as libc::c_int;
        match opcode {
            10 => {
                rs = 0 as *mut REExecState;
                if no_recurse != 0 {
                    return cptr as intptr_t;
                }
                ret = 1 as libc::c_int;
                current_block = 4409060704812261523;
            }
            2 => {
                val = get_u32(pc);
                pc = pc.offset(4 as libc::c_int as isize);
                current_block = 12575669259942314375;
            }
            1 => {
                val = get_u16(pc);
                pc = pc.offset(2 as libc::c_int as isize);
                current_block = 12575669259942314375;
            }
            8 | 9 => {
                let mut pc1: *const uint8_t = 0 as *const uint8_t;
                val = get_u32(pc);
                pc = pc.offset(4 as libc::c_int as isize);
                if opcode == REOP_split_next_first as libc::c_int {
                    pc1 = pc.offset(val as libc::c_int as isize);
                } else {
                    pc1 = pc;
                    pc = pc.offset(val as libc::c_int as isize);
                }
                ret = push_state(
                    s,
                    capture,
                    stack,
                    stack_len as size_t,
                    pc1,
                    cptr,
                    RE_EXEC_STATE_SPLIT,
                    0 as libc::c_int as size_t,
                );
                if ret < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                continue;
            }
            23 | 24 => {
                val = get_u32(pc);
                pc = pc.offset(4 as libc::c_int as isize);
                ret = push_state(
                    s,
                    capture,
                    stack,
                    stack_len as size_t,
                    pc.offset(val as libc::c_int as isize),
                    cptr,
                    (RE_EXEC_STATE_LOOKAHEAD as libc::c_int + opcode
                        - REOP_lookahead as libc::c_int) as REExecStateEnum,
                    0 as libc::c_int as size_t,
                );
                if ret < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                continue;
            }
            7 => {
                val = get_u32(pc);
                pc = pc.offset((4 as libc::c_int + val as libc::c_int) as isize);
                continue;
            }
            5 => {
                if cptr == (*s).cbuf {
                    continue;
                }
                if (*s).multi_line == 0 {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        c = *cptr.offset(-(1 as libc::c_int) as isize) as uint32_t;
                    } else {
                        let mut __c1_0: uint32_t = 0;
                        c = *(cptr as *mut uint16_t).offset(-(1 as libc::c_int) as isize)
                            as uint32_t;
                        if c >= 0xdc00 as libc::c_int as libc::c_uint
                            && c < 0xe000 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int
                            && cptr.offset(-(4 as libc::c_int as isize)) >= (*s).cbuf
                        {
                            __c1_0 = *(cptr as *mut uint16_t)
                                .offset(-(2 as libc::c_int) as isize) as uint32_t;
                            if __c1_0 >= 0xd800 as libc::c_int as libc::c_uint
                                && __c1_0 < 0xdc00 as libc::c_int as libc::c_uint
                            {
                                c = ((__c1_0 & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | c & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                            }
                        }
                    }
                    if !(is_line_terminator(c) == 0) {
                        continue;
                    }
                    current_block = 13870927760773930552;
                }
            }
            6 => {
                if cptr == cbuf_end {
                    continue;
                }
                if (*s).multi_line == 0 {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        c = *cptr.offset(0 as libc::c_int as isize) as uint32_t;
                    } else {
                        let mut __c1_1: uint32_t = 0;
                        c = *(cptr as *mut uint16_t).offset(0 as libc::c_int as isize)
                            as uint32_t;
                        if c >= 0xd800 as libc::c_int as libc::c_uint
                            && c < 0xdc00 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int
                            && cptr.offset(2 as libc::c_int as isize) < cbuf_end
                        {
                            __c1_1 = *(cptr as *mut uint16_t)
                                .offset(1 as libc::c_int as isize) as uint32_t;
                            if __c1_1 >= 0xdc00 as libc::c_int as libc::c_uint
                                && __c1_1 < 0xe000 as libc::c_int as libc::c_uint
                            {
                                c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | __c1_1 & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                            }
                        }
                    }
                    if !(is_line_terminator(c) == 0) {
                        continue;
                    }
                    current_block = 13870927760773930552;
                }
            }
            3 => {
                if cptr == cbuf_end {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        let fresh51 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh51 as uint32_t;
                    } else {
                        let mut __c1_2: uint32_t = 0;
                        c = *(cptr as *mut uint16_t) as uint32_t;
                        cptr = cptr.offset(2 as libc::c_int as isize);
                        if c >= 0xd800 as libc::c_int as libc::c_uint
                            && c < 0xdc00 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int && cptr < cbuf_end
                        {
                            __c1_2 = *(cptr as *mut uint16_t) as uint32_t;
                            if __c1_2 >= 0xdc00 as libc::c_int as libc::c_uint
                                && __c1_2 < 0xe000 as libc::c_int as libc::c_uint
                            {
                                c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | __c1_2 & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                cptr = cptr.offset(2 as libc::c_int as isize);
                            }
                        }
                    }
                    if !(is_line_terminator(c) != 0) {
                        continue;
                    }
                    current_block = 13870927760773930552;
                }
            }
            4 => {
                if cptr == cbuf_end {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        let fresh52 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh52 as uint32_t;
                    } else {
                        let mut __c1_3: uint32_t = 0;
                        c = *(cptr as *mut uint16_t) as uint32_t;
                        cptr = cptr.offset(2 as libc::c_int as isize);
                        if c >= 0xd800 as libc::c_int as libc::c_uint
                            && c < 0xdc00 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int && cptr < cbuf_end
                        {
                            __c1_3 = *(cptr as *mut uint16_t) as uint32_t;
                            if __c1_3 >= 0xdc00 as libc::c_int as libc::c_uint
                                && __c1_3 < 0xe000 as libc::c_int as libc::c_uint
                            {
                                c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | __c1_3 & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                cptr = cptr.offset(2 as libc::c_int as isize);
                            }
                        }
                    }
                    continue;
                }
            }
            11 | 12 => {
                let fresh53 = pc;
                pc = pc.offset(1);
                val = *fresh53 as uint32_t;
                assert((val < (*s).capture_count as libc::c_uint) as libc::c_int);
                let ref mut fresh54 = *capture
                    .offset(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(val)
                            .wrapping_add(opcode as libc::c_uint)
                            .wrapping_sub(REOP_save_start as libc::c_int as libc::c_uint)
                            as isize,
                    );
                *fresh54 = cptr as *mut uint8_t;
                continue;
            }
            13 => {
                let mut val2: uint32_t = 0;
                val = *pc.offset(0 as libc::c_int as isize) as uint32_t;
                val2 = *pc.offset(1 as libc::c_int as isize) as uint32_t;
                pc = pc.offset(2 as libc::c_int as isize);
                assert((val2 < (*s).capture_count as libc::c_uint) as libc::c_int);
                while val <= val2 {
                    let ref mut fresh55 = *capture
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_mul(val) as isize,
                        );
                    *fresh55 = 0 as *mut uint8_t;
                    let ref mut fresh56 = *capture
                        .offset(
                            (2 as libc::c_int as libc::c_uint)
                                .wrapping_mul(val)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        );
                    *fresh56 = 0 as *mut uint8_t;
                    val = val.wrapping_add(1);
                }
                continue;
            }
            15 => {
                val = get_u32(pc);
                pc = pc.offset(4 as libc::c_int as isize);
                let fresh57 = stack_len;
                stack_len = stack_len + 1;
                *stack.offset(fresh57 as isize) = val;
                continue;
            }
            16 => {
                stack_len -= 1;
                continue;
            }
            14 => {
                val = get_u32(pc);
                pc = pc.offset(4 as libc::c_int as isize);
                let ref mut fresh58 = *stack
                    .offset((stack_len - 1 as libc::c_int) as isize);
                *fresh58 = (*fresh58).wrapping_sub(1);
                if *fresh58 != 0 as libc::c_int as libc::c_uint {
                    pc = pc.offset(val as libc::c_int as isize);
                }
                continue;
            }
            25 => {
                let fresh59 = stack_len;
                stack_len = stack_len + 1;
                *stack.offset(fresh59 as isize) = cptr as uintptr_t;
                continue;
            }
            26 => {
                val = get_u32(pc);
                pc = pc.offset(4 as libc::c_int as isize);
                stack_len -= 1;
                if *stack.offset(stack_len as isize) != cptr as uintptr_t {
                    pc = pc.offset(val as libc::c_int as isize);
                }
                continue;
            }
            17 | 18 => {
                let mut v1: BOOL = 0;
                let mut v2: BOOL = 0;
                if cptr == (*s).cbuf {
                    v1 = FALSE as libc::c_int;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        c = *cptr.offset(-(1 as libc::c_int) as isize) as uint32_t;
                    } else {
                        let mut __c1_4: uint32_t = 0;
                        c = *(cptr as *mut uint16_t).offset(-(1 as libc::c_int) as isize)
                            as uint32_t;
                        if c >= 0xdc00 as libc::c_int as libc::c_uint
                            && c < 0xe000 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int
                            && cptr.offset(-(4 as libc::c_int as isize)) >= (*s).cbuf
                        {
                            __c1_4 = *(cptr as *mut uint16_t)
                                .offset(-(2 as libc::c_int) as isize) as uint32_t;
                            if __c1_4 >= 0xd800 as libc::c_int as libc::c_uint
                                && __c1_4 < 0xdc00 as libc::c_int as libc::c_uint
                            {
                                c = ((__c1_4 & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | c & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                            }
                        }
                    }
                    v1 = is_word_char(c);
                }
                if cptr >= cbuf_end {
                    v2 = FALSE as libc::c_int;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        c = *cptr.offset(0 as libc::c_int as isize) as uint32_t;
                    } else {
                        let mut __c1_5: uint32_t = 0;
                        c = *(cptr as *mut uint16_t).offset(0 as libc::c_int as isize)
                            as uint32_t;
                        if c >= 0xd800 as libc::c_int as libc::c_uint
                            && c < 0xdc00 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int
                            && cptr.offset(2 as libc::c_int as isize) < cbuf_end
                        {
                            __c1_5 = *(cptr as *mut uint16_t)
                                .offset(1 as libc::c_int as isize) as uint32_t;
                            if __c1_5 >= 0xdc00 as libc::c_int as libc::c_uint
                                && __c1_5 < 0xe000 as libc::c_int as libc::c_uint
                            {
                                c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | __c1_5 & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                            }
                        }
                    }
                    v2 = is_word_char(c);
                }
                if !(v1 ^ v2 ^ REOP_not_word_boundary as libc::c_int - opcode != 0) {
                    continue;
                }
                current_block = 13870927760773930552;
            }
            19 | 20 => {
                let mut cptr1: *const uint8_t = 0 as *const uint8_t;
                let mut cptr1_end: *const uint8_t = 0 as *const uint8_t;
                let mut cptr1_start: *const uint8_t = 0 as *const uint8_t;
                let mut c1: uint32_t = 0;
                let mut c2: uint32_t = 0;
                let fresh60 = pc;
                pc = pc.offset(1);
                val = *fresh60 as uint32_t;
                if val >= (*s).capture_count as libc::c_uint {
                    current_block = 13870927760773930552;
                } else {
                    cptr1_start = *capture
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_mul(val) as isize,
                        );
                    cptr1_end = *capture
                        .offset(
                            (2 as libc::c_int as libc::c_uint)
                                .wrapping_mul(val)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        );
                    if cptr1_start.is_null() || cptr1_end.is_null() {
                        continue;
                    }
                    if opcode == REOP_back_reference as libc::c_int {
                        cptr1 = cptr1_start;
                        loop {
                            if !(cptr1 < cptr1_end) {
                                continue 's_27;
                            }
                            if cptr >= cbuf_end {
                                break;
                            }
                            if cbuf_type == 0 as libc::c_int {
                                let fresh61 = cptr1;
                                cptr1 = cptr1.offset(1);
                                c1 = *fresh61 as uint32_t;
                            } else {
                                let mut __c1_6: uint32_t = 0;
                                c1 = *(cptr1 as *mut uint16_t) as uint32_t;
                                cptr1 = cptr1.offset(2 as libc::c_int as isize);
                                if c1 >= 0xd800 as libc::c_int as libc::c_uint
                                    && c1 < 0xdc00 as libc::c_int as libc::c_uint
                                    && cbuf_type == 2 as libc::c_int && cptr1 < cptr1_end
                                {
                                    __c1_6 = *(cptr1 as *mut uint16_t) as uint32_t;
                                    if __c1_6 >= 0xdc00 as libc::c_int as libc::c_uint
                                        && __c1_6 < 0xe000 as libc::c_int as libc::c_uint
                                    {
                                        c1 = ((c1 & 0x3ff as libc::c_int as libc::c_uint)
                                            << 10 as libc::c_int
                                            | __c1_6 & 0x3ff as libc::c_int as libc::c_uint)
                                            .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                        cptr1 = cptr1.offset(2 as libc::c_int as isize);
                                    }
                                }
                            }
                            if cbuf_type == 0 as libc::c_int {
                                let fresh62 = cptr;
                                cptr = cptr.offset(1);
                                c2 = *fresh62 as uint32_t;
                            } else {
                                let mut __c1_7: uint32_t = 0;
                                c2 = *(cptr as *mut uint16_t) as uint32_t;
                                cptr = cptr.offset(2 as libc::c_int as isize);
                                if c2 >= 0xd800 as libc::c_int as libc::c_uint
                                    && c2 < 0xdc00 as libc::c_int as libc::c_uint
                                    && cbuf_type == 2 as libc::c_int && cptr < cbuf_end
                                {
                                    __c1_7 = *(cptr as *mut uint16_t) as uint32_t;
                                    if __c1_7 >= 0xdc00 as libc::c_int as libc::c_uint
                                        && __c1_7 < 0xe000 as libc::c_int as libc::c_uint
                                    {
                                        c2 = ((c2 & 0x3ff as libc::c_int as libc::c_uint)
                                            << 10 as libc::c_int
                                            | __c1_7 & 0x3ff as libc::c_int as libc::c_uint)
                                            .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                        cptr = cptr.offset(2 as libc::c_int as isize);
                                    }
                                }
                            }
                            if (*s).ignore_case != 0 {
                                c1 = lre_canonicalize(c1, (*s).is_utf16);
                                c2 = lre_canonicalize(c2, (*s).is_utf16);
                            }
                            if c1 != c2 {
                                break;
                            }
                        }
                    } else {
                        cptr1 = cptr1_end;
                        loop {
                            if !(cptr1 > cptr1_start) {
                                continue 's_27;
                            }
                            if cptr == (*s).cbuf {
                                break;
                            }
                            if cbuf_type == 0 as libc::c_int {
                                cptr1 = cptr1.offset(-1);
                                c1 = *cptr1.offset(0 as libc::c_int as isize) as uint32_t;
                            } else {
                                let mut __c1_8: uint32_t = 0;
                                cptr1 = cptr1.offset(-(2 as libc::c_int as isize));
                                c1 = *(cptr1 as *mut uint16_t)
                                    .offset(0 as libc::c_int as isize) as uint32_t;
                                if c1 >= 0xdc00 as libc::c_int as libc::c_uint
                                    && c1 < 0xe000 as libc::c_int as libc::c_uint
                                    && cbuf_type == 2 as libc::c_int && cptr1 > cptr1_start
                                {
                                    __c1_8 = *(cptr1 as *mut uint16_t)
                                        .offset(-(1 as libc::c_int) as isize) as uint32_t;
                                    if __c1_8 >= 0xd800 as libc::c_int as libc::c_uint
                                        && __c1_8 < 0xdc00 as libc::c_int as libc::c_uint
                                    {
                                        cptr1 = cptr1.offset(-(2 as libc::c_int as isize));
                                        c1 = ((__c1_8 & 0x3ff as libc::c_int as libc::c_uint)
                                            << 10 as libc::c_int
                                            | c1 & 0x3ff as libc::c_int as libc::c_uint)
                                            .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                    }
                                }
                            }
                            if cbuf_type == 0 as libc::c_int {
                                cptr = cptr.offset(-1);
                                c2 = *cptr.offset(0 as libc::c_int as isize) as uint32_t;
                            } else {
                                let mut __c1_9: uint32_t = 0;
                                cptr = cptr.offset(-(2 as libc::c_int as isize));
                                c2 = *(cptr as *mut uint16_t)
                                    .offset(0 as libc::c_int as isize) as uint32_t;
                                if c2 >= 0xdc00 as libc::c_int as libc::c_uint
                                    && c2 < 0xe000 as libc::c_int as libc::c_uint
                                    && cbuf_type == 2 as libc::c_int && cptr > (*s).cbuf
                                {
                                    __c1_9 = *(cptr as *mut uint16_t)
                                        .offset(-(1 as libc::c_int) as isize) as uint32_t;
                                    if __c1_9 >= 0xd800 as libc::c_int as libc::c_uint
                                        && __c1_9 < 0xdc00 as libc::c_int as libc::c_uint
                                    {
                                        cptr = cptr.offset(-(2 as libc::c_int as isize));
                                        c2 = ((__c1_9 & 0x3ff as libc::c_int as libc::c_uint)
                                            << 10 as libc::c_int
                                            | c2 & 0x3ff as libc::c_int as libc::c_uint)
                                            .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                    }
                                }
                            }
                            if (*s).ignore_case != 0 {
                                c1 = lre_canonicalize(c1, (*s).is_utf16);
                                c2 = lre_canonicalize(c2, (*s).is_utf16);
                            }
                            if c1 != c2 {
                                break;
                            }
                        }
                    }
                    current_block = 13870927760773930552;
                }
            }
            21 => {
                let mut n: libc::c_int = 0;
                let mut low: uint32_t = 0;
                let mut high: uint32_t = 0;
                let mut idx_min: uint32_t = 0;
                let mut idx_max: uint32_t = 0;
                let mut idx: uint32_t = 0;
                n = get_u16(pc) as libc::c_int;
                pc = pc.offset(2 as libc::c_int as isize);
                if cptr >= cbuf_end {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        let fresh63 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh63 as uint32_t;
                    } else {
                        let mut __c1_10: uint32_t = 0;
                        c = *(cptr as *mut uint16_t) as uint32_t;
                        cptr = cptr.offset(2 as libc::c_int as isize);
                        if c >= 0xd800 as libc::c_int as libc::c_uint
                            && c < 0xdc00 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int && cptr < cbuf_end
                        {
                            __c1_10 = *(cptr as *mut uint16_t) as uint32_t;
                            if __c1_10 >= 0xdc00 as libc::c_int as libc::c_uint
                                && __c1_10 < 0xe000 as libc::c_int as libc::c_uint
                            {
                                c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | __c1_10 & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                cptr = cptr.offset(2 as libc::c_int as isize);
                            }
                        }
                    }
                    if (*s).ignore_case != 0 {
                        c = lre_canonicalize(c, (*s).is_utf16);
                    }
                    idx_min = 0 as libc::c_int as uint32_t;
                    low = get_u16(
                        pc.offset((0 as libc::c_int * 4 as libc::c_int) as isize),
                    );
                    if c < low {
                        current_block = 13870927760773930552;
                    } else {
                        idx_max = (n - 1 as libc::c_int) as uint32_t;
                        high = get_u16(
                            pc
                                .offset(
                                    idx_max.wrapping_mul(4 as libc::c_int as libc::c_uint)
                                        as isize,
                                )
                                .offset(2 as libc::c_int as isize),
                        );
                        if (c >= 0xffff as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                            && high == 0xffff as libc::c_int as libc::c_uint
                        {
                            current_block = 8503123874588926636;
                        } else if c > high {
                            current_block = 13870927760773930552;
                        } else {
                            loop {
                                if !(idx_min <= idx_max) {
                                    current_block = 13870927760773930552;
                                    break;
                                }
                                idx = idx_min
                                    .wrapping_add(idx_max)
                                    .wrapping_div(2 as libc::c_int as libc::c_uint);
                                low = get_u16(
                                    pc
                                        .offset(
                                            idx.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize,
                                        ),
                                );
                                high = get_u16(
                                    pc
                                        .offset(
                                            idx.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize,
                                        )
                                        .offset(2 as libc::c_int as isize),
                                );
                                if c < low {
                                    idx_max = idx
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                                } else {
                                    if !(c > high) {
                                        current_block = 8503123874588926636;
                                        break;
                                    }
                                    idx_min = idx
                                        .wrapping_add(1 as libc::c_int as libc::c_uint);
                                }
                            }
                        }
                        match current_block {
                            13870927760773930552 => {}
                            _ => {
                                pc = pc.offset((4 as libc::c_int * n) as isize);
                                continue;
                            }
                        }
                    }
                }
            }
            22 => {
                let mut n_0: libc::c_int = 0;
                let mut low_0: uint32_t = 0;
                let mut high_0: uint32_t = 0;
                let mut idx_min_0: uint32_t = 0;
                let mut idx_max_0: uint32_t = 0;
                let mut idx_0: uint32_t = 0;
                n_0 = get_u16(pc) as libc::c_int;
                pc = pc.offset(2 as libc::c_int as isize);
                if cptr >= cbuf_end {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        let fresh64 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh64 as uint32_t;
                    } else {
                        let mut __c1_11: uint32_t = 0;
                        c = *(cptr as *mut uint16_t) as uint32_t;
                        cptr = cptr.offset(2 as libc::c_int as isize);
                        if c >= 0xd800 as libc::c_int as libc::c_uint
                            && c < 0xdc00 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int && cptr < cbuf_end
                        {
                            __c1_11 = *(cptr as *mut uint16_t) as uint32_t;
                            if __c1_11 >= 0xdc00 as libc::c_int as libc::c_uint
                                && __c1_11 < 0xe000 as libc::c_int as libc::c_uint
                            {
                                c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | __c1_11 & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                cptr = cptr.offset(2 as libc::c_int as isize);
                            }
                        }
                    }
                    if (*s).ignore_case != 0 {
                        c = lre_canonicalize(c, (*s).is_utf16);
                    }
                    idx_min_0 = 0 as libc::c_int as uint32_t;
                    low_0 = get_u32(
                        pc.offset((0 as libc::c_int * 8 as libc::c_int) as isize),
                    );
                    if c < low_0 {
                        current_block = 13870927760773930552;
                    } else {
                        idx_max_0 = (n_0 - 1 as libc::c_int) as uint32_t;
                        high_0 = get_u32(
                            pc
                                .offset(
                                    idx_max_0.wrapping_mul(8 as libc::c_int as libc::c_uint)
                                        as isize,
                                )
                                .offset(4 as libc::c_int as isize),
                        );
                        if c > high_0 {
                            current_block = 13870927760773930552;
                        } else {
                            loop {
                                if !(idx_min_0 <= idx_max_0) {
                                    current_block = 13870927760773930552;
                                    break;
                                }
                                idx_0 = idx_min_0
                                    .wrapping_add(idx_max_0)
                                    .wrapping_div(2 as libc::c_int as libc::c_uint);
                                low_0 = get_u32(
                                    pc
                                        .offset(
                                            idx_0.wrapping_mul(8 as libc::c_int as libc::c_uint)
                                                as isize,
                                        ),
                                );
                                high_0 = get_u32(
                                    pc
                                        .offset(
                                            idx_0.wrapping_mul(8 as libc::c_int as libc::c_uint)
                                                as isize,
                                        )
                                        .offset(4 as libc::c_int as isize),
                                );
                                if c < low_0 {
                                    idx_max_0 = idx_0
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                                } else {
                                    if !(c > high_0) {
                                        current_block = 13117710326371364616;
                                        break;
                                    }
                                    idx_min_0 = idx_0
                                        .wrapping_add(1 as libc::c_int as libc::c_uint);
                                }
                            }
                            match current_block {
                                13870927760773930552 => {}
                                _ => {
                                    pc = pc.offset((8 as libc::c_int * n_0) as isize);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            27 => {
                if cptr == (*s).cbuf {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        cptr = cptr.offset(-1);
                    } else {
                        cptr = cptr.offset(-(2 as libc::c_int as isize));
                        if cbuf_type == 2 as libc::c_int {
                            c = *(cptr as *mut uint16_t)
                                .offset(0 as libc::c_int as isize) as uint32_t;
                            if c >= 0xdc00 as libc::c_int as libc::c_uint
                                && c < 0xe000 as libc::c_int as libc::c_uint
                                && cptr > (*s).cbuf
                            {
                                c = *(cptr as *mut uint16_t)
                                    .offset(-(1 as libc::c_int) as isize) as uint32_t;
                                if c >= 0xd800 as libc::c_int as libc::c_uint
                                    && c < 0xdc00 as libc::c_int as libc::c_uint
                                {
                                    cptr = cptr.offset(-(2 as libc::c_int as isize));
                                }
                            }
                        }
                    }
                    continue;
                }
            }
            28 => {
                let mut next_pos: uint32_t = 0;
                let mut quant_min: uint32_t = 0;
                let mut quant_max: uint32_t = 0;
                let mut q: size_t = 0;
                let mut res: intptr_t = 0;
                let mut pc1_0: *const uint8_t = 0 as *const uint8_t;
                next_pos = get_u32(pc);
                quant_min = get_u32(pc.offset(4 as libc::c_int as isize));
                quant_max = get_u32(pc.offset(8 as libc::c_int as isize));
                pc = pc.offset(16 as libc::c_int as isize);
                pc1_0 = pc;
                pc = pc.offset(next_pos as libc::c_int as isize);
                q = 0 as libc::c_int as size_t;
                loop {
                    res = lre_exec_backtrack(
                        s,
                        capture,
                        stack,
                        stack_len,
                        pc1_0,
                        cptr,
                        TRUE as libc::c_int,
                    );
                    if res == -(1 as libc::c_int) {
                        return res;
                    }
                    if res == 0 {
                        break;
                    }
                    cptr = res as *mut uint8_t;
                    q = q.wrapping_add(1);
                    if q >= quant_max as libc::c_ulong
                        && quant_max != 0x7fffffff as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                }
                if q < quant_min as libc::c_ulong {
                    current_block = 13870927760773930552;
                } else {
                    if q > quant_min as libc::c_ulong {
                        ret = push_state(
                            s,
                            capture,
                            stack,
                            stack_len as size_t,
                            pc1_0.offset(-(16 as libc::c_int as isize)),
                            cptr,
                            RE_EXEC_STATE_GREEDY_QUANT,
                            q.wrapping_sub(quant_min as libc::c_ulong),
                        );
                        if ret < 0 as libc::c_int {
                            return -(1 as libc::c_int);
                        }
                    }
                    continue;
                }
            }
            _ => {
                abort();
            }
        }
        match current_block {
            12575669259942314375 => {
                if cptr >= cbuf_end {
                    current_block = 13870927760773930552;
                } else {
                    if cbuf_type == 0 as libc::c_int {
                        let fresh50 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh50 as uint32_t;
                    } else {
                        let mut __c1: uint32_t = 0;
                        c = *(cptr as *mut uint16_t) as uint32_t;
                        cptr = cptr.offset(2 as libc::c_int as isize);
                        if c >= 0xd800 as libc::c_int as libc::c_uint
                            && c < 0xdc00 as libc::c_int as libc::c_uint
                            && cbuf_type == 2 as libc::c_int && cptr < cbuf_end
                        {
                            __c1 = *(cptr as *mut uint16_t) as uint32_t;
                            if __c1 >= 0xdc00 as libc::c_int as libc::c_uint
                                && __c1 < 0xe000 as libc::c_int as libc::c_uint
                            {
                                c = ((c & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | __c1 & 0x3ff as libc::c_int as libc::c_uint)
                                    .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                                cptr = cptr.offset(2 as libc::c_int as isize);
                            }
                        }
                    }
                    if (*s).ignore_case != 0 {
                        c = lre_canonicalize(c, (*s).is_utf16);
                    }
                    if !(val != c) {
                        continue;
                    }
                    current_block = 13870927760773930552;
                }
            }
            _ => {}
        }
        match current_block {
            13870927760773930552 => {
                if no_recurse != 0 {
                    return 0 as libc::c_int;
                }
                ret = 0 as libc::c_int;
            }
            _ => {}
        }
        let mut current_block_49: u64;
        loop {
            if (*s).state_stack_len == 0 as libc::c_int as libc::c_ulong {
                return ret;
            }
            rs = ((*s).state_stack)
                .offset(
                    ((*s).state_stack_len)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul((*s).state_size) as isize,
                ) as *mut REExecState;
            if (*rs).type_0() as libc::c_int == RE_EXEC_STATE_SPLIT as libc::c_int {
                if ret == 0 {
                    current_block_49 = 4443750533406005072;
                } else {
                    current_block_49 = 17075014677070940716;
                }
            } else if (*rs).type_0() as libc::c_int
                == RE_EXEC_STATE_GREEDY_QUANT as libc::c_int
            {
                if ret == 0 {
                    let mut char_count: uint32_t = 0;
                    let mut i: uint32_t = 0;
                    memcpy(
                        capture as *mut libc::c_void,
                        ((*rs).buf).as_mut_ptr() as *const libc::c_void,
                        (::core::mem::size_of::<*mut uint8_t>() as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul((*s).capture_count as libc::c_ulong),
                    );
                    stack_len = (*rs).stack_len as libc::c_int;
                    memcpy(
                        stack as *mut libc::c_void,
                        ((*rs).buf)
                            .as_mut_ptr()
                            .offset((2 as libc::c_int * (*s).capture_count) as isize)
                            as *const libc::c_void,
                        (stack_len as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<StackInt>() as libc::c_ulong,
                            ),
                    );
                    pc = (*rs).pc;
                    cptr = (*rs).cptr;
                    char_count = get_u32(pc.offset(12 as libc::c_int as isize));
                    i = 0 as libc::c_int as uint32_t;
                    while i < char_count {
                        if cbuf_type == 0 as libc::c_int {
                            cptr = cptr.offset(-1);
                        } else {
                            cptr = cptr.offset(-(2 as libc::c_int as isize));
                            if cbuf_type == 2 as libc::c_int {
                                c = *(cptr as *mut uint16_t)
                                    .offset(0 as libc::c_int as isize) as uint32_t;
                                if c >= 0xdc00 as libc::c_int as libc::c_uint
                                    && c < 0xe000 as libc::c_int as libc::c_uint
                                    && cptr > (*s).cbuf
                                {
                                    c = *(cptr as *mut uint16_t)
                                        .offset(-(1 as libc::c_int) as isize) as uint32_t;
                                    if c >= 0xd800 as libc::c_int as libc::c_uint
                                        && c < 0xdc00 as libc::c_int as libc::c_uint
                                    {
                                        cptr = cptr.offset(-(2 as libc::c_int as isize));
                                    }
                                }
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    pc = pc
                        .offset(16 as libc::c_int as isize)
                        .offset(get_u32(pc) as libc::c_int as isize);
                    let ref mut fresh46 = (*rs).cptr;
                    *fresh46 = cptr;
                    let ref mut fresh47 = (*rs).count;
                    *fresh47 = (*fresh47).wrapping_sub(1);
                    if (*rs).count == 0 as libc::c_int as libc::c_ulong {
                        let ref mut fresh48 = (*s).state_stack_len;
                        *fresh48 = (*fresh48).wrapping_sub(1);
                    }
                    break;
                } else {
                    current_block_49 = 17075014677070940716;
                }
            } else {
                ret = ((*rs).type_0() as libc::c_int
                    == RE_EXEC_STATE_LOOKAHEAD as libc::c_int && ret != 0
                    || (*rs).type_0() as libc::c_int
                        == RE_EXEC_STATE_NEGATIVE_LOOKAHEAD as libc::c_int && ret == 0)
                    as libc::c_int;
                if ret != 0 {
                    if (*rs).type_0() as libc::c_int
                        == RE_EXEC_STATE_LOOKAHEAD as libc::c_int
                    {
                        current_block_49 = 4751214920458933587;
                    } else {
                        current_block_49 = 4443750533406005072;
                    }
                } else {
                    current_block_49 = 17075014677070940716;
                }
            }
            match current_block_49 {
                17075014677070940716 => {
                    let ref mut fresh49 = (*s).state_stack_len;
                    *fresh49 = (*fresh49).wrapping_sub(1);
                    continue;
                }
                4443750533406005072 => {
                    memcpy(
                        capture as *mut libc::c_void,
                        ((*rs).buf).as_mut_ptr() as *const libc::c_void,
                        (::core::mem::size_of::<*mut uint8_t>() as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul((*s).capture_count as libc::c_ulong),
                    );
                }
                _ => {}
            }
            pc = (*rs).pc;
            cptr = (*rs).cptr;
            stack_len = (*rs).stack_len as libc::c_int;
            memcpy(
                stack as *mut libc::c_void,
                ((*rs).buf)
                    .as_mut_ptr()
                    .offset((2 as libc::c_int * (*s).capture_count) as isize)
                    as *const libc::c_void,
                (stack_len as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<StackInt>() as libc::c_ulong),
            );
            let ref mut fresh45 = (*s).state_stack_len;
            *fresh45 = (*fresh45).wrapping_sub(1);
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lre_exec(
    mut capture: *mut *mut uint8_t,
    mut bc_buf: *const uint8_t,
    mut cbuf: *const uint8_t,
    mut cindex: libc::c_int,
    mut clen: libc::c_int,
    mut cbuf_type: libc::c_int,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    let mut s_s: REExecContext = REExecContext {
        cbuf: 0 as *const uint8_t,
        cbuf_end: 0 as *const uint8_t,
        cbuf_type: 0,
        capture_count: 0,
        stack_size_max: 0,
        multi_line: 0,
        ignore_case: 0,
        is_utf16: 0,
        opaque: 0 as *mut libc::c_void,
        state_size: 0,
        state_stack: 0 as *mut uint8_t,
        state_stack_size: 0,
        state_stack_len: 0,
    };
    let mut s: *mut REExecContext = &mut s_s;
    let mut re_flags: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut alloca_size: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut stack_buf: *mut StackInt = 0 as *mut StackInt;
    re_flags = *bc_buf.offset(0 as libc::c_int as isize) as libc::c_int;
    (*s)
        .multi_line = (re_flags & (1 as libc::c_int) << 2 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    (*s)
        .ignore_case = (re_flags & (1 as libc::c_int) << 1 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    (*s)
        .is_utf16 = (re_flags & (1 as libc::c_int) << 4 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    (*s).capture_count = *bc_buf.offset(1 as libc::c_int as isize) as libc::c_int;
    (*s).stack_size_max = *bc_buf.offset(2 as libc::c_int as isize) as libc::c_int;
    let ref mut fresh65 = (*s).cbuf;
    *fresh65 = cbuf;
    let ref mut fresh66 = (*s).cbuf_end;
    *fresh66 = cbuf.offset((clen << cbuf_type) as isize);
    (*s).cbuf_type = cbuf_type;
    if (*s).cbuf_type == 1 as libc::c_int && (*s).is_utf16 != 0 {
        (*s).cbuf_type = 2 as libc::c_int;
    }
    let ref mut fresh67 = (*s).opaque;
    *fresh67 = opaque;
    (*s)
        .state_size = (::core::mem::size_of::<REExecState>() as libc::c_ulong)
        .wrapping_add(
            ((*s).capture_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut uint8_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            ((*s).stack_size_max as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<StackInt>() as libc::c_ulong),
        );
    let ref mut fresh68 = (*s).state_stack;
    *fresh68 = 0 as *mut uint8_t;
    (*s).state_stack_len = 0 as libc::c_int as size_t;
    (*s).state_stack_size = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < (*s).capture_count * 2 as libc::c_int {
        let ref mut fresh69 = *capture.offset(i as isize);
        *fresh69 = 0 as *mut uint8_t;
        i += 1;
    }
    alloca_size = ((*s).stack_size_max as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<StackInt>() as libc::c_ulong)
        as libc::c_int;
    stack_buf = alloca(alloca_size as libc::c_ulong) as *mut StackInt;
    ret = lre_exec_backtrack(
        s,
        capture,
        stack_buf,
        0 as libc::c_int,
        bc_buf.offset(7 as libc::c_int as isize),
        cbuf.offset((cindex << cbuf_type) as isize),
        FALSE as libc::c_int,
    );
    lre_realloc(
        (*s).opaque,
        (*s).state_stack as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lre_get_capture_count(
    mut bc_buf: *const uint8_t,
) -> libc::c_int {
    return *bc_buf.offset(1 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lre_get_flags(mut bc_buf: *const uint8_t) -> libc::c_int {
    return *bc_buf.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lre_get_groupnames(
    mut bc_buf: *const uint8_t,
) -> *const libc::c_char {
    let mut re_bytecode_len: uint32_t = 0;
    if lre_get_flags(bc_buf) & (1 as libc::c_int) << 7 as libc::c_int == 0 as libc::c_int
    {
        return 0 as *const libc::c_char;
    }
    re_bytecode_len = get_u32(bc_buf.offset(3 as libc::c_int as isize));
    return bc_buf.offset(7 as libc::c_int as isize).offset(re_bytecode_len as isize)
        as *const libc::c_char;
}
