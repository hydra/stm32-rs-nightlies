///Register `APB3FZR` reader
pub struct R(crate::R<APB3FZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3FZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3FZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3FZR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB3FZR` writer
pub struct W(crate::W<APB3FZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3FZR_SPEC>;
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
impl From<crate::W<APB3FZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3FZR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_I3C2_STOP` reader - I3C2 SCL stall counter stop in debug
pub type DBG_I3C2_STOP_R = crate::BitReader<bool>;
///Field `DBG_I3C2_STOP` writer - I3C2 SCL stall counter stop in debug
pub type DBG_I3C2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
///Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in debug
pub type DBG_LPTIM1_STOP_R = crate::BitReader<bool>;
///Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in debug
pub type DBG_LPTIM1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
///Field `DBG_RTC_STOP` reader - RTC stop in debug
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
///Field `DBG_RTC_STOP` writer - RTC stop in debug
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
impl R {
    ///Bit 12 - I3C2 SCL stall counter stop in debug
    #[inline(always)]
    pub fn dbg_i3c2_stop(&self) -> DBG_I3C2_STOP_R {
        DBG_I3C2_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 17 - LPTIM1 stop in debug
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 30 - RTC stop in debug
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 12 - I3C2 SCL stall counter stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_i3c2_stop(&mut self) -> DBG_I3C2_STOP_W<12> {
        DBG_I3C2_STOP_W::new(self)
    }
    ///Bit 17 - LPTIM1 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<17> {
        DBG_LPTIM1_STOP_W::new(self)
    }
    ///Bit 30 - RTC stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<30> {
        DBG_RTC_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBGMCU APB3 peripheral freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb3fzr](index.html) module
pub struct APB3FZR_SPEC;
impl crate::RegisterSpec for APB3FZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb3fzr::R](R) reader structure
impl crate::Readable for APB3FZR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb3fzr::W](W) writer structure
impl crate::Writable for APB3FZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB3FZR to value 0
impl crate::Resettable for APB3FZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
