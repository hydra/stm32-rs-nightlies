///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - channel configuration y register
    pub chcfg0r1: CHCFG0R1,
    ///0x04 - channel configuration y register
    pub chcfg0r2: CHCFG0R2,
    ///0x08 - analog watchdog and short-circuit detector register
    pub awscd0r: AWSCD0R,
    ///0x0c - channel watchdog filter data register
    pub chwdat0r: CHWDAT0R,
    ///0x10 - channel data input register
    pub chdatin0r: CHDATIN0R,
    _reserved5: [u8; 0x0c],
    ///0x20 - CHCFG1R1
    pub chcfg1r1: CHCFG1R1,
    ///0x24 - CHCFG1R2
    pub chcfg1r2: CHCFG1R2,
    ///0x28 - AWSCD1R
    pub awscd1r: AWSCD1R,
    ///0x2c - CHWDAT1R
    pub chwdat1r: CHWDAT1R,
    ///0x30 - CHDATIN1R
    pub chdatin1r: CHDATIN1R,
    _reserved10: [u8; 0x0c],
    ///0x40 - CHCFG2R1
    pub chcfg2r1: CHCFG2R1,
    ///0x44 - CHCFG2R2
    pub chcfg2r2: CHCFG2R2,
    ///0x48 - AWSCD2R
    pub awscd2r: AWSCD2R,
    ///0x4c - CHWDAT2R
    pub chwdat2r: CHWDAT2R,
    ///0x50 - CHDATIN2R
    pub chdatin2r: CHDATIN2R,
    _reserved15: [u8; 0x0c],
    ///0x60 - CHCFG3R1
    pub chcfg3r1: CHCFG3R1,
    ///0x64 - CHCFG3R2
    pub chcfg3r2: CHCFG3R2,
    ///0x68 - AWSCD3R
    pub awscd3r: AWSCD3R,
    ///0x6c - CHWDAT3R
    pub chwdat3r: CHWDAT3R,
    ///0x70 - CHDATIN3R
    pub chdatin3r: CHDATIN3R,
    _reserved20: [u8; 0x0c],
    ///0x80 - CHCFG4R1
    pub chcfg4r1: CHCFG4R1,
    ///0x84 - CHCFG4R2
    pub chcfg4r2: CHCFG4R2,
    ///0x88 - AWSCD4R
    pub awscd4r: AWSCD4R,
    ///0x8c - CHWDAT4R
    pub chwdat4r: CHWDAT4R,
    ///0x90 - CHDATIN4R
    pub chdatin4r: CHDATIN4R,
    _reserved25: [u8; 0x0c],
    ///0xa0 - CHCFG5R1
    pub chcfg5r1: CHCFG5R1,
    ///0xa4 - CHCFG5R2
    pub chcfg5r2: CHCFG5R2,
    ///0xa8 - AWSCD5R
    pub awscd5r: AWSCD5R,
    ///0xac - CHWDAT5R
    pub chwdat5r: CHWDAT5R,
    ///0xb0 - CHDATIN5R
    pub chdatin5r: CHDATIN5R,
    _reserved30: [u8; 0x0c],
    ///0xc0 - CHCFG6R1
    pub chcfg6r1: CHCFG6R1,
    ///0xc4 - CHCFG6R2
    pub chcfg6r2: CHCFG6R2,
    ///0xc8 - AWSCD6R
    pub awscd6r: AWSCD6R,
    ///0xcc - CHWDAT6R
    pub chwdat6r: CHWDAT6R,
    ///0xd0 - CHDATIN6R
    pub chdatin6r: CHDATIN6R,
    _reserved35: [u8; 0x0c],
    ///0xe0 - CHCFG7R1
    pub chcfg7r1: CHCFG7R1,
    ///0xe4 - CHCFG7R2
    pub chcfg7r2: CHCFG7R2,
    ///0xe8 - AWSCD7R
    pub awscd7r: AWSCD7R,
    ///0xec - CHWDAT7R
    pub chwdat7r: CHWDAT7R,
    ///0xf0 - CHDATIN7R
    pub chdatin7r: CHDATIN7R,
    _reserved40: [u8; 0x0c],
    ///0x100 - control register 1
    pub dfsdm0_cr1: DFSDM0_CR1,
    ///0x104 - control register 2
    pub dfsdm0_cr2: DFSDM0_CR2,
    ///0x108 - interrupt and status register
    pub dfsdm0_isr: DFSDM0_ISR,
    ///0x10c - interrupt flag clear register
    pub dfsdm0_icr: DFSDM0_ICR,
    ///0x110 - injected channel group selection register
    pub dfsdm0_jchgr: DFSDM0_JCHGR,
    ///0x114 - filter control register
    pub dfsdm0_fcr: DFSDM0_FCR,
    ///0x118 - data register for injected group
    pub dfsdm0_jdatar: DFSDM0_JDATAR,
    ///0x11c - data register for the regular channel
    pub dfsdm0_rdatar: DFSDM0_RDATAR,
    ///0x120 - analog watchdog high threshold register
    pub dfsdm0_awhtr: DFSDM0_AWHTR,
    ///0x124 - analog watchdog low threshold register
    pub dfsdm0_awltr: DFSDM0_AWLTR,
    ///0x128 - analog watchdog status register
    pub dfsdm0_awsr: DFSDM0_AWSR,
    ///0x12c - analog watchdog clear flag register
    pub dfsdm0_awcfr: DFSDM0_AWCFR,
    ///0x130 - Extremes detector maximum register
    pub dfsdm0_exmax: DFSDM0_EXMAX,
    ///0x134 - Extremes detector minimum register
    pub dfsdm0_exmin: DFSDM0_EXMIN,
    ///0x138 - conversion timer register
    pub dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    _reserved55: [u8; 0xc4],
    ///0x200 - control register 1
    pub cr1: CR1,
    ///0x204 - control register 2
    pub cr2: CR2,
    ///0x208 - interrupt and status register
    pub isr: ISR,
    ///0x20c - interrupt flag clear register
    pub icr: ICR,
    ///0x210 - injected channel group selection register
    pub jchgr: JCHGR,
    ///0x214 - filter control register
    pub fcr: FCR,
    ///0x218 - data register for injected group
    pub jdatar: JDATAR,
    ///0x21c - data register for the regular channel
    pub rdatar: RDATAR,
    ///0x220 - analog watchdog high threshold register
    pub awhtr: AWHTR,
    ///0x224 - analog watchdog low threshold register
    pub awltr: AWLTR,
    ///0x228 - analog watchdog status register
    pub awsr: AWSR,
    ///0x22c - analog watchdog clear flag register
    pub awcfr: AWCFR,
    ///0x230 - Extremes detector maximum register
    pub exmax: EXMAX,
    ///0x234 - Extremes detector minimum register
    pub exmin: EXMIN,
    ///0x238 - conversion timer register
    pub cnvtimr: CNVTIMR,
    _reserved70: [u8; 0xc4],
    ///0x300 - control register 1
    pub dfsdm2_cr1: DFSDM2_CR1,
    ///0x304 - control register 2
    pub dfsdm2_cr2: DFSDM2_CR2,
    ///0x308 - interrupt and status register
    pub dfsdm2_isr: DFSDM2_ISR,
    ///0x30c - interrupt flag clear register
    pub dfsdm2_icr: DFSDM2_ICR,
    ///0x310 - injected channel group selection register
    pub dfsdm2_jchgr: DFSDM2_JCHGR,
    ///0x314 - filter control register
    pub dfsdm2_fcr: DFSDM2_FCR,
    ///0x318 - data register for injected group
    pub dfsdm2_jdatar: DFSDM2_JDATAR,
    ///0x31c - data register for the regular channel
    pub dfsdm2_rdatar: DFSDM2_RDATAR,
    ///0x320 - analog watchdog high threshold register
    pub dfsdm2_awhtr: DFSDM2_AWHTR,
    ///0x324 - analog watchdog low threshold register
    pub dfsdm2_awltr: DFSDM2_AWLTR,
    ///0x328 - analog watchdog status register
    pub dfsdm2_awsr: DFSDM2_AWSR,
    ///0x32c - analog watchdog clear flag register
    pub dfsdm2_awcfr: DFSDM2_AWCFR,
    ///0x330 - Extremes detector maximum register
    pub dfsdm2_exmax: DFSDM2_EXMAX,
    ///0x334 - Extremes detector minimum register
    pub dfsdm2_exmin: DFSDM2_EXMIN,
    ///0x338 - conversion timer register
    pub dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    _reserved85: [u8; 0xc4],
    ///0x400 - control register 1
    pub dfsdm3_cr1: DFSDM3_CR1,
    ///0x404 - control register 2
    pub dfsdm3_cr2: DFSDM3_CR2,
    ///0x408 - interrupt and status register
    pub dfsdm3_isr: DFSDM3_ISR,
    ///0x40c - interrupt flag clear register
    pub dfsdm3_icr: DFSDM3_ICR,
    ///0x410 - injected channel group selection register
    pub dfsdm3_jchgr: DFSDM3_JCHGR,
    ///0x414 - filter control register
    pub dfsdm3_fcr: DFSDM3_FCR,
    ///0x418 - data register for injected group
    pub dfsdm3_jdatar: DFSDM3_JDATAR,
    ///0x41c - data register for the regular channel
    pub dfsdm3_rdatar: DFSDM3_RDATAR,
    ///0x420 - analog watchdog high threshold register
    pub dfsdm3_awhtr: DFSDM3_AWHTR,
    ///0x424 - analog watchdog low threshold register
    pub dfsdm3_awltr: DFSDM3_AWLTR,
    ///0x428 - analog watchdog status register
    pub dfsdm3_awsr: DFSDM3_AWSR,
    ///0x42c - analog watchdog clear flag register
    pub dfsdm3_awcfr: DFSDM3_AWCFR,
    ///0x430 - Extremes detector maximum register
    pub dfsdm3_exmax: DFSDM3_EXMAX,
    ///0x434 - Extremes detector minimum register
    pub dfsdm3_exmin: DFSDM3_EXMIN,
    ///0x438 - conversion timer register
    pub dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
}
///CHCFG0R1 (rw) register accessor: an alias for `Reg<CHCFG0R1_SPEC>`
pub type CHCFG0R1 = crate::Reg<chcfg0r1::CHCFG0R1_SPEC>;
///channel configuration y register
pub mod chcfg0r1;
///CHCFG0R2 (rw) register accessor: an alias for `Reg<CHCFG0R2_SPEC>`
pub type CHCFG0R2 = crate::Reg<chcfg0r2::CHCFG0R2_SPEC>;
///channel configuration y register
pub mod chcfg0r2;
///AWSCD0R (rw) register accessor: an alias for `Reg<AWSCD0R_SPEC>`
pub type AWSCD0R = crate::Reg<awscd0r::AWSCD0R_SPEC>;
///analog watchdog and short-circuit detector register
pub mod awscd0r;
///CHWDAT0R (rw) register accessor: an alias for `Reg<CHWDAT0R_SPEC>`
pub type CHWDAT0R = crate::Reg<chwdat0r::CHWDAT0R_SPEC>;
///channel watchdog filter data register
pub mod chwdat0r;
///CHDATIN0R (rw) register accessor: an alias for `Reg<CHDATIN0R_SPEC>`
pub type CHDATIN0R = crate::Reg<chdatin0r::CHDATIN0R_SPEC>;
///channel data input register
pub mod chdatin0r;
///CHCFG1R1 (rw) register accessor: an alias for `Reg<CHCFG1R1_SPEC>`
pub type CHCFG1R1 = crate::Reg<chcfg1r1::CHCFG1R1_SPEC>;
///CHCFG1R1
pub mod chcfg1r1;
///CHCFG1R2 (rw) register accessor: an alias for `Reg<CHCFG1R2_SPEC>`
pub type CHCFG1R2 = crate::Reg<chcfg1r2::CHCFG1R2_SPEC>;
///CHCFG1R2
pub mod chcfg1r2;
///AWSCD1R (rw) register accessor: an alias for `Reg<AWSCD1R_SPEC>`
pub type AWSCD1R = crate::Reg<awscd1r::AWSCD1R_SPEC>;
///AWSCD1R
pub mod awscd1r;
///CHWDAT1R (rw) register accessor: an alias for `Reg<CHWDAT1R_SPEC>`
pub type CHWDAT1R = crate::Reg<chwdat1r::CHWDAT1R_SPEC>;
///CHWDAT1R
pub mod chwdat1r;
///CHDATIN1R (rw) register accessor: an alias for `Reg<CHDATIN1R_SPEC>`
pub type CHDATIN1R = crate::Reg<chdatin1r::CHDATIN1R_SPEC>;
///CHDATIN1R
pub mod chdatin1r;
///CHCFG2R1 (rw) register accessor: an alias for `Reg<CHCFG2R1_SPEC>`
pub type CHCFG2R1 = crate::Reg<chcfg2r1::CHCFG2R1_SPEC>;
///CHCFG2R1
pub mod chcfg2r1;
///CHCFG2R2 (rw) register accessor: an alias for `Reg<CHCFG2R2_SPEC>`
pub type CHCFG2R2 = crate::Reg<chcfg2r2::CHCFG2R2_SPEC>;
///CHCFG2R2
pub mod chcfg2r2;
///AWSCD2R (rw) register accessor: an alias for `Reg<AWSCD2R_SPEC>`
pub type AWSCD2R = crate::Reg<awscd2r::AWSCD2R_SPEC>;
///AWSCD2R
pub mod awscd2r;
///CHWDAT2R (rw) register accessor: an alias for `Reg<CHWDAT2R_SPEC>`
pub type CHWDAT2R = crate::Reg<chwdat2r::CHWDAT2R_SPEC>;
///CHWDAT2R
pub mod chwdat2r;
///CHDATIN2R (rw) register accessor: an alias for `Reg<CHDATIN2R_SPEC>`
pub type CHDATIN2R = crate::Reg<chdatin2r::CHDATIN2R_SPEC>;
///CHDATIN2R
pub mod chdatin2r;
///CHCFG3R1 (rw) register accessor: an alias for `Reg<CHCFG3R1_SPEC>`
pub type CHCFG3R1 = crate::Reg<chcfg3r1::CHCFG3R1_SPEC>;
///CHCFG3R1
pub mod chcfg3r1;
///CHCFG3R2 (rw) register accessor: an alias for `Reg<CHCFG3R2_SPEC>`
pub type CHCFG3R2 = crate::Reg<chcfg3r2::CHCFG3R2_SPEC>;
///CHCFG3R2
pub mod chcfg3r2;
///AWSCD3R (rw) register accessor: an alias for `Reg<AWSCD3R_SPEC>`
pub type AWSCD3R = crate::Reg<awscd3r::AWSCD3R_SPEC>;
///AWSCD3R
pub mod awscd3r;
///CHWDAT3R (rw) register accessor: an alias for `Reg<CHWDAT3R_SPEC>`
pub type CHWDAT3R = crate::Reg<chwdat3r::CHWDAT3R_SPEC>;
///CHWDAT3R
pub mod chwdat3r;
///CHDATIN3R (rw) register accessor: an alias for `Reg<CHDATIN3R_SPEC>`
pub type CHDATIN3R = crate::Reg<chdatin3r::CHDATIN3R_SPEC>;
///CHDATIN3R
pub mod chdatin3r;
///CHCFG4R1 (rw) register accessor: an alias for `Reg<CHCFG4R1_SPEC>`
pub type CHCFG4R1 = crate::Reg<chcfg4r1::CHCFG4R1_SPEC>;
///CHCFG4R1
pub mod chcfg4r1;
///CHCFG4R2 (rw) register accessor: an alias for `Reg<CHCFG4R2_SPEC>`
pub type CHCFG4R2 = crate::Reg<chcfg4r2::CHCFG4R2_SPEC>;
///CHCFG4R2
pub mod chcfg4r2;
///AWSCD4R (rw) register accessor: an alias for `Reg<AWSCD4R_SPEC>`
pub type AWSCD4R = crate::Reg<awscd4r::AWSCD4R_SPEC>;
///AWSCD4R
pub mod awscd4r;
///CHWDAT4R (rw) register accessor: an alias for `Reg<CHWDAT4R_SPEC>`
pub type CHWDAT4R = crate::Reg<chwdat4r::CHWDAT4R_SPEC>;
///CHWDAT4R
pub mod chwdat4r;
///CHDATIN4R (rw) register accessor: an alias for `Reg<CHDATIN4R_SPEC>`
pub type CHDATIN4R = crate::Reg<chdatin4r::CHDATIN4R_SPEC>;
///CHDATIN4R
pub mod chdatin4r;
///CHCFG5R1 (rw) register accessor: an alias for `Reg<CHCFG5R1_SPEC>`
pub type CHCFG5R1 = crate::Reg<chcfg5r1::CHCFG5R1_SPEC>;
///CHCFG5R1
pub mod chcfg5r1;
///CHCFG5R2 (rw) register accessor: an alias for `Reg<CHCFG5R2_SPEC>`
pub type CHCFG5R2 = crate::Reg<chcfg5r2::CHCFG5R2_SPEC>;
///CHCFG5R2
pub mod chcfg5r2;
///AWSCD5R (rw) register accessor: an alias for `Reg<AWSCD5R_SPEC>`
pub type AWSCD5R = crate::Reg<awscd5r::AWSCD5R_SPEC>;
///AWSCD5R
pub mod awscd5r;
///CHWDAT5R (rw) register accessor: an alias for `Reg<CHWDAT5R_SPEC>`
pub type CHWDAT5R = crate::Reg<chwdat5r::CHWDAT5R_SPEC>;
///CHWDAT5R
pub mod chwdat5r;
///CHDATIN5R (rw) register accessor: an alias for `Reg<CHDATIN5R_SPEC>`
pub type CHDATIN5R = crate::Reg<chdatin5r::CHDATIN5R_SPEC>;
///CHDATIN5R
pub mod chdatin5r;
///CHCFG6R1 (rw) register accessor: an alias for `Reg<CHCFG6R1_SPEC>`
pub type CHCFG6R1 = crate::Reg<chcfg6r1::CHCFG6R1_SPEC>;
///CHCFG6R1
pub mod chcfg6r1;
///CHCFG6R2 (rw) register accessor: an alias for `Reg<CHCFG6R2_SPEC>`
pub type CHCFG6R2 = crate::Reg<chcfg6r2::CHCFG6R2_SPEC>;
///CHCFG6R2
pub mod chcfg6r2;
///AWSCD6R (rw) register accessor: an alias for `Reg<AWSCD6R_SPEC>`
pub type AWSCD6R = crate::Reg<awscd6r::AWSCD6R_SPEC>;
///AWSCD6R
pub mod awscd6r;
///CHWDAT6R (rw) register accessor: an alias for `Reg<CHWDAT6R_SPEC>`
pub type CHWDAT6R = crate::Reg<chwdat6r::CHWDAT6R_SPEC>;
///CHWDAT6R
pub mod chwdat6r;
///CHDATIN6R (rw) register accessor: an alias for `Reg<CHDATIN6R_SPEC>`
pub type CHDATIN6R = crate::Reg<chdatin6r::CHDATIN6R_SPEC>;
///CHDATIN6R
pub mod chdatin6r;
///CHCFG7R1 (rw) register accessor: an alias for `Reg<CHCFG7R1_SPEC>`
pub type CHCFG7R1 = crate::Reg<chcfg7r1::CHCFG7R1_SPEC>;
///CHCFG7R1
pub mod chcfg7r1;
///CHCFG7R2 (rw) register accessor: an alias for `Reg<CHCFG7R2_SPEC>`
pub type CHCFG7R2 = crate::Reg<chcfg7r2::CHCFG7R2_SPEC>;
///CHCFG7R2
pub mod chcfg7r2;
///AWSCD7R (rw) register accessor: an alias for `Reg<AWSCD7R_SPEC>`
pub type AWSCD7R = crate::Reg<awscd7r::AWSCD7R_SPEC>;
///AWSCD7R
pub mod awscd7r;
///CHWDAT7R (rw) register accessor: an alias for `Reg<CHWDAT7R_SPEC>`
pub type CHWDAT7R = crate::Reg<chwdat7r::CHWDAT7R_SPEC>;
///CHWDAT7R
pub mod chwdat7r;
///CHDATIN7R (rw) register accessor: an alias for `Reg<CHDATIN7R_SPEC>`
pub type CHDATIN7R = crate::Reg<chdatin7r::CHDATIN7R_SPEC>;
///CHDATIN7R
pub mod chdatin7r;
///DFSDM0_CR1 (rw) register accessor: an alias for `Reg<DFSDM0_CR1_SPEC>`
pub type DFSDM0_CR1 = crate::Reg<dfsdm0_cr1::DFSDM0_CR1_SPEC>;
///control register 1
pub mod dfsdm0_cr1;
///DFSDM0_CR2 (rw) register accessor: an alias for `Reg<DFSDM0_CR2_SPEC>`
pub type DFSDM0_CR2 = crate::Reg<dfsdm0_cr2::DFSDM0_CR2_SPEC>;
///control register 2
pub mod dfsdm0_cr2;
///DFSDM0_ISR (r) register accessor: an alias for `Reg<DFSDM0_ISR_SPEC>`
pub type DFSDM0_ISR = crate::Reg<dfsdm0_isr::DFSDM0_ISR_SPEC>;
///interrupt and status register
pub mod dfsdm0_isr;
///DFSDM0_ICR (rw) register accessor: an alias for `Reg<DFSDM0_ICR_SPEC>`
pub type DFSDM0_ICR = crate::Reg<dfsdm0_icr::DFSDM0_ICR_SPEC>;
///interrupt flag clear register
pub mod dfsdm0_icr;
///DFSDM0_JCHGR (rw) register accessor: an alias for `Reg<DFSDM0_JCHGR_SPEC>`
pub type DFSDM0_JCHGR = crate::Reg<dfsdm0_jchgr::DFSDM0_JCHGR_SPEC>;
///injected channel group selection register
pub mod dfsdm0_jchgr;
///DFSDM0_FCR (rw) register accessor: an alias for `Reg<DFSDM0_FCR_SPEC>`
pub type DFSDM0_FCR = crate::Reg<dfsdm0_fcr::DFSDM0_FCR_SPEC>;
///filter control register
pub mod dfsdm0_fcr;
///DFSDM0_JDATAR (r) register accessor: an alias for `Reg<DFSDM0_JDATAR_SPEC>`
pub type DFSDM0_JDATAR = crate::Reg<dfsdm0_jdatar::DFSDM0_JDATAR_SPEC>;
///data register for injected group
pub mod dfsdm0_jdatar;
///DFSDM0_RDATAR (r) register accessor: an alias for `Reg<DFSDM0_RDATAR_SPEC>`
pub type DFSDM0_RDATAR = crate::Reg<dfsdm0_rdatar::DFSDM0_RDATAR_SPEC>;
///data register for the regular channel
pub mod dfsdm0_rdatar;
///DFSDM0_AWHTR (rw) register accessor: an alias for `Reg<DFSDM0_AWHTR_SPEC>`
pub type DFSDM0_AWHTR = crate::Reg<dfsdm0_awhtr::DFSDM0_AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod dfsdm0_awhtr;
///DFSDM0_AWLTR (rw) register accessor: an alias for `Reg<DFSDM0_AWLTR_SPEC>`
pub type DFSDM0_AWLTR = crate::Reg<dfsdm0_awltr::DFSDM0_AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod dfsdm0_awltr;
///DFSDM0_AWSR (r) register accessor: an alias for `Reg<DFSDM0_AWSR_SPEC>`
pub type DFSDM0_AWSR = crate::Reg<dfsdm0_awsr::DFSDM0_AWSR_SPEC>;
///analog watchdog status register
pub mod dfsdm0_awsr;
///DFSDM0_AWCFR (rw) register accessor: an alias for `Reg<DFSDM0_AWCFR_SPEC>`
pub type DFSDM0_AWCFR = crate::Reg<dfsdm0_awcfr::DFSDM0_AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod dfsdm0_awcfr;
///DFSDM0_EXMAX (r) register accessor: an alias for `Reg<DFSDM0_EXMAX_SPEC>`
pub type DFSDM0_EXMAX = crate::Reg<dfsdm0_exmax::DFSDM0_EXMAX_SPEC>;
///Extremes detector maximum register
pub mod dfsdm0_exmax;
///DFSDM0_EXMIN (r) register accessor: an alias for `Reg<DFSDM0_EXMIN_SPEC>`
pub type DFSDM0_EXMIN = crate::Reg<dfsdm0_exmin::DFSDM0_EXMIN_SPEC>;
///Extremes detector minimum register
pub mod dfsdm0_exmin;
///DFSDM0_CNVTIMR (r) register accessor: an alias for `Reg<DFSDM0_CNVTIMR_SPEC>`
pub type DFSDM0_CNVTIMR = crate::Reg<dfsdm0_cnvtimr::DFSDM0_CNVTIMR_SPEC>;
///conversion timer register
pub mod dfsdm0_cnvtimr;
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt and status register
pub mod isr;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt flag clear register
pub mod icr;
///JCHGR (rw) register accessor: an alias for `Reg<JCHGR_SPEC>`
pub type JCHGR = crate::Reg<jchgr::JCHGR_SPEC>;
///injected channel group selection register
pub mod jchgr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///filter control register
pub mod fcr;
///JDATAR (r) register accessor: an alias for `Reg<JDATAR_SPEC>`
pub type JDATAR = crate::Reg<jdatar::JDATAR_SPEC>;
///data register for injected group
pub mod jdatar;
///RDATAR (r) register accessor: an alias for `Reg<RDATAR_SPEC>`
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
///data register for the regular channel
pub mod rdatar;
///AWHTR (rw) register accessor: an alias for `Reg<AWHTR_SPEC>`
pub type AWHTR = crate::Reg<awhtr::AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod awhtr;
///AWLTR (rw) register accessor: an alias for `Reg<AWLTR_SPEC>`
pub type AWLTR = crate::Reg<awltr::AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod awltr;
///AWSR (r) register accessor: an alias for `Reg<AWSR_SPEC>`
pub type AWSR = crate::Reg<awsr::AWSR_SPEC>;
///analog watchdog status register
pub mod awsr;
///AWCFR (rw) register accessor: an alias for `Reg<AWCFR_SPEC>`
pub type AWCFR = crate::Reg<awcfr::AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod awcfr;
///EXMAX (r) register accessor: an alias for `Reg<EXMAX_SPEC>`
pub type EXMAX = crate::Reg<exmax::EXMAX_SPEC>;
///Extremes detector maximum register
pub mod exmax;
///EXMIN (r) register accessor: an alias for `Reg<EXMIN_SPEC>`
pub type EXMIN = crate::Reg<exmin::EXMIN_SPEC>;
///Extremes detector minimum register
pub mod exmin;
///CNVTIMR (r) register accessor: an alias for `Reg<CNVTIMR_SPEC>`
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMR_SPEC>;
///conversion timer register
pub mod cnvtimr;
///DFSDM2_CR1 (rw) register accessor: an alias for `Reg<DFSDM2_CR1_SPEC>`
pub type DFSDM2_CR1 = crate::Reg<dfsdm2_cr1::DFSDM2_CR1_SPEC>;
///control register 1
pub mod dfsdm2_cr1;
///DFSDM2_CR2 (rw) register accessor: an alias for `Reg<DFSDM2_CR2_SPEC>`
pub type DFSDM2_CR2 = crate::Reg<dfsdm2_cr2::DFSDM2_CR2_SPEC>;
///control register 2
pub mod dfsdm2_cr2;
///DFSDM2_ISR (r) register accessor: an alias for `Reg<DFSDM2_ISR_SPEC>`
pub type DFSDM2_ISR = crate::Reg<dfsdm2_isr::DFSDM2_ISR_SPEC>;
///interrupt and status register
pub mod dfsdm2_isr;
///DFSDM2_ICR (rw) register accessor: an alias for `Reg<DFSDM2_ICR_SPEC>`
pub type DFSDM2_ICR = crate::Reg<dfsdm2_icr::DFSDM2_ICR_SPEC>;
///interrupt flag clear register
pub mod dfsdm2_icr;
///DFSDM2_JCHGR (rw) register accessor: an alias for `Reg<DFSDM2_JCHGR_SPEC>`
pub type DFSDM2_JCHGR = crate::Reg<dfsdm2_jchgr::DFSDM2_JCHGR_SPEC>;
///injected channel group selection register
pub mod dfsdm2_jchgr;
///DFSDM2_FCR (rw) register accessor: an alias for `Reg<DFSDM2_FCR_SPEC>`
pub type DFSDM2_FCR = crate::Reg<dfsdm2_fcr::DFSDM2_FCR_SPEC>;
///filter control register
pub mod dfsdm2_fcr;
///DFSDM2_JDATAR (r) register accessor: an alias for `Reg<DFSDM2_JDATAR_SPEC>`
pub type DFSDM2_JDATAR = crate::Reg<dfsdm2_jdatar::DFSDM2_JDATAR_SPEC>;
///data register for injected group
pub mod dfsdm2_jdatar;
///DFSDM2_RDATAR (r) register accessor: an alias for `Reg<DFSDM2_RDATAR_SPEC>`
pub type DFSDM2_RDATAR = crate::Reg<dfsdm2_rdatar::DFSDM2_RDATAR_SPEC>;
///data register for the regular channel
pub mod dfsdm2_rdatar;
///DFSDM2_AWHTR (rw) register accessor: an alias for `Reg<DFSDM2_AWHTR_SPEC>`
pub type DFSDM2_AWHTR = crate::Reg<dfsdm2_awhtr::DFSDM2_AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod dfsdm2_awhtr;
///DFSDM2_AWLTR (rw) register accessor: an alias for `Reg<DFSDM2_AWLTR_SPEC>`
pub type DFSDM2_AWLTR = crate::Reg<dfsdm2_awltr::DFSDM2_AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod dfsdm2_awltr;
///DFSDM2_AWSR (r) register accessor: an alias for `Reg<DFSDM2_AWSR_SPEC>`
pub type DFSDM2_AWSR = crate::Reg<dfsdm2_awsr::DFSDM2_AWSR_SPEC>;
///analog watchdog status register
pub mod dfsdm2_awsr;
///DFSDM2_AWCFR (rw) register accessor: an alias for `Reg<DFSDM2_AWCFR_SPEC>`
pub type DFSDM2_AWCFR = crate::Reg<dfsdm2_awcfr::DFSDM2_AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod dfsdm2_awcfr;
///DFSDM2_EXMAX (r) register accessor: an alias for `Reg<DFSDM2_EXMAX_SPEC>`
pub type DFSDM2_EXMAX = crate::Reg<dfsdm2_exmax::DFSDM2_EXMAX_SPEC>;
///Extremes detector maximum register
pub mod dfsdm2_exmax;
///DFSDM2_EXMIN (r) register accessor: an alias for `Reg<DFSDM2_EXMIN_SPEC>`
pub type DFSDM2_EXMIN = crate::Reg<dfsdm2_exmin::DFSDM2_EXMIN_SPEC>;
///Extremes detector minimum register
pub mod dfsdm2_exmin;
///DFSDM2_CNVTIMR (r) register accessor: an alias for `Reg<DFSDM2_CNVTIMR_SPEC>`
pub type DFSDM2_CNVTIMR = crate::Reg<dfsdm2_cnvtimr::DFSDM2_CNVTIMR_SPEC>;
///conversion timer register
pub mod dfsdm2_cnvtimr;
///DFSDM3_CR1 (rw) register accessor: an alias for `Reg<DFSDM3_CR1_SPEC>`
pub type DFSDM3_CR1 = crate::Reg<dfsdm3_cr1::DFSDM3_CR1_SPEC>;
///control register 1
pub mod dfsdm3_cr1;
///DFSDM3_CR2 (rw) register accessor: an alias for `Reg<DFSDM3_CR2_SPEC>`
pub type DFSDM3_CR2 = crate::Reg<dfsdm3_cr2::DFSDM3_CR2_SPEC>;
///control register 2
pub mod dfsdm3_cr2;
///DFSDM3_ISR (r) register accessor: an alias for `Reg<DFSDM3_ISR_SPEC>`
pub type DFSDM3_ISR = crate::Reg<dfsdm3_isr::DFSDM3_ISR_SPEC>;
///interrupt and status register
pub mod dfsdm3_isr;
///DFSDM3_ICR (rw) register accessor: an alias for `Reg<DFSDM3_ICR_SPEC>`
pub type DFSDM3_ICR = crate::Reg<dfsdm3_icr::DFSDM3_ICR_SPEC>;
///interrupt flag clear register
pub mod dfsdm3_icr;
///DFSDM3_JCHGR (rw) register accessor: an alias for `Reg<DFSDM3_JCHGR_SPEC>`
pub type DFSDM3_JCHGR = crate::Reg<dfsdm3_jchgr::DFSDM3_JCHGR_SPEC>;
///injected channel group selection register
pub mod dfsdm3_jchgr;
///DFSDM3_FCR (rw) register accessor: an alias for `Reg<DFSDM3_FCR_SPEC>`
pub type DFSDM3_FCR = crate::Reg<dfsdm3_fcr::DFSDM3_FCR_SPEC>;
///filter control register
pub mod dfsdm3_fcr;
///DFSDM3_JDATAR (r) register accessor: an alias for `Reg<DFSDM3_JDATAR_SPEC>`
pub type DFSDM3_JDATAR = crate::Reg<dfsdm3_jdatar::DFSDM3_JDATAR_SPEC>;
///data register for injected group
pub mod dfsdm3_jdatar;
///DFSDM3_RDATAR (r) register accessor: an alias for `Reg<DFSDM3_RDATAR_SPEC>`
pub type DFSDM3_RDATAR = crate::Reg<dfsdm3_rdatar::DFSDM3_RDATAR_SPEC>;
///data register for the regular channel
pub mod dfsdm3_rdatar;
///DFSDM3_AWHTR (rw) register accessor: an alias for `Reg<DFSDM3_AWHTR_SPEC>`
pub type DFSDM3_AWHTR = crate::Reg<dfsdm3_awhtr::DFSDM3_AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod dfsdm3_awhtr;
///DFSDM3_AWLTR (rw) register accessor: an alias for `Reg<DFSDM3_AWLTR_SPEC>`
pub type DFSDM3_AWLTR = crate::Reg<dfsdm3_awltr::DFSDM3_AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod dfsdm3_awltr;
///DFSDM3_AWSR (r) register accessor: an alias for `Reg<DFSDM3_AWSR_SPEC>`
pub type DFSDM3_AWSR = crate::Reg<dfsdm3_awsr::DFSDM3_AWSR_SPEC>;
///analog watchdog status register
pub mod dfsdm3_awsr;
///DFSDM3_AWCFR (rw) register accessor: an alias for `Reg<DFSDM3_AWCFR_SPEC>`
pub type DFSDM3_AWCFR = crate::Reg<dfsdm3_awcfr::DFSDM3_AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod dfsdm3_awcfr;
///DFSDM3_EXMAX (r) register accessor: an alias for `Reg<DFSDM3_EXMAX_SPEC>`
pub type DFSDM3_EXMAX = crate::Reg<dfsdm3_exmax::DFSDM3_EXMAX_SPEC>;
///Extremes detector maximum register
pub mod dfsdm3_exmax;
///DFSDM3_EXMIN (r) register accessor: an alias for `Reg<DFSDM3_EXMIN_SPEC>`
pub type DFSDM3_EXMIN = crate::Reg<dfsdm3_exmin::DFSDM3_EXMIN_SPEC>;
///Extremes detector minimum register
pub mod dfsdm3_exmin;
///DFSDM3_CNVTIMR (r) register accessor: an alias for `Reg<DFSDM3_CNVTIMR_SPEC>`
pub type DFSDM3_CNVTIMR = crate::Reg<dfsdm3_cnvtimr::DFSDM3_CNVTIMR_SPEC>;
///conversion timer register
pub mod dfsdm3_cnvtimr;
