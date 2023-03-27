///Register `RCC_MP_APB1LPENCLRR` reader
pub struct R(crate::R<RCC_MP_APB1LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB1LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB1LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB1LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_APB1LPENCLRR` writer
pub struct W(crate::W<RCC_MP_APB1LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB1LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_APB1LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB1LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2LPEN` reader - TIM2LPEN
pub type TIM2LPEN_R = crate::BitReader<bool>;
///Field `TIM2LPEN` writer - TIM2LPEN
pub type TIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM3LPEN` reader - TIM3LPEN
pub type TIM3LPEN_R = crate::BitReader<bool>;
///Field `TIM3LPEN` writer - TIM3LPEN
pub type TIM3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM4LPEN` reader - TIM4LPEN
pub type TIM4LPEN_R = crate::BitReader<bool>;
///Field `TIM4LPEN` writer - TIM4LPEN
pub type TIM4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM5LPEN` reader - TIM5LPEN
pub type TIM5LPEN_R = crate::BitReader<bool>;
///Field `TIM5LPEN` writer - TIM5LPEN
pub type TIM5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM6LPEN` reader - TIM6LPEN
pub type TIM6LPEN_R = crate::BitReader<bool>;
///Field `TIM6LPEN` writer - TIM6LPEN
pub type TIM6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM7LPEN` reader - TIM7LPEN
pub type TIM7LPEN_R = crate::BitReader<bool>;
///Field `TIM7LPEN` writer - TIM7LPEN
pub type TIM7LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM12LPEN` reader - TIM12LPEN
pub type TIM12LPEN_R = crate::BitReader<bool>;
///Field `TIM12LPEN` writer - TIM12LPEN
pub type TIM12LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM13LPEN` reader - TIM13LPEN
pub type TIM13LPEN_R = crate::BitReader<bool>;
///Field `TIM13LPEN` writer - TIM13LPEN
pub type TIM13LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `TIM14LPEN` reader - TIM14LPEN
pub type TIM14LPEN_R = crate::BitReader<bool>;
///Field `TIM14LPEN` writer - TIM14LPEN
pub type TIM14LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `LPTIM1LPEN` reader - LPTIM1LPEN
pub type LPTIM1LPEN_R = crate::BitReader<bool>;
///Field `LPTIM1LPEN` writer - LPTIM1LPEN
pub type LPTIM1LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `SPI2LPEN` reader - SPI2LPEN
pub type SPI2LPEN_R = crate::BitReader<bool>;
///Field `SPI2LPEN` writer - SPI2LPEN
pub type SPI2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `SPI3LPEN` reader - SPI3LPEN
pub type SPI3LPEN_R = crate::BitReader<bool>;
///Field `SPI3LPEN` writer - SPI3LPEN
pub type SPI3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `USART2LPEN` reader - USART2LPEN
pub type USART2LPEN_R = crate::BitReader<bool>;
///Field `USART2LPEN` writer - USART2LPEN
pub type USART2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `USART3LPEN` reader - USART3LPEN
pub type USART3LPEN_R = crate::BitReader<bool>;
///Field `USART3LPEN` writer - USART3LPEN
pub type USART3LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `UART4LPEN` reader - UART4LPEN
pub type UART4LPEN_R = crate::BitReader<bool>;
///Field `UART4LPEN` writer - UART4LPEN
pub type UART4LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `UART5LPEN` reader - UART5LPEN
pub type UART5LPEN_R = crate::BitReader<bool>;
///Field `UART5LPEN` writer - UART5LPEN
pub type UART5LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `UART7LPEN` reader - UART7LPEN
pub type UART7LPEN_R = crate::BitReader<bool>;
///Field `UART7LPEN` writer - UART7LPEN
pub type UART7LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `UART8LPEN` reader - UART8LPEN
pub type UART8LPEN_R = crate::BitReader<bool>;
///Field `UART8LPEN` writer - UART8LPEN
pub type UART8LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `I2C1LPEN` reader - I2C1LPEN
pub type I2C1LPEN_R = crate::BitReader<bool>;
///Field `I2C1LPEN` writer - I2C1LPEN
pub type I2C1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `I2C2LPEN` reader - I2C2LPEN
pub type I2C2LPEN_R = crate::BitReader<bool>;
///Field `I2C2LPEN` writer - I2C2LPEN
pub type I2C2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `I2C3LPEN` reader - I2C3LPEN
pub type I2C3LPEN_R = crate::BitReader<bool>;
///Field `I2C3LPEN` writer - I2C3LPEN
pub type I2C3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `I2C5LPEN` reader - I2C5LPEN
pub type I2C5LPEN_R = crate::BitReader<bool>;
///Field `I2C5LPEN` writer - I2C5LPEN
pub type I2C5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `SPDIFLPEN` reader - SPDIFLPEN
pub type SPDIFLPEN_R = crate::BitReader<bool>;
///Field `SPDIFLPEN` writer - SPDIFLPEN
pub type SPDIFLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `CECLPEN` reader - CECLPEN
pub type CECLPEN_R = crate::BitReader<bool>;
///Field `CECLPEN` writer - CECLPEN
pub type CECLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `DAC12LPEN` reader - DAC12LPEN
pub type DAC12LPEN_R = crate::BitReader<bool>;
///Field `DAC12LPEN` writer - DAC12LPEN
pub type DAC12LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
///Field `MDIOSLPEN` reader - MDIOSLPEN
pub type MDIOSLPEN_R = crate::BitReader<bool>;
///Field `MDIOSLPEN` writer - MDIOSLPEN
pub type MDIOSLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB1LPENCLRR_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2LPEN
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3LPEN
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4LPEN
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5LPEN
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6LPEN
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7LPEN
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM12LPEN
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM13LPEN
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM14LPEN
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPTIM1LPEN
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - SPI2LPEN
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI3LPEN
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART2LPEN
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - USART3LPEN
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - UART4LPEN
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - UART5LPEN
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - UART7LPEN
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART8LPEN
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - I2C1LPEN
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2LPEN
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3LPEN
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - I2C5LPEN
    #[inline(always)]
    pub fn i2c5lpen(&self) -> I2C5LPEN_R {
        I2C5LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - SPDIFLPEN
    #[inline(always)]
    pub fn spdiflpen(&self) -> SPDIFLPEN_R {
        SPDIFLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CECLPEN
    #[inline(always)]
    pub fn ceclpen(&self) -> CECLPEN_R {
        CECLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DAC12LPEN
    #[inline(always)]
    pub fn dac12lpen(&self) -> DAC12LPEN_R {
        DAC12LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - MDIOSLPEN
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<0> {
        TIM2LPEN_W::new(self)
    }
    ///Bit 1 - TIM3LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<1> {
        TIM3LPEN_W::new(self)
    }
    ///Bit 2 - TIM4LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<2> {
        TIM4LPEN_W::new(self)
    }
    ///Bit 3 - TIM5LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<3> {
        TIM5LPEN_W::new(self)
    }
    ///Bit 4 - TIM6LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<4> {
        TIM6LPEN_W::new(self)
    }
    ///Bit 5 - TIM7LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<5> {
        TIM7LPEN_W::new(self)
    }
    ///Bit 6 - TIM12LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W<6> {
        TIM12LPEN_W::new(self)
    }
    ///Bit 7 - TIM13LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W<7> {
        TIM13LPEN_W::new(self)
    }
    ///Bit 8 - TIM14LPEN
    #[inline(always)]
    #[must_use]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W<8> {
        TIM14LPEN_W::new(self)
    }
    ///Bit 9 - LPTIM1LPEN
    #[inline(always)]
    #[must_use]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<9> {
        LPTIM1LPEN_W::new(self)
    }
    ///Bit 11 - SPI2LPEN
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<11> {
        SPI2LPEN_W::new(self)
    }
    ///Bit 12 - SPI3LPEN
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<12> {
        SPI3LPEN_W::new(self)
    }
    ///Bit 14 - USART2LPEN
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<14> {
        USART2LPEN_W::new(self)
    }
    ///Bit 15 - USART3LPEN
    #[inline(always)]
    #[must_use]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<15> {
        USART3LPEN_W::new(self)
    }
    ///Bit 16 - UART4LPEN
    #[inline(always)]
    #[must_use]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<16> {
        UART4LPEN_W::new(self)
    }
    ///Bit 17 - UART5LPEN
    #[inline(always)]
    #[must_use]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<17> {
        UART5LPEN_W::new(self)
    }
    ///Bit 18 - UART7LPEN
    #[inline(always)]
    #[must_use]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W<18> {
        UART7LPEN_W::new(self)
    }
    ///Bit 19 - UART8LPEN
    #[inline(always)]
    #[must_use]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W<19> {
        UART8LPEN_W::new(self)
    }
    ///Bit 21 - I2C1LPEN
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<21> {
        I2C1LPEN_W::new(self)
    }
    ///Bit 22 - I2C2LPEN
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<22> {
        I2C2LPEN_W::new(self)
    }
    ///Bit 23 - I2C3LPEN
    #[inline(always)]
    #[must_use]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<23> {
        I2C3LPEN_W::new(self)
    }
    ///Bit 24 - I2C5LPEN
    #[inline(always)]
    #[must_use]
    pub fn i2c5lpen(&mut self) -> I2C5LPEN_W<24> {
        I2C5LPEN_W::new(self)
    }
    ///Bit 26 - SPDIFLPEN
    #[inline(always)]
    #[must_use]
    pub fn spdiflpen(&mut self) -> SPDIFLPEN_W<26> {
        SPDIFLPEN_W::new(self)
    }
    ///Bit 27 - CECLPEN
    #[inline(always)]
    #[must_use]
    pub fn ceclpen(&mut self) -> CECLPEN_W<27> {
        CECLPEN_W::new(self)
    }
    ///Bit 29 - DAC12LPEN
    #[inline(always)]
    #[must_use]
    pub fn dac12lpen(&mut self) -> DAC12LPEN_W<29> {
        DAC12LPEN_W::new(self)
    }
    ///Bit 31 - MDIOSLPEN
    #[inline(always)]
    #[must_use]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W<31> {
        MDIOSLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used by the MPU in order to clear the PERxLPEN bits .
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_apb1lpenclrr](index.html) module
pub struct RCC_MP_APB1LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB1LPENCLRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_apb1lpenclrr::R](R) reader structure
impl crate::Readable for RCC_MP_APB1LPENCLRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_apb1lpenclrr::W](W) writer structure
impl crate::Writable for RCC_MP_APB1LPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_APB1LPENCLRR to value 0xadef_dbff
impl crate::Resettable for RCC_MP_APB1LPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0xadef_dbff;
}
