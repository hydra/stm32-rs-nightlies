///Register `PWR_WUSCR` writer
pub struct W(crate::W<PWR_WUSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_WUSCR_SPEC>;
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
impl From<crate::W<PWR_WUSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_WUSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CWUF1` writer - Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
pub type CWUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
///Field `CWUF2` writer - Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
pub type CWUF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
///Field `CWUF3` writer - Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
pub type CWUF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
///Field `CWUF4` writer - Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
pub type CWUF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
///Field `CWUF5` writer - Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
pub type CWUF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
///Field `CWUF6` writer - Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
pub type CWUF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
///Field `CWUF7` writer - Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
pub type CWUF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
///Field `CWUF8` writer - Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
pub type CWUF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUSCR_SPEC, bool, O>;
impl W {
    ///Bit 0 - Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<0> {
        CWUF1_W::new(self)
    }
    ///Bit 1 - Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<1> {
        CWUF2_W::new(self)
    }
    ///Bit 2 - Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<2> {
        CWUF3_W::new(self)
    }
    ///Bit 3 - Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<3> {
        CWUF4_W::new(self)
    }
    ///Bit 4 - Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<4> {
        CWUF5_W::new(self)
    }
    ///Bit 5 - Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf6(&mut self) -> CWUF6_W<5> {
        CWUF6_W::new(self)
    }
    ///Bit 6 - Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf7(&mut self) -> CWUF7_W<6> {
        CWUF7_W::new(self)
    }
    ///Bit 7 - Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf8(&mut self) -> CWUF8_W<7> {
        CWUF8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR wakeup status clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_wuscr](index.html) module
pub struct PWR_WUSCR_SPEC;
impl crate::RegisterSpec for PWR_WUSCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [pwr_wuscr::W](W) writer structure
impl crate::Writable for PWR_WUSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_WUSCR to value 0
impl crate::Resettable for PWR_WUSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
