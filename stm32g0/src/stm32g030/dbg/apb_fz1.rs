///Register `APB_FZ1` reader
pub struct R(crate::R<APB_FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_FZ1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB_FZ1` writer
pub struct W(crate::W<APB_FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_FZ1_SPEC>;
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
impl From<crate::W<APB_FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_FZ1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_TIM2_STOP` reader - TIM2 counter stopped when core is halted
pub type DBG_TIM2_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM2_STOP` writer - TIM2 counter stopped when core is halted
pub type DBG_TIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
///Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
///Field `DBG_RTC_STOP` reader - RTC counter stopped when core is halted
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
///Field `DBG_RTC_STOP` writer - RTC counter stopped when core is halted
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
///Field `DBG_WWDG_STOP` reader - Window watchdog counter stopped when core is halted
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_WWDG_STOP` writer - Window watchdog counter stopped when core is halted
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
///Field `DBG_IWDG_STOP` reader - Independent watchdog counter stopped when core is halted
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_IWDG_STOP` writer - Independent watchdog counter stopped when core is halted
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout counter stopped when core is halted
pub type DBG_I2C1_STOP_R = crate::BitReader<bool>;
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout counter stopped when core is halted
pub type DBG_I2C1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - RTC counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Independent watchdog counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<0> {
        DBG_TIM2_STOP_W::new(self)
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    ///Bit 10 - RTC counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<10> {
        DBG_RTC_STOP_W::new(self)
    }
    ///Bit 11 - Window watchdog counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    ///Bit 12 - Independent watchdog counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    ///Bit 21 - I2C1 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<21> {
        DBG_I2C1_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU APB1 freeze register1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb_fz1](index.html) module
pub struct APB_FZ1_SPEC;
impl crate::RegisterSpec for APB_FZ1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb_fz1::R](R) reader structure
impl crate::Readable for APB_FZ1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb_fz1::W](W) writer structure
impl crate::Writable for APB_FZ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB_FZ1 to value 0
impl crate::Resettable for APB_FZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
