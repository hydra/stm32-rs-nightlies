///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DCMI control register
    pub dcmi_cr: DCMI_CR,
    ///0x04 - DCMI status register
    pub dcmi_sr: DCMI_SR,
    ///0x08 - DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.
    pub dcmi_ris: DCMI_RIS,
    ///0x0c - The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.
    pub dcmi_ier: DCMI_IER,
    ///0x10 - This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.
    pub dcmi_mis: DCMI_MIS,
    ///0x14 - The DCMI_ICR register is write-only.
    pub dcmi_icr: DCMI_ICR,
    ///0x18 - DCMI embedded synchronization code register
    pub dcmi_escr: DCMI_ESCR,
    ///0x1c - DCMI embedded synchronization unmask register
    pub dcmi_esur: DCMI_ESUR,
    ///0x20 - DCMI crop window start
    pub dcmi_cwstrt: DCMI_CWSTRT,
    ///0x24 - DCMI crop window size
    pub dcmi_cwsize: DCMI_CWSIZE,
    ///0x28 - DCMI data register
    pub dcmi_dr: DCMI_DR,
}
///DCMI_CR (rw) register accessor: an alias for `Reg<DCMI_CR_SPEC>`
pub type DCMI_CR = crate::Reg<dcmi_cr::DCMI_CR_SPEC>;
///DCMI control register
pub mod dcmi_cr;
///DCMI_SR (r) register accessor: an alias for `Reg<DCMI_SR_SPEC>`
pub type DCMI_SR = crate::Reg<dcmi_sr::DCMI_SR_SPEC>;
///DCMI status register
pub mod dcmi_sr;
///DCMI_RIS (r) register accessor: an alias for `Reg<DCMI_RIS_SPEC>`
pub type DCMI_RIS = crate::Reg<dcmi_ris::DCMI_RIS_SPEC>;
///DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.
pub mod dcmi_ris;
///DCMI_IER (rw) register accessor: an alias for `Reg<DCMI_IER_SPEC>`
pub type DCMI_IER = crate::Reg<dcmi_ier::DCMI_IER_SPEC>;
///The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.
pub mod dcmi_ier;
///DCMI_MIS (r) register accessor: an alias for `Reg<DCMI_MIS_SPEC>`
pub type DCMI_MIS = crate::Reg<dcmi_mis::DCMI_MIS_SPEC>;
///This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.
pub mod dcmi_mis;
///DCMI_ICR (w) register accessor: an alias for `Reg<DCMI_ICR_SPEC>`
pub type DCMI_ICR = crate::Reg<dcmi_icr::DCMI_ICR_SPEC>;
///The DCMI_ICR register is write-only.
pub mod dcmi_icr;
///DCMI_ESCR (rw) register accessor: an alias for `Reg<DCMI_ESCR_SPEC>`
pub type DCMI_ESCR = crate::Reg<dcmi_escr::DCMI_ESCR_SPEC>;
///DCMI embedded synchronization code register
pub mod dcmi_escr;
///DCMI_ESUR (rw) register accessor: an alias for `Reg<DCMI_ESUR_SPEC>`
pub type DCMI_ESUR = crate::Reg<dcmi_esur::DCMI_ESUR_SPEC>;
///DCMI embedded synchronization unmask register
pub mod dcmi_esur;
///DCMI_CWSTRT (rw) register accessor: an alias for `Reg<DCMI_CWSTRT_SPEC>`
pub type DCMI_CWSTRT = crate::Reg<dcmi_cwstrt::DCMI_CWSTRT_SPEC>;
///DCMI crop window start
pub mod dcmi_cwstrt;
///DCMI_CWSIZE (rw) register accessor: an alias for `Reg<DCMI_CWSIZE_SPEC>`
pub type DCMI_CWSIZE = crate::Reg<dcmi_cwsize::DCMI_CWSIZE_SPEC>;
///DCMI crop window size
pub mod dcmi_cwsize;
///DCMI_DR (r) register accessor: an alias for `Reg<DCMI_DR_SPEC>`
pub type DCMI_DR = crate::Reg<dcmi_dr::DCMI_DR_SPEC>;
///DCMI data register
pub mod dcmi_dr;
