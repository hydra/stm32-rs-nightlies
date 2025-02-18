///Register `PRIVCFGR2` reader
pub struct R(crate::R<PRIVCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRIVCFGR2` writer
pub struct W(crate::W<PRIVCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR2_SPEC>;
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
impl From<crate::W<PRIVCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM8PRIV` reader - TIM8PRIV
pub type TIM8PRIV_R = crate::BitReader<bool>;
///Field `TIM8PRIV` writer - TIM8PRIV
pub type TIM8PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `USART1PRIV` reader - USART1PRIV
pub type USART1PRIV_R = crate::BitReader<bool>;
///Field `USART1PRIV` writer - USART1PRIV
pub type USART1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `TIM15PRIV` reader - TIM15PRIV
pub type TIM15PRIV_R = crate::BitReader<bool>;
///Field `TIM15PRIV` writer - TIM15PRIV
pub type TIM15PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `TIM16PRIV` reader - TIM16PRIV
pub type TIM16PRIV_R = crate::BitReader<bool>;
///Field `TIM16PRIV` writer - TIM16PRIV
pub type TIM16PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `TIM17PRIV` reader - TIM17PRIV
pub type TIM17PRIV_R = crate::BitReader<bool>;
///Field `TIM17PRIV` writer - TIM17PRIV
pub type TIM17PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `SAI1PRIV` reader - SAI1PRIV
pub type SAI1PRIV_R = crate::BitReader<bool>;
///Field `SAI1PRIV` writer - SAI1PRIV
pub type SAI1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `SAI2PRIV` reader - SAI2PRIV
pub type SAI2PRIV_R = crate::BitReader<bool>;
///Field `SAI2PRIV` writer - SAI2PRIV
pub type SAI2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `DFSDM1PRIV` reader - DFSDM1PRIV
pub type DFSDM1PRIV_R = crate::BitReader<bool>;
///Field `DFSDM1PRIV` writer - DFSDM1PRIV
pub type DFSDM1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `CRCPRIV` reader - CRCPRIV
pub type CRCPRIV_R = crate::BitReader<bool>;
///Field `CRCPRIV` writer - CRCPRIV
pub type CRCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `TSCPRIV` reader - TSCPRIV
pub type TSCPRIV_R = crate::BitReader<bool>;
///Field `TSCPRIV` writer - TSCPRIV
pub type TSCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `ICACHEPRIV` reader - ICACHEPRIV
pub type ICACHEPRIV_R = crate::BitReader<bool>;
///Field `ICACHEPRIV` writer - ICACHEPRIV
pub type ICACHEPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `ADCPRIV` reader - ADCPRIV
pub type ADCPRIV_R = crate::BitReader<bool>;
///Field `ADCPRIV` writer - ADCPRIV
pub type ADCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `AESPRIV` reader - AESPRIV
pub type AESPRIV_R = crate::BitReader<bool>;
///Field `AESPRIV` writer - AESPRIV
pub type AESPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `HASHPRIV` reader - HASHPRIV
pub type HASHPRIV_R = crate::BitReader<bool>;
///Field `HASHPRIV` writer - HASHPRIV
pub type HASHPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `RNGPRIV` reader - RNGPRIV
pub type RNGPRIV_R = crate::BitReader<bool>;
///Field `RNGPRIV` writer - RNGPRIV
pub type RNGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `PKAPRIV` reader - PKAPRIV
pub type PKAPRIV_R = crate::BitReader<bool>;
///Field `PKAPRIV` writer - PKAPRIV
pub type PKAPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `SDMMC1PRIV` reader - SDMMC1PRIV
pub type SDMMC1PRIV_R = crate::BitReader<bool>;
///Field `SDMMC1PRIV` writer - SDMMC1PRIV
pub type SDMMC1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `FSMC_REGPRIV` reader - FSMC_REGPRIV
pub type FSMC_REGPRIV_R = crate::BitReader<bool>;
///Field `FSMC_REGPRIV` writer - FSMC_REGPRIV
pub type FSMC_REGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
///Field `OCTOSPI1_REGPRIV` reader - OCTOSPI1_REGRIV
pub type OCTOSPI1_REGPRIV_R = crate::BitReader<bool>;
///Field `OCTOSPI1_REGPRIV` writer - OCTOSPI1_REGRIV
pub type OCTOSPI1_REGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM8PRIV
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART1PRIV
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM15PRIV
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM16PRIV
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM17PRIV
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SAI1PRIV
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SAI2PRIV
    #[inline(always)]
    pub fn sai2priv(&self) -> SAI2PRIV_R {
        SAI2PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DFSDM1PRIV
    #[inline(always)]
    pub fn dfsdm1priv(&self) -> DFSDM1PRIV_R {
        DFSDM1PRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CRCPRIV
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TSCPRIV
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ICACHEPRIV
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ADCPRIV
    #[inline(always)]
    pub fn adcpriv(&self) -> ADCPRIV_R {
        ADCPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AESPRIV
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HASHPRIV
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RNGPRIV
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PKAPRIV
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SDMMC1PRIV
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FSMC_REGPRIV
    #[inline(always)]
    pub fn fsmc_regpriv(&self) -> FSMC_REGPRIV_R {
        FSMC_REGPRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - OCTOSPI1_REGRIV
    #[inline(always)]
    pub fn octospi1_regpriv(&self) -> OCTOSPI1_REGPRIV_R {
        OCTOSPI1_REGPRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM8PRIV
    #[inline(always)]
    #[must_use]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W<0> {
        TIM8PRIV_W::new(self)
    }
    ///Bit 1 - USART1PRIV
    #[inline(always)]
    #[must_use]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<1> {
        USART1PRIV_W::new(self)
    }
    ///Bit 2 - TIM15PRIV
    #[inline(always)]
    #[must_use]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W<2> {
        TIM15PRIV_W::new(self)
    }
    ///Bit 3 - TIM16PRIV
    #[inline(always)]
    #[must_use]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W<3> {
        TIM16PRIV_W::new(self)
    }
    ///Bit 4 - TIM17PRIV
    #[inline(always)]
    #[must_use]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W<4> {
        TIM17PRIV_W::new(self)
    }
    ///Bit 5 - SAI1PRIV
    #[inline(always)]
    #[must_use]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W<5> {
        SAI1PRIV_W::new(self)
    }
    ///Bit 6 - SAI2PRIV
    #[inline(always)]
    #[must_use]
    pub fn sai2priv(&mut self) -> SAI2PRIV_W<6> {
        SAI2PRIV_W::new(self)
    }
    ///Bit 7 - DFSDM1PRIV
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1priv(&mut self) -> DFSDM1PRIV_W<7> {
        DFSDM1PRIV_W::new(self)
    }
    ///Bit 8 - CRCPRIV
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<8> {
        CRCPRIV_W::new(self)
    }
    ///Bit 9 - TSCPRIV
    #[inline(always)]
    #[must_use]
    pub fn tscpriv(&mut self) -> TSCPRIV_W<9> {
        TSCPRIV_W::new(self)
    }
    ///Bit 10 - ICACHEPRIV
    #[inline(always)]
    #[must_use]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W<10> {
        ICACHEPRIV_W::new(self)
    }
    ///Bit 11 - ADCPRIV
    #[inline(always)]
    #[must_use]
    pub fn adcpriv(&mut self) -> ADCPRIV_W<11> {
        ADCPRIV_W::new(self)
    }
    ///Bit 12 - AESPRIV
    #[inline(always)]
    #[must_use]
    pub fn aespriv(&mut self) -> AESPRIV_W<12> {
        AESPRIV_W::new(self)
    }
    ///Bit 13 - HASHPRIV
    #[inline(always)]
    #[must_use]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<13> {
        HASHPRIV_W::new(self)
    }
    ///Bit 14 - RNGPRIV
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<14> {
        RNGPRIV_W::new(self)
    }
    ///Bit 15 - PKAPRIV
    #[inline(always)]
    #[must_use]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<15> {
        PKAPRIV_W::new(self)
    }
    ///Bit 16 - SDMMC1PRIV
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<16> {
        SDMMC1PRIV_W::new(self)
    }
    ///Bit 17 - FSMC_REGPRIV
    #[inline(always)]
    #[must_use]
    pub fn fsmc_regpriv(&mut self) -> FSMC_REGPRIV_W<17> {
        FSMC_REGPRIV_W::new(self)
    }
    ///Bit 18 - OCTOSPI1_REGRIV
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regpriv(&mut self) -> OCTOSPI1_REGPRIV_W<18> {
        OCTOSPI1_REGPRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC privilege configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [privcfgr2](index.html) module
pub struct PRIVCFGR2_SPEC;
impl crate::RegisterSpec for PRIVCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [privcfgr2::R](R) reader structure
impl crate::Readable for PRIVCFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [privcfgr2::W](W) writer structure
impl crate::Writable for PRIVCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRIVCFGR2 to value 0
impl crate::Resettable for PRIVCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
