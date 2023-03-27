///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x28 - Backup data register (BKP_DR)
    pub dr: [DR; 10],
    ///0x28 - RTC clock calibration register (BKP_RTCCR)
    pub rtccr: RTCCR,
    ///0x2c - Backup control register (BKP_CR)
    pub cr: CR,
    ///0x30 - BKP_CSR control/status register
    pub csr: CSR,
    _reserved4: [u8; 0x08],
    ///0x3c..0xbc - Backup data register (BKP_DR)
    pub bkp_dr: [BKP_DR; 32],
}
impl RegisterBlock {
    ///0x00 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr1(&self) -> &DR {
        &self.dr[0]
    }
    ///0x04 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr2(&self) -> &DR {
        &self.dr[1]
    }
    ///0x08 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr3(&self) -> &DR {
        &self.dr[2]
    }
    ///0x0c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr4(&self) -> &DR {
        &self.dr[3]
    }
    ///0x10 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr5(&self) -> &DR {
        &self.dr[4]
    }
    ///0x14 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr6(&self) -> &DR {
        &self.dr[5]
    }
    ///0x18 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr7(&self) -> &DR {
        &self.dr[6]
    }
    ///0x1c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr8(&self) -> &DR {
        &self.dr[7]
    }
    ///0x20 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr9(&self) -> &DR {
        &self.dr[8]
    }
    ///0x24 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr10(&self) -> &DR {
        &self.dr[9]
    }
    ///0x3c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr11(&self) -> &BKP_DR {
        &self.bkp_dr[0]
    }
    ///0x40 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr12(&self) -> &BKP_DR {
        &self.bkp_dr[1]
    }
    ///0x44 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr13(&self) -> &BKP_DR {
        &self.bkp_dr[2]
    }
    ///0x48 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr14(&self) -> &BKP_DR {
        &self.bkp_dr[3]
    }
    ///0x4c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr15(&self) -> &BKP_DR {
        &self.bkp_dr[4]
    }
    ///0x50 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr16(&self) -> &BKP_DR {
        &self.bkp_dr[5]
    }
    ///0x54 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr17(&self) -> &BKP_DR {
        &self.bkp_dr[6]
    }
    ///0x58 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr18(&self) -> &BKP_DR {
        &self.bkp_dr[7]
    }
    ///0x5c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr19(&self) -> &BKP_DR {
        &self.bkp_dr[8]
    }
    ///0x60 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr20(&self) -> &BKP_DR {
        &self.bkp_dr[9]
    }
    ///0x64 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr21(&self) -> &BKP_DR {
        &self.bkp_dr[10]
    }
    ///0x68 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr22(&self) -> &BKP_DR {
        &self.bkp_dr[11]
    }
    ///0x6c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr23(&self) -> &BKP_DR {
        &self.bkp_dr[12]
    }
    ///0x70 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr24(&self) -> &BKP_DR {
        &self.bkp_dr[13]
    }
    ///0x74 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr25(&self) -> &BKP_DR {
        &self.bkp_dr[14]
    }
    ///0x78 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr26(&self) -> &BKP_DR {
        &self.bkp_dr[15]
    }
    ///0x7c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr27(&self) -> &BKP_DR {
        &self.bkp_dr[16]
    }
    ///0x80 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr28(&self) -> &BKP_DR {
        &self.bkp_dr[17]
    }
    ///0x84 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr29(&self) -> &BKP_DR {
        &self.bkp_dr[18]
    }
    ///0x88 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr30(&self) -> &BKP_DR {
        &self.bkp_dr[19]
    }
    ///0x8c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr31(&self) -> &BKP_DR {
        &self.bkp_dr[20]
    }
    ///0x90 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr32(&self) -> &BKP_DR {
        &self.bkp_dr[21]
    }
    ///0x94 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr33(&self) -> &BKP_DR {
        &self.bkp_dr[22]
    }
    ///0x98 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr34(&self) -> &BKP_DR {
        &self.bkp_dr[23]
    }
    ///0x9c - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr35(&self) -> &BKP_DR {
        &self.bkp_dr[24]
    }
    ///0xa0 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr36(&self) -> &BKP_DR {
        &self.bkp_dr[25]
    }
    ///0xa4 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr37(&self) -> &BKP_DR {
        &self.bkp_dr[26]
    }
    ///0xa8 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr38(&self) -> &BKP_DR {
        &self.bkp_dr[27]
    }
    ///0xac - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr39(&self) -> &BKP_DR {
        &self.bkp_dr[28]
    }
    ///0xb0 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr40(&self) -> &BKP_DR {
        &self.bkp_dr[29]
    }
    ///0xb4 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr41(&self) -> &BKP_DR {
        &self.bkp_dr[30]
    }
    ///0xb8 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr42(&self) -> &BKP_DR {
        &self.bkp_dr[31]
    }
}
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Backup data register (BKP_DR)
pub mod dr;
///BKP_DR (rw) register accessor: an alias for `Reg<BKP_DR_SPEC>`
pub type BKP_DR = crate::Reg<bkp_dr::BKP_DR_SPEC>;
///Backup data register (BKP_DR)
pub mod bkp_dr;
///RTCCR (rw) register accessor: an alias for `Reg<RTCCR_SPEC>`
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
///RTC clock calibration register (BKP_RTCCR)
pub mod rtccr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Backup control register (BKP_CR)
pub mod cr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///BKP_CSR control/status register
pub mod csr;
