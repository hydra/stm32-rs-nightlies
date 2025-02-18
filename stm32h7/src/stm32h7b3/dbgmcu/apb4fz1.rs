///Register `APB4FZ1` reader
pub struct R(crate::R<APB4FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB4FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB4FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB4FZ1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB4FZ1` writer
pub struct W(crate::W<APB4FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB4FZ1_SPEC>;
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
impl From<crate::W<APB4FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB4FZ1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I2C4` reader - I2C4 SMBUS timeout stop in debug
pub type I2C4_R = crate::BitReader<bool>;
///Field `I2C4` writer - I2C4 SMBUS timeout stop in debug
pub type I2C4_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `LPTIM2` reader - LPTIM2 stop in debug
pub type LPTIM2_R = crate::BitReader<bool>;
///Field `LPTIM2` writer - LPTIM2 stop in debug
pub type LPTIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `LPTIM3` reader - LPTIM3 stop in debug
pub type LPTIM3_R = crate::BitReader<bool>;
///Field `LPTIM3` writer - LPTIM3 stop in debug
pub type LPTIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `RTC` reader - RTC stop in debug
pub type RTC_R = crate::BitReader<bool>;
///Field `RTC` writer - RTC stop in debug
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `WDGLSCD` reader - LS watchdog for CPU domain stop in debug
pub type WDGLSCD_R = crate::BitReader<bool>;
///Field `WDGLSCD` writer - LS watchdog for CPU domain stop in debug
pub type WDGLSCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
impl R {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn i2c4(&self) -> I2C4_R {
        I2C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 stop in debug
    #[inline(always)]
    pub fn lptim3(&self) -> LPTIM3_R {
        LPTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - LS watchdog for CPU domain stop in debug
    #[inline(always)]
    pub fn wdglscd(&self) -> WDGLSCD_R {
        WDGLSCD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    #[must_use]
    pub fn i2c4(&mut self) -> I2C4_W<7> {
        I2C4_W::new(self)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn lptim2(&mut self) -> LPTIM2_W<9> {
        LPTIM2_W::new(self)
    }
    ///Bit 10 - LPTIM3 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn lptim3(&mut self) -> LPTIM3_W<10> {
        LPTIM3_W::new(self)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<16> {
        RTC_W::new(self)
    }
    ///Bit 18 - LS watchdog for CPU domain stop in debug
    #[inline(always)]
    #[must_use]
    pub fn wdglscd(&mut self) -> WDGLSCD_W<18> {
        WDGLSCD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBGMCU APB4 peripheral freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb4fz1](index.html) module
pub struct APB4FZ1_SPEC;
impl crate::RegisterSpec for APB4FZ1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb4fz1::R](R) reader structure
impl crate::Readable for APB4FZ1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb4fz1::W](W) writer structure
impl crate::Writable for APB4FZ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB4FZ1 to value 0
impl crate::Resettable for APB4FZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
