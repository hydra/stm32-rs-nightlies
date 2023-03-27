///Register `FCR1` writer
pub struct W(crate::W<FCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR1_SPEC>;
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
impl From<crate::W<FCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTIM2F` writer - clear the illegal access flag for TIM2
pub type CTIM2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CTIM3F` writer - clear the illegal access flag for TIM3
pub type CTIM3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CTIM4F` writer - clear the illegal access flag for TIM4
pub type CTIM4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CTIM5F` writer - clear the illegal access flag for TIM5
pub type CTIM5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CTIM6F` writer - clear the illegal access flag for TIM6
pub type CTIM6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CTIM7F` writer - clear the illegal access flag for TIM7
pub type CTIM7F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CWWDGF` writer - clear the illegal access flag for WWDG
pub type CWWDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CIWDGF` writer - clear the illegal access flag for IWDG
pub type CIWDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CSPI2F` writer - clear the illegal access flag for SPI2
pub type CSPI2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CUSART2F` writer - clear the illegal access flag for USART2
pub type CUSART2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CUSART3F` writer - clear the illegal access flag for USART3
pub type CUSART3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CUART4F` writer - clear the illegal access flag for UART4
pub type CUART4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CUART5F` writer - clear the illegal access flag for UART5
pub type CUART5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CI2C1F` writer - clear the illegal access flag for I2C1
pub type CI2C1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CI2C2F` writer - clear the illegal access flag for I2C2
pub type CI2C2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CCRSF` writer - clear the illegal access flag for CRS
pub type CCRSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CI2C4F` writer - clear the illegal access flag for I2C4
pub type CI2C4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CLPTIM2F` writer - clear the illegal access flag for LPTIM2
pub type CLPTIM2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CFDCAN1F` writer - clear the illegal access flag for FDCAN1
pub type CFDCAN1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CUCPD1F` writer - clear the illegal access flag for UCPD1
pub type CUCPD1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
impl W {
    ///Bit 0 - clear the illegal access flag for TIM2
    #[inline(always)]
    #[must_use]
    pub fn ctim2f(&mut self) -> CTIM2F_W<0> {
        CTIM2F_W::new(self)
    }
    ///Bit 1 - clear the illegal access flag for TIM3
    #[inline(always)]
    #[must_use]
    pub fn ctim3f(&mut self) -> CTIM3F_W<1> {
        CTIM3F_W::new(self)
    }
    ///Bit 2 - clear the illegal access flag for TIM4
    #[inline(always)]
    #[must_use]
    pub fn ctim4f(&mut self) -> CTIM4F_W<2> {
        CTIM4F_W::new(self)
    }
    ///Bit 3 - clear the illegal access flag for TIM5
    #[inline(always)]
    #[must_use]
    pub fn ctim5f(&mut self) -> CTIM5F_W<3> {
        CTIM5F_W::new(self)
    }
    ///Bit 4 - clear the illegal access flag for TIM6
    #[inline(always)]
    #[must_use]
    pub fn ctim6f(&mut self) -> CTIM6F_W<4> {
        CTIM6F_W::new(self)
    }
    ///Bit 5 - clear the illegal access flag for TIM7
    #[inline(always)]
    #[must_use]
    pub fn ctim7f(&mut self) -> CTIM7F_W<5> {
        CTIM7F_W::new(self)
    }
    ///Bit 6 - clear the illegal access flag for WWDG
    #[inline(always)]
    #[must_use]
    pub fn cwwdgf(&mut self) -> CWWDGF_W<6> {
        CWWDGF_W::new(self)
    }
    ///Bit 7 - clear the illegal access flag for IWDG
    #[inline(always)]
    #[must_use]
    pub fn ciwdgf(&mut self) -> CIWDGF_W<7> {
        CIWDGF_W::new(self)
    }
    ///Bit 8 - clear the illegal access flag for SPI2
    #[inline(always)]
    #[must_use]
    pub fn cspi2f(&mut self) -> CSPI2F_W<8> {
        CSPI2F_W::new(self)
    }
    ///Bit 9 - clear the illegal access flag for USART2
    #[inline(always)]
    #[must_use]
    pub fn cusart2f(&mut self) -> CUSART2F_W<9> {
        CUSART2F_W::new(self)
    }
    ///Bit 10 - clear the illegal access flag for USART3
    #[inline(always)]
    #[must_use]
    pub fn cusart3f(&mut self) -> CUSART3F_W<10> {
        CUSART3F_W::new(self)
    }
    ///Bit 11 - clear the illegal access flag for UART4
    #[inline(always)]
    #[must_use]
    pub fn cuart4f(&mut self) -> CUART4F_W<11> {
        CUART4F_W::new(self)
    }
    ///Bit 12 - clear the illegal access flag for UART5
    #[inline(always)]
    #[must_use]
    pub fn cuart5f(&mut self) -> CUART5F_W<12> {
        CUART5F_W::new(self)
    }
    ///Bit 13 - clear the illegal access flag for I2C1
    #[inline(always)]
    #[must_use]
    pub fn ci2c1f(&mut self) -> CI2C1F_W<13> {
        CI2C1F_W::new(self)
    }
    ///Bit 14 - clear the illegal access flag for I2C2
    #[inline(always)]
    #[must_use]
    pub fn ci2c2f(&mut self) -> CI2C2F_W<14> {
        CI2C2F_W::new(self)
    }
    ///Bit 15 - clear the illegal access flag for CRS
    #[inline(always)]
    #[must_use]
    pub fn ccrsf(&mut self) -> CCRSF_W<15> {
        CCRSF_W::new(self)
    }
    ///Bit 16 - clear the illegal access flag for I2C4
    #[inline(always)]
    #[must_use]
    pub fn ci2c4f(&mut self) -> CI2C4F_W<16> {
        CI2C4F_W::new(self)
    }
    ///Bit 17 - clear the illegal access flag for LPTIM2
    #[inline(always)]
    #[must_use]
    pub fn clptim2f(&mut self) -> CLPTIM2F_W<17> {
        CLPTIM2F_W::new(self)
    }
    ///Bit 18 - clear the illegal access flag for FDCAN1
    #[inline(always)]
    #[must_use]
    pub fn cfdcan1f(&mut self) -> CFDCAN1F_W<18> {
        CFDCAN1F_W::new(self)
    }
    ///Bit 19 - clear the illegal access flag for UCPD1
    #[inline(always)]
    #[must_use]
    pub fn cucpd1f(&mut self) -> CUCPD1F_W<19> {
        CUCPD1F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC flag clear register 1
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr1](index.html) module
pub struct FCR1_SPEC;
impl crate::RegisterSpec for FCR1_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [fcr1::W](W) writer structure
impl crate::Writable for FCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
