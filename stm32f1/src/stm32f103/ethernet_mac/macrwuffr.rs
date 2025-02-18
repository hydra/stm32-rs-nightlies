///Register `MACRWUFFR` reader
pub struct R(crate::R<MACRWUFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRWUFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRWUFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRWUFFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACRWUFFR` writer
pub struct W(crate::W<MACRWUFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACRWUFFR_SPEC>;
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
impl From<crate::W<MACRWUFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACRWUFFR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macrwuffr](index.html) module
pub struct MACRWUFFR_SPEC;
impl crate::RegisterSpec for MACRWUFFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macrwuffr::R](R) reader structure
impl crate::Readable for MACRWUFFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macrwuffr::W](W) writer structure
impl crate::Writable for MACRWUFFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACRWUFFR to value 0
impl crate::Resettable for MACRWUFFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
