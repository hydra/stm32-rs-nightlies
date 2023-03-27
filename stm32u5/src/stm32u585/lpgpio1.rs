///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - LPGPIO port mode register
    pub lpgpio_moder: LPGPIO_MODER,
    _reserved1: [u8; 0x0c],
    ///0x10 - LPGPIO port input data register
    pub lpgpio_idr: LPGPIO_IDR,
    ///0x14 - LPGPIO port output data register
    pub lpgpio_odr: LPGPIO_ODR,
    ///0x18 - LPGPIO port bit set/reset register
    pub lpgpio_bsrr: LPGPIO_BSRR,
    _reserved4: [u8; 0x0c],
    ///0x28 - LPGPIO port bit reset register
    pub lpgpio_brr: LPGPIO_BRR,
}
///LPGPIO_MODER (rw) register accessor: an alias for `Reg<LPGPIO_MODER_SPEC>`
pub type LPGPIO_MODER = crate::Reg<lpgpio_moder::LPGPIO_MODER_SPEC>;
///LPGPIO port mode register
pub mod lpgpio_moder;
///LPGPIO_IDR (r) register accessor: an alias for `Reg<LPGPIO_IDR_SPEC>`
pub type LPGPIO_IDR = crate::Reg<lpgpio_idr::LPGPIO_IDR_SPEC>;
///LPGPIO port input data register
pub mod lpgpio_idr;
///LPGPIO_ODR (rw) register accessor: an alias for `Reg<LPGPIO_ODR_SPEC>`
pub type LPGPIO_ODR = crate::Reg<lpgpio_odr::LPGPIO_ODR_SPEC>;
///LPGPIO port output data register
pub mod lpgpio_odr;
///LPGPIO_BSRR (w) register accessor: an alias for `Reg<LPGPIO_BSRR_SPEC>`
pub type LPGPIO_BSRR = crate::Reg<lpgpio_bsrr::LPGPIO_BSRR_SPEC>;
///LPGPIO port bit set/reset register
pub mod lpgpio_bsrr;
///LPGPIO_BRR (r) register accessor: an alias for `Reg<LPGPIO_BRR_SPEC>`
pub type LPGPIO_BRR = crate::Reg<lpgpio_brr::LPGPIO_BRR_SPEC>;
///LPGPIO port bit reset register
pub mod lpgpio_brr;
