///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timacr: TIMACR,
    ///0x04 - Timerx Interrupt Status Register
    pub timaisr: TIMAISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timaicr: TIMAICR,
    ///0x0c - TIMxDIER5
    pub timadier5: TIMADIER5,
    ///0x10 - Timerx Counter Register
    pub cntar: CNTAR,
    ///0x14 - Timerx Period Register
    pub perar: PERAR,
    ///0x18 - Timerx Repetition Register
    pub repar: REPAR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1ar: CMP1AR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1car: CMP1CAR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2ar: CMP2AR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3ar: CMP3AR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4ar: CMP4AR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1ar: CPT1AR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2ar: CPT2AR,
    ///0x38 - Timerx Deadtime Register
    pub dtar: DTAR,
    ///0x3c - Timerx Output1 Set Register
    pub seta1r: SETA1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rsta1r: RSTA1R,
    ///0x44 - Timerx Output2 Set Register
    pub seta2r: SETA2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rsta2r: RSTA2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eefar1: EEFAR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eefar2: EEFAR2,
    ///0x54 - TimerA Reset Register
    pub rstar: RSTAR,
    ///0x58 - Timerx Chopper Register
    pub chpar: CHPAR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1acr: CPT1ACR,
    ///0x60 - CPT2xCR
    pub cpt2acr: CPT2ACR,
    ///0x64 - Timerx Output Register
    pub outar: OUTAR,
    ///0x68 - Timerx Fault Register
    pub fltar: FLTAR,
}
///TIMACR (rw) register accessor: an alias for `Reg<TIMACR_SPEC>`
pub type TIMACR = crate::Reg<timacr::TIMACR_SPEC>;
///Timerx Control Register
pub mod timacr;
///TIMAISR (r) register accessor: an alias for `Reg<TIMAISR_SPEC>`
pub type TIMAISR = crate::Reg<timaisr::TIMAISR_SPEC>;
///Timerx Interrupt Status Register
pub mod timaisr;
///TIMAICR (w) register accessor: an alias for `Reg<TIMAICR_SPEC>`
pub type TIMAICR = crate::Reg<timaicr::TIMAICR_SPEC>;
///Timerx Interrupt Clear Register
pub mod timaicr;
///TIMADIER5 (rw) register accessor: an alias for `Reg<TIMADIER5_SPEC>`
pub type TIMADIER5 = crate::Reg<timadier5::TIMADIER5_SPEC>;
///TIMxDIER5
pub mod timadier5;
///CNTAR (rw) register accessor: an alias for `Reg<CNTAR_SPEC>`
pub type CNTAR = crate::Reg<cntar::CNTAR_SPEC>;
///Timerx Counter Register
pub mod cntar;
///PERAR (rw) register accessor: an alias for `Reg<PERAR_SPEC>`
pub type PERAR = crate::Reg<perar::PERAR_SPEC>;
///Timerx Period Register
pub mod perar;
///REPAR (rw) register accessor: an alias for `Reg<REPAR_SPEC>`
pub type REPAR = crate::Reg<repar::REPAR_SPEC>;
///Timerx Repetition Register
pub mod repar;
///CMP1AR (rw) register accessor: an alias for `Reg<CMP1AR_SPEC>`
pub type CMP1AR = crate::Reg<cmp1ar::CMP1AR_SPEC>;
///Timerx Compare 1 Register
pub mod cmp1ar;
///CMP1CAR (rw) register accessor: an alias for `Reg<CMP1CAR_SPEC>`
pub type CMP1CAR = crate::Reg<cmp1car::CMP1CAR_SPEC>;
///Timerx Compare 1 Compound Register
pub mod cmp1car;
///CMP2AR (rw) register accessor: an alias for `Reg<CMP2AR_SPEC>`
pub type CMP2AR = crate::Reg<cmp2ar::CMP2AR_SPEC>;
///Timerx Compare 2 Register
pub mod cmp2ar;
///CMP3AR (rw) register accessor: an alias for `Reg<CMP3AR_SPEC>`
pub type CMP3AR = crate::Reg<cmp3ar::CMP3AR_SPEC>;
///Timerx Compare 3 Register
pub mod cmp3ar;
///CMP4AR (rw) register accessor: an alias for `Reg<CMP4AR_SPEC>`
pub type CMP4AR = crate::Reg<cmp4ar::CMP4AR_SPEC>;
///Timerx Compare 4 Register
pub mod cmp4ar;
///CPT1AR (r) register accessor: an alias for `Reg<CPT1AR_SPEC>`
pub type CPT1AR = crate::Reg<cpt1ar::CPT1AR_SPEC>;
///Timerx Capture 1 Register
pub mod cpt1ar;
///CPT2AR (r) register accessor: an alias for `Reg<CPT2AR_SPEC>`
pub type CPT2AR = crate::Reg<cpt2ar::CPT2AR_SPEC>;
///Timerx Capture 2 Register
pub mod cpt2ar;
///DTAR (rw) register accessor: an alias for `Reg<DTAR_SPEC>`
pub type DTAR = crate::Reg<dtar::DTAR_SPEC>;
///Timerx Deadtime Register
pub mod dtar;
///SETA1R (rw) register accessor: an alias for `Reg<SETA1R_SPEC>`
pub type SETA1R = crate::Reg<seta1r::SETA1R_SPEC>;
///Timerx Output1 Set Register
pub mod seta1r;
///RSTA1R (rw) register accessor: an alias for `Reg<RSTA1R_SPEC>`
pub type RSTA1R = crate::Reg<rsta1r::RSTA1R_SPEC>;
///Timerx Output1 Reset Register
pub mod rsta1r;
///SETA2R (rw) register accessor: an alias for `Reg<SETA2R_SPEC>`
pub type SETA2R = crate::Reg<seta2r::SETA2R_SPEC>;
///Timerx Output2 Set Register
pub mod seta2r;
///RSTA2R (rw) register accessor: an alias for `Reg<RSTA2R_SPEC>`
pub type RSTA2R = crate::Reg<rsta2r::RSTA2R_SPEC>;
///Timerx Output2 Reset Register
pub mod rsta2r;
///EEFAR1 (rw) register accessor: an alias for `Reg<EEFAR1_SPEC>`
pub type EEFAR1 = crate::Reg<eefar1::EEFAR1_SPEC>;
///Timerx External Event Filtering Register 1
pub mod eefar1;
///EEFAR2 (rw) register accessor: an alias for `Reg<EEFAR2_SPEC>`
pub type EEFAR2 = crate::Reg<eefar2::EEFAR2_SPEC>;
///Timerx External Event Filtering Register 2
pub mod eefar2;
///RSTAR (rw) register accessor: an alias for `Reg<RSTAR_SPEC>`
pub type RSTAR = crate::Reg<rstar::RSTAR_SPEC>;
///TimerA Reset Register
pub mod rstar;
///CHPAR (rw) register accessor: an alias for `Reg<CHPAR_SPEC>`
pub type CHPAR = crate::Reg<chpar::CHPAR_SPEC>;
///Timerx Chopper Register
pub mod chpar;
///CPT1ACR (rw) register accessor: an alias for `Reg<CPT1ACR_SPEC>`
pub type CPT1ACR = crate::Reg<cpt1acr::CPT1ACR_SPEC>;
///Timerx Capture 2 Control Register
pub mod cpt1acr;
///CPT2ACR (rw) register accessor: an alias for `Reg<CPT2ACR_SPEC>`
pub type CPT2ACR = crate::Reg<cpt2acr::CPT2ACR_SPEC>;
///CPT2xCR
pub mod cpt2acr;
///OUTAR (rw) register accessor: an alias for `Reg<OUTAR_SPEC>`
pub type OUTAR = crate::Reg<outar::OUTAR_SPEC>;
///Timerx Output Register
pub mod outar;
///FLTAR (rw) register accessor: an alias for `Reg<FLTAR_SPEC>`
pub type FLTAR = crate::Reg<fltar::FLTAR_SPEC>;
///Timerx Fault Register
pub mod fltar;
