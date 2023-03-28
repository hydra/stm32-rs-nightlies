///Register `DDRPERFM_CTL` writer
pub struct W(crate::W<DDRPERFM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPERFM_CTL_SPEC>;
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
impl From<crate::W<DDRPERFM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPERFM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `START` writer - START
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPERFM_CTL_SPEC, bool, O>;
///Field `STOP` writer - STOP
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPERFM_CTL_SPEC, bool, O>;
impl W {
    ///Bit 0 - START
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    ///Bit 1 - STOP
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Write-only register. A read request returns all zeros.
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrperfm_ctl](index.html) module
pub struct DDRPERFM_CTL_SPEC;
impl crate::RegisterSpec for DDRPERFM_CTL_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [ddrperfm_ctl::W](W) writer structure
impl crate::Writable for DDRPERFM_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPERFM_CTL to value 0
impl crate::Resettable for DDRPERFM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
