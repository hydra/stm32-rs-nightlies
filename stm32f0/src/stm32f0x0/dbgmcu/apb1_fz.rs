///Register `APB1_FZ` reader
pub struct R(crate::R<APB1_FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1_FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1_FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1_FZ_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1_FZ` writer
pub struct W(crate::W<APB1_FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1_FZ_SPEC>;
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
impl From<crate::W<APB1_FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1_FZ_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM6_STOP` reader - TIM6 counter stopped when core is halted
pub type DBG_TIM6_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM6_STOP` writer - TIM6 counter stopped when core is halted
pub type DBG_TIM6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted
pub type DBG_TIM7_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted
pub type DBG_TIM7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM14_STOP` reader - TIM14 counter stopped when core is halted
pub type DBG_TIM14_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM14_STOP` writer - TIM14 counter stopped when core is halted
pub type DBG_TIM14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_WWDG_STOP` reader - Debug window watchdog stopped when core is halted
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_WWDG_STOP` writer - Debug window watchdog stopped when core is halted
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_IWDG_STOP` reader - Debug independent watchdog stopped when core is halted
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_IWDG_STOP` writer - Debug independent watchdog stopped when core is halted
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when core is halted
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
///Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when core is halted
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
impl R {
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - TIM14 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Debug window watchdog stopped when core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Debug independent watchdog stopped when core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    ///Bit 4 - TIM6 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<4> {
        DBG_TIM6_STOP_W::new(self)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<5> {
        DBG_TIM7_STOP_W::new(self)
    }
    ///Bit 8 - TIM14 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<8> {
        DBG_TIM14_STOP_W::new(self)
    }
    ///Bit 11 - Debug window watchdog stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    ///Bit 12 - Debug independent watchdog stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    ///Bit 21 - SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<21> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU APB1 freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1_fz](index.html) module
pub struct APB1_FZ_SPEC;
impl crate::RegisterSpec for APB1_FZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1_fz::R](R) reader structure
impl crate::Readable for APB1_FZ_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1_fz::W](W) writer structure
impl crate::Writable for APB1_FZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1_FZ to value 0
impl crate::Resettable for APB1_FZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
