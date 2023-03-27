///Register `HDPLCR` reader
pub struct R(crate::R<HDPLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDPLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDPLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDPLCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HDPLCR` writer
pub struct W(crate::W<HDPLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDPLCR_SPEC>;
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
impl From<crate::W<HDPLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDPLCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INCR_HDPL` reader - increment HDPL value Other: all other values allow a HDPL level increment.
pub type INCR_HDPL_R = crate::FieldReader<u8, u8>;
///Field `INCR_HDPL` writer - increment HDPL value Other: all other values allow a HDPL level increment.
pub type INCR_HDPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDPLCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment.
    #[inline(always)]
    pub fn incr_hdpl(&self) -> INCR_HDPL_R {
        INCR_HDPL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment.
    #[inline(always)]
    #[must_use]
    pub fn incr_hdpl(&mut self) -> INCR_HDPL_W<0> {
        INCR_HDPL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS temporal isolation control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdplcr](index.html) module
pub struct HDPLCR_SPEC;
impl crate::RegisterSpec for HDPLCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdplcr::R](R) reader structure
impl crate::Readable for HDPLCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hdplcr::W](W) writer structure
impl crate::Writable for HDPLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HDPLCR to value 0xb4
impl crate::Resettable for HDPLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
