//! `notcurses_*` reimplemented functions.

use core::ptr::{null, null_mut};

use crate::{
    NcAlign, NcDimension, NcError, NcInput, NcNotcurses, NcOffset, NcPlane, NcResult, NcSignalSet,
    NcTime, NCALIGN_CENTER, NCALIGN_LEFT, NCALIGN_RIGHT, NCRESULT_ERR, NCRESULT_MAX,
};

/// Returns the offset into `availcols` at which `cols` ought be output given
/// the requirements of `align`.
///
/// Returns `-`[`NCRESULT_MAX`] if [NCALIGN_UNALIGNED][crate::NCALIGN_UNALIGNED]
/// or invalid [NcAlign].
///
/// *Method: NcNotcurses.[align()][NcNotcurses#method.align].*
#[inline]
pub fn notcurses_align(availcols: NcDimension, align: NcAlign, cols: NcDimension) -> NcOffset {
    if align == NCALIGN_LEFT {
        return 0;
    }
    if cols > availcols {
        return 0;
    }
    if align == NCALIGN_CENTER {
        return ((availcols - cols) / 2) as NcOffset;
    }
    if align == NCALIGN_RIGHT {
        return (availcols - cols) as NcOffset;
    }
    -NCRESULT_MAX // NCALIGN_UNALIGNED
}

///
/// If no event is ready, returns 0.
///
/// *Method: NcNotcurses.[getc_nblock()][NcNotcurses#method.getc_nblock].*
//
// TODO: use from_u32 & return Option.
#[inline]
pub fn notcurses_getc_nblock(nc: &mut NcNotcurses, input: &mut NcInput) -> char {
    unsafe {
        let mut sigmask = NcSignalSet::new();
        sigmask.fillset();
        let ts = NcTime {
            tv_sec: 0,
            tv_nsec: 0,
        };
        core::char::from_u32_unchecked(crate::notcurses_getc(nc, &ts, &mut sigmask, input))
    }
}

/// Blocks until an event is processed or a signal is received.
///
/// Optionally writes the event details in `input`.
///
/// In case of an invalid read (including on EOF) *-1 as char* is returned.
///
/// *Method: NcNotcurses.[getc_blocking()][NcNotcurses#method.getc_blocking].*
#[inline]
pub fn notcurses_getc_blocking(nc: &mut NcNotcurses, input: Option<&mut NcInput>) -> char {
    let input_ptr;
    if let Some(i) = input {
        input_ptr = i as *mut _;
    } else {
        input_ptr = null_mut();
    }
    unsafe {
        let mut sigmask = NcSignalSet::new();
        sigmask.emptyset();
        core::char::from_u32_unchecked(crate::notcurses_getc(nc, null(), &mut sigmask, input_ptr))
    }
}

/// [notcurses_stdplane()][crate::notcurses_stdplane], plus free bonus
/// dimensions written to non-NULL y/x!
///
/// *Method: NcNotcurses.[getc_stddim_yx()][NcNotcurses#method.stddim_yx].*
#[inline]
pub fn notcurses_stddim_yx<'a>(
    nc: &'a mut NcNotcurses,
    y: &mut NcDimension,
    x: &mut NcDimension,
) -> NcResult<&'a mut NcPlane> {
    unsafe {
        let sp = crate::notcurses_stdplane(nc);
        if sp != null_mut() {
            crate::ncplane_dim_yx(sp, &mut (*y as i32), &mut (*x as i32));
            return Ok(&mut *sp);
        }
    }
    Err(NcError::new(NCRESULT_ERR))
}

/// [notcurses_stdplane_const()][crate::notcurses_stdplane_const], plus free
/// bonus dimensions written to non-NULL y/x!
///
/// *Method: NcNotcurses.[getc_stddim_yx_const()][NcNotcurses#method.stddim_yx_const].*
#[inline]
pub fn notcurses_stddim_yx_const<'a>(
    nc: &'a NcNotcurses,
    y: &mut NcDimension,
    x: &mut NcDimension,
) -> NcResult<&'a NcPlane> {
    unsafe {
        let sp = crate::notcurses_stdplane_const(nc);
        if sp != null() {
            crate::ncplane_dim_yx(sp, &mut (*y as i32), &mut (*x as i32));
            return Ok(&*sp);
        }
    }
    Err(NcError::new(NCRESULT_ERR))
}

/// Returns our current idea of the terminal dimensions in rows and cols.
///
/// *Method: NcNotcurses.[getc_term_yx()][NcNotcurses#method.term_yx].*
#[inline]
pub fn notcurses_term_dim_yx(nc: &NcNotcurses) -> (NcDimension, NcDimension) {
    let (mut y, mut x) = (0, 0);
    unsafe {
        crate::ncplane_dim_yx(crate::notcurses_stdplane_const(nc), &mut y, &mut x);
    }
    (y as NcDimension, x as NcDimension)
}
