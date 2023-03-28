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
///Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM3_STOP` reader - DBG_TIM3 _STOP
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM3_STOP` writer - DBG_TIM3 _STOP
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP
pub type DBG_TIM4_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP
pub type DBG_TIM4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP
pub type DBG_TIM5_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP
pub type DBG_TIM5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM6_STOP` reader - DBG_TIM6_STOP
pub type DBG_TIM6_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM6_STOP` writer - DBG_TIM6_STOP
pub type DBG_TIM6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM7_STOP` reader - DBG_TIM7_STOP
pub type DBG_TIM7_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM7_STOP` writer - DBG_TIM7_STOP
pub type DBG_TIM7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM12_STOP` reader - DBG_TIM12_STOP
pub type DBG_TIM12_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM12_STOP` writer - DBG_TIM12_STOP
pub type DBG_TIM12_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM13_STOP` reader - DBG_TIM13_STOP
pub type DBG_TIM13_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM13_STOP` writer - DBG_TIM13_STOP
pub type DBG_TIM13_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_TIM14_STOP` reader - DBG_TIM14_STOP
pub type DBG_TIM14_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM14_STOP` writer - DBG_TIM14_STOP
pub type DBG_TIM14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_RTC_STOP` reader - DBG_RTC_STOP
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
///Field `DBG_RTC_STOP` writer - DBG_RTC_STOP
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_IWDG_STOP` reader - DBG_IWDEG_STOP
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_IWDG_STOP` writer - DBG_IWDEG_STOP
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_J2C1_SMBUS_TIMEOUT` reader - DBG_J2C1_SMBUS_TIMEOUT
pub type DBG_J2C1_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
///Field `DBG_J2C1_SMBUS_TIMEOUT` writer - DBG_J2C1_SMBUS_TIMEOUT
pub type DBG_J2C1_SMBUS_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_J2C2_SMBUS_TIMEOUT` reader - DBG_J2C2_SMBUS_TIMEOUT
pub type DBG_J2C2_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
///Field `DBG_J2C2_SMBUS_TIMEOUT` writer - DBG_J2C2_SMBUS_TIMEOUT
pub type DBG_J2C2_SMBUS_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_J2C3SMBUS_TIMEOUT` reader - DBG_J2C3SMBUS_TIMEOUT
pub type DBG_J2C3SMBUS_TIMEOUT_R = crate::BitReader<bool>;
///Field `DBG_J2C3SMBUS_TIMEOUT` writer - DBG_J2C3SMBUS_TIMEOUT
pub type DBG_J2C3SMBUS_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_CAN1_STOP` reader - DBG_CAN1_STOP
pub type DBG_CAN1_STOP_R = crate::BitReader<bool>;
///Field `DBG_CAN1_STOP` writer - DBG_CAN1_STOP
pub type DBG_CAN1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
///Field `DBG_CAN2_STOP` reader - DBG_CAN2_STOP
pub type DBG_CAN2_STOP_R = crate::BitReader<bool>;
///Field `DBG_CAN2_STOP` writer - DBG_CAN2_STOP
pub type DBG_CAN2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
impl R {
    ///Bit 0 - DBG_TIM2_STOP
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DBG_TIM3 _STOP
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DBG_TIM4_STOP
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DBG_TIM5_STOP
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DBG_TIM6_STOP
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DBG_TIM7_STOP
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DBG_TIM12_STOP
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DBG_TIM13_STOP
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DBG_TIM14_STOP
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - DBG_RTC_STOP
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DBG_WWDG_STOP
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DBG_IWDEG_STOP
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - DBG_J2C1_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c1_smbus_timeout(&self) -> DBG_J2C1_SMBUS_TIMEOUT_R {
        DBG_J2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DBG_J2C2_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c2_smbus_timeout(&self) -> DBG_J2C2_SMBUS_TIMEOUT_R {
        DBG_J2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - DBG_J2C3SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c3smbus_timeout(&self) -> DBG_J2C3SMBUS_TIMEOUT_R {
        DBG_J2C3SMBUS_TIMEOUT_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - DBG_CAN1_STOP
    #[inline(always)]
    pub fn dbg_can1_stop(&self) -> DBG_CAN1_STOP_R {
        DBG_CAN1_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - DBG_CAN2_STOP
    #[inline(always)]
    pub fn dbg_can2_stop(&self) -> DBG_CAN2_STOP_R {
        DBG_CAN2_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DBG_TIM2_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<0> {
        DBG_TIM2_STOP_W::new(self)
    }
    ///Bit 1 - DBG_TIM3 _STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    ///Bit 2 - DBG_TIM4_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<2> {
        DBG_TIM4_STOP_W::new(self)
    }
    ///Bit 3 - DBG_TIM5_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<3> {
        DBG_TIM5_STOP_W::new(self)
    }
    ///Bit 4 - DBG_TIM6_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<4> {
        DBG_TIM6_STOP_W::new(self)
    }
    ///Bit 5 - DBG_TIM7_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<5> {
        DBG_TIM7_STOP_W::new(self)
    }
    ///Bit 6 - DBG_TIM12_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<6> {
        DBG_TIM12_STOP_W::new(self)
    }
    ///Bit 7 - DBG_TIM13_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<7> {
        DBG_TIM13_STOP_W::new(self)
    }
    ///Bit 8 - DBG_TIM14_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<8> {
        DBG_TIM14_STOP_W::new(self)
    }
    ///Bit 10 - DBG_RTC_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<10> {
        DBG_RTC_STOP_W::new(self)
    }
    ///Bit 11 - DBG_WWDG_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    ///Bit 12 - DBG_IWDEG_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    ///Bit 21 - DBG_J2C1_SMBUS_TIMEOUT
    #[inline(always)]
    #[must_use]
    pub fn dbg_j2c1_smbus_timeout(&mut self) -> DBG_J2C1_SMBUS_TIMEOUT_W<21> {
        DBG_J2C1_SMBUS_TIMEOUT_W::new(self)
    }
    ///Bit 22 - DBG_J2C2_SMBUS_TIMEOUT
    #[inline(always)]
    #[must_use]
    pub fn dbg_j2c2_smbus_timeout(&mut self) -> DBG_J2C2_SMBUS_TIMEOUT_W<22> {
        DBG_J2C2_SMBUS_TIMEOUT_W::new(self)
    }
    ///Bit 23 - DBG_J2C3SMBUS_TIMEOUT
    #[inline(always)]
    #[must_use]
    pub fn dbg_j2c3smbus_timeout(&mut self) -> DBG_J2C3SMBUS_TIMEOUT_W<23> {
        DBG_J2C3SMBUS_TIMEOUT_W::new(self)
    }
    ///Bit 25 - DBG_CAN1_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_can1_stop(&mut self) -> DBG_CAN1_STOP_W<25> {
        DBG_CAN1_STOP_W::new(self)
    }
    ///Bit 26 - DBG_CAN2_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_can2_stop(&mut self) -> DBG_CAN2_STOP_W<26> {
        DBG_CAN2_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU APB1 Freeze registe
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
