///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - channel configuration y register
    pub ch0cfgr1: CH0CFGR1,
    ///0x04 - channel configuration y register
    pub ch0cfgr2: CH0CFGR2,
    ///0x08 - analog watchdog and short-circuit detector register
    pub ch0awscdr: CH0AWSCDR,
    ///0x0c - channel watchdog filter data register
    pub ch0wdatr: CH0WDATR,
    ///0x10 - channel data input register
    pub ch0datinr: CH0DATINR,
    ///0x14 - DFSDM channel y delay register
    pub ch0dlyr: CH0DLYR,
    _reserved6: [u8; 0x08],
    ///0x20 - CHCFG1R1
    pub ch1cfgr1: CH1CFGR1,
    ///0x24 - CHCFG1R2
    pub ch1cfgr2: CH1CFGR2,
    ///0x28 - AWSCD1R
    pub ch1awscdr: CH1AWSCDR,
    ///0x2c - CHWDAT1R
    pub ch1wdatr: CH1WDATR,
    ///0x30 - CHDATIN1R
    pub ch1datinr: CH1DATINR,
    ///0x34 - DFSDM channel y delay register
    pub ch1dlyr: CH1DLYR,
    _reserved12: [u8; 0x08],
    ///0x40 - CHCFG2R1
    pub ch2cfgr1: CH2CFGR1,
    ///0x44 - CHCFG2R2
    pub ch2cfgr2: CH2CFGR2,
    ///0x48 - AWSCD2R
    pub ch2awscdr: CH2AWSCDR,
    ///0x4c - CHWDAT2R
    pub ch2wdatr: CH2WDATR,
    ///0x50 - CHDATIN2R
    pub ch2datinr: CH2DATINR,
    ///0x54 - DFSDM channel y delay register
    pub ch2dlyr: CH2DLYR,
    _reserved18: [u8; 0x08],
    ///0x60 - CHCFG3R1
    pub ch3cfgr1: CH3CFGR1,
    ///0x64 - CHCFG3R2
    pub ch3cfgr2: CH3CFGR2,
    ///0x68 - AWSCD3R
    pub ch3awscdr: CH3AWSCDR,
    ///0x6c - CHWDAT3R
    pub ch3wdatr: CH3WDATR,
    ///0x70 - CHDATIN3R
    pub ch3datinr: CH3DATINR,
    ///0x74 - DFSDM channel y delay register
    pub ch3dlyr: CH3DLYR,
    _reserved24: [u8; 0x08],
    ///0x80 - CHCFG4R1
    pub ch4cfgr1: CH4CFGR1,
    ///0x84 - CHCFG4R2
    pub ch4cfgr2: CH4CFGR2,
    ///0x88 - AWSCD4R
    pub ch4awscdr: CH4AWSCDR,
    ///0x8c - CHWDAT4R
    pub ch4wdatr: CH4WDATR,
    ///0x90 - CHDATIN4R
    pub ch4datinr: CH4DATINR,
    ///0x94 - DFSDM channel y delay register
    pub ch4dlyr: CH4DLYR,
    _reserved30: [u8; 0x08],
    ///0xa0 - CHCFG5R1
    pub ch5cfgr1: CH5CFGR1,
    ///0xa4 - CHCFG5R2
    pub ch5cfgr2: CH5CFGR2,
    ///0xa8 - AWSCD5R
    pub ch5awscdr: CH5AWSCDR,
    ///0xac - CHWDAT5R
    pub ch5wdatr: CH5WDATR,
    ///0xb0 - CHDATIN5R
    pub ch5datinr: CH5DATINR,
    ///0xb4 - DFSDM channel y delay register
    pub ch5dlyr: CH5DLYR,
    _reserved36: [u8; 0x08],
    ///0xc0 - CHCFG6R1
    pub ch6cfgr1: CH6CFGR1,
    ///0xc4 - CH6CFGR2
    pub ch6cfgr2: CH6CFGR2,
    ///0xc8 - AWSCD6R
    pub ch6awscdr: CH6AWSCDR,
    ///0xcc - CHWDAT6R
    pub ch6wdatr: CH6WDATR,
    ///0xd0 - CHDATIN6R
    pub ch6datinr: CH6DATINR,
    ///0xd4 - DFSDM channel y delay register
    pub ch6dlyr: CH6DLYR,
    _reserved42: [u8; 0x08],
    ///0xe0 - CHCFG7R1
    pub ch7cfgr1: CH7CFGR1,
    ///0xe4 - CHCFG7R2
    pub ch7cfgr2: CH7CFGR2,
    ///0xe8 - AWSCD7R
    pub ch7awscdr: CH7AWSCDR,
    ///0xec - CHWDAT7R
    pub ch7wdatr: CH7WDATR,
    ///0xf0 - CHDATIN7R
    pub ch7datinr: CH7DATINR,
    ///0xf4 - DFSDM channel y delay register
    pub ch7dlyr: CH7DLYR,
    _reserved48: [u8; 0x08],
    ///0x100 - control register 1
    pub flt0cr1: FLT0CR1,
    ///0x104 - control register 2
    pub flt0cr2: FLT0CR2,
    ///0x108 - interrupt and status register
    pub flt0isr: FLT0ISR,
    ///0x10c - interrupt flag clear register
    pub flt0icr: FLT0ICR,
    ///0x110 - injected channel group selection register
    pub flt0jchgr: FLT0JCHGR,
    ///0x114 - filter control register
    pub flt0fcr: FLT0FCR,
    ///0x118 - data register for injected group
    pub flt0jdatar: FLT0JDATAR,
    ///0x11c - data register for the regular channel
    pub flt0rdatar: FLT0RDATAR,
    ///0x120 - analog watchdog high threshold register
    pub flt0awhtr: FLT0AWHTR,
    ///0x124 - analog watchdog low threshold register
    pub flt0awltr: FLT0AWLTR,
    ///0x128 - analog watchdog status register
    pub flt0awsr: FLT0AWSR,
    ///0x12c - analog watchdog clear flag register
    pub flt0awcfr: FLT0AWCFR,
    ///0x130 - Extremes detector maximum register
    pub flt0exmax: FLT0EXMAX,
    ///0x134 - Extremes detector minimum register
    pub flt0exmin: FLT0EXMIN,
    ///0x138 - conversion timer register
    pub flt0cnvtimr: FLT0CNVTIMR,
    _reserved63: [u8; 0x44],
    ///0x180 - control register 1
    pub flt1cr1: FLT1CR1,
    ///0x184 - control register 2
    pub flt1cr2: FLT1CR2,
    ///0x188 - interrupt and status register
    pub flt1isr: FLT1ISR,
    ///0x18c - interrupt flag clear register
    pub flt1icr: FLT1ICR,
    ///0x190 - injected channel group selection register
    pub flt1jchgr: FLT1JCHGR,
    ///0x194 - filter control register
    pub flt1fcr: FLT1FCR,
    ///0x198 - data register for injected group
    pub flt1jdatar: FLT1JDATAR,
    ///0x19c - data register for the regular channel
    pub flt1rdatar: FLT1RDATAR,
    _reserved71: [u8; 0x04],
    ///0x1a4 - analog watchdog low threshold register
    pub flt1awltr: FLT1AWLTR,
    ///0x1a8 - analog watchdog status register
    pub flt1awsr: FLT1AWSR,
    _reserved_73_flt: [u8; 0x04],
    ///0x1b0 - Extremes detector maximum register
    pub flt1exmax: FLT1EXMAX,
    ///0x1b4 - Extremes detector minimum register
    pub flt1exmin: FLT1EXMIN,
    ///0x1b8 - conversion timer register
    pub flt1cnvtimr: FLT1CNVTIMR,
    _reserved77: [u8; 0x44],
    ///0x200 - control register 1
    pub flt2cr1: FLT2CR1,
    ///0x204 - control register 2
    pub flt2cr2: FLT2CR2,
    ///0x208 - interrupt and status register
    pub flt2isr: FLT2ISR,
    ///0x20c - interrupt flag clear register
    pub flt2icr: FLT2ICR,
    ///0x210 - injected channel group selection register
    pub flt2jchgr: FLT2JCHGR,
    ///0x214 - filter control register
    pub flt2fcr: FLT2FCR,
    ///0x218 - data register for injected group
    pub flt2jdatar: FLT2JDATAR,
    ///0x21c - data register for the regular channel
    pub flt2rdatar: FLT2RDATAR,
    ///0x220 - analog watchdog high threshold register
    pub flt2awhtr: FLT2AWHTR,
    ///0x224 - analog watchdog low threshold register
    pub flt2awltr: FLT2AWLTR,
    ///0x228 - analog watchdog status register
    pub flt2awsr: FLT2AWSR,
    ///0x22c - analog watchdog clear flag register
    pub flt2awcfr: FLT2AWCFR,
    ///0x230 - Extremes detector maximum register
    pub flt2exmax: FLT2EXMAX,
    ///0x234 - Extremes detector minimum register
    pub flt2exmin: FLT2EXMIN,
    ///0x238 - conversion timer register
    pub flt2cnvtimr: FLT2CNVTIMR,
    _reserved92: [u8; 0x44],
    ///0x280 - control register 1
    pub flt3cr1: FLT3CR1,
    ///0x284 - control register 2
    pub flt3cr2: FLT3CR2,
    ///0x288 - interrupt and status register
    pub flt3isr: FLT3ISR,
    ///0x28c - interrupt flag clear register
    pub flt3icr: FLT3ICR,
    ///0x290 - injected channel group selection register
    pub flt3jchgr: FLT3JCHGR,
    ///0x294 - filter control register
    pub flt3fcr: FLT3FCR,
    ///0x298 - data register for injected group
    pub flt3jdatar: FLT3JDATAR,
    ///0x29c - data register for the regular channel
    pub flt3rdatar: FLT3RDATAR,
    ///0x2a0 - analog watchdog high threshold register
    pub flt3awhtr: FLT3AWHTR,
    ///0x2a4 - analog watchdog low threshold register
    pub flt3awltr: FLT3AWLTR,
    ///0x2a8 - analog watchdog status register
    pub flt3awsr: FLT3AWSR,
    ///0x2ac - analog watchdog clear flag register
    pub flt3awcfr: FLT3AWCFR,
    ///0x2b0 - Extremes detector maximum register
    pub flt3exmax: FLT3EXMAX,
    ///0x2b4 - Extremes detector minimum register
    pub flt3exmin: FLT3EXMIN,
    ///0x2b8 - conversion timer register
    pub flt3cnvtimr: FLT3CNVTIMR,
}
impl RegisterBlock {
    ///0x1ac - analog watchdog clear flag register
    #[inline(always)]
    pub const fn flt1awcfr(&self) -> &FLT1AWCFR {
        unsafe { &*(self as *const Self).cast::<u8>().add(428usize).cast() }
    }
    ///0x1ac - analog watchdog high threshold register
    #[inline(always)]
    pub const fn flt1awhtr(&self) -> &FLT1AWHTR {
        unsafe { &*(self as *const Self).cast::<u8>().add(428usize).cast() }
    }
}
///CH0CFGR1 (rw) register accessor: an alias for `Reg<CH0CFGR1_SPEC>`
pub type CH0CFGR1 = crate::Reg<ch0cfgr1::CH0CFGR1_SPEC>;
///channel configuration y register
pub mod ch0cfgr1;
///CH0CFGR2 (rw) register accessor: an alias for `Reg<CH0CFGR2_SPEC>`
pub type CH0CFGR2 = crate::Reg<ch0cfgr2::CH0CFGR2_SPEC>;
///channel configuration y register
pub mod ch0cfgr2;
///CH0AWSCDR (rw) register accessor: an alias for `Reg<CH0AWSCDR_SPEC>`
pub type CH0AWSCDR = crate::Reg<ch0awscdr::CH0AWSCDR_SPEC>;
///analog watchdog and short-circuit detector register
pub mod ch0awscdr;
///CH0WDATR (rw) register accessor: an alias for `Reg<CH0WDATR_SPEC>`
pub type CH0WDATR = crate::Reg<ch0wdatr::CH0WDATR_SPEC>;
///channel watchdog filter data register
pub mod ch0wdatr;
///CH0DATINR (rw) register accessor: an alias for `Reg<CH0DATINR_SPEC>`
pub type CH0DATINR = crate::Reg<ch0datinr::CH0DATINR_SPEC>;
///channel data input register
pub mod ch0datinr;
///CH1CFGR1 (rw) register accessor: an alias for `Reg<CH1CFGR1_SPEC>`
pub type CH1CFGR1 = crate::Reg<ch1cfgr1::CH1CFGR1_SPEC>;
///CHCFG1R1
pub mod ch1cfgr1;
///CH1CFGR2 (rw) register accessor: an alias for `Reg<CH1CFGR2_SPEC>`
pub type CH1CFGR2 = crate::Reg<ch1cfgr2::CH1CFGR2_SPEC>;
///CHCFG1R2
pub mod ch1cfgr2;
///CH1AWSCDR (rw) register accessor: an alias for `Reg<CH1AWSCDR_SPEC>`
pub type CH1AWSCDR = crate::Reg<ch1awscdr::CH1AWSCDR_SPEC>;
///AWSCD1R
pub mod ch1awscdr;
///CH1WDATR (rw) register accessor: an alias for `Reg<CH1WDATR_SPEC>`
pub type CH1WDATR = crate::Reg<ch1wdatr::CH1WDATR_SPEC>;
///CHWDAT1R
pub mod ch1wdatr;
///CH1DATINR (rw) register accessor: an alias for `Reg<CH1DATINR_SPEC>`
pub type CH1DATINR = crate::Reg<ch1datinr::CH1DATINR_SPEC>;
///CHDATIN1R
pub mod ch1datinr;
///CH2CFGR1 (rw) register accessor: an alias for `Reg<CH2CFGR1_SPEC>`
pub type CH2CFGR1 = crate::Reg<ch2cfgr1::CH2CFGR1_SPEC>;
///CHCFG2R1
pub mod ch2cfgr1;
///CH2CFGR2 (rw) register accessor: an alias for `Reg<CH2CFGR2_SPEC>`
pub type CH2CFGR2 = crate::Reg<ch2cfgr2::CH2CFGR2_SPEC>;
///CHCFG2R2
pub mod ch2cfgr2;
///CH2AWSCDR (rw) register accessor: an alias for `Reg<CH2AWSCDR_SPEC>`
pub type CH2AWSCDR = crate::Reg<ch2awscdr::CH2AWSCDR_SPEC>;
///AWSCD2R
pub mod ch2awscdr;
///CH2WDATR (rw) register accessor: an alias for `Reg<CH2WDATR_SPEC>`
pub type CH2WDATR = crate::Reg<ch2wdatr::CH2WDATR_SPEC>;
///CHWDAT2R
pub mod ch2wdatr;
///CH2DATINR (rw) register accessor: an alias for `Reg<CH2DATINR_SPEC>`
pub type CH2DATINR = crate::Reg<ch2datinr::CH2DATINR_SPEC>;
///CHDATIN2R
pub mod ch2datinr;
///CH3CFGR1 (rw) register accessor: an alias for `Reg<CH3CFGR1_SPEC>`
pub type CH3CFGR1 = crate::Reg<ch3cfgr1::CH3CFGR1_SPEC>;
///CHCFG3R1
pub mod ch3cfgr1;
///CH3CFGR2 (rw) register accessor: an alias for `Reg<CH3CFGR2_SPEC>`
pub type CH3CFGR2 = crate::Reg<ch3cfgr2::CH3CFGR2_SPEC>;
///CHCFG3R2
pub mod ch3cfgr2;
///CH3AWSCDR (rw) register accessor: an alias for `Reg<CH3AWSCDR_SPEC>`
pub type CH3AWSCDR = crate::Reg<ch3awscdr::CH3AWSCDR_SPEC>;
///AWSCD3R
pub mod ch3awscdr;
///CH3WDATR (rw) register accessor: an alias for `Reg<CH3WDATR_SPEC>`
pub type CH3WDATR = crate::Reg<ch3wdatr::CH3WDATR_SPEC>;
///CHWDAT3R
pub mod ch3wdatr;
///CH3DATINR (rw) register accessor: an alias for `Reg<CH3DATINR_SPEC>`
pub type CH3DATINR = crate::Reg<ch3datinr::CH3DATINR_SPEC>;
///CHDATIN3R
pub mod ch3datinr;
///CH4CFGR1 (rw) register accessor: an alias for `Reg<CH4CFGR1_SPEC>`
pub type CH4CFGR1 = crate::Reg<ch4cfgr1::CH4CFGR1_SPEC>;
///CHCFG4R1
pub mod ch4cfgr1;
///CH4CFGR2 (rw) register accessor: an alias for `Reg<CH4CFGR2_SPEC>`
pub type CH4CFGR2 = crate::Reg<ch4cfgr2::CH4CFGR2_SPEC>;
///CHCFG4R2
pub mod ch4cfgr2;
///CH4AWSCDR (rw) register accessor: an alias for `Reg<CH4AWSCDR_SPEC>`
pub type CH4AWSCDR = crate::Reg<ch4awscdr::CH4AWSCDR_SPEC>;
///AWSCD4R
pub mod ch4awscdr;
///CH4WDATR (rw) register accessor: an alias for `Reg<CH4WDATR_SPEC>`
pub type CH4WDATR = crate::Reg<ch4wdatr::CH4WDATR_SPEC>;
///CHWDAT4R
pub mod ch4wdatr;
///CH4DATINR (rw) register accessor: an alias for `Reg<CH4DATINR_SPEC>`
pub type CH4DATINR = crate::Reg<ch4datinr::CH4DATINR_SPEC>;
///CHDATIN4R
pub mod ch4datinr;
///CH5CFGR1 (rw) register accessor: an alias for `Reg<CH5CFGR1_SPEC>`
pub type CH5CFGR1 = crate::Reg<ch5cfgr1::CH5CFGR1_SPEC>;
///CHCFG5R1
pub mod ch5cfgr1;
///CH5CFGR2 (rw) register accessor: an alias for `Reg<CH5CFGR2_SPEC>`
pub type CH5CFGR2 = crate::Reg<ch5cfgr2::CH5CFGR2_SPEC>;
///CHCFG5R2
pub mod ch5cfgr2;
///CH5AWSCDR (rw) register accessor: an alias for `Reg<CH5AWSCDR_SPEC>`
pub type CH5AWSCDR = crate::Reg<ch5awscdr::CH5AWSCDR_SPEC>;
///AWSCD5R
pub mod ch5awscdr;
///CH5WDATR (rw) register accessor: an alias for `Reg<CH5WDATR_SPEC>`
pub type CH5WDATR = crate::Reg<ch5wdatr::CH5WDATR_SPEC>;
///CHWDAT5R
pub mod ch5wdatr;
///CH5DATINR (rw) register accessor: an alias for `Reg<CH5DATINR_SPEC>`
pub type CH5DATINR = crate::Reg<ch5datinr::CH5DATINR_SPEC>;
///CHDATIN5R
pub mod ch5datinr;
///CH6CFGR1 (rw) register accessor: an alias for `Reg<CH6CFGR1_SPEC>`
pub type CH6CFGR1 = crate::Reg<ch6cfgr1::CH6CFGR1_SPEC>;
///CHCFG6R1
pub mod ch6cfgr1;
///CH6CFGR2 (rw) register accessor: an alias for `Reg<CH6CFGR2_SPEC>`
pub type CH6CFGR2 = crate::Reg<ch6cfgr2::CH6CFGR2_SPEC>;
///CH6CFGR2
pub mod ch6cfgr2;
///CH6AWSCDR (rw) register accessor: an alias for `Reg<CH6AWSCDR_SPEC>`
pub type CH6AWSCDR = crate::Reg<ch6awscdr::CH6AWSCDR_SPEC>;
///AWSCD6R
pub mod ch6awscdr;
///CH6WDATR (rw) register accessor: an alias for `Reg<CH6WDATR_SPEC>`
pub type CH6WDATR = crate::Reg<ch6wdatr::CH6WDATR_SPEC>;
///CHWDAT6R
pub mod ch6wdatr;
///CH6DATINR (rw) register accessor: an alias for `Reg<CH6DATINR_SPEC>`
pub type CH6DATINR = crate::Reg<ch6datinr::CH6DATINR_SPEC>;
///CHDATIN6R
pub mod ch6datinr;
///CH7CFGR1 (rw) register accessor: an alias for `Reg<CH7CFGR1_SPEC>`
pub type CH7CFGR1 = crate::Reg<ch7cfgr1::CH7CFGR1_SPEC>;
///CHCFG7R1
pub mod ch7cfgr1;
///CH7CFGR2 (rw) register accessor: an alias for `Reg<CH7CFGR2_SPEC>`
pub type CH7CFGR2 = crate::Reg<ch7cfgr2::CH7CFGR2_SPEC>;
///CHCFG7R2
pub mod ch7cfgr2;
///CH7AWSCDR (rw) register accessor: an alias for `Reg<CH7AWSCDR_SPEC>`
pub type CH7AWSCDR = crate::Reg<ch7awscdr::CH7AWSCDR_SPEC>;
///AWSCD7R
pub mod ch7awscdr;
///CH7WDATR (rw) register accessor: an alias for `Reg<CH7WDATR_SPEC>`
pub type CH7WDATR = crate::Reg<ch7wdatr::CH7WDATR_SPEC>;
///CHWDAT7R
pub mod ch7wdatr;
///CH7DATINR (rw) register accessor: an alias for `Reg<CH7DATINR_SPEC>`
pub type CH7DATINR = crate::Reg<ch7datinr::CH7DATINR_SPEC>;
///CHDATIN7R
pub mod ch7datinr;
///FLT0CR1 (rw) register accessor: an alias for `Reg<FLT0CR1_SPEC>`
pub type FLT0CR1 = crate::Reg<flt0cr1::FLT0CR1_SPEC>;
///control register 1
pub mod flt0cr1;
///FLT0CR2 (rw) register accessor: an alias for `Reg<FLT0CR2_SPEC>`
pub type FLT0CR2 = crate::Reg<flt0cr2::FLT0CR2_SPEC>;
///control register 2
pub mod flt0cr2;
///FLT0ISR (r) register accessor: an alias for `Reg<FLT0ISR_SPEC>`
pub type FLT0ISR = crate::Reg<flt0isr::FLT0ISR_SPEC>;
///interrupt and status register
pub mod flt0isr;
///FLT0ICR (rw) register accessor: an alias for `Reg<FLT0ICR_SPEC>`
pub type FLT0ICR = crate::Reg<flt0icr::FLT0ICR_SPEC>;
///interrupt flag clear register
pub mod flt0icr;
///FLT0JCHGR (rw) register accessor: an alias for `Reg<FLT0JCHGR_SPEC>`
pub type FLT0JCHGR = crate::Reg<flt0jchgr::FLT0JCHGR_SPEC>;
///injected channel group selection register
pub mod flt0jchgr;
///FLT0FCR (rw) register accessor: an alias for `Reg<FLT0FCR_SPEC>`
pub type FLT0FCR = crate::Reg<flt0fcr::FLT0FCR_SPEC>;
///filter control register
pub mod flt0fcr;
///FLT0JDATAR (r) register accessor: an alias for `Reg<FLT0JDATAR_SPEC>`
pub type FLT0JDATAR = crate::Reg<flt0jdatar::FLT0JDATAR_SPEC>;
///data register for injected group
pub mod flt0jdatar;
///FLT0RDATAR (r) register accessor: an alias for `Reg<FLT0RDATAR_SPEC>`
pub type FLT0RDATAR = crate::Reg<flt0rdatar::FLT0RDATAR_SPEC>;
///data register for the regular channel
pub mod flt0rdatar;
///FLT0AWHTR (rw) register accessor: an alias for `Reg<FLT0AWHTR_SPEC>`
pub type FLT0AWHTR = crate::Reg<flt0awhtr::FLT0AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod flt0awhtr;
///FLT0AWLTR (rw) register accessor: an alias for `Reg<FLT0AWLTR_SPEC>`
pub type FLT0AWLTR = crate::Reg<flt0awltr::FLT0AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod flt0awltr;
///FLT0AWSR (r) register accessor: an alias for `Reg<FLT0AWSR_SPEC>`
pub type FLT0AWSR = crate::Reg<flt0awsr::FLT0AWSR_SPEC>;
///analog watchdog status register
pub mod flt0awsr;
///FLT0AWCFR (rw) register accessor: an alias for `Reg<FLT0AWCFR_SPEC>`
pub type FLT0AWCFR = crate::Reg<flt0awcfr::FLT0AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod flt0awcfr;
///FLT0EXMAX (r) register accessor: an alias for `Reg<FLT0EXMAX_SPEC>`
pub type FLT0EXMAX = crate::Reg<flt0exmax::FLT0EXMAX_SPEC>;
///Extremes detector maximum register
pub mod flt0exmax;
///FLT0EXMIN (r) register accessor: an alias for `Reg<FLT0EXMIN_SPEC>`
pub type FLT0EXMIN = crate::Reg<flt0exmin::FLT0EXMIN_SPEC>;
///Extremes detector minimum register
pub mod flt0exmin;
///FLT0CNVTIMR (r) register accessor: an alias for `Reg<FLT0CNVTIMR_SPEC>`
pub type FLT0CNVTIMR = crate::Reg<flt0cnvtimr::FLT0CNVTIMR_SPEC>;
///conversion timer register
pub mod flt0cnvtimr;
///FLT1CR1 (rw) register accessor: an alias for `Reg<FLT1CR1_SPEC>`
pub type FLT1CR1 = crate::Reg<flt1cr1::FLT1CR1_SPEC>;
///control register 1
pub mod flt1cr1;
///FLT1CR2 (rw) register accessor: an alias for `Reg<FLT1CR2_SPEC>`
pub type FLT1CR2 = crate::Reg<flt1cr2::FLT1CR2_SPEC>;
///control register 2
pub mod flt1cr2;
///FLT1ISR (r) register accessor: an alias for `Reg<FLT1ISR_SPEC>`
pub type FLT1ISR = crate::Reg<flt1isr::FLT1ISR_SPEC>;
///interrupt and status register
pub mod flt1isr;
///FLT1ICR (rw) register accessor: an alias for `Reg<FLT1ICR_SPEC>`
pub type FLT1ICR = crate::Reg<flt1icr::FLT1ICR_SPEC>;
///interrupt flag clear register
pub mod flt1icr;
///FLT1JCHGR (rw) register accessor: an alias for `Reg<FLT1JCHGR_SPEC>`
pub type FLT1JCHGR = crate::Reg<flt1jchgr::FLT1JCHGR_SPEC>;
///injected channel group selection register
pub mod flt1jchgr;
///FLT1FCR (rw) register accessor: an alias for `Reg<FLT1FCR_SPEC>`
pub type FLT1FCR = crate::Reg<flt1fcr::FLT1FCR_SPEC>;
///filter control register
pub mod flt1fcr;
///FLT1JDATAR (r) register accessor: an alias for `Reg<FLT1JDATAR_SPEC>`
pub type FLT1JDATAR = crate::Reg<flt1jdatar::FLT1JDATAR_SPEC>;
///data register for injected group
pub mod flt1jdatar;
///FLT1RDATAR (r) register accessor: an alias for `Reg<FLT1RDATAR_SPEC>`
pub type FLT1RDATAR = crate::Reg<flt1rdatar::FLT1RDATAR_SPEC>;
///data register for the regular channel
pub mod flt1rdatar;
///FLT1AWHTR (rw) register accessor: an alias for `Reg<FLT1AWHTR_SPEC>`
pub type FLT1AWHTR = crate::Reg<flt1awhtr::FLT1AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod flt1awhtr;
///FLT1AWLTR (rw) register accessor: an alias for `Reg<FLT1AWLTR_SPEC>`
pub type FLT1AWLTR = crate::Reg<flt1awltr::FLT1AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod flt1awltr;
///FLT1AWSR (r) register accessor: an alias for `Reg<FLT1AWSR_SPEC>`
pub type FLT1AWSR = crate::Reg<flt1awsr::FLT1AWSR_SPEC>;
///analog watchdog status register
pub mod flt1awsr;
///FLT1AWCFR (rw) register accessor: an alias for `Reg<FLT1AWCFR_SPEC>`
pub type FLT1AWCFR = crate::Reg<flt1awcfr::FLT1AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod flt1awcfr;
///FLT1EXMAX (r) register accessor: an alias for `Reg<FLT1EXMAX_SPEC>`
pub type FLT1EXMAX = crate::Reg<flt1exmax::FLT1EXMAX_SPEC>;
///Extremes detector maximum register
pub mod flt1exmax;
///FLT1EXMIN (r) register accessor: an alias for `Reg<FLT1EXMIN_SPEC>`
pub type FLT1EXMIN = crate::Reg<flt1exmin::FLT1EXMIN_SPEC>;
///Extremes detector minimum register
pub mod flt1exmin;
///FLT1CNVTIMR (r) register accessor: an alias for `Reg<FLT1CNVTIMR_SPEC>`
pub type FLT1CNVTIMR = crate::Reg<flt1cnvtimr::FLT1CNVTIMR_SPEC>;
///conversion timer register
pub mod flt1cnvtimr;
///FLT2CR1 (rw) register accessor: an alias for `Reg<FLT2CR1_SPEC>`
pub type FLT2CR1 = crate::Reg<flt2cr1::FLT2CR1_SPEC>;
///control register 1
pub mod flt2cr1;
///FLT2CR2 (rw) register accessor: an alias for `Reg<FLT2CR2_SPEC>`
pub type FLT2CR2 = crate::Reg<flt2cr2::FLT2CR2_SPEC>;
///control register 2
pub mod flt2cr2;
///FLT2ISR (r) register accessor: an alias for `Reg<FLT2ISR_SPEC>`
pub type FLT2ISR = crate::Reg<flt2isr::FLT2ISR_SPEC>;
///interrupt and status register
pub mod flt2isr;
///FLT2ICR (rw) register accessor: an alias for `Reg<FLT2ICR_SPEC>`
pub type FLT2ICR = crate::Reg<flt2icr::FLT2ICR_SPEC>;
///interrupt flag clear register
pub mod flt2icr;
///FLT2JCHGR (rw) register accessor: an alias for `Reg<FLT2JCHGR_SPEC>`
pub type FLT2JCHGR = crate::Reg<flt2jchgr::FLT2JCHGR_SPEC>;
///injected channel group selection register
pub mod flt2jchgr;
///FLT2FCR (rw) register accessor: an alias for `Reg<FLT2FCR_SPEC>`
pub type FLT2FCR = crate::Reg<flt2fcr::FLT2FCR_SPEC>;
///filter control register
pub mod flt2fcr;
///FLT2JDATAR (r) register accessor: an alias for `Reg<FLT2JDATAR_SPEC>`
pub type FLT2JDATAR = crate::Reg<flt2jdatar::FLT2JDATAR_SPEC>;
///data register for injected group
pub mod flt2jdatar;
///FLT2RDATAR (r) register accessor: an alias for `Reg<FLT2RDATAR_SPEC>`
pub type FLT2RDATAR = crate::Reg<flt2rdatar::FLT2RDATAR_SPEC>;
///data register for the regular channel
pub mod flt2rdatar;
///FLT2AWHTR (rw) register accessor: an alias for `Reg<FLT2AWHTR_SPEC>`
pub type FLT2AWHTR = crate::Reg<flt2awhtr::FLT2AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod flt2awhtr;
///FLT2AWLTR (rw) register accessor: an alias for `Reg<FLT2AWLTR_SPEC>`
pub type FLT2AWLTR = crate::Reg<flt2awltr::FLT2AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod flt2awltr;
///FLT2AWSR (r) register accessor: an alias for `Reg<FLT2AWSR_SPEC>`
pub type FLT2AWSR = crate::Reg<flt2awsr::FLT2AWSR_SPEC>;
///analog watchdog status register
pub mod flt2awsr;
///FLT2AWCFR (rw) register accessor: an alias for `Reg<FLT2AWCFR_SPEC>`
pub type FLT2AWCFR = crate::Reg<flt2awcfr::FLT2AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod flt2awcfr;
///FLT2EXMAX (r) register accessor: an alias for `Reg<FLT2EXMAX_SPEC>`
pub type FLT2EXMAX = crate::Reg<flt2exmax::FLT2EXMAX_SPEC>;
///Extremes detector maximum register
pub mod flt2exmax;
///FLT2EXMIN (r) register accessor: an alias for `Reg<FLT2EXMIN_SPEC>`
pub type FLT2EXMIN = crate::Reg<flt2exmin::FLT2EXMIN_SPEC>;
///Extremes detector minimum register
pub mod flt2exmin;
///FLT2CNVTIMR (r) register accessor: an alias for `Reg<FLT2CNVTIMR_SPEC>`
pub type FLT2CNVTIMR = crate::Reg<flt2cnvtimr::FLT2CNVTIMR_SPEC>;
///conversion timer register
pub mod flt2cnvtimr;
///FLT3CR1 (rw) register accessor: an alias for `Reg<FLT3CR1_SPEC>`
pub type FLT3CR1 = crate::Reg<flt3cr1::FLT3CR1_SPEC>;
///control register 1
pub mod flt3cr1;
///FLT3CR2 (rw) register accessor: an alias for `Reg<FLT3CR2_SPEC>`
pub type FLT3CR2 = crate::Reg<flt3cr2::FLT3CR2_SPEC>;
///control register 2
pub mod flt3cr2;
///FLT3ISR (r) register accessor: an alias for `Reg<FLT3ISR_SPEC>`
pub type FLT3ISR = crate::Reg<flt3isr::FLT3ISR_SPEC>;
///interrupt and status register
pub mod flt3isr;
///FLT3ICR (rw) register accessor: an alias for `Reg<FLT3ICR_SPEC>`
pub type FLT3ICR = crate::Reg<flt3icr::FLT3ICR_SPEC>;
///interrupt flag clear register
pub mod flt3icr;
///FLT3JCHGR (rw) register accessor: an alias for `Reg<FLT3JCHGR_SPEC>`
pub type FLT3JCHGR = crate::Reg<flt3jchgr::FLT3JCHGR_SPEC>;
///injected channel group selection register
pub mod flt3jchgr;
///FLT3FCR (rw) register accessor: an alias for `Reg<FLT3FCR_SPEC>`
pub type FLT3FCR = crate::Reg<flt3fcr::FLT3FCR_SPEC>;
///filter control register
pub mod flt3fcr;
///FLT3JDATAR (r) register accessor: an alias for `Reg<FLT3JDATAR_SPEC>`
pub type FLT3JDATAR = crate::Reg<flt3jdatar::FLT3JDATAR_SPEC>;
///data register for injected group
pub mod flt3jdatar;
///FLT3RDATAR (r) register accessor: an alias for `Reg<FLT3RDATAR_SPEC>`
pub type FLT3RDATAR = crate::Reg<flt3rdatar::FLT3RDATAR_SPEC>;
///data register for the regular channel
pub mod flt3rdatar;
///FLT3AWHTR (rw) register accessor: an alias for `Reg<FLT3AWHTR_SPEC>`
pub type FLT3AWHTR = crate::Reg<flt3awhtr::FLT3AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod flt3awhtr;
///FLT3AWLTR (rw) register accessor: an alias for `Reg<FLT3AWLTR_SPEC>`
pub type FLT3AWLTR = crate::Reg<flt3awltr::FLT3AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod flt3awltr;
///FLT3AWSR (r) register accessor: an alias for `Reg<FLT3AWSR_SPEC>`
pub type FLT3AWSR = crate::Reg<flt3awsr::FLT3AWSR_SPEC>;
///analog watchdog status register
pub mod flt3awsr;
///FLT3AWCFR (rw) register accessor: an alias for `Reg<FLT3AWCFR_SPEC>`
pub type FLT3AWCFR = crate::Reg<flt3awcfr::FLT3AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod flt3awcfr;
///FLT3EXMAX (r) register accessor: an alias for `Reg<FLT3EXMAX_SPEC>`
pub type FLT3EXMAX = crate::Reg<flt3exmax::FLT3EXMAX_SPEC>;
///Extremes detector maximum register
pub mod flt3exmax;
///FLT3EXMIN (r) register accessor: an alias for `Reg<FLT3EXMIN_SPEC>`
pub type FLT3EXMIN = crate::Reg<flt3exmin::FLT3EXMIN_SPEC>;
///Extremes detector minimum register
pub mod flt3exmin;
///FLT3CNVTIMR (r) register accessor: an alias for `Reg<FLT3CNVTIMR_SPEC>`
pub type FLT3CNVTIMR = crate::Reg<flt3cnvtimr::FLT3CNVTIMR_SPEC>;
///conversion timer register
pub mod flt3cnvtimr;
///CH0DLYR (rw) register accessor: an alias for `Reg<CH0DLYR_SPEC>`
pub type CH0DLYR = crate::Reg<ch0dlyr::CH0DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch0dlyr;
///CH1DLYR (rw) register accessor: an alias for `Reg<CH1DLYR_SPEC>`
pub type CH1DLYR = crate::Reg<ch1dlyr::CH1DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch1dlyr;
///CH2DLYR (rw) register accessor: an alias for `Reg<CH2DLYR_SPEC>`
pub type CH2DLYR = crate::Reg<ch2dlyr::CH2DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch2dlyr;
///CH3DLYR (rw) register accessor: an alias for `Reg<CH3DLYR_SPEC>`
pub type CH3DLYR = crate::Reg<ch3dlyr::CH3DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch3dlyr;
///CH4DLYR (rw) register accessor: an alias for `Reg<CH4DLYR_SPEC>`
pub type CH4DLYR = crate::Reg<ch4dlyr::CH4DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch4dlyr;
///CH5DLYR (rw) register accessor: an alias for `Reg<CH5DLYR_SPEC>`
pub type CH5DLYR = crate::Reg<ch5dlyr::CH5DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch5dlyr;
///CH6DLYR (rw) register accessor: an alias for `Reg<CH6DLYR_SPEC>`
pub type CH6DLYR = crate::Reg<ch6dlyr::CH6DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch6dlyr;
///CH7DLYR (rw) register accessor: an alias for `Reg<CH7DLYR_SPEC>`
pub type CH7DLYR = crate::Reg<ch7dlyr::CH7DLYR_SPEC>;
///DFSDM channel y delay register
pub mod ch7dlyr;
