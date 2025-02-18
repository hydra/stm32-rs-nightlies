///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DFSDM channel configuration 0 register 1
    pub dfsdm_chcfg0r1: DFSDM_CHCFG0R1,
    ///0x04 - DFSDM channel configuration 0 register 2
    pub dfsdm_chcfg0r2: DFSDM_CHCFG0R2,
    ///0x08 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd0r: DFSDM_AWSCD0R,
    ///0x0c - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat0r: DFSDM_CHWDAT0R,
    ///0x10 - DFSDM channel data input register
    pub dfsdm_chdatin0r: DFSDM_CHDATIN0R,
    _reserved5: [u8; 0x0c],
    ///0x20 - DFSDM channel configuration 1 register 1
    pub dfsdm_chcfg1r1: DFSDM_CHCFG1R1,
    ///0x24 - DFSDM channel configuration 1 register 2
    pub dfsdm_chcfg1r2: DFSDM_CHCFG1R2,
    ///0x28 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd1r: DFSDM_AWSCD1R,
    ///0x2c - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat1r: DFSDM_CHWDAT1R,
    ///0x30 - DFSDM channel data input register
    pub dfsdm_chdatin1r: DFSDM_CHDATIN1R,
    _reserved10: [u8; 0x0c],
    ///0x40 - DFSDM channel configuration 2 register 1
    pub dfsdm_chcfg2r1: DFSDM_CHCFG2R1,
    ///0x44 - DFSDM channel configuration 2 register 2
    pub dfsdm_chcfg2r2: DFSDM_CHCFG2R2,
    ///0x48 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd2r: DFSDM_AWSCD2R,
    ///0x4c - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat2r: DFSDM_CHWDAT2R,
    ///0x50 - DFSDM channel data input register
    pub dfsdm_chdatin2r: DFSDM_CHDATIN2R,
    _reserved15: [u8; 0x0c],
    ///0x60 - DFSDM channel configuration 3 register 1
    pub dfsdm_chcfg3r1: DFSDM_CHCFG3R1,
    ///0x64 - DFSDM channel configuration 3 register 2
    pub dfsdm_chcfg3r2: DFSDM_CHCFG3R2,
    ///0x68 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd3r: DFSDM_AWSCD3R,
    ///0x6c - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat3r: DFSDM_CHWDAT3R,
    ///0x70 - DFSDM channel data input register
    pub dfsdm_chdatin3r: DFSDM_CHDATIN3R,
    _reserved20: [u8; 0x0c],
    ///0x80 - DFSDM channel configuration 4 register 1
    pub dfsdm_chcfg4r1: DFSDM_CHCFG4R1,
    ///0x84 - DFSDM channel configuration 4 register 2
    pub dfsdm_chcfg4r2: DFSDM_CHCFG4R2,
    ///0x88 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd4r: DFSDM_AWSCD4R,
    ///0x8c - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat4r: DFSDM_CHWDAT4R,
    ///0x90 - DFSDM channel data input register
    pub dfsdm_chdatin4r: DFSDM_CHDATIN4R,
    _reserved25: [u8; 0x0c],
    ///0xa0 - DFSDM channel configuration 5 register 1
    pub dfsdm_chcfg5r1: DFSDM_CHCFG5R1,
    ///0xa4 - DFSDM channel configuration 5 register 2
    pub dfsdm_chcfg5r2: DFSDM_CHCFG5R2,
    ///0xa8 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd5r: DFSDM_AWSCD5R,
    ///0xac - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat5r: DFSDM_CHWDAT5R,
    ///0xb0 - DFSDM channel data input register
    pub dfsdm_chdatin5r: DFSDM_CHDATIN5R,
    _reserved30: [u8; 0x0c],
    ///0xc0 - DFSDM channel configuration 6 register 1
    pub dfsdm_chcfg6r1: DFSDM_CHCFG6R1,
    ///0xc4 - DFSDM channel configuration 6 register 2
    pub dfsdm_chcfg6r2: DFSDM_CHCFG6R2,
    ///0xc8 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd6r: DFSDM_AWSCD6R,
    ///0xcc - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat6r: DFSDM_CHWDAT6R,
    ///0xd0 - DFSDM channel data input register
    pub dfsdm_chdatin6r: DFSDM_CHDATIN6R,
    _reserved35: [u8; 0x0c],
    ///0xe0 - DFSDM channel configuration 7 register 1
    pub dfsdm_chcfg7r1: DFSDM_CHCFG7R1,
    ///0xe4 - DFSDM channel configuration 7 register 2
    pub dfsdm_chcfg7r2: DFSDM_CHCFG7R2,
    ///0xe8 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd7r: DFSDM_AWSCD7R,
    ///0xec - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat7r: DFSDM_CHWDAT7R,
    ///0xf0 - DFSDM channel data input register
    pub dfsdm_chdatin7r: DFSDM_CHDATIN7R,
    _reserved40: [u8; 0x0c],
    ///0x100 - DFSDM control register 1
    pub dfsdm0_cr1: DFSDM0_CR1,
    ///0x104 - DFSDM control register 2
    pub dfsdm0_cr2: DFSDM0_CR2,
    ///0x108 - DFSDM interrupt and status register
    pub dfsdm0_isr: DFSDM0_ISR,
    ///0x10c - DFSDM interrupt flag clear register
    pub dfsdm0_icr: DFSDM0_ICR,
    ///0x110 - DFSDM injected channel group selection register
    pub dfsdm0_jchgr: DFSDM0_JCHGR,
    ///0x114 - DFSDM filter control register
    pub dfsdm0_fcr: DFSDM0_FCR,
    ///0x118 - DFSDM data register for injected group
    pub dfsdm0_jdatar: DFSDM0_JDATAR,
    ///0x11c - DFSDM data register for the regular channel
    pub dfsdm0_rdatar: DFSDM0_RDATAR,
    ///0x120 - DFSDM analog watchdog high threshold register
    pub dfsdm0_awhtr: DFSDM0_AWHTR,
    ///0x124 - DFSDM analog watchdog low threshold register
    pub dfsdm0_awltr: DFSDM0_AWLTR,
    ///0x128 - DFSDM analog watchdog status register
    pub dfsdm0_awsr: DFSDM0_AWSR,
    ///0x12c - DFSDM analog watchdog clear flag register
    pub dfsdm0_awcfr: DFSDM0_AWCFR,
    ///0x130 - DFSDM Extremes detector maximum register
    pub dfsdm0_exmax: DFSDM0_EXMAX,
    ///0x134 - DFSDM Extremes detector minimum register
    pub dfsdm0_exmin: DFSDM0_EXMIN,
    ///0x138 - DFSDM conversion timer register
    pub dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    _reserved55: [u8; 0x44],
    ///0x180 - DFSDM control register 1
    pub cr1: CR1,
    ///0x184 - DFSDM control register 2
    pub cr2: CR2,
    ///0x188 - DFSDM interrupt and status register
    pub isr: ISR,
    ///0x18c - DFSDM interrupt flag clear register
    pub icr: ICR,
    ///0x190 - DFSDM injected channel group selection register
    pub jchgr: JCHGR,
    ///0x194 - DFSDM filter control register
    pub fcr: FCR,
    _reserved_61_jdatar: [u8; 0x04],
    _reserved62: [u8; 0x04],
    ///0x1a0 - DFSDM analog watchdog high threshold register
    pub awhtr: AWHTR,
    ///0x1a4 - DFSDM analog watchdog low threshold register
    pub awltr: AWLTR,
    ///0x1a8 - DFSDM analog watchdog status register
    pub awsr: AWSR,
    ///0x1ac - DFSDM analog watchdog clear flag register
    pub awcfr: AWCFR,
    ///0x1b0 - DFSDM Extremes detector maximum register
    pub exmax: EXMAX,
    ///0x1b4 - DFSDM Extremes detector minimum register
    pub exmin: EXMIN,
    ///0x1b8 - DFSDM conversion timer register
    pub cnvtimr: CNVTIMR,
    _reserved69: [u8; 0x44],
    ///0x200 - DFSDM control register 1
    pub dfsdm2_cr1: DFSDM2_CR1,
    ///0x204 - DFSDM control register 2
    pub dfsdm2_cr2: DFSDM2_CR2,
    ///0x208 - DFSDM interrupt and status register
    pub dfsdm2_isr: DFSDM2_ISR,
    ///0x20c - DFSDM interrupt flag clear register
    pub dfsdm2_icr: DFSDM2_ICR,
    ///0x210 - DFSDM injected channel group selection register
    pub dfsdm2_jchgr: DFSDM2_JCHGR,
    ///0x214 - DFSDM filter control register
    pub dfsdm2_fcr: DFSDM2_FCR,
    _reserved_75_dfsdm2: [u8; 0x04],
    _reserved76: [u8; 0x04],
    ///0x220 - DFSDM analog watchdog high threshold register
    pub dfsdm2_awhtr: DFSDM2_AWHTR,
    ///0x224 - DFSDM analog watchdog low threshold register
    pub dfsdm2_awltr: DFSDM2_AWLTR,
    ///0x228 - DFSDM analog watchdog status register
    pub dfsdm2_awsr: DFSDM2_AWSR,
    ///0x22c - DFSDM analog watchdog clear flag register
    pub dfsdm2_awcfr: DFSDM2_AWCFR,
    ///0x230 - DFSDM Extremes detector maximum register
    pub dfsdm2_exmax: DFSDM2_EXMAX,
    ///0x234 - DFSDM Extremes detector minimum register
    pub dfsdm2_exmin: DFSDM2_EXMIN,
    ///0x238 - DFSDM conversion timer register
    pub dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    _reserved83: [u8; 0x64],
    ///0x2a0 - DFSDM analog watchdog high threshold register
    pub dfsdm3_awhtr: DFSDM3_AWHTR,
    ///0x2a4 - DFSDM analog watchdog low threshold register
    pub dfsdm3_awltr: DFSDM3_AWLTR,
    ///0x2a8 - DFSDM analog watchdog status register
    pub dfsdm3_awsr: DFSDM3_AWSR,
    ///0x2ac - DFSDM analog watchdog clear flag register
    pub dfsdm3_awcfr: DFSDM3_AWCFR,
    ///0x2b0 - DFSDM Extremes detector maximum register
    pub dfsdm3_exmax: DFSDM3_EXMAX,
    ///0x2b4 - DFSDM Extremes detector minimum register
    pub dfsdm3_exmin: DFSDM3_EXMIN,
    ///0x2b8 - DFSDM conversion timer register
    pub dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
    _reserved90: [u8; 0x54],
    ///0x310 - DFSDM injected channel group selection register
    pub dfsdm3_jchgr: DFSDM3_JCHGR,
    ///0x314 - DFSDM filter control register
    pub dfsdm3_fcr: DFSDM3_FCR,
    _reserved_92_dfsdm3: [u8; 0x04],
    _reserved93: [u8; 0x64],
    ///0x380 - DFSDM control register 1
    pub dfsdm3_cr1: DFSDM3_CR1,
    ///0x384 - DFSDM control register 2
    pub dfsdm3_cr2: DFSDM3_CR2,
    ///0x388 - DFSDM interrupt and status register
    pub dfsdm3_isr: DFSDM3_ISR,
    ///0x38c - DFSDM interrupt flag clear register
    pub dfsdm3_icr: DFSDM3_ICR,
}
impl RegisterBlock {
    ///0x198 - DFSDM data register for the regular channel
    #[inline(always)]
    pub const fn rdatar(&self) -> &RDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(408usize).cast() }
    }
    ///0x198 - DFSDM data register for injected group
    #[inline(always)]
    pub const fn jdatar(&self) -> &JDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(408usize).cast() }
    }
    ///0x218 - DFSDM data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm2_rdatar(&self) -> &DFSDM2_RDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(536usize).cast() }
    }
    ///0x218 - DFSDM data register for injected group
    #[inline(always)]
    pub const fn dfsdm2_jdatar(&self) -> &DFSDM2_JDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(536usize).cast() }
    }
    ///0x318 - DFSDM data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm3_rdatar(&self) -> &DFSDM3_RDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(792usize).cast() }
    }
    ///0x318 - DFSDM data register for injected group
    #[inline(always)]
    pub const fn dfsdm3_jdatar(&self) -> &DFSDM3_JDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(792usize).cast() }
    }
}
///DFSDM_CHCFG0R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG0R1_SPEC>`
pub type DFSDM_CHCFG0R1 = crate::Reg<dfsdm_chcfg0r1::DFSDM_CHCFG0R1_SPEC>;
///DFSDM channel configuration 0 register 1
pub mod dfsdm_chcfg0r1;
///DFSDM_CHCFG1R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG1R1_SPEC>`
pub type DFSDM_CHCFG1R1 = crate::Reg<dfsdm_chcfg1r1::DFSDM_CHCFG1R1_SPEC>;
///DFSDM channel configuration 1 register 1
pub mod dfsdm_chcfg1r1;
///DFSDM_CHCFG2R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG2R1_SPEC>`
pub type DFSDM_CHCFG2R1 = crate::Reg<dfsdm_chcfg2r1::DFSDM_CHCFG2R1_SPEC>;
///DFSDM channel configuration 2 register 1
pub mod dfsdm_chcfg2r1;
///DFSDM_CHCFG3R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG3R1_SPEC>`
pub type DFSDM_CHCFG3R1 = crate::Reg<dfsdm_chcfg3r1::DFSDM_CHCFG3R1_SPEC>;
///DFSDM channel configuration 3 register 1
pub mod dfsdm_chcfg3r1;
///DFSDM_CHCFG4R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG4R1_SPEC>`
pub type DFSDM_CHCFG4R1 = crate::Reg<dfsdm_chcfg4r1::DFSDM_CHCFG4R1_SPEC>;
///DFSDM channel configuration 4 register 1
pub mod dfsdm_chcfg4r1;
///DFSDM_CHCFG5R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG5R1_SPEC>`
pub type DFSDM_CHCFG5R1 = crate::Reg<dfsdm_chcfg5r1::DFSDM_CHCFG5R1_SPEC>;
///DFSDM channel configuration 5 register 1
pub mod dfsdm_chcfg5r1;
///DFSDM_CHCFG6R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG6R1_SPEC>`
pub type DFSDM_CHCFG6R1 = crate::Reg<dfsdm_chcfg6r1::DFSDM_CHCFG6R1_SPEC>;
///DFSDM channel configuration 6 register 1
pub mod dfsdm_chcfg6r1;
///DFSDM_CHCFG7R1 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG7R1_SPEC>`
pub type DFSDM_CHCFG7R1 = crate::Reg<dfsdm_chcfg7r1::DFSDM_CHCFG7R1_SPEC>;
///DFSDM channel configuration 7 register 1
pub mod dfsdm_chcfg7r1;
///DFSDM_CHCFG0R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG0R2_SPEC>`
pub type DFSDM_CHCFG0R2 = crate::Reg<dfsdm_chcfg0r2::DFSDM_CHCFG0R2_SPEC>;
///DFSDM channel configuration 0 register 2
pub mod dfsdm_chcfg0r2;
///DFSDM_CHCFG1R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG1R2_SPEC>`
pub type DFSDM_CHCFG1R2 = crate::Reg<dfsdm_chcfg1r2::DFSDM_CHCFG1R2_SPEC>;
///DFSDM channel configuration 1 register 2
pub mod dfsdm_chcfg1r2;
///DFSDM_CHCFG2R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG2R2_SPEC>`
pub type DFSDM_CHCFG2R2 = crate::Reg<dfsdm_chcfg2r2::DFSDM_CHCFG2R2_SPEC>;
///DFSDM channel configuration 2 register 2
pub mod dfsdm_chcfg2r2;
///DFSDM_CHCFG3R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG3R2_SPEC>`
pub type DFSDM_CHCFG3R2 = crate::Reg<dfsdm_chcfg3r2::DFSDM_CHCFG3R2_SPEC>;
///DFSDM channel configuration 3 register 2
pub mod dfsdm_chcfg3r2;
///DFSDM_CHCFG4R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG4R2_SPEC>`
pub type DFSDM_CHCFG4R2 = crate::Reg<dfsdm_chcfg4r2::DFSDM_CHCFG4R2_SPEC>;
///DFSDM channel configuration 4 register 2
pub mod dfsdm_chcfg4r2;
///DFSDM_CHCFG5R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG5R2_SPEC>`
pub type DFSDM_CHCFG5R2 = crate::Reg<dfsdm_chcfg5r2::DFSDM_CHCFG5R2_SPEC>;
///DFSDM channel configuration 5 register 2
pub mod dfsdm_chcfg5r2;
///DFSDM_CHCFG6R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG6R2_SPEC>`
pub type DFSDM_CHCFG6R2 = crate::Reg<dfsdm_chcfg6r2::DFSDM_CHCFG6R2_SPEC>;
///DFSDM channel configuration 6 register 2
pub mod dfsdm_chcfg6r2;
///DFSDM_CHCFG7R2 (rw) register accessor: an alias for `Reg<DFSDM_CHCFG7R2_SPEC>`
pub type DFSDM_CHCFG7R2 = crate::Reg<dfsdm_chcfg7r2::DFSDM_CHCFG7R2_SPEC>;
///DFSDM channel configuration 7 register 2
pub mod dfsdm_chcfg7r2;
///DFSDM_AWSCD0R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD0R_SPEC>`
pub type DFSDM_AWSCD0R = crate::Reg<dfsdm_awscd0r::DFSDM_AWSCD0R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd0r;
///DFSDM_AWSCD1R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD1R_SPEC>`
pub type DFSDM_AWSCD1R = crate::Reg<dfsdm_awscd1r::DFSDM_AWSCD1R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd1r;
///DFSDM_AWSCD2R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD2R_SPEC>`
pub type DFSDM_AWSCD2R = crate::Reg<dfsdm_awscd2r::DFSDM_AWSCD2R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd2r;
///DFSDM_AWSCD3R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD3R_SPEC>`
pub type DFSDM_AWSCD3R = crate::Reg<dfsdm_awscd3r::DFSDM_AWSCD3R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd3r;
///DFSDM_AWSCD4R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD4R_SPEC>`
pub type DFSDM_AWSCD4R = crate::Reg<dfsdm_awscd4r::DFSDM_AWSCD4R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd4r;
///DFSDM_AWSCD5R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD5R_SPEC>`
pub type DFSDM_AWSCD5R = crate::Reg<dfsdm_awscd5r::DFSDM_AWSCD5R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd5r;
///DFSDM_AWSCD6R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD6R_SPEC>`
pub type DFSDM_AWSCD6R = crate::Reg<dfsdm_awscd6r::DFSDM_AWSCD6R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd6r;
///DFSDM_AWSCD7R (rw) register accessor: an alias for `Reg<DFSDM_AWSCD7R_SPEC>`
pub type DFSDM_AWSCD7R = crate::Reg<dfsdm_awscd7r::DFSDM_AWSCD7R_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd7r;
///DFSDM_CHWDAT0R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT0R_SPEC>`
pub type DFSDM_CHWDAT0R = crate::Reg<dfsdm_chwdat0r::DFSDM_CHWDAT0R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat0r;
///DFSDM_CHWDAT1R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT1R_SPEC>`
pub type DFSDM_CHWDAT1R = crate::Reg<dfsdm_chwdat1r::DFSDM_CHWDAT1R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat1r;
///DFSDM_CHWDAT2R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT2R_SPEC>`
pub type DFSDM_CHWDAT2R = crate::Reg<dfsdm_chwdat2r::DFSDM_CHWDAT2R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat2r;
///DFSDM_CHWDAT3R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT3R_SPEC>`
pub type DFSDM_CHWDAT3R = crate::Reg<dfsdm_chwdat3r::DFSDM_CHWDAT3R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat3r;
///DFSDM_CHWDAT4R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT4R_SPEC>`
pub type DFSDM_CHWDAT4R = crate::Reg<dfsdm_chwdat4r::DFSDM_CHWDAT4R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat4r;
///DFSDM_CHWDAT5R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT5R_SPEC>`
pub type DFSDM_CHWDAT5R = crate::Reg<dfsdm_chwdat5r::DFSDM_CHWDAT5R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat5r;
///DFSDM_CHWDAT6R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT6R_SPEC>`
pub type DFSDM_CHWDAT6R = crate::Reg<dfsdm_chwdat6r::DFSDM_CHWDAT6R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat6r;
///DFSDM_CHWDAT7R (r) register accessor: an alias for `Reg<DFSDM_CHWDAT7R_SPEC>`
pub type DFSDM_CHWDAT7R = crate::Reg<dfsdm_chwdat7r::DFSDM_CHWDAT7R_SPEC>;
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat7r;
///DFSDM_CHDATIN0R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN0R_SPEC>`
pub type DFSDM_CHDATIN0R = crate::Reg<dfsdm_chdatin0r::DFSDM_CHDATIN0R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin0r;
///DFSDM_CHDATIN1R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN1R_SPEC>`
pub type DFSDM_CHDATIN1R = crate::Reg<dfsdm_chdatin1r::DFSDM_CHDATIN1R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin1r;
///DFSDM_CHDATIN2R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN2R_SPEC>`
pub type DFSDM_CHDATIN2R = crate::Reg<dfsdm_chdatin2r::DFSDM_CHDATIN2R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin2r;
///DFSDM_CHDATIN3R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN3R_SPEC>`
pub type DFSDM_CHDATIN3R = crate::Reg<dfsdm_chdatin3r::DFSDM_CHDATIN3R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin3r;
///DFSDM_CHDATIN4R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN4R_SPEC>`
pub type DFSDM_CHDATIN4R = crate::Reg<dfsdm_chdatin4r::DFSDM_CHDATIN4R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin4r;
///DFSDM_CHDATIN5R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN5R_SPEC>`
pub type DFSDM_CHDATIN5R = crate::Reg<dfsdm_chdatin5r::DFSDM_CHDATIN5R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin5r;
///DFSDM_CHDATIN6R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN6R_SPEC>`
pub type DFSDM_CHDATIN6R = crate::Reg<dfsdm_chdatin6r::DFSDM_CHDATIN6R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin6r;
///DFSDM_CHDATIN7R (rw) register accessor: an alias for `Reg<DFSDM_CHDATIN7R_SPEC>`
pub type DFSDM_CHDATIN7R = crate::Reg<dfsdm_chdatin7r::DFSDM_CHDATIN7R_SPEC>;
///DFSDM channel data input register
pub mod dfsdm_chdatin7r;
///DFSDM0_CR1 (rw) register accessor: an alias for `Reg<DFSDM0_CR1_SPEC>`
pub type DFSDM0_CR1 = crate::Reg<dfsdm0_cr1::DFSDM0_CR1_SPEC>;
///DFSDM control register 1
pub mod dfsdm0_cr1;
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///DFSDM control register 1
pub mod cr1;
///DFSDM2_CR1 (rw) register accessor: an alias for `Reg<DFSDM2_CR1_SPEC>`
pub type DFSDM2_CR1 = crate::Reg<dfsdm2_cr1::DFSDM2_CR1_SPEC>;
///DFSDM control register 1
pub mod dfsdm2_cr1;
///DFSDM3_CR1 (rw) register accessor: an alias for `Reg<DFSDM3_CR1_SPEC>`
pub type DFSDM3_CR1 = crate::Reg<dfsdm3_cr1::DFSDM3_CR1_SPEC>;
///DFSDM control register 1
pub mod dfsdm3_cr1;
///DFSDM0_CR2 (rw) register accessor: an alias for `Reg<DFSDM0_CR2_SPEC>`
pub type DFSDM0_CR2 = crate::Reg<dfsdm0_cr2::DFSDM0_CR2_SPEC>;
///DFSDM control register 2
pub mod dfsdm0_cr2;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///DFSDM control register 2
pub mod cr2;
///DFSDM2_CR2 (rw) register accessor: an alias for `Reg<DFSDM2_CR2_SPEC>`
pub type DFSDM2_CR2 = crate::Reg<dfsdm2_cr2::DFSDM2_CR2_SPEC>;
///DFSDM control register 2
pub mod dfsdm2_cr2;
///DFSDM3_CR2 (rw) register accessor: an alias for `Reg<DFSDM3_CR2_SPEC>`
pub type DFSDM3_CR2 = crate::Reg<dfsdm3_cr2::DFSDM3_CR2_SPEC>;
///DFSDM control register 2
pub mod dfsdm3_cr2;
///DFSDM0_ISR (r) register accessor: an alias for `Reg<DFSDM0_ISR_SPEC>`
pub type DFSDM0_ISR = crate::Reg<dfsdm0_isr::DFSDM0_ISR_SPEC>;
///DFSDM interrupt and status register
pub mod dfsdm0_isr;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///DFSDM interrupt and status register
pub mod isr;
///DFSDM2_ISR (r) register accessor: an alias for `Reg<DFSDM2_ISR_SPEC>`
pub type DFSDM2_ISR = crate::Reg<dfsdm2_isr::DFSDM2_ISR_SPEC>;
///DFSDM interrupt and status register
pub mod dfsdm2_isr;
///DFSDM3_ISR (r) register accessor: an alias for `Reg<DFSDM3_ISR_SPEC>`
pub type DFSDM3_ISR = crate::Reg<dfsdm3_isr::DFSDM3_ISR_SPEC>;
///DFSDM interrupt and status register
pub mod dfsdm3_isr;
///DFSDM0_ICR (rw) register accessor: an alias for `Reg<DFSDM0_ICR_SPEC>`
pub type DFSDM0_ICR = crate::Reg<dfsdm0_icr::DFSDM0_ICR_SPEC>;
///DFSDM interrupt flag clear register
pub mod dfsdm0_icr;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///DFSDM interrupt flag clear register
pub mod icr;
///DFSDM2_ICR (rw) register accessor: an alias for `Reg<DFSDM2_ICR_SPEC>`
pub type DFSDM2_ICR = crate::Reg<dfsdm2_icr::DFSDM2_ICR_SPEC>;
///DFSDM interrupt flag clear register
pub mod dfsdm2_icr;
///DFSDM3_ICR (rw) register accessor: an alias for `Reg<DFSDM3_ICR_SPEC>`
pub type DFSDM3_ICR = crate::Reg<dfsdm3_icr::DFSDM3_ICR_SPEC>;
///DFSDM interrupt flag clear register
pub mod dfsdm3_icr;
///DFSDM0_JCHGR (rw) register accessor: an alias for `Reg<DFSDM0_JCHGR_SPEC>`
pub type DFSDM0_JCHGR = crate::Reg<dfsdm0_jchgr::DFSDM0_JCHGR_SPEC>;
///DFSDM injected channel group selection register
pub mod dfsdm0_jchgr;
///JCHGR (rw) register accessor: an alias for `Reg<JCHGR_SPEC>`
pub type JCHGR = crate::Reg<jchgr::JCHGR_SPEC>;
///DFSDM injected channel group selection register
pub mod jchgr;
///DFSDM2_JCHGR (rw) register accessor: an alias for `Reg<DFSDM2_JCHGR_SPEC>`
pub type DFSDM2_JCHGR = crate::Reg<dfsdm2_jchgr::DFSDM2_JCHGR_SPEC>;
///DFSDM injected channel group selection register
pub mod dfsdm2_jchgr;
///DFSDM3_JCHGR (rw) register accessor: an alias for `Reg<DFSDM3_JCHGR_SPEC>`
pub type DFSDM3_JCHGR = crate::Reg<dfsdm3_jchgr::DFSDM3_JCHGR_SPEC>;
///DFSDM injected channel group selection register
pub mod dfsdm3_jchgr;
///DFSDM0_FCR (rw) register accessor: an alias for `Reg<DFSDM0_FCR_SPEC>`
pub type DFSDM0_FCR = crate::Reg<dfsdm0_fcr::DFSDM0_FCR_SPEC>;
///DFSDM filter control register
pub mod dfsdm0_fcr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///DFSDM filter control register
pub mod fcr;
///DFSDM2_FCR (rw) register accessor: an alias for `Reg<DFSDM2_FCR_SPEC>`
pub type DFSDM2_FCR = crate::Reg<dfsdm2_fcr::DFSDM2_FCR_SPEC>;
///DFSDM filter control register
pub mod dfsdm2_fcr;
///DFSDM3_FCR (rw) register accessor: an alias for `Reg<DFSDM3_FCR_SPEC>`
pub type DFSDM3_FCR = crate::Reg<dfsdm3_fcr::DFSDM3_FCR_SPEC>;
///DFSDM filter control register
pub mod dfsdm3_fcr;
///DFSDM0_JDATAR (r) register accessor: an alias for `Reg<DFSDM0_JDATAR_SPEC>`
pub type DFSDM0_JDATAR = crate::Reg<dfsdm0_jdatar::DFSDM0_JDATAR_SPEC>;
///DFSDM data register for injected group
pub mod dfsdm0_jdatar;
///JDATAR (r) register accessor: an alias for `Reg<JDATAR_SPEC>`
pub type JDATAR = crate::Reg<jdatar::JDATAR_SPEC>;
///DFSDM data register for injected group
pub mod jdatar;
///DFSDM2_JDATAR (r) register accessor: an alias for `Reg<DFSDM2_JDATAR_SPEC>`
pub type DFSDM2_JDATAR = crate::Reg<dfsdm2_jdatar::DFSDM2_JDATAR_SPEC>;
///DFSDM data register for injected group
pub mod dfsdm2_jdatar;
///DFSDM3_JDATAR (r) register accessor: an alias for `Reg<DFSDM3_JDATAR_SPEC>`
pub type DFSDM3_JDATAR = crate::Reg<dfsdm3_jdatar::DFSDM3_JDATAR_SPEC>;
///DFSDM data register for injected group
pub mod dfsdm3_jdatar;
///DFSDM0_RDATAR (r) register accessor: an alias for `Reg<DFSDM0_RDATAR_SPEC>`
pub type DFSDM0_RDATAR = crate::Reg<dfsdm0_rdatar::DFSDM0_RDATAR_SPEC>;
///DFSDM data register for the regular channel
pub mod dfsdm0_rdatar;
///RDATAR (r) register accessor: an alias for `Reg<RDATAR_SPEC>`
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
///DFSDM data register for the regular channel
pub mod rdatar;
///DFSDM2_RDATAR (r) register accessor: an alias for `Reg<DFSDM2_RDATAR_SPEC>`
pub type DFSDM2_RDATAR = crate::Reg<dfsdm2_rdatar::DFSDM2_RDATAR_SPEC>;
///DFSDM data register for the regular channel
pub mod dfsdm2_rdatar;
///DFSDM3_RDATAR (r) register accessor: an alias for `Reg<DFSDM3_RDATAR_SPEC>`
pub type DFSDM3_RDATAR = crate::Reg<dfsdm3_rdatar::DFSDM3_RDATAR_SPEC>;
///DFSDM data register for the regular channel
pub mod dfsdm3_rdatar;
///DFSDM0_AWHTR (rw) register accessor: an alias for `Reg<DFSDM0_AWHTR_SPEC>`
pub type DFSDM0_AWHTR = crate::Reg<dfsdm0_awhtr::DFSDM0_AWHTR_SPEC>;
///DFSDM analog watchdog high threshold register
pub mod dfsdm0_awhtr;
///AWHTR (rw) register accessor: an alias for `Reg<AWHTR_SPEC>`
pub type AWHTR = crate::Reg<awhtr::AWHTR_SPEC>;
///DFSDM analog watchdog high threshold register
pub mod awhtr;
///DFSDM2_AWHTR (rw) register accessor: an alias for `Reg<DFSDM2_AWHTR_SPEC>`
pub type DFSDM2_AWHTR = crate::Reg<dfsdm2_awhtr::DFSDM2_AWHTR_SPEC>;
///DFSDM analog watchdog high threshold register
pub mod dfsdm2_awhtr;
///DFSDM3_AWHTR (rw) register accessor: an alias for `Reg<DFSDM3_AWHTR_SPEC>`
pub type DFSDM3_AWHTR = crate::Reg<dfsdm3_awhtr::DFSDM3_AWHTR_SPEC>;
///DFSDM analog watchdog high threshold register
pub mod dfsdm3_awhtr;
///DFSDM0_AWLTR (rw) register accessor: an alias for `Reg<DFSDM0_AWLTR_SPEC>`
pub type DFSDM0_AWLTR = crate::Reg<dfsdm0_awltr::DFSDM0_AWLTR_SPEC>;
///DFSDM analog watchdog low threshold register
pub mod dfsdm0_awltr;
///AWLTR (rw) register accessor: an alias for `Reg<AWLTR_SPEC>`
pub type AWLTR = crate::Reg<awltr::AWLTR_SPEC>;
///DFSDM analog watchdog low threshold register
pub mod awltr;
///DFSDM2_AWLTR (rw) register accessor: an alias for `Reg<DFSDM2_AWLTR_SPEC>`
pub type DFSDM2_AWLTR = crate::Reg<dfsdm2_awltr::DFSDM2_AWLTR_SPEC>;
///DFSDM analog watchdog low threshold register
pub mod dfsdm2_awltr;
///DFSDM3_AWLTR (rw) register accessor: an alias for `Reg<DFSDM3_AWLTR_SPEC>`
pub type DFSDM3_AWLTR = crate::Reg<dfsdm3_awltr::DFSDM3_AWLTR_SPEC>;
///DFSDM analog watchdog low threshold register
pub mod dfsdm3_awltr;
///DFSDM0_AWSR (r) register accessor: an alias for `Reg<DFSDM0_AWSR_SPEC>`
pub type DFSDM0_AWSR = crate::Reg<dfsdm0_awsr::DFSDM0_AWSR_SPEC>;
///DFSDM analog watchdog status register
pub mod dfsdm0_awsr;
///AWSR (r) register accessor: an alias for `Reg<AWSR_SPEC>`
pub type AWSR = crate::Reg<awsr::AWSR_SPEC>;
///DFSDM analog watchdog status register
pub mod awsr;
///DFSDM2_AWSR (r) register accessor: an alias for `Reg<DFSDM2_AWSR_SPEC>`
pub type DFSDM2_AWSR = crate::Reg<dfsdm2_awsr::DFSDM2_AWSR_SPEC>;
///DFSDM analog watchdog status register
pub mod dfsdm2_awsr;
///DFSDM3_AWSR (r) register accessor: an alias for `Reg<DFSDM3_AWSR_SPEC>`
pub type DFSDM3_AWSR = crate::Reg<dfsdm3_awsr::DFSDM3_AWSR_SPEC>;
///DFSDM analog watchdog status register
pub mod dfsdm3_awsr;
///DFSDM0_AWCFR (rw) register accessor: an alias for `Reg<DFSDM0_AWCFR_SPEC>`
pub type DFSDM0_AWCFR = crate::Reg<dfsdm0_awcfr::DFSDM0_AWCFR_SPEC>;
///DFSDM analog watchdog clear flag register
pub mod dfsdm0_awcfr;
///AWCFR (rw) register accessor: an alias for `Reg<AWCFR_SPEC>`
pub type AWCFR = crate::Reg<awcfr::AWCFR_SPEC>;
///DFSDM analog watchdog clear flag register
pub mod awcfr;
///DFSDM2_AWCFR (rw) register accessor: an alias for `Reg<DFSDM2_AWCFR_SPEC>`
pub type DFSDM2_AWCFR = crate::Reg<dfsdm2_awcfr::DFSDM2_AWCFR_SPEC>;
///DFSDM analog watchdog clear flag register
pub mod dfsdm2_awcfr;
///DFSDM3_AWCFR (rw) register accessor: an alias for `Reg<DFSDM3_AWCFR_SPEC>`
pub type DFSDM3_AWCFR = crate::Reg<dfsdm3_awcfr::DFSDM3_AWCFR_SPEC>;
///DFSDM analog watchdog clear flag register
pub mod dfsdm3_awcfr;
///DFSDM0_EXMAX (r) register accessor: an alias for `Reg<DFSDM0_EXMAX_SPEC>`
pub type DFSDM0_EXMAX = crate::Reg<dfsdm0_exmax::DFSDM0_EXMAX_SPEC>;
///DFSDM Extremes detector maximum register
pub mod dfsdm0_exmax;
///EXMAX (r) register accessor: an alias for `Reg<EXMAX_SPEC>`
pub type EXMAX = crate::Reg<exmax::EXMAX_SPEC>;
///DFSDM Extremes detector maximum register
pub mod exmax;
///DFSDM2_EXMAX (r) register accessor: an alias for `Reg<DFSDM2_EXMAX_SPEC>`
pub type DFSDM2_EXMAX = crate::Reg<dfsdm2_exmax::DFSDM2_EXMAX_SPEC>;
///DFSDM Extremes detector maximum register
pub mod dfsdm2_exmax;
///DFSDM3_EXMAX (r) register accessor: an alias for `Reg<DFSDM3_EXMAX_SPEC>`
pub type DFSDM3_EXMAX = crate::Reg<dfsdm3_exmax::DFSDM3_EXMAX_SPEC>;
///DFSDM Extremes detector maximum register
pub mod dfsdm3_exmax;
///DFSDM0_EXMIN (r) register accessor: an alias for `Reg<DFSDM0_EXMIN_SPEC>`
pub type DFSDM0_EXMIN = crate::Reg<dfsdm0_exmin::DFSDM0_EXMIN_SPEC>;
///DFSDM Extremes detector minimum register
pub mod dfsdm0_exmin;
///EXMIN (r) register accessor: an alias for `Reg<EXMIN_SPEC>`
pub type EXMIN = crate::Reg<exmin::EXMIN_SPEC>;
///DFSDM Extremes detector minimum register
pub mod exmin;
///DFSDM2_EXMIN (r) register accessor: an alias for `Reg<DFSDM2_EXMIN_SPEC>`
pub type DFSDM2_EXMIN = crate::Reg<dfsdm2_exmin::DFSDM2_EXMIN_SPEC>;
///DFSDM Extremes detector minimum register
pub mod dfsdm2_exmin;
///DFSDM3_EXMIN (r) register accessor: an alias for `Reg<DFSDM3_EXMIN_SPEC>`
pub type DFSDM3_EXMIN = crate::Reg<dfsdm3_exmin::DFSDM3_EXMIN_SPEC>;
///DFSDM Extremes detector minimum register
pub mod dfsdm3_exmin;
///DFSDM0_CNVTIMR (r) register accessor: an alias for `Reg<DFSDM0_CNVTIMR_SPEC>`
pub type DFSDM0_CNVTIMR = crate::Reg<dfsdm0_cnvtimr::DFSDM0_CNVTIMR_SPEC>;
///DFSDM conversion timer register
pub mod dfsdm0_cnvtimr;
///CNVTIMR (r) register accessor: an alias for `Reg<CNVTIMR_SPEC>`
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMR_SPEC>;
///DFSDM conversion timer register
pub mod cnvtimr;
///DFSDM2_CNVTIMR (r) register accessor: an alias for `Reg<DFSDM2_CNVTIMR_SPEC>`
pub type DFSDM2_CNVTIMR = crate::Reg<dfsdm2_cnvtimr::DFSDM2_CNVTIMR_SPEC>;
///DFSDM conversion timer register
pub mod dfsdm2_cnvtimr;
///DFSDM3_CNVTIMR (r) register accessor: an alias for `Reg<DFSDM3_CNVTIMR_SPEC>`
pub type DFSDM3_CNVTIMR = crate::Reg<dfsdm3_cnvtimr::DFSDM3_CNVTIMR_SPEC>;
///DFSDM conversion timer register
pub mod dfsdm3_cnvtimr;
