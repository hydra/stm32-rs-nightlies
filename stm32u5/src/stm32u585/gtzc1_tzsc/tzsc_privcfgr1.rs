///Register `TZSC_PRIVCFGR1` reader
pub struct R(crate::R<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_PRIVCFGR1` writer
pub struct W(crate::W<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_PRIVCFGR1_SPEC>;
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
impl From<crate::W<TZSC_PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2PRIV` reader - privileged access mode for TIM2
pub type TIM2PRIV_R = crate::BitReader<bool>;
///Field `TIM2PRIV` writer - privileged access mode for TIM2
pub type TIM2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `TIM3PRIV` reader - privileged access mode for TIM3
pub type TIM3PRIV_R = crate::BitReader<bool>;
///Field `TIM3PRIV` writer - privileged access mode for TIM3
pub type TIM3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `TIM4PRIV` reader - privileged access mode for TIM4
pub type TIM4PRIV_R = crate::BitReader<bool>;
///Field `TIM4PRIV` writer - privileged access mode for TIM4
pub type TIM4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `TIM5PRIV` reader - privileged access mode for TIM5
pub type TIM5PRIV_R = crate::BitReader<bool>;
///Field `TIM5PRIV` writer - privileged access mode for TIM5
pub type TIM5PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `TIM6PRIV` reader - privileged access mode for TIM6
pub type TIM6PRIV_R = crate::BitReader<bool>;
///Field `TIM6PRIV` writer - privileged access mode for TIM6
pub type TIM6PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `TIM7PRIV` reader - privileged access mode for TIM7
pub type TIM7PRIV_R = crate::BitReader<bool>;
///Field `TIM7PRIV` writer - privileged access mode for TIM7
pub type TIM7PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `WWDGPRIV` reader - privileged access mode for WWDG
pub type WWDGPRIV_R = crate::BitReader<bool>;
///Field `WWDGPRIV` writer - privileged access mode for WWDG
pub type WWDGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `IWDGPRIV` reader - privileged access mode for IWDG
pub type IWDGPRIV_R = crate::BitReader<bool>;
///Field `IWDGPRIV` writer - privileged access mode for IWDG
pub type IWDGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `SPI2PRIV` reader - privileged access mode for SPI2
pub type SPI2PRIV_R = crate::BitReader<bool>;
///Field `SPI2PRIV` writer - privileged access mode for SPI2
pub type SPI2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `USART2PRIV` reader - privileged access mode for USART2
pub type USART2PRIV_R = crate::BitReader<bool>;
///Field `USART2PRIV` writer - privileged access mode for USART2
pub type USART2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `USART3PRIV` reader - privileged access mode for USART3
pub type USART3PRIV_R = crate::BitReader<bool>;
///Field `USART3PRIV` writer - privileged access mode for USART3
pub type USART3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `UART4PRIV` reader - privileged access mode for UART4
pub type UART4PRIV_R = crate::BitReader<bool>;
///Field `UART4PRIV` writer - privileged access mode for UART4
pub type UART4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `UART5PRIV` reader - privileged access mode for UART5
pub type UART5PRIV_R = crate::BitReader<bool>;
///Field `UART5PRIV` writer - privileged access mode for UART5
pub type UART5PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `I2C1PRIV` reader - privileged access mode for I2C1
pub type I2C1PRIV_R = crate::BitReader<bool>;
///Field `I2C1PRIV` writer - privileged access mode for I2C1
pub type I2C1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `I2C2PRIV` reader - privileged access mode for I2C2
pub type I2C2PRIV_R = crate::BitReader<bool>;
///Field `I2C2PRIV` writer - privileged access mode for I2C2
pub type I2C2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `CRSPRIV` reader - privileged access mode for CRS
pub type CRSPRIV_R = crate::BitReader<bool>;
///Field `CRSPRIV` writer - privileged access mode for CRS
pub type CRSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `I2C4PRIV` reader - privileged access mode for I2C4
pub type I2C4PRIV_R = crate::BitReader<bool>;
///Field `I2C4PRIV` writer - privileged access mode for I2C4
pub type I2C4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `LPTIM2PRIV` reader - privileged access mode for LPTIM2
pub type LPTIM2PRIV_R = crate::BitReader<bool>;
///Field `LPTIM2PRIV` writer - privileged access mode for LPTIM2
pub type LPTIM2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `FDCAN1PRIV` reader - privileged access mode for FDCAN1
pub type FDCAN1PRIV_R = crate::BitReader<bool>;
///Field `FDCAN1PRIV` writer - privileged access mode for FDCAN1
pub type FDCAN1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
///Field `UCPD1PRIV` reader - privileged access mode for UCPD1
pub type UCPD1PRIV_R = crate::BitReader<bool>;
///Field `UCPD1PRIV` writer - privileged access mode for UCPD1
pub type UCPD1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZSC_PRIVCFGR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - privileged access mode for TIM2
    #[inline(always)]
    pub fn tim2priv(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for TIM3
    #[inline(always)]
    pub fn tim3priv(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for TIM4
    #[inline(always)]
    pub fn tim4priv(&self) -> TIM4PRIV_R {
        TIM4PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for TIM5
    #[inline(always)]
    pub fn tim5priv(&self) -> TIM5PRIV_R {
        TIM5PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for TIM6
    #[inline(always)]
    pub fn tim6priv(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for TIM7
    #[inline(always)]
    pub fn tim7priv(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for WWDG
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for IWDG
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for SPI2
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for USART2
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - privileged access mode for USART3
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for UART4
    #[inline(always)]
    pub fn uart4priv(&self) -> UART4PRIV_R {
        UART4PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for UART5
    #[inline(always)]
    pub fn uart5priv(&self) -> UART5PRIV_R {
        UART5PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - privileged access mode for I2C1
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - privileged access mode for I2C2
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - privileged access mode for CRS
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - privileged access mode for I2C4
    #[inline(always)]
    pub fn i2c4priv(&self) -> I2C4PRIV_R {
        I2C4PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - privileged access mode for LPTIM2
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - privileged access mode for FDCAN1
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - privileged access mode for UCPD1
    #[inline(always)]
    pub fn ucpd1priv(&self) -> UCPD1PRIV_R {
        UCPD1PRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - privileged access mode for TIM2
    #[inline(always)]
    #[must_use]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W<0> {
        TIM2PRIV_W::new(self)
    }
    ///Bit 1 - privileged access mode for TIM3
    #[inline(always)]
    #[must_use]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W<1> {
        TIM3PRIV_W::new(self)
    }
    ///Bit 2 - privileged access mode for TIM4
    #[inline(always)]
    #[must_use]
    pub fn tim4priv(&mut self) -> TIM4PRIV_W<2> {
        TIM4PRIV_W::new(self)
    }
    ///Bit 3 - privileged access mode for TIM5
    #[inline(always)]
    #[must_use]
    pub fn tim5priv(&mut self) -> TIM5PRIV_W<3> {
        TIM5PRIV_W::new(self)
    }
    ///Bit 4 - privileged access mode for TIM6
    #[inline(always)]
    #[must_use]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W<4> {
        TIM6PRIV_W::new(self)
    }
    ///Bit 5 - privileged access mode for TIM7
    #[inline(always)]
    #[must_use]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W<5> {
        TIM7PRIV_W::new(self)
    }
    ///Bit 6 - privileged access mode for WWDG
    #[inline(always)]
    #[must_use]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W<6> {
        WWDGPRIV_W::new(self)
    }
    ///Bit 7 - privileged access mode for IWDG
    #[inline(always)]
    #[must_use]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W<7> {
        IWDGPRIV_W::new(self)
    }
    ///Bit 8 - privileged access mode for SPI2
    #[inline(always)]
    #[must_use]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W<8> {
        SPI2PRIV_W::new(self)
    }
    ///Bit 9 - privileged access mode for USART2
    #[inline(always)]
    #[must_use]
    pub fn usart2priv(&mut self) -> USART2PRIV_W<9> {
        USART2PRIV_W::new(self)
    }
    ///Bit 10 - privileged access mode for USART3
    #[inline(always)]
    #[must_use]
    pub fn usart3priv(&mut self) -> USART3PRIV_W<10> {
        USART3PRIV_W::new(self)
    }
    ///Bit 11 - privileged access mode for UART4
    #[inline(always)]
    #[must_use]
    pub fn uart4priv(&mut self) -> UART4PRIV_W<11> {
        UART4PRIV_W::new(self)
    }
    ///Bit 12 - privileged access mode for UART5
    #[inline(always)]
    #[must_use]
    pub fn uart5priv(&mut self) -> UART5PRIV_W<12> {
        UART5PRIV_W::new(self)
    }
    ///Bit 13 - privileged access mode for I2C1
    #[inline(always)]
    #[must_use]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W<13> {
        I2C1PRIV_W::new(self)
    }
    ///Bit 14 - privileged access mode for I2C2
    #[inline(always)]
    #[must_use]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W<14> {
        I2C2PRIV_W::new(self)
    }
    ///Bit 15 - privileged access mode for CRS
    #[inline(always)]
    #[must_use]
    pub fn crspriv(&mut self) -> CRSPRIV_W<15> {
        CRSPRIV_W::new(self)
    }
    ///Bit 16 - privileged access mode for I2C4
    #[inline(always)]
    #[must_use]
    pub fn i2c4priv(&mut self) -> I2C4PRIV_W<16> {
        I2C4PRIV_W::new(self)
    }
    ///Bit 17 - privileged access mode for LPTIM2
    #[inline(always)]
    #[must_use]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W<17> {
        LPTIM2PRIV_W::new(self)
    }
    ///Bit 18 - privileged access mode for FDCAN1
    #[inline(always)]
    #[must_use]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W<18> {
        FDCAN1PRIV_W::new(self)
    }
    ///Bit 19 - privileged access mode for UCPD1
    #[inline(always)]
    #[must_use]
    pub fn ucpd1priv(&mut self) -> UCPD1PRIV_W<19> {
        UCPD1PRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC privilege configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_privcfgr1](index.html) module
pub struct TZSC_PRIVCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_privcfgr1::R](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_privcfgr1::W](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_PRIVCFGR1 to value 0
impl crate::Resettable for TZSC_PRIVCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
