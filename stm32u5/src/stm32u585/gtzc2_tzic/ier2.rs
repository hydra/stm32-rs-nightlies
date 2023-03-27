///Register `IER2` reader
pub struct R(crate::R<IER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER2` writer
pub struct W(crate::W<IER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER2_SPEC>;
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
impl From<crate::W<IER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGIE` reader - illegal access interrupt enable for SYSCFG
pub type SYSCFGIE_R = crate::BitReader<bool>;
///Field `SYSCFGIE` writer - illegal access interrupt enable for SYSCFG
pub type SYSCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `RTCIE` reader - illegal access interrupt enable for RTC
pub type RTCIE_R = crate::BitReader<bool>;
///Field `RTCIE` writer - illegal access interrupt enable for RTC
pub type RTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `TAMPIE` reader - illegal access interrupt enable for TAMP
pub type TAMPIE_R = crate::BitReader<bool>;
///Field `TAMPIE` writer - illegal access interrupt enable for TAMP
pub type TAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `PWRIE` reader - illegal access interrupt enable for PWR
pub type PWRIE_R = crate::BitReader<bool>;
///Field `PWRIE` writer - illegal access interrupt enable for PWR
pub type PWRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `RCCIE` reader - illegal access interrupt enable for RCC
pub type RCCIE_R = crate::BitReader<bool>;
///Field `RCCIE` writer - illegal access interrupt enable for RCC
pub type RCCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `LPDMA1IE` reader - illegal access interrupt enable for LPDMA
pub type LPDMA1IE_R = crate::BitReader<bool>;
///Field `LPDMA1IE` writer - illegal access interrupt enable for LPDMA
pub type LPDMA1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `EXTIIE` reader - illegal access interrupt enable for EXTI
pub type EXTIIE_R = crate::BitReader<bool>;
///Field `EXTIIE` writer - illegal access interrupt enable for EXTI
pub type EXTIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `TZSC2IE` reader - illegal access interrupt enable for GTZC2 TZSC registers
pub type TZSC2IE_R = crate::BitReader<bool>;
///Field `TZSC2IE` writer - illegal access interrupt enable for GTZC2 TZSC registers
pub type TZSC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `TZIC2IE` reader - illegal access interrupt enable for GTZC2 TZIC registers
pub type TZIC2IE_R = crate::BitReader<bool>;
///Field `TZIC2IE` writer - illegal access interrupt enable for GTZC2 TZIC registers
pub type TZIC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `SRAM4IE` reader - illegal access interrupt enable for SRAM4
pub type SRAM4IE_R = crate::BitReader<bool>;
///Field `SRAM4IE` writer - illegal access interrupt enable for SRAM4
pub type SRAM4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `MPCBB4_REGIE` reader - illegal access interrupt enable for MPCBB4 registers
pub type MPCBB4_REGIE_R = crate::BitReader<bool>;
///Field `MPCBB4_REGIE` writer - illegal access interrupt enable for MPCBB4 registers
pub type MPCBB4_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
impl R {
    ///Bit 0 - illegal access interrupt enable for SYSCFG
    #[inline(always)]
    pub fn syscfgie(&self) -> SYSCFGIE_R {
        SYSCFGIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for RTC
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for TAMP
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for PWR
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for RCC
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for LPDMA
    #[inline(always)]
    pub fn lpdma1ie(&self) -> LPDMA1IE_R {
        LPDMA1IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for EXTI
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC2 TZSC registers
    #[inline(always)]
    pub fn tzsc2ie(&self) -> TZSC2IE_R {
        TZSC2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC2 TZIC registers
    #[inline(always)]
    pub fn tzic2ie(&self) -> TZIC2IE_R {
        TZIC2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM4
    #[inline(always)]
    pub fn sram4ie(&self) -> SRAM4IE_R {
        SRAM4IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB4 registers
    #[inline(always)]
    pub fn mpcbb4_regie(&self) -> MPCBB4_REGIE_R {
        MPCBB4_REGIE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for SYSCFG
    #[inline(always)]
    #[must_use]
    pub fn syscfgie(&mut self) -> SYSCFGIE_W<0> {
        SYSCFGIE_W::new(self)
    }
    ///Bit 1 - illegal access interrupt enable for RTC
    #[inline(always)]
    #[must_use]
    pub fn rtcie(&mut self) -> RTCIE_W<1> {
        RTCIE_W::new(self)
    }
    ///Bit 2 - illegal access interrupt enable for TAMP
    #[inline(always)]
    #[must_use]
    pub fn tampie(&mut self) -> TAMPIE_W<2> {
        TAMPIE_W::new(self)
    }
    ///Bit 3 - illegal access interrupt enable for PWR
    #[inline(always)]
    #[must_use]
    pub fn pwrie(&mut self) -> PWRIE_W<3> {
        PWRIE_W::new(self)
    }
    ///Bit 4 - illegal access interrupt enable for RCC
    #[inline(always)]
    #[must_use]
    pub fn rccie(&mut self) -> RCCIE_W<4> {
        RCCIE_W::new(self)
    }
    ///Bit 5 - illegal access interrupt enable for LPDMA
    #[inline(always)]
    #[must_use]
    pub fn lpdma1ie(&mut self) -> LPDMA1IE_W<5> {
        LPDMA1IE_W::new(self)
    }
    ///Bit 6 - illegal access interrupt enable for EXTI
    #[inline(always)]
    #[must_use]
    pub fn extiie(&mut self) -> EXTIIE_W<6> {
        EXTIIE_W::new(self)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC2 TZSC registers
    #[inline(always)]
    #[must_use]
    pub fn tzsc2ie(&mut self) -> TZSC2IE_W<14> {
        TZSC2IE_W::new(self)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC2 TZIC registers
    #[inline(always)]
    #[must_use]
    pub fn tzic2ie(&mut self) -> TZIC2IE_W<15> {
        TZIC2IE_W::new(self)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM4
    #[inline(always)]
    #[must_use]
    pub fn sram4ie(&mut self) -> SRAM4IE_W<24> {
        SRAM4IE_W::new(self)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB4 registers
    #[inline(always)]
    #[must_use]
    pub fn mpcbb4_regie(&mut self) -> MPCBB4_REGIE_W<25> {
        MPCBB4_REGIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt enable register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier2](index.html) module
pub struct IER2_SPEC;
impl crate::RegisterSpec for IER2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier2::R](R) reader structure
impl crate::Readable for IER2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier2::W](W) writer structure
impl crate::Writable for IER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER2 to value 0
impl crate::Resettable for IER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
