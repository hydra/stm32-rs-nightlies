///Register `ALRL` writer
pub struct W(crate::W<ALRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRL_SPEC>;
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
impl From<crate::W<ALRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ALRL` writer - RTC alarm register low
pub type ALRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRL_SPEC, u16, u16, 16, O>;
impl W {
    ///Bits 0:15 - RTC alarm register low
    #[inline(always)]
    #[must_use]
    pub fn alrl(&mut self) -> ALRL_W<0> {
        ALRL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC Alarm Register Low
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrl](index.html) module
pub struct ALRL_SPEC;
impl crate::RegisterSpec for ALRL_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [alrl::W](W) writer structure
impl crate::Writable for ALRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ALRL to value 0xffff
impl crate::Resettable for ALRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
