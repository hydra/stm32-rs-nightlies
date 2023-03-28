///Register `APB1LFZR` reader
pub struct R(crate::R<APB1LFZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LFZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LFZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LFZR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1LFZR` writer
pub struct W(crate::W<APB1LFZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LFZR_SPEC>;
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
impl From<crate::W<APB1LFZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LFZR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_TIM2_STOP` reader - TIM2 stop in debug
pub type DBG_TIM2_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM2_STOP` writer - TIM2 stop in debug
pub type DBG_TIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_TIM3_STOP` reader - TIM3 stop in debug
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM3_STOP` writer - TIM3 stop in debug
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_TIM6_STOP` reader - TIM6 stop in debug
pub type DBG_TIM6_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM6_STOP` writer - TIM6 stop in debug
pub type DBG_TIM6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_TIM7_STOP` reader - TIM7 stop in debug
pub type DBG_TIM7_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM7_STOP` writer - TIM7 stop in debug
pub type DBG_TIM7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_WWDG_STOP` reader - WWDG stop in debug
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_WWDG_STOP` writer - WWDG stop in debug
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_IWDG_STOP` reader - IWDG stop in debug
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_IWDG_STOP` writer - IWDG stop in debug
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in debug
pub type DBG_I2C1_STOP_R = crate::BitReader<bool>;
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in debug
pub type DBG_I2C1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout stop in debug
pub type DBG_I2C2_STOP_R = crate::BitReader<bool>;
///Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout stop in debug
pub type DBG_I2C2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
///Field `DBG_I3C1_STOP` reader - I3C1 SCL stall counter stop in debug
pub type DBG_I3C1_STOP_R = crate::BitReader<bool>;
///Field `DBG_I3C1_STOP` writer - I3C1 SCL stall counter stop in debug
pub type DBG_I3C1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZR_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM2 stop in debug
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 stop in debug
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 stop in debug
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 stop in debug
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - WWDG stop in debug
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IWDG stop in debug
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I3C1 SCL stall counter stop in debug
    #[inline(always)]
    pub fn dbg_i3c1_stop(&self) -> DBG_I3C1_STOP_R {
        DBG_I3C1_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<0> {
        DBG_TIM2_STOP_W::new(self)
    }
    ///Bit 1 - TIM3 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    ///Bit 4 - TIM6 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<4> {
        DBG_TIM6_STOP_W::new(self)
    }
    ///Bit 5 - TIM7 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<5> {
        DBG_TIM7_STOP_W::new(self)
    }
    ///Bit 11 - WWDG stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    ///Bit 12 - IWDG stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<21> {
        DBG_I2C1_STOP_W::new(self)
    }
    ///Bit 22 - I2C2 SMBUS timeout stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<22> {
        DBG_I2C2_STOP_W::new(self)
    }
    ///Bit 23 - I3C1 SCL stall counter stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_i3c1_stop(&mut self) -> DBG_I3C1_STOP_W<23> {
        DBG_I3C1_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBGMCU APB1L peripheral freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1lfzr](index.html) module
pub struct APB1LFZR_SPEC;
impl crate::RegisterSpec for APB1LFZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1lfzr::R](R) reader structure
impl crate::Readable for APB1LFZR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1lfzr::W](W) writer structure
impl crate::Writable for APB1LFZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB1LFZR to value 0
impl crate::Resettable for APB1LFZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
