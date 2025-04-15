use ratatui::style::Color;

pub const BLACK: u32 = 0x000000;
pub const WHITE: u32 = 0xFFFFFF;
pub const GREY: u32 = 0x999999;

// wood floor
pub const BROWN1: u32 = 0x833e20;
pub const BROWN2: u32 = 0x8f4c28;
pub const BROWN3: u32 = 0x965a31;
pub const BROWN4: u32 = 0xa67c51;
pub const BROWN5: u32 = 0xb99d67;

// forest green
pub const GREEN1: u32 = 0x003101;
pub const GREEN2: u32 = 0x053800;
pub const GREEN3: u32 = 0x024000;
pub const GREEN4: u32 = 0x064900;
pub const GREEN5: u32 = 0x00510a;
pub const GREEN6: u32 = 0x008800;

// rock gray
pub const GRAY1: u32 = 0x2d2c2c;
pub const GRAY2: u32 = 0x3a3232;
pub const GRAY3: u32 = 0x493c3c;
pub const GRAY4: u32 = 0x5c4949;
pub const GRAY5: u32 = 0x655353;
pub const GRAY6: u32 = 0x858893;

// reds
pub const RED1: u32 = 0xb62020;
pub const RED2: u32 = 0xcb2424;
pub const RED3: u32 = 0xfe2e2e;
pub const RED4: u32 = 0xfe5757;
pub const RED5: u32 = 0xfe8181;

// yellow
pub const YELLOW1: u32 = 0xf9e909;
pub const YELLOW2: u32 = 0xfdf25d;
pub const YELLOW3: u32 = 0xfcff83;
pub const YELLOW4: u32 = 0xfbfd9e;
pub const YELLOW5: u32 = 0xfeffc3;

// blues
pub const BLUE1: u32 = 0x001eff;
pub const BLUE2: u32 = 0x001be7;
pub const BLUE3: u32 = 0x0119cb;
pub const BLUE4: u32 = 0x021496;
pub const BLUE5: u32 = 0x000b5e;

pub const DEEPSEA1: u32 = 0x0200c5;
pub const DEEPSEA2: u32 = 0x0100af;
pub const DEEPSEA3: u32 = 0x0100a0;
pub const DEEPSEA4: u32 = 0x010090;
pub const DEEPSEA5: u32 = 0x010088;

pub const SHALLOWWATERS1: u32 = 0x77d9d9;
pub const SHALLOWWATERS2: u32 = 0x77c5d9;
pub const SHALLOWWATERS3: u32 = 0x77b2d9;
pub const SHALLOWWATERS4: u32 = 0x779ed9;
pub const SHALLOWWATERS5: u32 = 0x778bd9;

pub fn c(color: u32) -> Color {
    Color::from_u32(color)
}
