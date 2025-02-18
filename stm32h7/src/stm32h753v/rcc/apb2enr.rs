///Register `APB2ENR` reader
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB2ENR` writer
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM1EN` reader - TIM1 peripheral clock enable
pub type TIM1EN_R = crate::BitReader<TIM1EN_A>;
///TIM1 peripheral clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::Disabled,
            true => TIM1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN_A::Enabled
    }
}
///Field `TIM1EN` writer - TIM1 peripheral clock enable
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, TIM1EN_A, O>;
impl<'a, const O: u8> TIM1EN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Enabled)
    }
}
///Field `TIM8EN` reader - TIM8 peripheral clock enable
pub use TIM1EN_R as TIM8EN_R;
///Field `USART1EN` reader - USART1 Peripheral Clocks Enable
pub use TIM1EN_R as USART1EN_R;
///Field `USART6EN` reader - USART6 Peripheral Clocks Enable
pub use TIM1EN_R as USART6EN_R;
///Field `SPI1EN` reader - SPI1 Peripheral Clocks Enable
pub use TIM1EN_R as SPI1EN_R;
///Field `SPI4EN` reader - SPI4 Peripheral Clocks Enable
pub use TIM1EN_R as SPI4EN_R;
///Field `TIM15EN` reader - TIM15 peripheral clock enable
pub use TIM1EN_R as TIM15EN_R;
///Field `TIM16EN` reader - TIM16 peripheral clock enable
pub use TIM1EN_R as TIM16EN_R;
///Field `TIM17EN` reader - TIM17 peripheral clock enable
pub use TIM1EN_R as TIM17EN_R;
///Field `SPI5EN` reader - SPI5 Peripheral Clocks Enable
pub use TIM1EN_R as SPI5EN_R;
///Field `SAI1EN` reader - SAI1 Peripheral Clocks Enable
pub use TIM1EN_R as SAI1EN_R;
///Field `SAI2EN` reader - SAI2 Peripheral Clocks Enable
pub use TIM1EN_R as SAI2EN_R;
///Field `SAI3EN` reader - SAI3 Peripheral Clocks Enable
pub use TIM1EN_R as SAI3EN_R;
///Field `DFSDM1EN` reader - DFSDM1 Peripheral Clocks Enable
pub use TIM1EN_R as DFSDM1EN_R;
///Field `HRTIMEN` reader - HRTIM peripheral clock enable
pub use TIM1EN_R as HRTIMEN_R;
///Field `TIM8EN` writer - TIM8 peripheral clock enable
pub use TIM1EN_W as TIM8EN_W;
///Field `USART1EN` writer - USART1 Peripheral Clocks Enable
pub use TIM1EN_W as USART1EN_W;
///Field `USART6EN` writer - USART6 Peripheral Clocks Enable
pub use TIM1EN_W as USART6EN_W;
///Field `SPI1EN` writer - SPI1 Peripheral Clocks Enable
pub use TIM1EN_W as SPI1EN_W;
///Field `SPI4EN` writer - SPI4 Peripheral Clocks Enable
pub use TIM1EN_W as SPI4EN_W;
///Field `TIM15EN` writer - TIM15 peripheral clock enable
pub use TIM1EN_W as TIM15EN_W;
///Field `TIM16EN` writer - TIM16 peripheral clock enable
pub use TIM1EN_W as TIM16EN_W;
///Field `TIM17EN` writer - TIM17 peripheral clock enable
pub use TIM1EN_W as TIM17EN_W;
///Field `SPI5EN` writer - SPI5 Peripheral Clocks Enable
pub use TIM1EN_W as SPI5EN_W;
///Field `SAI1EN` writer - SAI1 Peripheral Clocks Enable
pub use TIM1EN_W as SAI1EN_W;
///Field `SAI2EN` writer - SAI2 Peripheral Clocks Enable
pub use TIM1EN_W as SAI2EN_W;
///Field `SAI3EN` writer - SAI3 Peripheral Clocks Enable
pub use TIM1EN_W as SAI3EN_W;
///Field `DFSDM1EN` writer - DFSDM1 Peripheral Clocks Enable
pub use TIM1EN_W as DFSDM1EN_W;
///Field `HRTIMEN` writer - HRTIM peripheral clock enable
pub use TIM1EN_W as HRTIMEN_W;
impl R {
    ///Bit 0 - TIM1 peripheral clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 peripheral clock enable
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 Peripheral Clocks Enable
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - SPI1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 Peripheral Clocks Enable
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - TIM15 peripheral clock enable
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 peripheral clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 peripheral clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - SPI5 Peripheral Clocks Enable
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SAI1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SAI2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SAI3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn sai3en(&self) -> SAI3EN_R {
        SAI3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - DFSDM1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn dfsdm1en(&self) -> DFSDM1EN_R {
        DFSDM1EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - HRTIM peripheral clock enable
    #[inline(always)]
    pub fn hrtimen(&self) -> HRTIMEN_R {
        HRTIMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM1 peripheral clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<0> {
        TIM1EN_W::new(self)
    }
    ///Bit 1 - TIM8 peripheral clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<1> {
        TIM8EN_W::new(self)
    }
    ///Bit 4 - USART1 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<4> {
        USART1EN_W::new(self)
    }
    ///Bit 5 - USART6 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<5> {
        USART6EN_W::new(self)
    }
    ///Bit 12 - SPI1 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    ///Bit 13 - SPI4 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<13> {
        SPI4EN_W::new(self)
    }
    ///Bit 16 - TIM15 peripheral clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<16> {
        TIM15EN_W::new(self)
    }
    ///Bit 17 - TIM16 peripheral clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    ///Bit 18 - TIM17 peripheral clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    ///Bit 20 - SPI5 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<20> {
        SPI5EN_W::new(self)
    }
    ///Bit 22 - SAI1 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<22> {
        SAI1EN_W::new(self)
    }
    ///Bit 23 - SAI2 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<23> {
        SAI2EN_W::new(self)
    }
    ///Bit 24 - SAI3 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn sai3en(&mut self) -> SAI3EN_W<24> {
        SAI3EN_W::new(self)
    }
    ///Bit 28 - DFSDM1 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1en(&mut self) -> DFSDM1EN_W<28> {
        DFSDM1EN_W::new(self)
    }
    ///Bit 29 - HRTIM peripheral clock enable
    #[inline(always)]
    #[must_use]
    pub fn hrtimen(&mut self) -> HRTIMEN_W<29> {
        HRTIMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB2 Clock Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2enr](index.html) module
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb2enr::R](R) reader structure
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb2enr::W](W) writer structure
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
