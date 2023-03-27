///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control Register 1
    pub cr1: CR1,
    ///0x04 - Control Register 2
    pub cr2: CR2,
    ///0x08 - Interrupt Status Register
    pub isr: ISR,
    ///0x0c - Interrupt Clear Register
    pub icr: ICR,
    ///0x10 - Interrupt Enable Register
    pub ier: IER,
    ///0x14 - Output Enable Register
    pub oenr: OENR,
    ///0x18 - ODISR
    pub odisr: ODISR,
    ///0x1c - Output Disable Status Register
    pub odsr: ODSR,
    ///0x20 - Burst Mode Control Register
    pub bmcr: BMCR,
    ///0x24 - BMTRG
    pub bmtrg: BMTRG,
    ///0x28 - BMCMPR
    pub bmcmpr: BMCMPR,
    ///0x2c - Burst Mode Period Register
    pub bmper: BMPER,
    ///0x30 - Timer External Event Control Register 1
    pub eecr1: EECR1,
    ///0x34 - Timer External Event Control Register 2
    pub eecr2: EECR2,
    ///0x38 - Timer External Event Control Register 3
    pub eecr3: EECR3,
    ///0x3c - ADC Trigger 1 Register
    pub adc1r: ADC1R,
    ///0x40 - ADC Trigger 2 Register
    pub adc2r: ADC2R,
    ///0x44 - ADC Trigger 3 Register
    pub adc3r: ADC3R,
    ///0x48 - ADC Trigger 4 Register
    pub adc4r: ADC4R,
    ///0x4c - DLL Control Register
    pub dllcr: DLLCR,
    ///0x50 - HRTIM Fault Input Register 1
    pub fltinr1: FLTINR1,
    ///0x54 - HRTIM Fault Input Register 2
    pub fltinr2: FLTINR2,
    ///0x58 - BDMUPDR
    pub bdmupdr: BDMUPDR,
    ///0x5c - Burst DMA Timerx update Register
    pub bdtaupr: BDTAUPR,
    ///0x60 - Burst DMA Timerx update Register
    pub bdtbupr: BDTBUPR,
    ///0x64 - Burst DMA Timerx update Register
    pub bdtcupr: BDTCUPR,
    ///0x68 - Burst DMA Timerx update Register
    pub bdtdupr: BDTDUPR,
    ///0x6c - Burst DMA Timerx update Register
    pub bdteupr: BDTEUPR,
    ///0x70 - Burst DMA Data Register
    pub bdmadr: BDMADR,
    ///0x74 - Burst DMA Timerx update Register
    pub bdtfupr: BDTFUPR,
    ///0x78 - HRTIM ADC Extended Trigger Register
    pub adcer: ADCER,
    ///0x7c - HRTIM ADC Trigger Update Register
    pub adcur: ADCUR,
    ///0x80 - HRTIM ADC Post Scaler Register 1
    pub adcps1: ADCPS1,
    ///0x84 - HRTIM ADC Post Scaler Register 2
    pub adcps2: ADCPS2,
    ///0x88 - HRTIM Fault Input Register 3
    pub fltinr3: FLTINR3,
    ///0x8c - HRTIM Fault Input Register 4
    pub fltinr4: FLTINR4,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Control Register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control Register 2
pub mod cr2;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt Status Register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt Clear Register
pub mod icr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///Interrupt Enable Register
pub mod ier;
///OENR (rw) register accessor: an alias for `Reg<OENR_SPEC>`
pub type OENR = crate::Reg<oenr::OENR_SPEC>;
///Output Enable Register
pub mod oenr;
///ODISR (w) register accessor: an alias for `Reg<ODISR_SPEC>`
pub type ODISR = crate::Reg<odisr::ODISR_SPEC>;
///ODISR
pub mod odisr;
///ODSR (r) register accessor: an alias for `Reg<ODSR_SPEC>`
pub type ODSR = crate::Reg<odsr::ODSR_SPEC>;
///Output Disable Status Register
pub mod odsr;
///BMCR (rw) register accessor: an alias for `Reg<BMCR_SPEC>`
pub type BMCR = crate::Reg<bmcr::BMCR_SPEC>;
///Burst Mode Control Register
pub mod bmcr;
///BMTRG (rw) register accessor: an alias for `Reg<BMTRG_SPEC>`
pub type BMTRG = crate::Reg<bmtrg::BMTRG_SPEC>;
///BMTRG
pub mod bmtrg;
///BMCMPR (rw) register accessor: an alias for `Reg<BMCMPR_SPEC>`
pub type BMCMPR = crate::Reg<bmcmpr::BMCMPR_SPEC>;
///BMCMPR
pub mod bmcmpr;
///BMPER (rw) register accessor: an alias for `Reg<BMPER_SPEC>`
pub type BMPER = crate::Reg<bmper::BMPER_SPEC>;
///Burst Mode Period Register
pub mod bmper;
///EECR1 (rw) register accessor: an alias for `Reg<EECR1_SPEC>`
pub type EECR1 = crate::Reg<eecr1::EECR1_SPEC>;
///Timer External Event Control Register 1
pub mod eecr1;
///EECR2 (rw) register accessor: an alias for `Reg<EECR2_SPEC>`
pub type EECR2 = crate::Reg<eecr2::EECR2_SPEC>;
///Timer External Event Control Register 2
pub mod eecr2;
///EECR3 (rw) register accessor: an alias for `Reg<EECR3_SPEC>`
pub type EECR3 = crate::Reg<eecr3::EECR3_SPEC>;
///Timer External Event Control Register 3
pub mod eecr3;
///ADC1R (rw) register accessor: an alias for `Reg<ADC1R_SPEC>`
pub type ADC1R = crate::Reg<adc1r::ADC1R_SPEC>;
///ADC Trigger 1 Register
pub mod adc1r;
///ADC2R (rw) register accessor: an alias for `Reg<ADC2R_SPEC>`
pub type ADC2R = crate::Reg<adc2r::ADC2R_SPEC>;
///ADC Trigger 2 Register
pub mod adc2r;
///ADC3R (rw) register accessor: an alias for `Reg<ADC3R_SPEC>`
pub type ADC3R = crate::Reg<adc3r::ADC3R_SPEC>;
///ADC Trigger 3 Register
pub mod adc3r;
///ADC4R (rw) register accessor: an alias for `Reg<ADC4R_SPEC>`
pub type ADC4R = crate::Reg<adc4r::ADC4R_SPEC>;
///ADC Trigger 4 Register
pub mod adc4r;
///DLLCR (rw) register accessor: an alias for `Reg<DLLCR_SPEC>`
pub type DLLCR = crate::Reg<dllcr::DLLCR_SPEC>;
///DLL Control Register
pub mod dllcr;
///FLTINR1 (rw) register accessor: an alias for `Reg<FLTINR1_SPEC>`
pub type FLTINR1 = crate::Reg<fltinr1::FLTINR1_SPEC>;
///HRTIM Fault Input Register 1
pub mod fltinr1;
///FLTINR2 (rw) register accessor: an alias for `Reg<FLTINR2_SPEC>`
pub type FLTINR2 = crate::Reg<fltinr2::FLTINR2_SPEC>;
///HRTIM Fault Input Register 2
pub mod fltinr2;
///BDMUPDR (rw) register accessor: an alias for `Reg<BDMUPDR_SPEC>`
pub type BDMUPDR = crate::Reg<bdmupdr::BDMUPDR_SPEC>;
///BDMUPDR
pub mod bdmupdr;
///BDTAUPR (rw) register accessor: an alias for `Reg<BDTAUPR_SPEC>`
pub type BDTAUPR = crate::Reg<bdtaupr::BDTAUPR_SPEC>;
///Burst DMA Timerx update Register
pub mod bdtaupr;
///BDTBUPR (rw) register accessor: an alias for `Reg<BDTBUPR_SPEC>`
pub type BDTBUPR = crate::Reg<bdtbupr::BDTBUPR_SPEC>;
///Burst DMA Timerx update Register
pub mod bdtbupr;
///BDTCUPR (rw) register accessor: an alias for `Reg<BDTCUPR_SPEC>`
pub type BDTCUPR = crate::Reg<bdtcupr::BDTCUPR_SPEC>;
///Burst DMA Timerx update Register
pub mod bdtcupr;
///BDTDUPR (rw) register accessor: an alias for `Reg<BDTDUPR_SPEC>`
pub type BDTDUPR = crate::Reg<bdtdupr::BDTDUPR_SPEC>;
///Burst DMA Timerx update Register
pub mod bdtdupr;
///BDTEUPR (rw) register accessor: an alias for `Reg<BDTEUPR_SPEC>`
pub type BDTEUPR = crate::Reg<bdteupr::BDTEUPR_SPEC>;
///Burst DMA Timerx update Register
pub mod bdteupr;
///BDTFUPR (rw) register accessor: an alias for `Reg<BDTFUPR_SPEC>`
pub type BDTFUPR = crate::Reg<bdtfupr::BDTFUPR_SPEC>;
///Burst DMA Timerx update Register
pub mod bdtfupr;
///BDMADR (w) register accessor: an alias for `Reg<BDMADR_SPEC>`
pub type BDMADR = crate::Reg<bdmadr::BDMADR_SPEC>;
///Burst DMA Data Register
pub mod bdmadr;
///ADCER (rw) register accessor: an alias for `Reg<ADCER_SPEC>`
pub type ADCER = crate::Reg<adcer::ADCER_SPEC>;
///HRTIM ADC Extended Trigger Register
pub mod adcer;
///ADCUR (rw) register accessor: an alias for `Reg<ADCUR_SPEC>`
pub type ADCUR = crate::Reg<adcur::ADCUR_SPEC>;
///HRTIM ADC Trigger Update Register
pub mod adcur;
///ADCPS1 (rw) register accessor: an alias for `Reg<ADCPS1_SPEC>`
pub type ADCPS1 = crate::Reg<adcps1::ADCPS1_SPEC>;
///HRTIM ADC Post Scaler Register 1
pub mod adcps1;
///ADCPS2 (rw) register accessor: an alias for `Reg<ADCPS2_SPEC>`
pub type ADCPS2 = crate::Reg<adcps2::ADCPS2_SPEC>;
///HRTIM ADC Post Scaler Register 2
pub mod adcps2;
///FLTINR3 (rw) register accessor: an alias for `Reg<FLTINR3_SPEC>`
pub type FLTINR3 = crate::Reg<fltinr3::FLTINR3_SPEC>;
///HRTIM Fault Input Register 3
pub mod fltinr3;
///FLTINR4 (rw) register accessor: an alias for `Reg<FLTINR4_SPEC>`
pub type FLTINR4 = crate::Reg<fltinr4::FLTINR4_SPEC>;
///HRTIM Fault Input Register 4
pub mod fltinr4;
