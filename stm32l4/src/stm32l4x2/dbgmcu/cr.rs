///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_SLEEP` reader - Debug Sleep mode
pub type DBG_SLEEP_R = crate::BitReader<bool>;
///Field `DBG_SLEEP` writer - Debug Sleep mode
pub type DBG_SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DBG_STOP` reader - Debug Stop mode
pub type DBG_STOP_R = crate::BitReader<bool>;
///Field `DBG_STOP` writer - Debug Stop mode
pub type DBG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DBG_STANDBY` reader - Debug Standby mode
pub type DBG_STANDBY_R = crate::BitReader<bool>;
///Field `DBG_STANDBY` writer - Debug Standby mode
pub type DBG_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TRACE_IOEN` reader - Trace pin assignment control
pub type TRACE_IOEN_R = crate::BitReader<bool>;
///Field `TRACE_IOEN` writer - Trace pin assignment control
pub type TRACE_IOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TRACE_MODE` reader - Trace pin assignment control
pub type TRACE_MODE_R = crate::FieldReader<u8, u8>;
///Field `TRACE_MODE` writer - Trace pin assignment control
pub type TRACE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Debug Sleep mode
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Debug Stop mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Debug Standby mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Debug Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<0> {
        DBG_SLEEP_W::new(self)
    }
    ///Bit 1 - Debug Stop mode
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<1> {
        DBG_STOP_W::new(self)
    }
    ///Bit 2 - Debug Standby mode
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<2> {
        DBG_STANDBY_W::new(self)
    }
    ///Bit 5 - Trace pin assignment control
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<5> {
        TRACE_IOEN_W::new(self)
    }
    ///Bits 6:7 - Trace pin assignment control
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<6> {
        TRACE_MODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
