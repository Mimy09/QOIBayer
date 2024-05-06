use bitfield_struct::bitfield;

#[derive(Default)]
pub struct QOIHeader
{
    pub magic: [u8; 4],
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8,
}

impl QOIHeader {
    pub fn new(width: u32, height: u32) -> QOIHeader {
        let mut header: QOIHeader = QOIHeader::default();
        header.magic = [b'q', b'o', b'i', b'f'];
        header.width = width;
        header.height = height;
        header.channels = 4;
        header.colorspace = 1;
        header
    }
}

#[bitfield(u32)]
pub struct QOITypeRGB
{
    #[bits(8, access = RO, default = 0b1111_1110)]
    pub tag: u8,

    #[bits(8)]
    pub r: u8,
    #[bits(8)]
    pub g: u8,
    #[bits(8)]
    pub b: u8,
}
#[bitfield(u64)]
pub struct QOITypeRGBA
{
    #[bits(8, access = RO, default = 0b1111_1111)]
    pub tag: u8,

    #[bits(8)]
    pub r: u8,
    #[bits(8)]
    pub g: u8,
    #[bits(8)]
    pub b: u8,
    #[bits(8)]
    pub a: u8,

    #[bits(24)]
    __: u32,
}
#[bitfield(u8)]
pub struct QOITypeIndex
{
    #[bits(2, access = RO, default = 0b00)]
    pub tag: u8,

    #[bits(6)]
    pub index: u8,
}
#[bitfield(u16)]
pub struct QOITypeDiff
{
    #[bits(2, access = RO, default = 0b01)]
    pub tag: u8,

    #[bits(3)]
    pub dr: u8,
    #[bits(3)]
    pub dg: u8,
    #[bits(3)]
    pub db: u8,
    #[bits(3)]
    pub da: u8,

    #[bits(2)]
    __: u8,
}
#[bitfield(u16)]
pub struct QOITypeLuma
{
    #[bits(2, access = RO, default = 0b10)]
    pub tag: u8,

    #[bits(6)]
    pub diff_green: u8,
    #[bits(4)]
    pub dr_dg: u8,
    #[bits(4)]
    pub db_dg: u8,
}
#[bitfield(u8)]
pub struct QOITypeRun
{
    #[bits(2, access = RO, default = 0b11)]
    pub tag: u8,

    #[bits(6)]
    pub run: u8,
}

pub enum QOIType {
    RGB(QOITypeRGB),
    RGBA(QOITypeRGBA),
    Index(QOITypeIndex),
    Diff(QOITypeDiff),
    Luma(QOITypeLuma),
    Run(QOITypeRun),
}