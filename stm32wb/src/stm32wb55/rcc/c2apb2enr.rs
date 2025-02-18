///Register `C2APB2ENR` reader
pub struct R(crate::R<C2APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2APB2ENR` writer
pub struct W(crate::W<C2APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB2ENR_SPEC>;
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
impl From<crate::W<C2APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM1EN` reader - CPU2 TIM1 timer clock enable
pub type TIM1EN_R = crate::BitReader<bool>;
///Field `TIM1EN` writer - CPU2 TIM1 timer clock enable
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2ENR_SPEC, bool, O>;
///Field `SPI1EN` reader - CPU2 SPI1 clock enable
pub type SPI1EN_R = crate::BitReader<bool>;
///Field `SPI1EN` writer - CPU2 SPI1 clock enable
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2ENR_SPEC, bool, O>;
///Field `USART1EN` reader - CPU2 USART1clock enable
pub type USART1EN_R = crate::BitReader<bool>;
///Field `USART1EN` writer - CPU2 USART1clock enable
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2ENR_SPEC, bool, O>;
///Field `TIM16EN` reader - CPU2 TIM16 timer clock enable
pub type TIM16EN_R = crate::BitReader<bool>;
///Field `TIM16EN` writer - CPU2 TIM16 timer clock enable
pub type TIM16EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2ENR_SPEC, bool, O>;
///Field `TIM17EN` reader - CPU2 TIM17 timer clock enable
pub type TIM17EN_R = crate::BitReader<bool>;
///Field `TIM17EN` writer - CPU2 TIM17 timer clock enable
pub type TIM17EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2ENR_SPEC, bool, O>;
///Field `SAI1EN` reader - CPU2 SAI1 clock enable
pub type SAI1EN_R = crate::BitReader<bool>;
///Field `SAI1EN` writer - CPU2 SAI1 clock enable
pub type SAI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2ENR_SPEC, bool, O>;
impl R {
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - CPU2 USART1clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - CPU2 SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    ///Bit 14 - CPU2 USART1clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    ///Bit 21 - CPU2 SAI1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<21> {
        SAI1EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 APB2ENR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2apb2enr](index.html) module
pub struct C2APB2ENR_SPEC;
impl crate::RegisterSpec for C2APB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2apb2enr::R](R) reader structure
impl crate::Readable for C2APB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2apb2enr::W](W) writer structure
impl crate::Writable for C2APB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2APB2ENR to value 0
impl crate::Resettable for C2APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
