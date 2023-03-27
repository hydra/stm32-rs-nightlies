///Register `RCC_APB2RSTR` reader
pub struct R(crate::R<RCC_APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB2RSTR` writer
pub struct W(crate::W<RCC_APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2RSTR_SPEC>;
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
impl From<crate::W<RCC_APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM1RST` reader - TIM1 reset Set and cleared by software.
pub type TIM1RST_R = crate::BitReader<bool>;
///Field `TIM1RST` writer - TIM1 reset Set and cleared by software.
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `SPI1RST` reader - SPI1 reset Set and cleared by software.
pub type SPI1RST_R = crate::BitReader<bool>;
///Field `SPI1RST` writer - SPI1 reset Set and cleared by software.
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `TIM8RST` reader - TIM8 reset Set and cleared by software.
pub type TIM8RST_R = crate::BitReader<bool>;
///Field `TIM8RST` writer - TIM8 reset Set and cleared by software.
pub type TIM8RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `USART1RST` reader - USART1 reset Set and cleared by software.
pub type USART1RST_R = crate::BitReader<bool>;
///Field `USART1RST` writer - USART1 reset Set and cleared by software.
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `TIM15RST` reader - TIM15 reset Set and cleared by software.
pub type TIM15RST_R = crate::BitReader<bool>;
///Field `TIM15RST` writer - TIM15 reset Set and cleared by software.
pub type TIM15RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `TIM16RST` reader - TIM16 reset Set and cleared by software.
pub type TIM16RST_R = crate::BitReader<bool>;
///Field `TIM16RST` writer - TIM16 reset Set and cleared by software.
pub type TIM16RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `TIM17RST` reader - TIM17 reset Set and cleared by software.
pub type TIM17RST_R = crate::BitReader<bool>;
///Field `TIM17RST` writer - TIM17 reset Set and cleared by software.
pub type TIM17RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `SAI1RST` reader - SAI1 reset Set and cleared by software.
pub type SAI1RST_R = crate::BitReader<bool>;
///Field `SAI1RST` writer - SAI1 reset Set and cleared by software.
pub type SAI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
///Field `SAI2RST` reader - SAI2 reset Set and cleared by software.
pub type SAI2RST_R = crate::BitReader<bool>;
///Field `SAI2RST` writer - SAI2 reset Set and cleared by software.
pub type SAI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTR_SPEC, bool, O>;
impl R {
    ///Bit 11 - TIM1 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 reset Set and cleared by software.
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1 reset Set and cleared by software.
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 reset Set and cleared by software.
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 11 - TIM1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    ///Bit 12 - SPI1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    ///Bit 13 - TIM8 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<13> {
        TIM8RST_W::new(self)
    }
    ///Bit 14 - USART1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    ///Bit 16 - TIM15 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    ///Bit 17 - TIM16 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    ///Bit 18 - TIM17 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    ///Bit 21 - SAI1 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<21> {
        SAI1RST_W::new(self)
    }
    ///Bit 22 - SAI2 reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<22> {
        SAI2RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb2rstr](index.html) module
pub struct RCC_APB2RSTR_SPEC;
impl crate::RegisterSpec for RCC_APB2RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb2rstr::R](R) reader structure
impl crate::Readable for RCC_APB2RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb2rstr::W](W) writer structure
impl crate::Writable for RCC_APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB2RSTR to value 0
impl crate::Resettable for RCC_APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
