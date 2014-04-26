/*!
A binding for SDL2_ttf.
 */

#![feature(macro_rules)]

#![crate_id="sdl2_ttf#sdl2_ttf:0.1"]
#![crate_type = "lib"]
#![desc = "SDL2_ttf bindings and wrappers"]
#![comment = "SDL2_ttf bindings and wrappers"]
#![license = "MIT"]

extern crate libc;
extern crate sdl2;

use libc::{c_int, c_long};
use std::c_str::CString;
use std::num::FromPrimitive;
use sdl2::surface::Surface;
use sdl2::get_error;
use sdl2::pixels::ToColor;
use sdl2::rwops::RWops;
use sdl2::version::Version;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_ttf")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_ttf")]
    extern {}
}

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod others {
    #[link(name="SDL2_ttf")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;
mod flag;

/// Font Style
#[deriving(Show)]
flag_type!(FontStyle : c_int {
    StyleNormal = ffi::TTF_STYLE_NORMAL,
    StyleBold   = ffi::TTF_STYLE_BOLD,
    StyleItalic = ffi::TTF_STYLE_ITALIC,
    StyleUnderline = ffi::TTF_STYLE_UNDERLINE,
    StyleStrikeThrough = ffi::TTF_STYLE_STRIKETHROUGH
})

#[deriving(Show, Eq, FromPrimitive)]
pub enum Hinting {
    HintingNormal = ffi::TTF_HINTING_NORMAL as int,
    HintingLight  = ffi::TTF_HINTING_LIGHT  as int,
    HintingMono   = ffi::TTF_HINTING_MONO   as int,
    HintingNone   = ffi::TTF_HINTING_NONE   as int
}

/// Glyph Metrics
#[deriving(Eq, Clone, Show)]
pub struct GlyphMetrics {
    pub minx: int,
    pub maxx: int,
    pub miny: int,
    pub maxy: int,
    pub advance: int
}

/// Returns the version of the dynamically linked SDL_ttf library
pub fn get_linked_version() -> Version {
    unsafe {
        Version::from_ll(ffi::TTF_Linked_Version())
    }
}

pub fn init() -> bool {
    //! Initialize the truetype font API.
    unsafe {
        if ffi::TTF_WasInit() == 1 {
            true
        } else {
            ffi::TTF_Init() == 0
        }
    }
}

pub fn was_inited() -> bool {
    //! Query the initilization status of the truetype font API.
    unsafe {
        ffi::TTF_WasInit() == 1
    }
}

pub fn quit() {
    //! Shutdown and cleanup the truetype font API.
    unsafe { ffi::TTF_Quit(); }
}

/// The opaque holder of a loaded font.
#[allow(raw_pointer_deriving)]
#[deriving(Eq)]
pub struct Font {
    raw: *ffi::TTF_Font,
    owned: bool
}

impl Drop for Font {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                // avoid close font after quit()
                if ffi::TTF_WasInit() == 1 {
                    ffi::TTF_CloseFont(self.raw);
                }
            }
        }
    }
}

impl Font {
    pub fn from_file(filename: &Path, ptsize: int) -> Result<~Font, ~str> {
        //! Load file for use as a font, at ptsize size.
        unsafe {
            let raw = ffi::TTF_OpenFont(filename.to_c_str().unwrap(), ptsize as c_int);
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Font { raw: raw, owned: true })
            }
        }
    }

    pub fn from_file_index(filename: &Path, ptsize: int, index: int) -> Result<~Font, ~str> {
        //! Load file, face index, for use as a font, at ptsize size.
        unsafe {
            let raw = ffi::TTF_OpenFontIndex(filename.to_c_str().unwrap(), ptsize as c_int, index as c_long);
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Font { raw: raw, owned: true })
            }
        }
    }

    pub fn get_style(&self) -> FontStyle {
        //! Get font render style
        let raw = unsafe { ffi::TTF_GetFontStyle(self.raw) };
        FontStyle::new(raw)
    }

    pub fn set_style(&mut self, styles: FontStyle) {
        //! Set font render style.
        unsafe {
            ffi::TTF_SetFontStyle(self.raw, styles.get())
        }
    }

    pub fn get_outline(&self) -> int {
        //! Get font outline width.
        unsafe {
            ffi::TTF_GetFontOutline(self.raw) as int
        }
    }

    pub fn set_outline(&mut self, outline: int) {
        //! Set font outline width.
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, outline as c_int)
        }
    }

    pub fn get_hinting(&self) -> Hinting {
        //! Get freetype hinter setting.
        unsafe {
            FromPrimitive::from_i32(ffi::TTF_GetFontHinting(self.raw)).unwrap()
        }
    }

    pub fn set_hinting(&mut self, hinting: Hinting) {
        //! Set freetype hinter setting.
        unsafe {
            ffi::TTF_SetFontHinting(self.raw, hinting as c_int)
        }
    }

    pub fn get_kerning(&self) -> bool {
        //! Get freetype kerning setting.
        unsafe {
            ffi::TTF_GetFontKerning(self.raw) != 0
        }
    }

    pub fn set_kerning(&mut self, kerning: bool) {
        //! Set freetype kerning setting.
        unsafe {
            ffi::TTF_SetFontKerning(self.raw, kerning as c_int)
        }
    }

    pub fn height(&self) -> int {
        //! Get font maximum total height.
        unsafe {
            ffi::TTF_FontHeight(self.raw) as int
        }
    }

    pub fn ascent(&self) -> int {
        //! Get font highest ascent (height above base).
        unsafe {
            ffi::TTF_FontAscent(self.raw) as int
        }
    }

    pub fn descent(&self) -> int {
        //! Get font lowest descent (height below base).
        unsafe {
            ffi::TTF_FontDescent(self.raw) as int
        }
    }

    pub fn line_skip(&self) -> int {
        //! Get font recommended line spacing.
        unsafe {
            ffi::TTF_FontLineSkip(self.raw) as int
        }
    }

    pub fn faces(&self) -> int {
        //! Get the number of faces in a font.
        unsafe {
            ffi::TTF_FontFaces(self.raw) as int
        }
    }

    pub fn face_is_fixed_width(&self) -> bool {
        //! Get whether font is monospaced or not.
        unsafe {
            ffi::TTF_FontFaceIsFixedWidth(self.raw) != 0
        }
    }

    pub fn face_family_name(&self) -> Option<~str> {
        //! Get current font face family name string.
        unsafe {
            // not owns buffer
            let cname = ffi::TTF_FontFaceFamilyName(self.raw);
            if cname.is_null() {
                None
            } else {
                Some(CString::new(cname, false).as_str().unwrap().into_owned())
            }
        }
    }

    pub fn face_style_name(&self) -> Option<~str> {
        //! Get current font face style name string.
        unsafe {
            let cname = ffi::TTF_FontFaceStyleName(self.raw);
            if cname.is_null() {
                None
            } else {
                Some(CString::new(cname, false).as_str().unwrap().into_owned())
            }
        }
    }

    pub fn index_of_char(&self, ch: char) -> Option<uint> {
        //! Get individual font glyph availability.
        unsafe {
            let ret = ffi::TTF_GlyphIsProvided(self.raw, ch as u16);
            if ret == 0 {
                None
            } else {
                Some(ret as uint)
            }
        }
    }

    pub fn metrics_of_char(&self, ch: char) -> Option<GlyphMetrics> {
        //! Get individual font glyph metrics.
        let minx = 0;
        let maxx = 0;
        let miny = 0;
        let maxy = 0;
        let advance = 0;
        let ret = unsafe {
            ffi::TTF_GlyphMetrics(self.raw, ch as u16,
                                  &minx, &maxx, &miny, &maxy, &advance)
        };
        if ret != 0 {
            None
        } else {
            Some(GlyphMetrics { minx: minx as int, maxx: maxx as int,
                                miny: miny as int, maxy: maxy as int,
                                advance: advance as int })
        }
    }

    pub fn size_of_bytes(&self, text: &[u8]) -> Result<(int, int), ~str> {
        //! Get size of LATIN1 text string as would be rendered.
        let w = 0;
        let h = 0;
        let ret = unsafe {
            text.with_c_str(|ctext| {
                    ffi::TTF_SizeText(self.raw, ctext, &w, &h)
                })
        };
        if ret != 0 {
            Err(get_error())
        } else {
            Ok((w as int, h as int))
        }
    }

    pub fn size_of_str(&self, text: &str) -> Result<(int, int), ~str> {
        //! Get size of UTF8 text string as would be rendered.
        let w = 0;
        let h = 0;
        let ret = unsafe {
            text.with_c_str(|ctext| {
                    ffi::TTF_SizeUTF8(self.raw, ctext, &w, &h)
                })
        };
        if ret != 0 {
            Err(get_error())
        } else {
            Ok((w as int, h as int))
        }
    }

    pub fn render_bytes_solid<C: ToColor>(&self, text: &[u8], fg: C) -> Result<~Surface, ~str> {
        //! Draw LATIN1 text in solid mode.
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderText_Solid(self.raw, ctext, fg.to_color())
                });
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_str_solid<C: ToColor>(&self, text: &str, fg: C) -> Result<~Surface, ~str> {
        //! Draw UTF8 text in solid mode.
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderUTF8_Solid(self.raw, ctext, fg.to_color())
                });
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_char_solid<C: ToColor>(&self, ch: char, fg: C) -> Result<~Surface, ~str> {
        //! Draw a UNICODE glyph in solid mode.
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Solid(self.raw, ch as u16, fg.to_color());
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_bytes_shaded<C: ToColor>(&self, text: &[u8], fg: C, bg: C) -> Result<~Surface, ~str> {
        //! Draw LATIN1 text in shaded mode.
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderText_Shaded(self.raw, ctext, fg.to_color(), bg.to_color())
                });
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_str_shaded<C: ToColor>(&self, text: &str, fg: C, bg: C) -> Result<~Surface, ~str> {
        //! Draw UTF8 text in shaded mode.
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderUTF8_Shaded(self.raw, ctext, fg.to_color(), bg.to_color())
                });
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_char_shaded<C: ToColor>(&self, ch: char, fg: C, bg: C) -> Result<~Surface, ~str> {
        //! Draw a UNICODE glyph in shaded mode.
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Shaded(self.raw, ch as u16, fg.to_color(), bg.to_color());
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_bytes_blended<C: ToColor>(&self, text: &[u8], fg: C) -> Result<~Surface, ~str> {
        //! Draw LATIN1 text in blended mode.
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderText_Blended(self.raw, ctext, fg.to_color())
                });
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_str_blended<C: ToColor>(&self, text: &str, fg: C) -> Result<~Surface, ~str> {
        //! Draw UTF8 text in blended mode.
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderUTF8_Blended(self.raw, ctext, fg.to_color())
                });
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_char_blended<C: ToColor>(&self, ch: char, fg: C) -> Result<~Surface, ~str> {
        //! Draw a UNICODE glyph in blended mode.
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Blended(self.raw, ch as u16, fg.to_color());
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }
}


/// Loader trait for RWops
pub trait LoaderRWops {
    /// Load src for use as a font.
    fn load_font(&self, ptsize: int) -> Result<~Font, ~str>;
    /// Load src for use as a font.
    fn load_font_index(&self, ptsize: int, index: int) -> Result<~Font, ~str>;
}

impl LoaderRWops for RWops {
    fn load_font(&self, ptsize: int) -> Result<~Font, ~str> {
        let raw = unsafe {
            ffi::TTF_OpenFontRW(self.raw, 0, ptsize as c_int)
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(~Font { raw: raw, owned: true })
        }
    }
    fn load_font_index(&self, ptsize: int, index: int) -> Result<~Font, ~str> {
        let raw = unsafe {
            ffi::TTF_OpenFontIndexRW(self.raw, 0, ptsize as c_int, index as c_long)
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(~Font { raw: raw, owned: true })
        }
    }
}
