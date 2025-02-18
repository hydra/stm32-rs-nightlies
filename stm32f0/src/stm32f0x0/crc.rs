///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dr: [u8; 0x04],
    ///0x04 - Independent data register
    pub idr: IDR,
    ///0x08 - Control register
    pub cr: CR,
    _reserved3: [u8; 0x04],
    ///0x10 - Initial CRC value
    pub init: INIT,
}
impl RegisterBlock {
    ///0x00 - Data register - half-word sized
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x00 - Data register - byte sized
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x00 - Data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Data register
pub mod dr;
///IDR (rw) register accessor: an alias for `Reg<IDR_SPEC>`
pub type IDR = crate::Reg<idr::IDR_SPEC>;
///Independent data register
pub mod idr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///INIT (rw) register accessor: an alias for `Reg<INIT_SPEC>`
pub type INIT = crate::Reg<init::INIT_SPEC>;
///Initial CRC value
pub mod init;
///DR8 (rw) register accessor: an alias for `Reg<DR8_SPEC>`
pub type DR8 = crate::Reg<dr8::DR8_SPEC>;
///Data register - byte sized
pub mod dr8;
///DR16 (rw) register accessor: an alias for `Reg<DR16_SPEC>`
pub type DR16 = crate::Reg<dr16::DR16_SPEC>;
///Data register - half-word sized
pub mod dr16;
