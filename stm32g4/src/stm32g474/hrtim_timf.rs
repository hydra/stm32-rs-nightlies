///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timfcr: TIMFCR,
    ///0x04 - Timerx Interrupt Status Register
    pub timfisr: TIMFISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timficr: TIMFICR,
    ///0x0c - TIMxDIER
    pub timfdier: TIMFDIER,
    ///0x10 - Timerx Counter Register
    pub cntfr: CNTFR,
    ///0x14 - Timerx Period Register
    pub perfr: PERFR,
    ///0x18 - Timerx Repetition Register
    pub repfr: REPFR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1fr: CMP1FR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1cfr: CMP1CFR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2fr: CMP2FR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3fr: CMP3FR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4fr: CMP4FR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1fr: CPT1FR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2fr: CPT2FR,
    ///0x38 - Timerx Deadtime Register
    pub dtfr: DTFR,
    ///0x3c - Timerx Output1 Set Register
    pub setf1r: SETF1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rste1r: RSTE1R,
    ///0x44 - Timerx Output2 Set Register
    pub setf2r: SETF2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rstf2r: RSTF2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eeffr1: EEFFR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eeffr2: EEFFR2,
    ///0x54 - TimerA Reset Register
    pub rstfr: RSTFR,
    ///0x58 - Timerx Chopper Register
    pub chpfr: CHPFR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1fcr: CPT1FCR,
    ///0x60 - CPT2xCR
    pub cpt2fcr: CPT2FCR,
    ///0x64 - Timerx Output Register
    pub outfr: OUTFR,
    ///0x68 - Timerx Fault Register
    pub fltfr: FLTFR,
    ///0x6c - HRTIM Timerx Control Register 2
    pub timfcr2: TIMFCR2,
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    pub feefr3: FEEFR3,
}
///TIMFCR (rw) register accessor: an alias for `Reg<TIMFCR_SPEC>`
pub type TIMFCR = crate::Reg<timfcr::TIMFCR_SPEC>;
///Timerx Control Register
pub mod timfcr;
///TIMFISR (r) register accessor: an alias for `Reg<TIMFISR_SPEC>`
pub type TIMFISR = crate::Reg<timfisr::TIMFISR_SPEC>;
///Timerx Interrupt Status Register
pub mod timfisr;
///TIMFICR (w) register accessor: an alias for `Reg<TIMFICR_SPEC>`
pub type TIMFICR = crate::Reg<timficr::TIMFICR_SPEC>;
///Timerx Interrupt Clear Register
pub mod timficr;
///TIMFDIER (rw) register accessor: an alias for `Reg<TIMFDIER_SPEC>`
pub type TIMFDIER = crate::Reg<timfdier::TIMFDIER_SPEC>;
///TIMxDIER
pub mod timfdier;
///CNTFR (rw) register accessor: an alias for `Reg<CNTFR_SPEC>`
pub type CNTFR = crate::Reg<cntfr::CNTFR_SPEC>;
///Timerx Counter Register
pub mod cntfr;
///PERFR (rw) register accessor: an alias for `Reg<PERFR_SPEC>`
pub type PERFR = crate::Reg<perfr::PERFR_SPEC>;
///Timerx Period Register
pub mod perfr;
///REPFR (rw) register accessor: an alias for `Reg<REPFR_SPEC>`
pub type REPFR = crate::Reg<repfr::REPFR_SPEC>;
///Timerx Repetition Register
pub mod repfr;
///CMP1FR (rw) register accessor: an alias for `Reg<CMP1FR_SPEC>`
pub type CMP1FR = crate::Reg<cmp1fr::CMP1FR_SPEC>;
///Timerx Compare 1 Register
pub mod cmp1fr;
///CMP1CFR (rw) register accessor: an alias for `Reg<CMP1CFR_SPEC>`
pub type CMP1CFR = crate::Reg<cmp1cfr::CMP1CFR_SPEC>;
///Timerx Compare 1 Compound Register
pub mod cmp1cfr;
///CMP2FR (rw) register accessor: an alias for `Reg<CMP2FR_SPEC>`
pub type CMP2FR = crate::Reg<cmp2fr::CMP2FR_SPEC>;
///Timerx Compare 2 Register
pub mod cmp2fr;
///CMP3FR (rw) register accessor: an alias for `Reg<CMP3FR_SPEC>`
pub type CMP3FR = crate::Reg<cmp3fr::CMP3FR_SPEC>;
///Timerx Compare 3 Register
pub mod cmp3fr;
///CMP4FR (rw) register accessor: an alias for `Reg<CMP4FR_SPEC>`
pub type CMP4FR = crate::Reg<cmp4fr::CMP4FR_SPEC>;
///Timerx Compare 4 Register
pub mod cmp4fr;
///CPT1FR (r) register accessor: an alias for `Reg<CPT1FR_SPEC>`
pub type CPT1FR = crate::Reg<cpt1fr::CPT1FR_SPEC>;
///Timerx Capture 1 Register
pub mod cpt1fr;
///CPT2FR (r) register accessor: an alias for `Reg<CPT2FR_SPEC>`
pub type CPT2FR = crate::Reg<cpt2fr::CPT2FR_SPEC>;
///Timerx Capture 2 Register
pub mod cpt2fr;
///DTFR (rw) register accessor: an alias for `Reg<DTFR_SPEC>`
pub type DTFR = crate::Reg<dtfr::DTFR_SPEC>;
///Timerx Deadtime Register
pub mod dtfr;
///SETF1R (rw) register accessor: an alias for `Reg<SETF1R_SPEC>`
pub type SETF1R = crate::Reg<setf1r::SETF1R_SPEC>;
///Timerx Output1 Set Register
pub mod setf1r;
///RSTE1R (rw) register accessor: an alias for `Reg<RSTE1R_SPEC>`
pub type RSTE1R = crate::Reg<rste1r::RSTE1R_SPEC>;
///Timerx Output1 Reset Register
pub mod rste1r;
///SETF2R (rw) register accessor: an alias for `Reg<SETF2R_SPEC>`
pub type SETF2R = crate::Reg<setf2r::SETF2R_SPEC>;
///Timerx Output2 Set Register
pub mod setf2r;
///RSTF2R (rw) register accessor: an alias for `Reg<RSTF2R_SPEC>`
pub type RSTF2R = crate::Reg<rstf2r::RSTF2R_SPEC>;
///Timerx Output2 Reset Register
pub mod rstf2r;
///EEFFR1 (rw) register accessor: an alias for `Reg<EEFFR1_SPEC>`
pub type EEFFR1 = crate::Reg<eeffr1::EEFFR1_SPEC>;
///Timerx External Event Filtering Register 1
pub mod eeffr1;
///EEFFR2 (rw) register accessor: an alias for `Reg<EEFFR2_SPEC>`
pub type EEFFR2 = crate::Reg<eeffr2::EEFFR2_SPEC>;
///Timerx External Event Filtering Register 2
pub mod eeffr2;
///RSTFR (rw) register accessor: an alias for `Reg<RSTFR_SPEC>`
pub type RSTFR = crate::Reg<rstfr::RSTFR_SPEC>;
///TimerA Reset Register
pub mod rstfr;
///CHPFR (rw) register accessor: an alias for `Reg<CHPFR_SPEC>`
pub type CHPFR = crate::Reg<chpfr::CHPFR_SPEC>;
///Timerx Chopper Register
pub mod chpfr;
///CPT1FCR (rw) register accessor: an alias for `Reg<CPT1FCR_SPEC>`
pub type CPT1FCR = crate::Reg<cpt1fcr::CPT1FCR_SPEC>;
///Timerx Capture 2 Control Register
pub mod cpt1fcr;
///CPT2FCR (rw) register accessor: an alias for `Reg<CPT2FCR_SPEC>`
pub type CPT2FCR = crate::Reg<cpt2fcr::CPT2FCR_SPEC>;
///CPT2xCR
pub mod cpt2fcr;
///OUTFR (rw) register accessor: an alias for `Reg<OUTFR_SPEC>`
pub type OUTFR = crate::Reg<outfr::OUTFR_SPEC>;
///Timerx Output Register
pub mod outfr;
///FLTFR (rw) register accessor: an alias for `Reg<FLTFR_SPEC>`
pub type FLTFR = crate::Reg<fltfr::FLTFR_SPEC>;
///Timerx Fault Register
pub mod fltfr;
///TIMFCR2 (rw) register accessor: an alias for `Reg<TIMFCR2_SPEC>`
pub type TIMFCR2 = crate::Reg<timfcr2::TIMFCR2_SPEC>;
///HRTIM Timerx Control Register 2
pub mod timfcr2;
///FEEFR3 (rw) register accessor: an alias for `Reg<FEEFR3_SPEC>`
pub type FEEFR3 = crate::Reg<feefr3::FEEFR3_SPEC>;
///HRTIM Timerx External Event Filtering Register 3
pub mod feefr3;
