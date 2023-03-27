///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DDRPHYC revision ID register
    pub ddrphyc_ridr: DDRPHYC_RIDR,
    ///0x04 - DDRPHYC PHY initialization register
    pub ddrphyc_pir: DDRPHYC_PIR,
    ///0x08 - DDRPHYC PHY global control register
    pub ddrphyc_pgcr: DDRPHYC_PGCR,
    ///0x0c - DDRPHYC PHY global status register
    pub ddrphyc_pgsr: DDRPHYC_PGSR,
    ///0x10 - DDRPHYC DDR global control register
    pub ddrphyc_dllgcr: DDRPHYC_DLLGCR,
    ///0x14 - DDRPHYC AC DLL control register
    pub ddrphyc_acdllcr: DDRPHYC_ACDLLCR,
    ///0x18 - DDRPHYC PT register 0
    pub ddrphyc_ptr0: DDRPHYC_PTR0,
    ///0x1c - DDRPHYC PT register 1
    pub ddrphyc_ptr1: DDRPHYC_PTR1,
    ///0x20 - DDRPHYC PT register 2
    pub ddrphyc_ptr2: DDRPHYC_PTR2,
    ///0x24 - DDRPHYC ACIOC register
    pub ddrphyc_aciocr: DDRPHYC_ACIOCR,
    ///0x28 - DDRPHYC DXCC register
    pub ddrphyc_dxccr: DDRPHYC_DXCCR,
    ///0x2c - DDRPHYC DSGC register
    pub ddrphyc_dsgcr: DDRPHYC_DSGCR,
    ///0x30 - DDRPHYC DC register
    pub ddrphyc_dcr: DDRPHYC_DCR,
    ///0x34 - DDRPHYC DTP register 0
    pub ddrphyc_dtpr0: DDRPHYC_DTPR0,
    ///0x38 - DDRPHYC DTP register 1
    pub ddrphyc_dtpr1: DDRPHYC_DTPR1,
    ///0x3c - DDRPHYC DTP register 2
    pub ddrphyc_dtpr2: DDRPHYC_DTPR2,
    ///0x40 - DDRPHYC MR0 register for DDR3
    pub ddrphyc_ddr3_mr0: DDRPHYC_DDR3_MR0,
    _reserved17: [u8; 0x02],
    ///0x44 - DDRPHYC MR1 register for DDR3
    pub ddrphyc_ddr3_mr1: DDRPHYC_DDR3_MR1,
    _reserved18: [u8; 0x02],
    ///0x48 - DDRPHYC MR2 register for DDR3
    pub ddrphyc_ddr3_mr2: DDRPHYC_DDR3_MR2,
    _reserved19: [u8; 0x02],
    ///0x4c - DDRPHYC MR3 register for DDR3
    pub ddrphyc_ddr3_mr3: DDRPHYC_DDR3_MR3,
    _reserved20: [u8; 0x03],
    ///0x50 - DDRPHYC ODTC register
    pub ddrphyc_odtcr: DDRPHYC_ODTCR,
    ///0x54 - DDRPHYC DTA register
    pub ddrphyc_dtar: DDRPHYC_DTAR,
    ///0x58 - DDRPHYC DTD register 0
    pub ddrphyc_dtdr0: DDRPHYC_DTDR0,
    ///0x5c - DDRPHYC DTD register 1
    pub ddrphyc_dtdr1: DDRPHYC_DTDR1,
    _reserved24: [u8; 0x0118],
    ///0x178 - DDRPHYC general purpose register 0
    pub ddrphyc_gpr0: DDRPHYC_GPR0,
    ///0x17c - DDRPHYC general purpose register 1
    pub ddrphyc_gpr1: DDRPHYC_GPR1,
    ///0x180 - DDRPHYC ZQ0C register 0
    pub ddrphyc_zq0cr0: DDRPHYC_ZQ0CR0,
    ///0x184 - DDRPHYC ZQ0CR1 register
    pub ddrphyc_zq0cr1: DDRPHYC_ZQ0CR1,
    _reserved28: [u8; 0x03],
    ///0x188 - DDRPHYC ZQ0S register 0
    pub ddrphyc_zq0sr0: DDRPHYC_ZQ0SR0,
    ///0x18c - DDRPHYC ZQ0S register 1
    pub ddrphyc_zq0sr1: DDRPHYC_ZQ0SR1,
    _reserved30: [u8; 0x33],
    ///0x1c0 - DDRPHYC byte lane 0 GC register
    pub ddrphyc_dx0gcr: DDRPHYC_DX0GCR,
    ///0x1c4 - DDRPHYC byte lane 0 GS register 0
    pub ddrphyc_dx0gsr0: DDRPHYC_DX0GSR0,
    _reserved32: [u8; 0x02],
    ///0x1c8 - DDRPHYC byte lane 0 GS register 1
    pub ddrphyc_dx0gsr1: DDRPHYC_DX0GSR1,
    ///0x1cc - DDRPHYC byte lane 0 DLLC register
    pub ddrphyc_dx0dllcr: DDRPHYC_DX0DLLCR,
    ///0x1d0 - DDRPHYC byte lane 0 DQT register
    pub ddrphyc_dx0dqtr: DDRPHYC_DX0DQTR,
    ///0x1d4 - DDRPHYC byte lane 0 DQST register
    pub ddrphyc_dx0dqstr: DDRPHYC_DX0DQSTR,
    _reserved36: [u8; 0x28],
    ///0x200 - DDRPHYC byte lane 1 GC register
    pub ddrphyc_dx1gcr: DDRPHYC_DX1GCR,
    ///0x204 - DDRPHYC byte lane 1 GS register 0
    pub ddrphyc_dx1gsr0: DDRPHYC_DX1GSR0,
    _reserved38: [u8; 0x02],
    ///0x208 - DDRPHYC byte lane 1 GS register 1
    pub ddrphyc_dx1gsr1: DDRPHYC_DX1GSR1,
    ///0x20c - DDRPHYC byte lane 1 DLLC register
    pub ddrphyc_dx1dllcr: DDRPHYC_DX1DLLCR,
    ///0x210 - DDRPHYC byte lane 1 DQT register
    pub ddrphyc_dx1dqtr: DDRPHYC_DX1DQTR,
    ///0x214 - DDRPHYC byte lane 1 DQST register
    pub ddrphyc_dx1dqstr: DDRPHYC_DX1DQSTR,
    _reserved42: [u8; 0x28],
    ///0x240 - DDRPHYC byte lane 2 GC register
    pub ddrphyc_dx2gcr: DDRPHYC_DX2GCR,
    ///0x244 - DDRPHYC byte lane 2 GS register 0
    pub ddrphyc_dx2gsr0: DDRPHYC_DX2GSR0,
    _reserved44: [u8; 0x02],
    ///0x248 - DDRPHYC byte lane 2 GS register 1
    pub ddrphyc_dx2gsr1: DDRPHYC_DX2GSR1,
    ///0x24c - DDRPHYC byte lane 2 DLLC register
    pub ddrphyc_dx2dllcr: DDRPHYC_DX2DLLCR,
    ///0x250 - DDRPHYC byte lane 2 DQT register
    pub ddrphyc_dx2dqtr: DDRPHYC_DX2DQTR,
    ///0x254 - DDRPHYC byte lane 2 DQST register
    pub ddrphyc_dx2dqstr: DDRPHYC_DX2DQSTR,
    _reserved48: [u8; 0x28],
    ///0x280 - DDRPHYC byte lane 3 GC register
    pub ddrphyc_dx3gcr: DDRPHYC_DX3GCR,
    ///0x284 - DDRPHYC byte lane 3 GS register 0
    pub ddrphyc_dx3gsr0: DDRPHYC_DX3GSR0,
    _reserved50: [u8; 0x02],
    ///0x288 - DDRPHYC byte lane 3 GS register 1
    pub ddrphyc_dx3gsr1: DDRPHYC_DX3GSR1,
    ///0x28c - DDRPHYC byte lane 3 DLLC register
    pub ddrphyc_dx3dllcr: DDRPHYC_DX3DLLCR,
    ///0x290 - DDRPHYC byte lane 3 DQT register
    pub ddrphyc_dx3dqtr: DDRPHYC_DX3DQTR,
    ///0x294 - DDRPHYC byte lane 3 DQST register
    pub ddrphyc_dx3dqstr: DDRPHYC_DX3DQSTR,
}
///DDRPHYC_RIDR (r) register accessor: an alias for `Reg<DDRPHYC_RIDR_SPEC>`
pub type DDRPHYC_RIDR = crate::Reg<ddrphyc_ridr::DDRPHYC_RIDR_SPEC>;
///DDRPHYC revision ID register
pub mod ddrphyc_ridr;
///DDRPHYC_PIR (w) register accessor: an alias for `Reg<DDRPHYC_PIR_SPEC>`
pub type DDRPHYC_PIR = crate::Reg<ddrphyc_pir::DDRPHYC_PIR_SPEC>;
///DDRPHYC PHY initialization register
pub mod ddrphyc_pir;
///DDRPHYC_PGCR (rw) register accessor: an alias for `Reg<DDRPHYC_PGCR_SPEC>`
pub type DDRPHYC_PGCR = crate::Reg<ddrphyc_pgcr::DDRPHYC_PGCR_SPEC>;
///DDRPHYC PHY global control register
pub mod ddrphyc_pgcr;
///DDRPHYC_PGSR (r) register accessor: an alias for `Reg<DDRPHYC_PGSR_SPEC>`
pub type DDRPHYC_PGSR = crate::Reg<ddrphyc_pgsr::DDRPHYC_PGSR_SPEC>;
///DDRPHYC PHY global status register
pub mod ddrphyc_pgsr;
///DDRPHYC_DLLGCR (rw) register accessor: an alias for `Reg<DDRPHYC_DLLGCR_SPEC>`
pub type DDRPHYC_DLLGCR = crate::Reg<ddrphyc_dllgcr::DDRPHYC_DLLGCR_SPEC>;
///DDRPHYC DDR global control register
pub mod ddrphyc_dllgcr;
///DDRPHYC_ACDLLCR (rw) register accessor: an alias for `Reg<DDRPHYC_ACDLLCR_SPEC>`
pub type DDRPHYC_ACDLLCR = crate::Reg<ddrphyc_acdllcr::DDRPHYC_ACDLLCR_SPEC>;
///DDRPHYC AC DLL control register
pub mod ddrphyc_acdllcr;
///DDRPHYC_PTR0 (rw) register accessor: an alias for `Reg<DDRPHYC_PTR0_SPEC>`
pub type DDRPHYC_PTR0 = crate::Reg<ddrphyc_ptr0::DDRPHYC_PTR0_SPEC>;
///DDRPHYC PT register 0
pub mod ddrphyc_ptr0;
///DDRPHYC_PTR1 (rw) register accessor: an alias for `Reg<DDRPHYC_PTR1_SPEC>`
pub type DDRPHYC_PTR1 = crate::Reg<ddrphyc_ptr1::DDRPHYC_PTR1_SPEC>;
///DDRPHYC PT register 1
pub mod ddrphyc_ptr1;
///DDRPHYC_PTR2 (rw) register accessor: an alias for `Reg<DDRPHYC_PTR2_SPEC>`
pub type DDRPHYC_PTR2 = crate::Reg<ddrphyc_ptr2::DDRPHYC_PTR2_SPEC>;
///DDRPHYC PT register 2
pub mod ddrphyc_ptr2;
///DDRPHYC_ACIOCR (rw) register accessor: an alias for `Reg<DDRPHYC_ACIOCR_SPEC>`
pub type DDRPHYC_ACIOCR = crate::Reg<ddrphyc_aciocr::DDRPHYC_ACIOCR_SPEC>;
///DDRPHYC ACIOC register
pub mod ddrphyc_aciocr;
///DDRPHYC_DXCCR (rw) register accessor: an alias for `Reg<DDRPHYC_DXCCR_SPEC>`
pub type DDRPHYC_DXCCR = crate::Reg<ddrphyc_dxccr::DDRPHYC_DXCCR_SPEC>;
///DDRPHYC DXCC register
pub mod ddrphyc_dxccr;
///DDRPHYC_DSGCR (rw) register accessor: an alias for `Reg<DDRPHYC_DSGCR_SPEC>`
pub type DDRPHYC_DSGCR = crate::Reg<ddrphyc_dsgcr::DDRPHYC_DSGCR_SPEC>;
///DDRPHYC DSGC register
pub mod ddrphyc_dsgcr;
///DDRPHYC_DCR (rw) register accessor: an alias for `Reg<DDRPHYC_DCR_SPEC>`
pub type DDRPHYC_DCR = crate::Reg<ddrphyc_dcr::DDRPHYC_DCR_SPEC>;
///DDRPHYC DC register
pub mod ddrphyc_dcr;
///DDRPHYC_DTPR0 (rw) register accessor: an alias for `Reg<DDRPHYC_DTPR0_SPEC>`
pub type DDRPHYC_DTPR0 = crate::Reg<ddrphyc_dtpr0::DDRPHYC_DTPR0_SPEC>;
///DDRPHYC DTP register 0
pub mod ddrphyc_dtpr0;
///DDRPHYC_DTPR1 (rw) register accessor: an alias for `Reg<DDRPHYC_DTPR1_SPEC>`
pub type DDRPHYC_DTPR1 = crate::Reg<ddrphyc_dtpr1::DDRPHYC_DTPR1_SPEC>;
///DDRPHYC DTP register 1
pub mod ddrphyc_dtpr1;
///DDRPHYC_DTPR2 (rw) register accessor: an alias for `Reg<DDRPHYC_DTPR2_SPEC>`
pub type DDRPHYC_DTPR2 = crate::Reg<ddrphyc_dtpr2::DDRPHYC_DTPR2_SPEC>;
///DDRPHYC DTP register 2
pub mod ddrphyc_dtpr2;
///DDRPHYC_DDR3_MR0 (rw) register accessor: an alias for `Reg<DDRPHYC_DDR3_MR0_SPEC>`
pub type DDRPHYC_DDR3_MR0 = crate::Reg<ddrphyc_ddr3_mr0::DDRPHYC_DDR3_MR0_SPEC>;
///DDRPHYC MR0 register for DDR3
pub mod ddrphyc_ddr3_mr0;
///DDRPHYC_DDR3_MR1 (rw) register accessor: an alias for `Reg<DDRPHYC_DDR3_MR1_SPEC>`
pub type DDRPHYC_DDR3_MR1 = crate::Reg<ddrphyc_ddr3_mr1::DDRPHYC_DDR3_MR1_SPEC>;
///DDRPHYC MR1 register for DDR3
pub mod ddrphyc_ddr3_mr1;
///DDRPHYC_DDR3_MR2 (rw) register accessor: an alias for `Reg<DDRPHYC_DDR3_MR2_SPEC>`
pub type DDRPHYC_DDR3_MR2 = crate::Reg<ddrphyc_ddr3_mr2::DDRPHYC_DDR3_MR2_SPEC>;
///DDRPHYC MR2 register for DDR3
pub mod ddrphyc_ddr3_mr2;
///DDRPHYC_DDR3_MR3 (rw) register accessor: an alias for `Reg<DDRPHYC_DDR3_MR3_SPEC>`
pub type DDRPHYC_DDR3_MR3 = crate::Reg<ddrphyc_ddr3_mr3::DDRPHYC_DDR3_MR3_SPEC>;
///DDRPHYC MR3 register for DDR3
pub mod ddrphyc_ddr3_mr3;
///DDRPHYC_ODTCR (rw) register accessor: an alias for `Reg<DDRPHYC_ODTCR_SPEC>`
pub type DDRPHYC_ODTCR = crate::Reg<ddrphyc_odtcr::DDRPHYC_ODTCR_SPEC>;
///DDRPHYC ODTC register
pub mod ddrphyc_odtcr;
///DDRPHYC_DTAR (rw) register accessor: an alias for `Reg<DDRPHYC_DTAR_SPEC>`
pub type DDRPHYC_DTAR = crate::Reg<ddrphyc_dtar::DDRPHYC_DTAR_SPEC>;
///DDRPHYC DTA register
pub mod ddrphyc_dtar;
///DDRPHYC_DTDR0 (rw) register accessor: an alias for `Reg<DDRPHYC_DTDR0_SPEC>`
pub type DDRPHYC_DTDR0 = crate::Reg<ddrphyc_dtdr0::DDRPHYC_DTDR0_SPEC>;
///DDRPHYC DTD register 0
pub mod ddrphyc_dtdr0;
///DDRPHYC_DTDR1 (rw) register accessor: an alias for `Reg<DDRPHYC_DTDR1_SPEC>`
pub type DDRPHYC_DTDR1 = crate::Reg<ddrphyc_dtdr1::DDRPHYC_DTDR1_SPEC>;
///DDRPHYC DTD register 1
pub mod ddrphyc_dtdr1;
///DDRPHYC_GPR0 (rw) register accessor: an alias for `Reg<DDRPHYC_GPR0_SPEC>`
pub type DDRPHYC_GPR0 = crate::Reg<ddrphyc_gpr0::DDRPHYC_GPR0_SPEC>;
///DDRPHYC general purpose register 0
pub mod ddrphyc_gpr0;
///DDRPHYC_GPR1 (rw) register accessor: an alias for `Reg<DDRPHYC_GPR1_SPEC>`
pub type DDRPHYC_GPR1 = crate::Reg<ddrphyc_gpr1::DDRPHYC_GPR1_SPEC>;
///DDRPHYC general purpose register 1
pub mod ddrphyc_gpr1;
///DDRPHYC_ZQ0CR0 (rw) register accessor: an alias for `Reg<DDRPHYC_ZQ0CR0_SPEC>`
pub type DDRPHYC_ZQ0CR0 = crate::Reg<ddrphyc_zq0cr0::DDRPHYC_ZQ0CR0_SPEC>;
///DDRPHYC ZQ0C register 0
pub mod ddrphyc_zq0cr0;
///DDRPHYC_ZQ0CR1 (rw) register accessor: an alias for `Reg<DDRPHYC_ZQ0CR1_SPEC>`
pub type DDRPHYC_ZQ0CR1 = crate::Reg<ddrphyc_zq0cr1::DDRPHYC_ZQ0CR1_SPEC>;
///DDRPHYC ZQ0CR1 register
pub mod ddrphyc_zq0cr1;
///DDRPHYC_ZQ0SR0 (r) register accessor: an alias for `Reg<DDRPHYC_ZQ0SR0_SPEC>`
pub type DDRPHYC_ZQ0SR0 = crate::Reg<ddrphyc_zq0sr0::DDRPHYC_ZQ0SR0_SPEC>;
///DDRPHYC ZQ0S register 0
pub mod ddrphyc_zq0sr0;
///DDRPHYC_ZQ0SR1 (r) register accessor: an alias for `Reg<DDRPHYC_ZQ0SR1_SPEC>`
pub type DDRPHYC_ZQ0SR1 = crate::Reg<ddrphyc_zq0sr1::DDRPHYC_ZQ0SR1_SPEC>;
///DDRPHYC ZQ0S register 1
pub mod ddrphyc_zq0sr1;
///DDRPHYC_DX0GCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX0GCR_SPEC>`
pub type DDRPHYC_DX0GCR = crate::Reg<ddrphyc_dx0gcr::DDRPHYC_DX0GCR_SPEC>;
///DDRPHYC byte lane 0 GC register
pub mod ddrphyc_dx0gcr;
///DDRPHYC_DX0GSR0 (r) register accessor: an alias for `Reg<DDRPHYC_DX0GSR0_SPEC>`
pub type DDRPHYC_DX0GSR0 = crate::Reg<ddrphyc_dx0gsr0::DDRPHYC_DX0GSR0_SPEC>;
///DDRPHYC byte lane 0 GS register 0
pub mod ddrphyc_dx0gsr0;
///DDRPHYC_DX0GSR1 (r) register accessor: an alias for `Reg<DDRPHYC_DX0GSR1_SPEC>`
pub type DDRPHYC_DX0GSR1 = crate::Reg<ddrphyc_dx0gsr1::DDRPHYC_DX0GSR1_SPEC>;
///DDRPHYC byte lane 0 GS register 1
pub mod ddrphyc_dx0gsr1;
///DDRPHYC_DX0DLLCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX0DLLCR_SPEC>`
pub type DDRPHYC_DX0DLLCR = crate::Reg<ddrphyc_dx0dllcr::DDRPHYC_DX0DLLCR_SPEC>;
///DDRPHYC byte lane 0 DLLC register
pub mod ddrphyc_dx0dllcr;
///DDRPHYC_DX0DQTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX0DQTR_SPEC>`
pub type DDRPHYC_DX0DQTR = crate::Reg<ddrphyc_dx0dqtr::DDRPHYC_DX0DQTR_SPEC>;
///DDRPHYC byte lane 0 DQT register
pub mod ddrphyc_dx0dqtr;
///DDRPHYC_DX0DQSTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX0DQSTR_SPEC>`
pub type DDRPHYC_DX0DQSTR = crate::Reg<ddrphyc_dx0dqstr::DDRPHYC_DX0DQSTR_SPEC>;
///DDRPHYC byte lane 0 DQST register
pub mod ddrphyc_dx0dqstr;
///DDRPHYC_DX1GCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX1GCR_SPEC>`
pub type DDRPHYC_DX1GCR = crate::Reg<ddrphyc_dx1gcr::DDRPHYC_DX1GCR_SPEC>;
///DDRPHYC byte lane 1 GC register
pub mod ddrphyc_dx1gcr;
///DDRPHYC_DX1GSR0 (r) register accessor: an alias for `Reg<DDRPHYC_DX1GSR0_SPEC>`
pub type DDRPHYC_DX1GSR0 = crate::Reg<ddrphyc_dx1gsr0::DDRPHYC_DX1GSR0_SPEC>;
///DDRPHYC byte lane 1 GS register 0
pub mod ddrphyc_dx1gsr0;
///DDRPHYC_DX1GSR1 (r) register accessor: an alias for `Reg<DDRPHYC_DX1GSR1_SPEC>`
pub type DDRPHYC_DX1GSR1 = crate::Reg<ddrphyc_dx1gsr1::DDRPHYC_DX1GSR1_SPEC>;
///DDRPHYC byte lane 1 GS register 1
pub mod ddrphyc_dx1gsr1;
///DDRPHYC_DX1DLLCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX1DLLCR_SPEC>`
pub type DDRPHYC_DX1DLLCR = crate::Reg<ddrphyc_dx1dllcr::DDRPHYC_DX1DLLCR_SPEC>;
///DDRPHYC byte lane 1 DLLC register
pub mod ddrphyc_dx1dllcr;
///DDRPHYC_DX1DQTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX1DQTR_SPEC>`
pub type DDRPHYC_DX1DQTR = crate::Reg<ddrphyc_dx1dqtr::DDRPHYC_DX1DQTR_SPEC>;
///DDRPHYC byte lane 1 DQT register
pub mod ddrphyc_dx1dqtr;
///DDRPHYC_DX1DQSTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX1DQSTR_SPEC>`
pub type DDRPHYC_DX1DQSTR = crate::Reg<ddrphyc_dx1dqstr::DDRPHYC_DX1DQSTR_SPEC>;
///DDRPHYC byte lane 1 DQST register
pub mod ddrphyc_dx1dqstr;
///DDRPHYC_DX2GCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX2GCR_SPEC>`
pub type DDRPHYC_DX2GCR = crate::Reg<ddrphyc_dx2gcr::DDRPHYC_DX2GCR_SPEC>;
///DDRPHYC byte lane 2 GC register
pub mod ddrphyc_dx2gcr;
///DDRPHYC_DX2GSR0 (r) register accessor: an alias for `Reg<DDRPHYC_DX2GSR0_SPEC>`
pub type DDRPHYC_DX2GSR0 = crate::Reg<ddrphyc_dx2gsr0::DDRPHYC_DX2GSR0_SPEC>;
///DDRPHYC byte lane 2 GS register 0
pub mod ddrphyc_dx2gsr0;
///DDRPHYC_DX2GSR1 (r) register accessor: an alias for `Reg<DDRPHYC_DX2GSR1_SPEC>`
pub type DDRPHYC_DX2GSR1 = crate::Reg<ddrphyc_dx2gsr1::DDRPHYC_DX2GSR1_SPEC>;
///DDRPHYC byte lane 2 GS register 1
pub mod ddrphyc_dx2gsr1;
///DDRPHYC_DX2DLLCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX2DLLCR_SPEC>`
pub type DDRPHYC_DX2DLLCR = crate::Reg<ddrphyc_dx2dllcr::DDRPHYC_DX2DLLCR_SPEC>;
///DDRPHYC byte lane 2 DLLC register
pub mod ddrphyc_dx2dllcr;
///DDRPHYC_DX2DQTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX2DQTR_SPEC>`
pub type DDRPHYC_DX2DQTR = crate::Reg<ddrphyc_dx2dqtr::DDRPHYC_DX2DQTR_SPEC>;
///DDRPHYC byte lane 2 DQT register
pub mod ddrphyc_dx2dqtr;
///DDRPHYC_DX2DQSTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX2DQSTR_SPEC>`
pub type DDRPHYC_DX2DQSTR = crate::Reg<ddrphyc_dx2dqstr::DDRPHYC_DX2DQSTR_SPEC>;
///DDRPHYC byte lane 2 DQST register
pub mod ddrphyc_dx2dqstr;
///DDRPHYC_DX3GCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX3GCR_SPEC>`
pub type DDRPHYC_DX3GCR = crate::Reg<ddrphyc_dx3gcr::DDRPHYC_DX3GCR_SPEC>;
///DDRPHYC byte lane 3 GC register
pub mod ddrphyc_dx3gcr;
///DDRPHYC_DX3GSR0 (r) register accessor: an alias for `Reg<DDRPHYC_DX3GSR0_SPEC>`
pub type DDRPHYC_DX3GSR0 = crate::Reg<ddrphyc_dx3gsr0::DDRPHYC_DX3GSR0_SPEC>;
///DDRPHYC byte lane 3 GS register 0
pub mod ddrphyc_dx3gsr0;
///DDRPHYC_DX3GSR1 (r) register accessor: an alias for `Reg<DDRPHYC_DX3GSR1_SPEC>`
pub type DDRPHYC_DX3GSR1 = crate::Reg<ddrphyc_dx3gsr1::DDRPHYC_DX3GSR1_SPEC>;
///DDRPHYC byte lane 3 GS register 1
pub mod ddrphyc_dx3gsr1;
///DDRPHYC_DX3DLLCR (rw) register accessor: an alias for `Reg<DDRPHYC_DX3DLLCR_SPEC>`
pub type DDRPHYC_DX3DLLCR = crate::Reg<ddrphyc_dx3dllcr::DDRPHYC_DX3DLLCR_SPEC>;
///DDRPHYC byte lane 3 DLLC register
pub mod ddrphyc_dx3dllcr;
///DDRPHYC_DX3DQTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX3DQTR_SPEC>`
pub type DDRPHYC_DX3DQTR = crate::Reg<ddrphyc_dx3dqtr::DDRPHYC_DX3DQTR_SPEC>;
///DDRPHYC byte lane 3 DQT register
pub mod ddrphyc_dx3dqtr;
///DDRPHYC_DX3DQSTR (rw) register accessor: an alias for `Reg<DDRPHYC_DX3DQSTR_SPEC>`
pub type DDRPHYC_DX3DQSTR = crate::Reg<ddrphyc_dx3dqstr::DDRPHYC_DX3DQSTR_SPEC>;
///DDRPHYC byte lane 3 DQST register
pub mod ddrphyc_dx3dqstr;
