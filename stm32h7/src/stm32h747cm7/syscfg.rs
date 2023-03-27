///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ///0x04 - peripheral mode configuration register
    pub pmcr: PMCR,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
    ///0x18 - configuration register
    pub cfgr: CFGR,
    _reserved6: [u8; 0x04],
    ///0x20 - compensation cell control/status register
    pub cccsr: CCCSR,
    ///0x24 - SYSCFG compensation cell value register
    pub ccvr: CCVR,
    ///0x28 - SYSCFG compensation cell code register
    pub cccr: CCCR,
    ///0x2c - SYSCFG power control register
    pub pwrcr: PWRCR,
    _reserved10: [u8; 0xf4],
    ///0x124 - SYSCFG package register
    pub pkgr: PKGR,
    _reserved11: [u8; 0x01d8],
    ///0x300 - SYSCFG user register 0
    pub ur0: UR0,
    ///0x304 - SYSCFG user register 1
    pub ur1: UR1,
    ///0x308 - SYSCFG user register 2
    pub ur2: UR2,
    ///0x30c - SYSCFG user register 3
    pub ur3: UR3,
    ///0x310 - SYSCFG user register 4
    pub ur4: UR4,
    ///0x314 - SYSCFG user register 5
    pub ur5: UR5,
    ///0x318 - SYSCFG user register 6
    pub ur6: UR6,
    ///0x31c - SYSCFG user register 7
    pub ur7: UR7,
    ///0x320 - SYSCFG user register 8
    pub ur8: UR8,
    ///0x324 - SYSCFG user register 9
    pub ur9: UR9,
    ///0x328 - SYSCFG user register 10
    pub ur10: UR10,
    ///0x32c - SYSCFG user register 11
    pub ur11: UR11,
    ///0x330 - SYSCFG user register 12
    pub ur12: UR12,
    ///0x334 - SYSCFG user register 13
    pub ur13: UR13,
    ///0x338 - SYSCFG user register 14
    pub ur14: UR14,
    ///0x33c - SYSCFG user register 15
    pub ur15: UR15,
    ///0x340 - SYSCFG user register 16
    pub ur16: UR16,
    ///0x344 - SYSCFG user register 17
    pub ur17: UR17,
}
///PMCR (rw) register accessor: an alias for `Reg<PMCR_SPEC>`
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
///peripheral mode configuration register
pub mod pmcr;
///EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///external interrupt configuration register 1
pub mod exticr1;
///EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///external interrupt configuration register 2
pub mod exticr2;
///EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///external interrupt configuration register 3
pub mod exticr3;
///EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///external interrupt configuration register 4
pub mod exticr4;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///configuration register
pub mod cfgr;
///CCCSR (rw) register accessor: an alias for `Reg<CCCSR_SPEC>`
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
///compensation cell control/status register
pub mod cccsr;
///CCVR (r) register accessor: an alias for `Reg<CCVR_SPEC>`
pub type CCVR = crate::Reg<ccvr::CCVR_SPEC>;
///SYSCFG compensation cell value register
pub mod ccvr;
///CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///SYSCFG compensation cell code register
pub mod cccr;
///PWRCR (rw) register accessor: an alias for `Reg<PWRCR_SPEC>`
pub type PWRCR = crate::Reg<pwrcr::PWRCR_SPEC>;
///SYSCFG power control register
pub mod pwrcr;
///PKGR (r) register accessor: an alias for `Reg<PKGR_SPEC>`
pub type PKGR = crate::Reg<pkgr::PKGR_SPEC>;
///SYSCFG package register
pub mod pkgr;
///UR0 (r) register accessor: an alias for `Reg<UR0_SPEC>`
pub type UR0 = crate::Reg<ur0::UR0_SPEC>;
///SYSCFG user register 0
pub mod ur0;
///UR1 (rw) register accessor: an alias for `Reg<UR1_SPEC>`
pub type UR1 = crate::Reg<ur1::UR1_SPEC>;
///SYSCFG user register 1
pub mod ur1;
///UR2 (rw) register accessor: an alias for `Reg<UR2_SPEC>`
pub type UR2 = crate::Reg<ur2::UR2_SPEC>;
///SYSCFG user register 2
pub mod ur2;
///UR3 (rw) register accessor: an alias for `Reg<UR3_SPEC>`
pub type UR3 = crate::Reg<ur3::UR3_SPEC>;
///SYSCFG user register 3
pub mod ur3;
///UR4 (rw) register accessor: an alias for `Reg<UR4_SPEC>`
pub type UR4 = crate::Reg<ur4::UR4_SPEC>;
///SYSCFG user register 4
pub mod ur4;
///UR5 (r) register accessor: an alias for `Reg<UR5_SPEC>`
pub type UR5 = crate::Reg<ur5::UR5_SPEC>;
///SYSCFG user register 5
pub mod ur5;
///UR6 (r) register accessor: an alias for `Reg<UR6_SPEC>`
pub type UR6 = crate::Reg<ur6::UR6_SPEC>;
///SYSCFG user register 6
pub mod ur6;
///UR7 (r) register accessor: an alias for `Reg<UR7_SPEC>`
pub type UR7 = crate::Reg<ur7::UR7_SPEC>;
///SYSCFG user register 7
pub mod ur7;
///UR8 (r) register accessor: an alias for `Reg<UR8_SPEC>`
pub type UR8 = crate::Reg<ur8::UR8_SPEC>;
///SYSCFG user register 8
pub mod ur8;
///UR9 (r) register accessor: an alias for `Reg<UR9_SPEC>`
pub type UR9 = crate::Reg<ur9::UR9_SPEC>;
///SYSCFG user register 9
pub mod ur9;
///UR10 (r) register accessor: an alias for `Reg<UR10_SPEC>`
pub type UR10 = crate::Reg<ur10::UR10_SPEC>;
///SYSCFG user register 10
pub mod ur10;
///UR11 (r) register accessor: an alias for `Reg<UR11_SPEC>`
pub type UR11 = crate::Reg<ur11::UR11_SPEC>;
///SYSCFG user register 11
pub mod ur11;
///UR12 (r) register accessor: an alias for `Reg<UR12_SPEC>`
pub type UR12 = crate::Reg<ur12::UR12_SPEC>;
///SYSCFG user register 12
pub mod ur12;
///UR13 (r) register accessor: an alias for `Reg<UR13_SPEC>`
pub type UR13 = crate::Reg<ur13::UR13_SPEC>;
///SYSCFG user register 13
pub mod ur13;
///UR14 (rw) register accessor: an alias for `Reg<UR14_SPEC>`
pub type UR14 = crate::Reg<ur14::UR14_SPEC>;
///SYSCFG user register 14
pub mod ur14;
///UR15 (rw) register accessor: an alias for `Reg<UR15_SPEC>`
pub type UR15 = crate::Reg<ur15::UR15_SPEC>;
///SYSCFG user register 15
pub mod ur15;
///UR16 (r) register accessor: an alias for `Reg<UR16_SPEC>`
pub type UR16 = crate::Reg<ur16::UR16_SPEC>;
///SYSCFG user register 16
pub mod ur16;
///UR17 (r) register accessor: an alias for `Reg<UR17_SPEC>`
pub type UR17 = crate::Reg<ur17::UR17_SPEC>;
///SYSCFG user register 17
pub mod ur17;
