use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
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
}
pub type __builtin_va_list = *mut libc::c_void;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type uintptr_t = libc::c_uint;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type BOOL = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
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
pub type cmp_f = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        *const libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type exchange_f = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub base: *mut uint8_t,
    pub count: size_t,
    pub depth: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn pstrcpy(
    mut buf: *mut libc::c_char,
    mut buf_size: libc::c_int,
    mut str: *const libc::c_char,
) {
    let mut c: libc::c_int = 0;
    let mut q: *mut libc::c_char = buf;
    if buf_size <= 0 as libc::c_int {
        return;
    }
    loop {
        let fresh0 = str;
        str = str.offset(1);
        c = *fresh0 as libc::c_int;
        if c == 0 as libc::c_int
            || q >= buf.offset(buf_size as isize).offset(-(1 as libc::c_int as isize))
        {
            break;
        }
        let fresh1 = q;
        q = q.offset(1);
        *fresh1 = c as libc::c_char;
    }
    *q = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn pstrcat(
    mut buf: *mut libc::c_char,
    mut buf_size: libc::c_int,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    len = strlen(buf) as libc::c_int;
    if len < buf_size {
        pstrcpy(buf.offset(len as isize), buf_size - len, s);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn strstart(
    mut str: *const libc::c_char,
    mut val: *const libc::c_char,
    mut ptr: *mut *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    p = str;
    q = val;
    while *q as libc::c_int != '\0' as i32 {
        if *p as libc::c_int != *q as libc::c_int {
            return 0 as libc::c_int;
        }
        p = p.offset(1);
        q = q.offset(1);
    }
    if !ptr.is_null() {
        *ptr = p;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn has_suffix(
    mut str: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> libc::c_int {
    let mut len: size_t = strlen(str);
    let mut slen: size_t = strlen(suffix);
    return (len >= slen
        && memcmp(
            str.offset(len as isize).offset(-(slen as isize)) as *const libc::c_void,
            suffix as *const libc::c_void,
            slen,
        ) == 0) as libc::c_int;
}
unsafe extern "C" fn dbuf_default_realloc(
    mut opaque: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return realloc(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_init2(
    mut s: *mut DynBuf,
    mut opaque: *mut libc::c_void,
    mut realloc_func: Option::<DynBufReallocFunc>,
) {
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<DynBuf>() as libc::c_ulong,
    );
    if realloc_func.is_none() {
        realloc_func = Some(
            dbuf_default_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                ) -> *mut libc::c_void,
        );
    }
    let ref mut fresh2 = (*s).opaque;
    *fresh2 = opaque;
    let ref mut fresh3 = (*s).realloc_func;
    *fresh3 = realloc_func;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_init(mut s: *mut DynBuf) {
    dbuf_init2(s, 0 as *mut libc::c_void, None);
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_realloc(
    mut s: *mut DynBuf,
    mut new_size: size_t,
) -> libc::c_int {
    let mut size: size_t = 0;
    let mut new_buf: *mut uint8_t = 0 as *mut uint8_t;
    if new_size > (*s).allocated_size {
        if (*s).error != 0 {
            return -(1 as libc::c_int);
        }
        size = ((*s).allocated_size)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if size > new_size {
            new_size = size;
        }
        new_buf = ((*s).realloc_func)
            .expect(
                "non-null function pointer",
            )((*s).opaque, (*s).buf as *mut libc::c_void, new_size) as *mut uint8_t;
        if new_buf.is_null() {
            (*s).error = TRUE as libc::c_int;
            return -(1 as libc::c_int);
        }
        let ref mut fresh4 = (*s).buf;
        *fresh4 = new_buf;
        (*s).allocated_size = new_size;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_write(
    mut s: *mut DynBuf,
    mut offset: size_t,
    mut data: *const uint8_t,
    mut len: size_t,
) -> libc::c_int {
    let mut end: size_t = 0;
    end = offset.wrapping_add(len);
    if dbuf_realloc(s, end) != 0 {
        return -(1 as libc::c_int);
    }
    memcpy(
        ((*s).buf).offset(offset as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        len,
    );
    if end > (*s).size {
        (*s).size = end;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_put(
    mut s: *mut DynBuf,
    mut data: *const uint8_t,
    mut len: size_t,
) -> libc::c_int {
    if (((*s).size).wrapping_add(len) > (*s).allocated_size) as libc::c_int
        as libc::c_long != 0
    {
        if dbuf_realloc(s, ((*s).size).wrapping_add(len)) != 0 {
            return -(1 as libc::c_int);
        }
    }
    memcpy(
        ((*s).buf).offset((*s).size as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        len,
    );
    let ref mut fresh5 = (*s).size;
    *fresh5 = (*fresh5 as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_put_self(
    mut s: *mut DynBuf,
    mut offset: size_t,
    mut len: size_t,
) -> libc::c_int {
    if (((*s).size).wrapping_add(len) > (*s).allocated_size) as libc::c_int
        as libc::c_long != 0
    {
        if dbuf_realloc(s, ((*s).size).wrapping_add(len)) != 0 {
            return -(1 as libc::c_int);
        }
    }
    memcpy(
        ((*s).buf).offset((*s).size as isize) as *mut libc::c_void,
        ((*s).buf).offset(offset as isize) as *const libc::c_void,
        len,
    );
    let ref mut fresh6 = (*s).size;
    *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_putc(mut s: *mut DynBuf, mut c: uint8_t) -> libc::c_int {
    return dbuf_put(s, &mut c, 1 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_putstr(
    mut s: *mut DynBuf,
    mut str: *const libc::c_char,
) -> libc::c_int {
    return dbuf_put(s, str as *const uint8_t, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_printf(
    mut s: *mut DynBuf,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: va_list = 0 as *mut libc::c_void;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut len: libc::c_int = 0;
    ap = args.clone();
    len = vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        fmt,
        ap,
    );
    if (len as libc::c_ulong)
        < ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
    {
        return dbuf_put(s, buf.as_mut_ptr() as *mut uint8_t, len as size_t)
    } else {
        if dbuf_realloc(
            s,
            ((*s).size)
                .wrapping_add(len as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
        ap = args.clone();
        vsnprintf(
            ((*s).buf).offset((*s).size as isize) as *mut libc::c_char,
            ((*s).allocated_size).wrapping_sub((*s).size),
            fmt,
            ap,
        );
        let ref mut fresh7 = (*s).size;
        *fresh7 = (*fresh7 as libc::c_ulong).wrapping_add(len as libc::c_ulong) as size_t
            as size_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_free(mut s: *mut DynBuf) {
    if !((*s).buf).is_null() {
        ((*s).realloc_func)
            .expect(
                "non-null function pointer",
            )((*s).opaque, (*s).buf as *mut libc::c_void, 0 as libc::c_int as size_t);
    }
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<DynBuf>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn unicode_to_utf8(
    mut buf: *mut uint8_t,
    mut c: libc::c_uint,
) -> libc::c_int {
    let mut q: *mut uint8_t = buf;
    if c < 0x80 as libc::c_int as libc::c_uint {
        let fresh8 = q;
        q = q.offset(1);
        *fresh8 = c as uint8_t;
    } else {
        if c < 0x800 as libc::c_int as libc::c_uint {
            let fresh9 = q;
            q = q.offset(1);
            *fresh9 = (c >> 6 as libc::c_int | 0xc0 as libc::c_int as libc::c_uint)
                as uint8_t;
        } else {
            if c < 0x10000 as libc::c_int as libc::c_uint {
                let fresh10 = q;
                q = q.offset(1);
                *fresh10 = (c >> 12 as libc::c_int | 0xe0 as libc::c_int as libc::c_uint)
                    as uint8_t;
            } else {
                if c < 0x200000 as libc::c_int as libc::c_uint {
                    let fresh11 = q;
                    q = q.offset(1);
                    *fresh11 = (c >> 18 as libc::c_int
                        | 0xf0 as libc::c_int as libc::c_uint) as uint8_t;
                } else {
                    if c < 0x4000000 as libc::c_int as libc::c_uint {
                        let fresh12 = q;
                        q = q.offset(1);
                        *fresh12 = (c >> 24 as libc::c_int
                            | 0xf8 as libc::c_int as libc::c_uint) as uint8_t;
                    } else if c < 0x80000000 as libc::c_uint {
                        let fresh13 = q;
                        q = q.offset(1);
                        *fresh13 = (c >> 30 as libc::c_int
                            | 0xfc as libc::c_int as libc::c_uint) as uint8_t;
                        let fresh14 = q;
                        q = q.offset(1);
                        *fresh14 = (c >> 24 as libc::c_int
                            & 0x3f as libc::c_int as libc::c_uint
                            | 0x80 as libc::c_int as libc::c_uint) as uint8_t;
                    } else {
                        return 0 as libc::c_int
                    }
                    let fresh15 = q;
                    q = q.offset(1);
                    *fresh15 = (c >> 18 as libc::c_int
                        & 0x3f as libc::c_int as libc::c_uint
                        | 0x80 as libc::c_int as libc::c_uint) as uint8_t;
                }
                let fresh16 = q;
                q = q.offset(1);
                *fresh16 = (c >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
                    | 0x80 as libc::c_int as libc::c_uint) as uint8_t;
            }
            let fresh17 = q;
            q = q.offset(1);
            *fresh17 = (c >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
                | 0x80 as libc::c_int as libc::c_uint) as uint8_t;
        }
        let fresh18 = q;
        q = q.offset(1);
        *fresh18 = (c & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint) as uint8_t;
    }
    return q.offset_from(buf) as libc::c_long as libc::c_int;
}
static mut utf8_min_code: [libc::c_uint; 5] = [
    0x80 as libc::c_int as libc::c_uint,
    0x800 as libc::c_int as libc::c_uint,
    0x10000 as libc::c_int as libc::c_uint,
    0x200000 as libc::c_int as libc::c_uint,
    0x4000000 as libc::c_int as libc::c_uint,
];
static mut utf8_first_code_mask: [libc::c_uchar; 5] = [
    0x1f as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn unicode_from_utf8(
    mut p: *const uint8_t,
    mut max_len: libc::c_int,
    mut pp: *mut *const uint8_t,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let fresh19 = p;
    p = p.offset(1);
    c = *fresh19 as libc::c_int;
    if c < 0x80 as libc::c_int {
        *pp = p;
        return c;
    }
    match c {
        192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 | 200 | 201 | 202 | 203 | 204 | 205
        | 206 | 207 | 208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 216 | 217 | 218
        | 219 | 220 | 221 | 222 | 223 => {
            l = 1 as libc::c_int;
        }
        224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237
        | 238 | 239 => {
            l = 2 as libc::c_int;
        }
        240 | 241 | 242 | 243 | 244 | 245 | 246 | 247 => {
            l = 3 as libc::c_int;
        }
        248 | 249 | 250 | 251 => {
            l = 4 as libc::c_int;
        }
        252 | 253 => {
            l = 5 as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    }
    if l > max_len - 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    c &= utf8_first_code_mask[(l - 1 as libc::c_int) as usize] as libc::c_int;
    i = 0 as libc::c_int;
    while i < l {
        let fresh20 = p;
        p = p.offset(1);
        b = *fresh20 as libc::c_int;
        if b < 0x80 as libc::c_int || b >= 0xc0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        c = c << 6 as libc::c_int | b & 0x3f as libc::c_int;
        i += 1;
    }
    if (c as libc::c_uint) < utf8_min_code[(l - 1 as libc::c_int) as usize] {
        return -(1 as libc::c_int);
    }
    *pp = p;
    return c;
}
unsafe extern "C" fn exchange_bytes(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint8_t = a as *mut uint8_t;
    let mut bp: *mut uint8_t = b as *mut uint8_t;
    loop {
        let fresh21 = size;
        size = size.wrapping_sub(1);
        if !(fresh21 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut t: uint8_t = *ap;
        let fresh22 = ap;
        ap = ap.offset(1);
        *fresh22 = *bp;
        let fresh23 = bp;
        bp = bp.offset(1);
        *fresh23 = t;
    };
}
unsafe extern "C" fn exchange_one_byte(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint8_t = a as *mut uint8_t;
    let mut bp: *mut uint8_t = b as *mut uint8_t;
    let mut t: uint8_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int16s(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint16_t = a as *mut uint16_t;
    let mut bp: *mut uint16_t = b as *mut uint16_t;
    size = (size as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<uint16_t>() as libc::c_ulong) as size_t
        as size_t;
    loop {
        let fresh24 = size;
        size = size.wrapping_sub(1);
        if !(fresh24 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut t: uint16_t = *ap;
        let fresh25 = ap;
        ap = ap.offset(1);
        *fresh25 = *bp;
        let fresh26 = bp;
        bp = bp.offset(1);
        *fresh26 = t;
    };
}
unsafe extern "C" fn exchange_one_int16(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint16_t = a as *mut uint16_t;
    let mut bp: *mut uint16_t = b as *mut uint16_t;
    let mut t: uint16_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int32s(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint32_t = a as *mut uint32_t;
    let mut bp: *mut uint32_t = b as *mut uint32_t;
    size = (size as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<uint32_t>() as libc::c_ulong) as size_t
        as size_t;
    loop {
        let fresh27 = size;
        size = size.wrapping_sub(1);
        if !(fresh27 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut t: uint32_t = *ap;
        let fresh28 = ap;
        ap = ap.offset(1);
        *fresh28 = *bp;
        let fresh29 = bp;
        bp = bp.offset(1);
        *fresh29 = t;
    };
}
unsafe extern "C" fn exchange_one_int32(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint32_t = a as *mut uint32_t;
    let mut bp: *mut uint32_t = b as *mut uint32_t;
    let mut t: uint32_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int64s(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    size = (size as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<uint64_t>() as libc::c_ulong) as size_t
        as size_t;
    loop {
        let fresh30 = size;
        size = size.wrapping_sub(1);
        if !(fresh30 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut t: uint64_t = *ap;
        let fresh31 = ap;
        ap = ap.offset(1);
        *fresh31 = *bp;
        let fresh32 = bp;
        bp = bp.offset(1);
        *fresh32 = t;
    };
}
unsafe extern "C" fn exchange_one_int64(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    let mut t: uint64_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int128s(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    size = (size as libc::c_ulong)
        .wrapping_div(
            (::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    loop {
        let fresh33 = size;
        size = size.wrapping_sub(1);
        if !(fresh33 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut t: uint64_t = *ap.offset(0 as libc::c_int as isize);
        let mut u: uint64_t = *ap.offset(1 as libc::c_int as isize);
        *ap.offset(0 as libc::c_int as isize) = *bp.offset(0 as libc::c_int as isize);
        *ap.offset(1 as libc::c_int as isize) = *bp.offset(1 as libc::c_int as isize);
        *bp.offset(0 as libc::c_int as isize) = t;
        *bp.offset(1 as libc::c_int as isize) = u;
        ap = ap.offset(2 as libc::c_int as isize);
        bp = bp.offset(2 as libc::c_int as isize);
    };
}
unsafe extern "C" fn exchange_one_int128(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    let mut t: uint64_t = *ap.offset(0 as libc::c_int as isize);
    let mut u: uint64_t = *ap.offset(1 as libc::c_int as isize);
    *ap.offset(0 as libc::c_int as isize) = *bp.offset(0 as libc::c_int as isize);
    *ap.offset(1 as libc::c_int as isize) = *bp.offset(1 as libc::c_int as isize);
    *bp.offset(0 as libc::c_int as isize) = t;
    *bp.offset(1 as libc::c_int as isize) = u;
}
#[inline]
unsafe extern "C" fn exchange_func(
    mut base: *const libc::c_void,
    mut size: size_t,
) -> exchange_f {
    match (base as uintptr_t | size as uintptr_t) & 15 as libc::c_int as libc::c_uint {
        0 => {
            if size
                == (::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            {
                return Some(
                    exchange_one_int128
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            } else {
                return Some(
                    exchange_int128s
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            }
        }
        8 => {
            if size == ::core::mem::size_of::<uint64_t>() as libc::c_ulong {
                return Some(
                    exchange_one_int64
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            } else {
                return Some(
                    exchange_int64s
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            }
        }
        4 | 12 => {
            if size == ::core::mem::size_of::<uint32_t>() as libc::c_ulong {
                return Some(
                    exchange_one_int32
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            } else {
                return Some(
                    exchange_int32s
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            }
        }
        2 | 6 | 10 | 14 => {
            if size == ::core::mem::size_of::<uint16_t>() as libc::c_ulong {
                return Some(
                    exchange_one_int16
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            } else {
                return Some(
                    exchange_int16s
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            }
        }
        _ => {
            if size == 1 as libc::c_int as libc::c_ulong {
                return Some(
                    exchange_one_byte
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            } else {
                return Some(
                    exchange_bytes
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            size_t,
                        ) -> (),
                )
            }
        }
    };
}
unsafe extern "C" fn heapsortx(
    mut base: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmp: cmp_f,
    mut opaque: *mut libc::c_void,
) {
    let mut basep: *mut uint8_t = base as *mut uint8_t;
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut swap: exchange_f = exchange_func(base, size);
    if nmemb > 1 as libc::c_int as libc::c_ulong {
        i = nmemb.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
        n = nmemb.wrapping_mul(size);
        while i > 0 as libc::c_int as libc::c_ulong {
            i = (i as libc::c_ulong).wrapping_sub(size) as size_t as size_t;
            r = i;
            loop {
                c = r.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_add(size);
                if !(c < n) {
                    break;
                }
                if c < n.wrapping_sub(size)
                    && cmp
                        .expect(
                            "non-null function pointer",
                        )(
                        basep.offset(c as isize) as *const libc::c_void,
                        basep.offset(c as isize).offset(size as isize)
                            as *const libc::c_void,
                        opaque,
                    ) <= 0 as libc::c_int
                {
                    c = (c as libc::c_ulong).wrapping_add(size) as size_t as size_t;
                }
                if cmp
                    .expect(
                        "non-null function pointer",
                    )(
                    basep.offset(r as isize) as *const libc::c_void,
                    basep.offset(c as isize) as *const libc::c_void,
                    opaque,
                ) > 0 as libc::c_int
                {
                    break;
                }
                swap
                    .expect(
                        "non-null function pointer",
                    )(
                    basep.offset(r as isize) as *mut libc::c_void,
                    basep.offset(c as isize) as *mut libc::c_void,
                    size,
                );
                r = c;
            }
        }
        i = n.wrapping_sub(size);
        while i > 0 as libc::c_int as libc::c_ulong {
            swap
                .expect(
                    "non-null function pointer",
                )(
                basep as *mut libc::c_void,
                basep.offset(i as isize) as *mut libc::c_void,
                size,
            );
            r = 0 as libc::c_int as size_t;
            loop {
                c = r.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_add(size);
                if !(c < i) {
                    break;
                }
                if c < i.wrapping_sub(size)
                    && cmp
                        .expect(
                            "non-null function pointer",
                        )(
                        basep.offset(c as isize) as *const libc::c_void,
                        basep.offset(c as isize).offset(size as isize)
                            as *const libc::c_void,
                        opaque,
                    ) <= 0 as libc::c_int
                {
                    c = (c as libc::c_ulong).wrapping_add(size) as size_t as size_t;
                }
                if cmp
                    .expect(
                        "non-null function pointer",
                    )(
                    basep.offset(r as isize) as *const libc::c_void,
                    basep.offset(c as isize) as *const libc::c_void,
                    opaque,
                ) > 0 as libc::c_int
                {
                    break;
                }
                swap
                    .expect(
                        "non-null function pointer",
                    )(
                    basep.offset(r as isize) as *mut libc::c_void,
                    basep.offset(c as isize) as *mut libc::c_void,
                    size,
                );
                r = c;
            }
            i = (i as libc::c_ulong).wrapping_sub(size) as size_t as size_t;
        }
    }
}
#[inline]
unsafe extern "C" fn med3(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut c: *mut libc::c_void,
    mut cmp: cmp_f,
    mut opaque: *mut libc::c_void,
) -> *mut libc::c_void {
    return if cmp.expect("non-null function pointer")(a, b, opaque) < 0 as libc::c_int {
        if cmp.expect("non-null function pointer")(b, c, opaque) < 0 as libc::c_int {
            b
        } else if cmp.expect("non-null function pointer")(a, c, opaque)
            < 0 as libc::c_int
        {
            c
        } else {
            a
        }
    } else if cmp.expect("non-null function pointer")(b, c, opaque) > 0 as libc::c_int {
        b
    } else if cmp.expect("non-null function pointer")(a, c, opaque) < 0 as libc::c_int {
        a
    } else {
        c
    };
}
#[no_mangle]
pub unsafe extern "C" fn rqsort(
    mut base: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmp: cmp_f,
    mut opaque: *mut libc::c_void,
) {
    let mut stack: [C2RustUnnamed_0; 50] = [C2RustUnnamed_0 {
        base: 0 as *mut uint8_t,
        count: 0,
        depth: 0,
    }; 50];
    let mut sp: *mut C2RustUnnamed_0 = stack.as_mut_ptr();
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let mut pi: *mut uint8_t = 0 as *mut uint8_t;
    let mut pj: *mut uint8_t = 0 as *mut uint8_t;
    let mut plt: *mut uint8_t = 0 as *mut uint8_t;
    let mut pgt: *mut uint8_t = 0 as *mut uint8_t;
    let mut top: *mut uint8_t = 0 as *mut uint8_t;
    let mut m: *mut uint8_t = 0 as *mut uint8_t;
    let mut m4: size_t = 0;
    let mut i: size_t = 0;
    let mut lt: size_t = 0;
    let mut gt: size_t = 0;
    let mut span: size_t = 0;
    let mut span2: size_t = 0;
    let mut c: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut swap: exchange_f = exchange_func(base, size);
    let mut swap_block: exchange_f = exchange_func(
        base,
        size | 128 as libc::c_int as libc::c_ulong,
    );
    if nmemb < 2 as libc::c_int as libc::c_ulong
        || size <= 0 as libc::c_int as libc::c_ulong
    {
        return;
    }
    let ref mut fresh34 = (*sp).base;
    *fresh34 = base as *mut uint8_t;
    (*sp).count = nmemb;
    (*sp).depth = 0 as libc::c_int;
    sp = sp.offset(1);
    while sp > stack.as_mut_ptr() {
        sp = sp.offset(-1);
        ptr = (*sp).base;
        nmemb = (*sp).count;
        depth = (*sp).depth;
        while nmemb > 6 as libc::c_int as libc::c_ulong {
            depth += 1;
            if depth > 50 as libc::c_int {
                heapsortx(ptr as *mut libc::c_void, nmemb, size, cmp, opaque);
                nmemb = 0 as libc::c_int as size_t;
                break;
            } else {
                m4 = (nmemb >> 2 as libc::c_int).wrapping_mul(size);
                m = med3(
                    ptr.offset(m4 as isize) as *mut libc::c_void,
                    ptr
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(m4) as isize,
                        ) as *mut libc::c_void,
                    ptr
                        .offset(
                            (3 as libc::c_int as libc::c_ulong).wrapping_mul(m4) as isize,
                        ) as *mut libc::c_void,
                    cmp,
                    opaque,
                ) as *mut uint8_t;
                swap
                    .expect(
                        "non-null function pointer",
                    )(ptr as *mut libc::c_void, m as *mut libc::c_void, size);
                lt = 1 as libc::c_int as size_t;
                i = lt;
                plt = ptr.offset(size as isize);
                pi = plt;
                gt = nmemb;
                top = ptr.offset(nmemb.wrapping_mul(size) as isize);
                pgt = top;
                pj = pgt;
                loop {
                    while pi < pj
                        && {
                            c = cmp
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr as *const libc::c_void,
                                pi as *const libc::c_void,
                                opaque,
                            );
                            c >= 0 as libc::c_int
                        }
                    {
                        if c == 0 as libc::c_int {
                            swap
                                .expect(
                                    "non-null function pointer",
                                )(plt as *mut libc::c_void, pi as *mut libc::c_void, size);
                            lt = lt.wrapping_add(1);
                            plt = plt.offset(size as isize);
                        }
                        i = i.wrapping_add(1);
                        pi = pi.offset(size as isize);
                    }
                    loop {
                        pj = pj.offset(-(size as isize));
                        if !(pi < pj
                            && {
                                c = cmp
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ptr as *const libc::c_void,
                                    pj as *const libc::c_void,
                                    opaque,
                                );
                                c <= 0 as libc::c_int
                            })
                        {
                            break;
                        }
                        if c == 0 as libc::c_int {
                            gt = gt.wrapping_sub(1);
                            pgt = pgt.offset(-(size as isize));
                            swap
                                .expect(
                                    "non-null function pointer",
                                )(pgt as *mut libc::c_void, pj as *mut libc::c_void, size);
                        }
                    }
                    if pi >= pj {
                        break;
                    }
                    swap
                        .expect(
                            "non-null function pointer",
                        )(pi as *mut libc::c_void, pj as *mut libc::c_void, size);
                    i = i.wrapping_add(1);
                    pi = pi.offset(size as isize);
                }
                span = plt.offset_from(ptr) as libc::c_long as size_t;
                span2 = pi.offset_from(plt) as libc::c_long as size_t;
                lt = i.wrapping_sub(lt);
                if span > span2 {
                    span = span2;
                }
                swap_block
                    .expect(
                        "non-null function pointer",
                    )(
                    ptr as *mut libc::c_void,
                    pi.offset(-(span as isize)) as *mut libc::c_void,
                    span,
                );
                span = top.offset_from(pgt) as libc::c_long as size_t;
                span2 = pgt.offset_from(pi) as libc::c_long as size_t;
                pgt = top.offset(-(span2 as isize));
                gt = nmemb.wrapping_sub(gt.wrapping_sub(i));
                if span > span2 {
                    span = span2;
                }
                swap_block
                    .expect(
                        "non-null function pointer",
                    )(
                    pi as *mut libc::c_void,
                    top.offset(-(span as isize)) as *mut libc::c_void,
                    span,
                );
                if lt > nmemb.wrapping_sub(gt) {
                    let ref mut fresh35 = (*sp).base;
                    *fresh35 = ptr;
                    (*sp).count = lt;
                    (*sp).depth = depth;
                    sp = sp.offset(1);
                    ptr = pgt;
                    nmemb = (nmemb as libc::c_ulong).wrapping_sub(gt) as size_t
                        as size_t;
                } else {
                    let ref mut fresh36 = (*sp).base;
                    *fresh36 = pgt;
                    (*sp).count = nmemb.wrapping_sub(gt);
                    (*sp).depth = depth;
                    sp = sp.offset(1);
                    nmemb = lt;
                }
            }
        }
        pi = ptr.offset(size as isize);
        top = ptr.offset(nmemb.wrapping_mul(size) as isize);
        while pi < top {
            pj = pi;
            while pj > ptr
                && cmp
                    .expect(
                        "non-null function pointer",
                    )(
                    pj.offset(-(size as isize)) as *const libc::c_void,
                    pj as *const libc::c_void,
                    opaque,
                ) > 0 as libc::c_int
            {
                swap
                    .expect(
                        "non-null function pointer",
                    )(
                    pj as *mut libc::c_void,
                    pj.offset(-(size as isize)) as *mut libc::c_void,
                    size,
                );
                pj = pj.offset(-(size as isize));
            }
            pi = pi.offset(size as isize);
        }
    }
}
