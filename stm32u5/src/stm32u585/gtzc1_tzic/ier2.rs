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
///Field `TIM1IE` reader - illegal access interrupt enable for TIM1
pub type TIM1IE_R = crate::BitReader<bool>;
///Field `TIM1IE` writer - illegal access interrupt enable for TIM1
pub type TIM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `SPI1IE` reader - illegal access interrupt enable for SPI1
pub type SPI1IE_R = crate::BitReader<bool>;
///Field `SPI1IE` writer - illegal access interrupt enable for SPI1
pub type SPI1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `TIM8IE` reader - illegal access interrupt enable for TIM8
pub type TIM8IE_R = crate::BitReader<bool>;
///Field `TIM8IE` writer - illegal access interrupt enable for TIM8
pub type TIM8IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `USART1IE` reader - illegal access interrupt enable for USART1
pub type USART1IE_R = crate::BitReader<bool>;
///Field `USART1IE` writer - illegal access interrupt enable for USART1
pub type USART1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `TIM15IE` reader - illegal access interrupt enable for TIM5
pub type TIM15IE_R = crate::BitReader<bool>;
///Field `TIM15IE` writer - illegal access interrupt enable for TIM5
pub type TIM15IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `TIM16IE` reader - illegal access interrupt enable for TIM6
pub type TIM16IE_R = crate::BitReader<bool>;
///Field `TIM16IE` writer - illegal access interrupt enable for TIM6
pub type TIM16IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `TIM17IE` reader - illegal access interrupt enable for TIM7
pub type TIM17IE_R = crate::BitReader<bool>;
///Field `TIM17IE` writer - illegal access interrupt enable for TIM7
pub type TIM17IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `SAI1IE` reader - illegal access interrupt enable for SAI1
pub type SAI1IE_R = crate::BitReader<bool>;
///Field `SAI1IE` writer - illegal access interrupt enable for SAI1
pub type SAI1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
///Field `SAI2IE` reader - illegal access interrupt enable for SAI2
pub type SAI2IE_R = crate::BitReader<bool>;
///Field `SAI2IE` writer - illegal access interrupt enable for SAI2
pub type SAI2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
impl R {
    ///Bit 0 - illegal access interrupt enable for TIM1
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for SPI1
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for TIM8
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for USART1
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for TIM5
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for TIM6
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for TIM7
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for SAI1
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for SAI2
    #[inline(always)]
    pub fn sai2ie(&self) -> SAI2IE_R {
        SAI2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for TIM1
    #[inline(always)]
    #[must_use]
    pub fn tim1ie(&mut self) -> TIM1IE_W<0> {
        TIM1IE_W::new(self)
    }
    ///Bit 1 - illegal access interrupt enable for SPI1
    #[inline(always)]
    #[must_use]
    pub fn spi1ie(&mut self) -> SPI1IE_W<1> {
        SPI1IE_W::new(self)
    }
    ///Bit 2 - illegal access interrupt enable for TIM8
    #[inline(always)]
    #[must_use]
    pub fn tim8ie(&mut self) -> TIM8IE_W<2> {
        TIM8IE_W::new(self)
    }
    ///Bit 3 - illegal access interrupt enable for USART1
    #[inline(always)]
    #[must_use]
    pub fn usart1ie(&mut self) -> USART1IE_W<3> {
        USART1IE_W::new(self)
    }
    ///Bit 4 - illegal access interrupt enable for TIM5
    #[inline(always)]
    #[must_use]
    pub fn tim15ie(&mut self) -> TIM15IE_W<4> {
        TIM15IE_W::new(self)
    }
    ///Bit 5 - illegal access interrupt enable for TIM6
    #[inline(always)]
    #[must_use]
    pub fn tim16ie(&mut self) -> TIM16IE_W<5> {
        TIM16IE_W::new(self)
    }
    ///Bit 6 - illegal access interrupt enable for TIM7
    #[inline(always)]
    #[must_use]
    pub fn tim17ie(&mut self) -> TIM17IE_W<6> {
        TIM17IE_W::new(self)
    }
    ///Bit 7 - illegal access interrupt enable for SAI1
    #[inline(always)]
    #[must_use]
    pub fn sai1ie(&mut self) -> SAI1IE_W<7> {
        SAI1IE_W::new(self)
    }
    ///Bit 8 - illegal access interrupt enable for SAI2
    #[inline(always)]
    #[must_use]
    pub fn sai2ie(&mut self) -> SAI2IE_W<8> {
        SAI2IE_W::new(self)
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
