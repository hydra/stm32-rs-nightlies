///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timbcr: TIMBCR,
    ///0x04 - Timerx Interrupt Status Register
    pub timbisr: TIMBISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timbicr: TIMBICR,
    ///0x0c - TIMxDIER5
    pub timbdier5: TIMBDIER5,
    ///0x10 - Timerx Counter Register
    pub cntr: CNTR,
    ///0x14 - Timerx Period Register
    pub perbr: PERBR,
    ///0x18 - Timerx Repetition Register
    pub repbr: REPBR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1br: CMP1BR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1cbr: CMP1CBR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2br: CMP2BR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3br: CMP3BR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4br: CMP4BR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1br: CPT1BR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2br: CPT2BR,
    ///0x38 - Timerx Deadtime Register
    pub dtbr: DTBR,
    ///0x3c - Timerx Output1 Set Register
    pub setb1r: SETB1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rstb1r: RSTB1R,
    ///0x44 - Timerx Output2 Set Register
    pub setb2r: SETB2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rstb2r: RSTB2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eefbr1: EEFBR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eefbr2: EEFBR2,
    ///0x54 - TimerA Reset Register
    pub rstbr: RSTBR,
    ///0x58 - Timerx Chopper Register
    pub chpbr: CHPBR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1bcr: CPT1BCR,
    ///0x60 - CPT2xCR
    pub cpt2bcr: CPT2BCR,
    ///0x64 - Timerx Output Register
    pub outbr: OUTBR,
    ///0x68 - Timerx Fault Register
    pub fltbr: FLTBR,
}
///TIMBCR (rw) register accessor: an alias for `Reg<TIMBCR_SPEC>`
pub type TIMBCR = crate::Reg<timbcr::TIMBCR_SPEC>;
///Timerx Control Register
pub mod timbcr;
///TIMBISR (r) register accessor: an alias for `Reg<TIMBISR_SPEC>`
pub type TIMBISR = crate::Reg<timbisr::TIMBISR_SPEC>;
///Timerx Interrupt Status Register
pub mod timbisr;
///TIMBICR (w) register accessor: an alias for `Reg<TIMBICR_SPEC>`
pub type TIMBICR = crate::Reg<timbicr::TIMBICR_SPEC>;
///Timerx Interrupt Clear Register
pub mod timbicr;
///TIMBDIER5 (rw) register accessor: an alias for `Reg<TIMBDIER5_SPEC>`
pub type TIMBDIER5 = crate::Reg<timbdier5::TIMBDIER5_SPEC>;
///TIMxDIER5
pub mod timbdier5;
///CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
///Timerx Counter Register
pub mod cntr;
///PERBR (rw) register accessor: an alias for `Reg<PERBR_SPEC>`
pub type PERBR = crate::Reg<perbr::PERBR_SPEC>;
///Timerx Period Register
pub mod perbr;
///REPBR (rw) register accessor: an alias for `Reg<REPBR_SPEC>`
pub type REPBR = crate::Reg<repbr::REPBR_SPEC>;
///Timerx Repetition Register
pub mod repbr;
///CMP1BR (rw) register accessor: an alias for `Reg<CMP1BR_SPEC>`
pub type CMP1BR = crate::Reg<cmp1br::CMP1BR_SPEC>;
///Timerx Compare 1 Register
pub mod cmp1br;
///CMP1CBR (rw) register accessor: an alias for `Reg<CMP1CBR_SPEC>`
pub type CMP1CBR = crate::Reg<cmp1cbr::CMP1CBR_SPEC>;
///Timerx Compare 1 Compound Register
pub mod cmp1cbr;
///CMP2BR (rw) register accessor: an alias for `Reg<CMP2BR_SPEC>`
pub type CMP2BR = crate::Reg<cmp2br::CMP2BR_SPEC>;
///Timerx Compare 2 Register
pub mod cmp2br;
///CMP3BR (rw) register accessor: an alias for `Reg<CMP3BR_SPEC>`
pub type CMP3BR = crate::Reg<cmp3br::CMP3BR_SPEC>;
///Timerx Compare 3 Register
pub mod cmp3br;
///CMP4BR (rw) register accessor: an alias for `Reg<CMP4BR_SPEC>`
pub type CMP4BR = crate::Reg<cmp4br::CMP4BR_SPEC>;
///Timerx Compare 4 Register
pub mod cmp4br;
///CPT1BR (r) register accessor: an alias for `Reg<CPT1BR_SPEC>`
pub type CPT1BR = crate::Reg<cpt1br::CPT1BR_SPEC>;
///Timerx Capture 1 Register
pub mod cpt1br;
///CPT2BR (r) register accessor: an alias for `Reg<CPT2BR_SPEC>`
pub type CPT2BR = crate::Reg<cpt2br::CPT2BR_SPEC>;
///Timerx Capture 2 Register
pub mod cpt2br;
///DTBR (rw) register accessor: an alias for `Reg<DTBR_SPEC>`
pub type DTBR = crate::Reg<dtbr::DTBR_SPEC>;
///Timerx Deadtime Register
pub mod dtbr;
///SETB1R (rw) register accessor: an alias for `Reg<SETB1R_SPEC>`
pub type SETB1R = crate::Reg<setb1r::SETB1R_SPEC>;
///Timerx Output1 Set Register
pub mod setb1r;
///RSTB1R (rw) register accessor: an alias for `Reg<RSTB1R_SPEC>`
pub type RSTB1R = crate::Reg<rstb1r::RSTB1R_SPEC>;
///Timerx Output1 Reset Register
pub mod rstb1r;
///SETB2R (rw) register accessor: an alias for `Reg<SETB2R_SPEC>`
pub type SETB2R = crate::Reg<setb2r::SETB2R_SPEC>;
///Timerx Output2 Set Register
pub mod setb2r;
///RSTB2R (rw) register accessor: an alias for `Reg<RSTB2R_SPEC>`
pub type RSTB2R = crate::Reg<rstb2r::RSTB2R_SPEC>;
///Timerx Output2 Reset Register
pub mod rstb2r;
///EEFBR1 (rw) register accessor: an alias for `Reg<EEFBR1_SPEC>`
pub type EEFBR1 = crate::Reg<eefbr1::EEFBR1_SPEC>;
///Timerx External Event Filtering Register 1
pub mod eefbr1;
///EEFBR2 (rw) register accessor: an alias for `Reg<EEFBR2_SPEC>`
pub type EEFBR2 = crate::Reg<eefbr2::EEFBR2_SPEC>;
///Timerx External Event Filtering Register 2
pub mod eefbr2;
///RSTBR (rw) register accessor: an alias for `Reg<RSTBR_SPEC>`
pub type RSTBR = crate::Reg<rstbr::RSTBR_SPEC>;
///TimerA Reset Register
pub mod rstbr;
///CHPBR (rw) register accessor: an alias for `Reg<CHPBR_SPEC>`
pub type CHPBR = crate::Reg<chpbr::CHPBR_SPEC>;
///Timerx Chopper Register
pub mod chpbr;
///CPT1BCR (rw) register accessor: an alias for `Reg<CPT1BCR_SPEC>`
pub type CPT1BCR = crate::Reg<cpt1bcr::CPT1BCR_SPEC>;
///Timerx Capture 2 Control Register
pub mod cpt1bcr;
///CPT2BCR (rw) register accessor: an alias for `Reg<CPT2BCR_SPEC>`
pub type CPT2BCR = crate::Reg<cpt2bcr::CPT2BCR_SPEC>;
///CPT2xCR
pub mod cpt2bcr;
///OUTBR (rw) register accessor: an alias for `Reg<OUTBR_SPEC>`
pub type OUTBR = crate::Reg<outbr::OUTBR_SPEC>;
///Timerx Output Register
pub mod outbr;
///FLTBR (rw) register accessor: an alias for `Reg<FLTBR_SPEC>`
pub type FLTBR = crate::Reg<fltbr::FLTBR_SPEC>;
///Timerx Fault Register
pub mod fltbr;
