///Register `FCR2` writer
pub struct W(crate::W<FCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR2_SPEC>;
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
impl From<crate::W<FCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSYSCFGF` writer - clear the illegal access flag for SYSCFG
pub type CSYSCFGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CRTCF` writer - clear the illegal access flag for RTC
pub type CRTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CTAMPF` writer - clear the illegal access flag for TAMP
pub type CTAMPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CPWRF` writer - clear the illegal access flag for PWR
pub type CPWRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CRCCF` writer - clear the illegal access flag for RCC
pub type CRCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CLPDMA1F` writer - clear the illegal access flag for LPDMA
pub type CLPDMA1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CEXTIF` writer - clear the illegal access flag for EXTI
pub type CEXTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CTZSC2F` writer - clear the illegal access flag for GTZC2 TZSC registers
pub type CTZSC2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CTZIC2F` writer - clear the illegal access flag for GTZC2 TZIC registers
pub type CTZIC2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CSRAM4F` writer - clear the illegal access flag for SRAM4
pub type CSRAM4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
///Field `CMPCBB4_REGF` writer - clear the illegal access flag for MPCBB4 registers
pub type CMPCBB4_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
impl W {
    ///Bit 0 - clear the illegal access flag for SYSCFG
    #[inline(always)]
    #[must_use]
    pub fn csyscfgf(&mut self) -> CSYSCFGF_W<0> {
        CSYSCFGF_W::new(self)
    }
    ///Bit 1 - clear the illegal access flag for RTC
    #[inline(always)]
    #[must_use]
    pub fn crtcf(&mut self) -> CRTCF_W<1> {
        CRTCF_W::new(self)
    }
    ///Bit 2 - clear the illegal access flag for TAMP
    #[inline(always)]
    #[must_use]
    pub fn ctampf(&mut self) -> CTAMPF_W<2> {
        CTAMPF_W::new(self)
    }
    ///Bit 3 - clear the illegal access flag for PWR
    #[inline(always)]
    #[must_use]
    pub fn cpwrf(&mut self) -> CPWRF_W<3> {
        CPWRF_W::new(self)
    }
    ///Bit 4 - clear the illegal access flag for RCC
    #[inline(always)]
    #[must_use]
    pub fn crccf(&mut self) -> CRCCF_W<4> {
        CRCCF_W::new(self)
    }
    ///Bit 5 - clear the illegal access flag for LPDMA
    #[inline(always)]
    #[must_use]
    pub fn clpdma1f(&mut self) -> CLPDMA1F_W<5> {
        CLPDMA1F_W::new(self)
    }
    ///Bit 6 - clear the illegal access flag for EXTI
    #[inline(always)]
    #[must_use]
    pub fn cextif(&mut self) -> CEXTIF_W<6> {
        CEXTIF_W::new(self)
    }
    ///Bit 14 - clear the illegal access flag for GTZC2 TZSC registers
    #[inline(always)]
    #[must_use]
    pub fn ctzsc2f(&mut self) -> CTZSC2F_W<14> {
        CTZSC2F_W::new(self)
    }
    ///Bit 15 - clear the illegal access flag for GTZC2 TZIC registers
    #[inline(always)]
    #[must_use]
    pub fn ctzic2f(&mut self) -> CTZIC2F_W<15> {
        CTZIC2F_W::new(self)
    }
    ///Bit 24 - clear the illegal access flag for SRAM4
    #[inline(always)]
    #[must_use]
    pub fn csram4f(&mut self) -> CSRAM4F_W<24> {
        CSRAM4F_W::new(self)
    }
    ///Bit 25 - clear the illegal access flag for MPCBB4 registers
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb4_regf(&mut self) -> CMPCBB4_REGF_W<25> {
        CMPCBB4_REGF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC flag clear register 2
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr2](index.html) module
pub struct FCR2_SPEC;
impl crate::RegisterSpec for FCR2_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [fcr2::W](W) writer structure
impl crate::Writable for FCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR2 to value 0
impl crate::Resettable for FCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
