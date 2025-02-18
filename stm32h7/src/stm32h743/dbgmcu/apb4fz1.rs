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
///Field `DBG_I2C4` reader - I2C4 SMBUS timeout stop in debug
pub type DBG_I2C4_R = crate::BitReader<bool>;
///Field `DBG_I2C4` writer - I2C4 SMBUS timeout stop in debug
pub type DBG_I2C4_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `DBG_LPTIM2` reader - LPTIM2 stop in debug
pub type DBG_LPTIM2_R = crate::BitReader<bool>;
///Field `DBG_LPTIM2` writer - LPTIM2 stop in debug
pub type DBG_LPTIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `DBG_LPTIM3` reader - LPTIM2 stop in debug
pub type DBG_LPTIM3_R = crate::BitReader<bool>;
///Field `DBG_LPTIM3` writer - LPTIM2 stop in debug
pub type DBG_LPTIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `DBG_LPTIM4` reader - LPTIM4 stop in debug
pub type DBG_LPTIM4_R = crate::BitReader<bool>;
///Field `DBG_LPTIM4` writer - LPTIM4 stop in debug
pub type DBG_LPTIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `DBG_LPTIM5` reader - LPTIM5 stop in debug
pub type DBG_LPTIM5_R = crate::BitReader<bool>;
///Field `DBG_LPTIM5` writer - LPTIM5 stop in debug
pub type DBG_LPTIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `DBG_RTC` reader - RTC stop in debug
pub type DBG_RTC_R = crate::BitReader<bool>;
///Field `DBG_RTC` writer - RTC stop in debug
pub type DBG_RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
///Field `DBG_IWDG1` reader - Independent watchdog for D1 stop in debug
pub type DBG_IWDG1_R = crate::BitReader<bool>;
///Field `DBG_IWDG1` writer - Independent watchdog for D1 stop in debug
pub type DBG_IWDG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4FZ1_SPEC, bool, O>;
impl R {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c4(&self) -> DBG_I2C4_R {
        DBG_I2C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim2(&self) -> DBG_LPTIM2_R {
        DBG_LPTIM2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim3(&self) -> DBG_LPTIM3_R {
        DBG_LPTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn dbg_lptim4(&self) -> DBG_LPTIM4_R {
        DBG_LPTIM4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    pub fn dbg_lptim5(&self) -> DBG_LPTIM5_R {
        DBG_LPTIM5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn dbg_rtc(&self) -> DBG_RTC_R {
        DBG_RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog for D1 stop in debug
    #[inline(always)]
    pub fn dbg_iwdg1(&self) -> DBG_IWDG1_R {
        DBG_IWDG1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c4(&mut self) -> DBG_I2C4_W<7> {
        DBG_I2C4_W::new(self)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2(&mut self) -> DBG_LPTIM2_W<9> {
        DBG_LPTIM2_W::new(self)
    }
    ///Bit 10 - LPTIM2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3(&mut self) -> DBG_LPTIM3_W<10> {
        DBG_LPTIM3_W::new(self)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim4(&mut self) -> DBG_LPTIM4_W<11> {
        DBG_LPTIM4_W::new(self)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim5(&mut self) -> DBG_LPTIM5_W<12> {
        DBG_LPTIM5_W::new(self)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc(&mut self) -> DBG_RTC_W<16> {
        DBG_RTC_W::new(self)
    }
    ///Bit 18 - Independent watchdog for D1 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg1(&mut self) -> DBG_IWDG1_W<18> {
        DBG_IWDG1_W::new(self)
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
