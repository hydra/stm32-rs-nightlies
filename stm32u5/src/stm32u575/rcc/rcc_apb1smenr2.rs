///Register `RCC_APB1SMENR2` reader
pub struct R(crate::R<RCC_APB1SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB1SMENR2` writer
pub struct W(crate::W<RCC_APB1SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1SMENR2_SPEC>;
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
impl From<crate::W<RCC_APB1SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C4SMEN_R = crate::BitReader<bool>;
///Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR2_SPEC, bool, O>;
///Field `LPTIM2SMEN` reader - LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM2SMEN_R = crate::BitReader<bool>;
///Field `LPTIM2SMEN` writer - LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR2_SPEC, bool, O>;
///Field `FDCAN1SMEN` reader - FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type FDCAN1SMEN_R = crate::BitReader<bool>;
///Field `FDCAN1SMEN` writer - FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type FDCAN1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR2_SPEC, bool, O>;
///Field `UCPD1SMEN` reader - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type UCPD1SMEN_R = crate::BitReader<bool>;
///Field `UCPD1SMEN` writer - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type UCPD1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1SMENR2_SPEC, bool, O>;
impl R {
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn fdcan1smen(&self) -> FDCAN1SMEN_R {
        FDCAN1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<1> {
        I2C4SMEN_W::new(self)
    }
    ///Bit 5 - LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<5> {
        LPTIM2SMEN_W::new(self)
    }
    ///Bit 9 - FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fdcan1smen(&mut self) -> FDCAN1SMEN_W<9> {
        FDCAN1SMEN_W::new(self)
    }
    ///Bit 23 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<23> {
        UCPD1SMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb1smenr2](index.html) module
pub struct RCC_APB1SMENR2_SPEC;
impl crate::RegisterSpec for RCC_APB1SMENR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb1smenr2::R](R) reader structure
impl crate::Readable for RCC_APB1SMENR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb1smenr2::W](W) writer structure
impl crate::Writable for RCC_APB1SMENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB1SMENR2 to value 0xffff_ffff
impl crate::Resettable for RCC_APB1SMENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
