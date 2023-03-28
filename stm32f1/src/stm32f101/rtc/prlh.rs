///Register `PRLH` writer
pub struct W(crate::W<PRLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRLH_SPEC>;
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
impl From<crate::W<PRLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRLH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRLH` writer - RTC Prescaler Load Register High
pub type PRLH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRLH_SPEC, u8, u8, 4, O>;
impl W {
    ///Bits 0:3 - RTC Prescaler Load Register High
    #[inline(always)]
    #[must_use]
    pub fn prlh(&mut self) -> PRLH_W<0> {
        PRLH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC Prescaler Load Register High
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [prlh](index.html) module
pub struct PRLH_SPEC;
impl crate::RegisterSpec for PRLH_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [prlh::W](W) writer structure
impl crate::Writable for PRLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRLH to value 0
impl crate::Resettable for PRLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
