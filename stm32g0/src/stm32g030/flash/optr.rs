///Register `OPTR` reader
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPTR` writer
pub struct W(crate::W<OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTR_SPEC>;
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
impl From<crate::W<OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RDP` reader - Read protection level
pub type RDP_R = crate::FieldReader<u8, u8>;
///Field `RDP` writer - Read protection level
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 8, O>;
///Field `BOREN` reader - BOR reset Level
pub type BOREN_R = crate::BitReader<bool>;
///Field `BOREN` writer - BOR reset Level
pub type BOREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `BORF_LEV` reader - These bits contain the VDD supply level threshold that activates the reset
pub type BORF_LEV_R = crate::FieldReader<u8, u8>;
///Field `BORF_LEV` writer - These bits contain the VDD supply level threshold that activates the reset
pub type BORF_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 2, O>;
///Field `BORR_LEV` reader - These bits contain the VDD supply level threshold that releases the reset.
pub type BORR_LEV_R = crate::FieldReader<u8, u8>;
///Field `BORR_LEV` writer - These bits contain the VDD supply level threshold that releases the reset.
pub type BORR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 2, O>;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader<bool>;
///Field `nRST_STOP` writer - nRST_STOP
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader<bool>;
///Field `nRST_STDBY` writer - nRST_STDBY
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nRSTS_HDW` reader - nRSTS_HDW
pub type N_RSTS_HDW_R = crate::BitReader<bool>;
///Field `nRSTS_HDW` writer - nRSTS_HDW
pub type N_RSTS_HDW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `IDWG_SW` reader - Independent watchdog selection
pub type IDWG_SW_R = crate::BitReader<bool>;
///Field `IDWG_SW` writer - Independent watchdog selection
pub type IDWG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<bool>;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader<bool>;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader<bool>;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `RAM_PARITY_CHECK` reader - SRAM parity check control
pub type RAM_PARITY_CHECK_R = crate::BitReader<bool>;
///Field `RAM_PARITY_CHECK` writer - SRAM parity check control
pub type RAM_PARITY_CHECK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nBOOT_SEL` reader - nBOOT_SEL
pub type N_BOOT_SEL_R = crate::BitReader<bool>;
///Field `nBOOT_SEL` writer - nBOOT_SEL
pub type N_BOOT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nBOOT1` reader - Boot configuration
pub type N_BOOT1_R = crate::BitReader<bool>;
///Field `nBOOT1` writer - Boot configuration
pub type N_BOOT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader<bool>;
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `NRST_MODE` reader - NRST_MODE
pub type NRST_MODE_R = crate::FieldReader<u8, u8>;
///Field `NRST_MODE` writer - NRST_MODE
pub type NRST_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 2, O>;
///Field `IRHEN` reader - Internal reset holder enable bit
pub type IRHEN_R = crate::BitReader<bool>;
///Field `IRHEN` writer - Internal reset holder enable bit
pub type IRHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - BOR reset Level
    #[inline(always)]
    pub fn boren(&self) -> BOREN_R {
        BOREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset
    #[inline(always)]
    pub fn borf_lev(&self) -> BORF_LEV_R {
        BORF_LEV_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset.
    #[inline(always)]
    pub fn borr_lev(&self) -> BORR_LEV_R {
        BORR_LEV_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - nRSTS_HDW
    #[inline(always)]
    pub fn n_rsts_hdw(&self) -> N_RSTS_HDW_R {
        N_RSTS_HDW_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 22 - SRAM parity check control
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - nBOOT_SEL
    #[inline(always)]
    pub fn n_boot_sel(&self) -> N_BOOT_SEL_R {
        N_BOOT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - NRST_MODE
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<0> {
        RDP_W::new(self)
    }
    ///Bit 8 - BOR reset Level
    #[inline(always)]
    #[must_use]
    pub fn boren(&mut self) -> BOREN_W<8> {
        BOREN_W::new(self)
    }
    ///Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset
    #[inline(always)]
    #[must_use]
    pub fn borf_lev(&mut self) -> BORF_LEV_W<9> {
        BORF_LEV_W::new(self)
    }
    ///Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset.
    #[inline(always)]
    #[must_use]
    pub fn borr_lev(&mut self) -> BORR_LEV_W<11> {
        BORR_LEV_W::new(self)
    }
    ///Bit 13 - nRST_STOP
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<13> {
        N_RST_STOP_W::new(self)
    }
    ///Bit 14 - nRST_STDBY
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<14> {
        N_RST_STDBY_W::new(self)
    }
    ///Bit 15 - nRSTS_HDW
    #[inline(always)]
    #[must_use]
    pub fn n_rsts_hdw(&mut self) -> N_RSTS_HDW_W<15> {
        N_RSTS_HDW_W::new(self)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<16> {
        IDWG_SW_W::new(self)
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
    ///Bit 22 - SRAM parity check control
    #[inline(always)]
    #[must_use]
    pub fn ram_parity_check(&mut self) -> RAM_PARITY_CHECK_W<22> {
        RAM_PARITY_CHECK_W::new(self)
    }
    ///Bit 24 - nBOOT_SEL
    #[inline(always)]
    #[must_use]
    pub fn n_boot_sel(&mut self) -> N_BOOT_SEL_W<24> {
        N_BOOT_SEL_W::new(self)
    }
    ///Bit 25 - Boot configuration
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<25> {
        N_BOOT1_W::new(self)
    }
    ///Bit 26 - nBOOT0 option bit
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<26> {
        N_BOOT0_W::new(self)
    }
    ///Bits 27:28 - NRST_MODE
    #[inline(always)]
    #[must_use]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<27> {
        NRST_MODE_W::new(self)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    #[must_use]
    pub fn irhen(&mut self) -> IRHEN_W<29> {
        IRHEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optr](index.html) module
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [optr::R](R) reader structure
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [optr::W](W) writer structure
impl crate::Writable for OPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPTR to value 0xf000_0000
impl crate::Resettable for OPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf000_0000;
}
