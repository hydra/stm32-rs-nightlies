///Register `MACRWUFFER` reader
pub struct R(crate::R<MACRWUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRWUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRWUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRWUFFER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACRWUFFER` writer
pub struct W(crate::W<MACRWUFFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACRWUFFER_SPEC>;
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
impl From<crate::W<MACRWUFFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACRWUFFER_SPEC>) -> Self {
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
///Ethernet MAC remote wakeup frame filter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macrwuffer](index.html) module
pub struct MACRWUFFER_SPEC;
impl crate::RegisterSpec for MACRWUFFER_SPEC {
    type Ux = u32;
}
///`read()` method returns [macrwuffer::R](R) reader structure
impl crate::Readable for MACRWUFFER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macrwuffer::W](W) writer structure
impl crate::Writable for MACRWUFFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACRWUFFER to value 0xffff_ffff
impl crate::Resettable for MACRWUFFER_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
