///Register `FLASH_OPTR` reader
pub struct R(crate::R<FLASH_OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_OPTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_OPTR` writer
pub struct W(crate::W<FLASH_OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FLASH_OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_OPTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RDP` reader - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
pub type RDP_R = crate::FieldReader<u8, u8>;
///Field `RDP` writer - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_OPTR_SPEC, u8, u8, 8, O>;
///Field `BOR_LEV` reader - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
///Field `BOR_LEV` writer - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_OPTR_SPEC, u8, u8, 3, O>;
///Field `nRST_STOP` reader - Reset generation in Stop mode
pub type N_RST_STOP_R = crate::BitReader<bool>;
///Field `nRST_STOP` writer - Reset generation in Stop mode
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `nRST_STDBY` reader - Reset generation in Standby mode
pub type N_RST_STDBY_R = crate::BitReader<bool>;
///Field `nRST_STDBY` writer - Reset generation in Standby mode
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `nRST_SHDW` reader - Reset generation in Shutdown mode
pub type N_RST_SHDW_R = crate::BitReader<bool>;
///Field `nRST_SHDW` writer - Reset generation in Shutdown mode
pub type N_RST_SHDW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `SRAM1345_RST` reader - SRAM1, SRAM3 and SRAM4 erase upon system reset
pub type SRAM1345_RST_R = crate::BitReader<bool>;
///Field `SRAM1345_RST` writer - SRAM1, SRAM3 and SRAM4 erase upon system reset
pub type SRAM1345_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `IWDG_SW` reader - Independent watchdog selection
pub type IWDG_SW_R = crate::BitReader<bool>;
///Field `IWDG_SW` writer - Independent watchdog selection
pub type IWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<bool>;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader<bool>;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader<bool>;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `SWAP_BANK` reader - Swap banks
pub type SWAP_BANK_R = crate::BitReader<bool>;
///Field `SWAP_BANK` writer - Swap banks
pub type SWAP_BANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `DUALBANK` reader - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
pub type DUALBANK_R = crate::BitReader<bool>;
///Field `DUALBANK` writer - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
pub type DUALBANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction enable
pub type BKPRAM_ECC_R = crate::BitReader<bool>;
///Field `BKPRAM_ECC` writer - Backup RAM ECC detection and correction enable
pub type BKPRAM_ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `SRAM3_ECC` reader - SRAM3 ECC detection and correction enable
pub type SRAM3_ECC_R = crate::BitReader<bool>;
///Field `SRAM3_ECC` writer - SRAM3 ECC detection and correction enable
pub type SRAM3_ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction enable
pub type SRAM2_ECC_R = crate::BitReader<bool>;
///Field `SRAM2_ECC` writer - SRAM2 ECC detection and correction enable
pub type SRAM2_ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `SRAM2_RST` reader - SRAM2 erase when system reset
pub type SRAM2_RST_R = crate::BitReader<bool>;
///Field `SRAM2_RST` writer - SRAM2 erase when system reset
pub type SRAM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `nSWBOOT0` reader - Software BOOT0
pub type N_SWBOOT0_R = crate::BitReader<bool>;
///Field `nSWBOOT0` writer - Software BOOT0
pub type N_SWBOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader<bool>;
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `PA15_PUPEN` reader - PA15 pull-up enable
pub type PA15_PUPEN_R = crate::BitReader<bool>;
///Field `PA15_PUPEN` writer - PA15 pull-up enable
pub type PA15_PUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `IO_VDD_HSLV` reader - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
pub type IO_VDD_HSLV_R = crate::BitReader<bool>;
///Field `IO_VDD_HSLV` writer - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
pub type IO_VDD_HSLV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `IO_VDDIO2_HSLV` reader - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
pub type IO_VDDIO2_HSLV_R = crate::BitReader<bool>;
///Field `IO_VDDIO2_HSLV` writer - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
pub type IO_VDDIO2_HSLV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
///Field `TZEN` reader - Global TrustZone security enable
pub type TZEN_R = crate::BitReader<bool>;
///Field `TZEN` writer - Global TrustZone security enable
pub type TZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OPTR_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - Reset generation in Stop mode
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reset generation in Standby mode
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Reset generation in Shutdown mode
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SRAM1, SRAM3 and SRAM4 erase upon system reset
    #[inline(always)]
    pub fn sram1345_rst(&self) -> SRAM1345_RST_R {
        SRAM1345_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Swap banks
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
    #[inline(always)]
    pub fn dualbank(&self) -> DUALBANK_R {
        DUALBANK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Backup RAM ECC detection and correction enable
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SRAM3 ECC detection and correction enable
    #[inline(always)]
    pub fn sram3_ecc(&self) -> SRAM3_ECC_R {
        SRAM3_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 ECC detection and correction enable
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PA15 pull-up enable
    #[inline(always)]
    pub fn pa15_pupen(&self) -> PA15_PUPEN_R {
        PA15_PUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
    #[inline(always)]
    pub fn io_vdd_hslv(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
    #[inline(always)]
    pub fn io_vddio2_hslv(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Global TrustZone security enable
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<0> {
        RDP_W::new(self)
    }
    ///Bits 8:10 - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<8> {
        BOR_LEV_W::new(self)
    }
    ///Bit 12 - Reset generation in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<12> {
        N_RST_STOP_W::new(self)
    }
    ///Bit 13 - Reset generation in Standby mode
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<13> {
        N_RST_STDBY_W::new(self)
    }
    ///Bit 14 - Reset generation in Shutdown mode
    #[inline(always)]
    #[must_use]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<14> {
        N_RST_SHDW_W::new(self)
    }
    ///Bit 15 - SRAM1, SRAM3 and SRAM4 erase upon system reset
    #[inline(always)]
    #[must_use]
    pub fn sram1345_rst(&mut self) -> SRAM1345_RST_W<15> {
        SRAM1345_RST_W::new(self)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<16> {
        IWDG_SW_W::new(self)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<17> {
        IWDG_STOP_W::new(self)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<18> {
        IWDG_STDBY_W::new(self)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<19> {
        WWDG_SW_W::new(self)
    }
    ///Bit 20 - Swap banks
    #[inline(always)]
    #[must_use]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<20> {
        SWAP_BANK_W::new(self)
    }
    ///Bit 21 - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
    #[inline(always)]
    #[must_use]
    pub fn dualbank(&mut self) -> DUALBANK_W<21> {
        DUALBANK_W::new(self)
    }
    ///Bit 22 - Backup RAM ECC detection and correction enable
    #[inline(always)]
    #[must_use]
    pub fn bkpram_ecc(&mut self) -> BKPRAM_ECC_W<22> {
        BKPRAM_ECC_W::new(self)
    }
    ///Bit 23 - SRAM3 ECC detection and correction enable
    #[inline(always)]
    #[must_use]
    pub fn sram3_ecc(&mut self) -> SRAM3_ECC_W<23> {
        SRAM3_ECC_W::new(self)
    }
    ///Bit 24 - SRAM2 ECC detection and correction enable
    #[inline(always)]
    #[must_use]
    pub fn sram2_ecc(&mut self) -> SRAM2_ECC_W<24> {
        SRAM2_ECC_W::new(self)
    }
    ///Bit 25 - SRAM2 erase when system reset
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<25> {
        SRAM2_RST_W::new(self)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<26> {
        N_SWBOOT0_W::new(self)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<27> {
        N_BOOT0_W::new(self)
    }
    ///Bit 28 - PA15 pull-up enable
    #[inline(always)]
    #[must_use]
    pub fn pa15_pupen(&mut self) -> PA15_PUPEN_W<28> {
        PA15_PUPEN_W::new(self)
    }
    ///Bit 29 - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
    #[inline(always)]
    #[must_use]
    pub fn io_vdd_hslv(&mut self) -> IO_VDD_HSLV_W<29> {
        IO_VDD_HSLV_W::new(self)
    }
    ///Bit 30 - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
    #[inline(always)]
    #[must_use]
    pub fn io_vddio2_hslv(&mut self) -> IO_VDDIO2_HSLV_W<30> {
        IO_VDDIO2_HSLV_W::new(self)
    }
    ///Bit 31 - Global TrustZone security enable
    #[inline(always)]
    #[must_use]
    pub fn tzen(&mut self) -> TZEN_W<31> {
        TZEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_optr](index.html) module
pub struct FLASH_OPTR_SPEC;
impl crate::RegisterSpec for FLASH_OPTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_optr::R](R) reader structure
impl crate::Readable for FLASH_OPTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_optr::W](W) writer structure
impl crate::Writable for FLASH_OPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_OPTR to value 0
impl crate::Resettable for FLASH_OPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
