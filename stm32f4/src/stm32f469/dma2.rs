///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - low interrupt status register
    pub lisr: LISR,
    ///0x04 - high interrupt status register
    pub hisr: HISR,
    ///0x08 - low interrupt flag clear register
    pub lifcr: LIFCR,
    ///0x0c - high interrupt flag clear register
    pub hifcr: HIFCR,
    ///0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers
    pub st: [ST; 8],
}
///LISR (r) register accessor: an alias for `Reg<LISR_SPEC>`
pub type LISR = crate::Reg<lisr::LISR_SPEC>;
///low interrupt status register
pub mod lisr;
///HISR (r) register accessor: an alias for `Reg<HISR_SPEC>`
pub type HISR = crate::Reg<hisr::HISR_SPEC>;
///high interrupt status register
pub mod hisr;
///LIFCR (w) register accessor: an alias for `Reg<LIFCR_SPEC>`
pub type LIFCR = crate::Reg<lifcr::LIFCR_SPEC>;
///low interrupt flag clear register
pub mod lifcr;
///HIFCR (w) register accessor: an alias for `Reg<HIFCR_SPEC>`
pub type HIFCR = crate::Reg<hifcr::HIFCR_SPEC>;
///high interrupt flag clear register
pub mod hifcr;
///Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers
pub use self::st::ST;
///Cluster
///Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers
pub mod st;
