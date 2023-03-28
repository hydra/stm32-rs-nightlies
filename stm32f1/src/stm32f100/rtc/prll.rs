///Register `PRLL` writer
pub struct W(crate::W<PRLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRLL_SPEC>;
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
impl From<crate::W<PRLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRLL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRLL` writer - RTC Prescaler Divider Register Low
pub type PRLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRLL_SPEC, u16, u16, 16, O>;
impl W {
    ///Bits 0:15 - RTC Prescaler Divider Register Low
    #[inline(always)]
    #[must_use]
    pub fn prll(&mut self) -> PRLL_W<0> {
        PRLL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC Prescaler Load Register Low
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [prll](index.html) module
pub struct PRLL_SPEC;
impl crate::RegisterSpec for PRLL_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [prll::W](W) writer structure
impl crate::Writable for PRLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRLL to value 0x8000
impl crate::Resettable for PRLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
