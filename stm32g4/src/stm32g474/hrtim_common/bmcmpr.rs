///Register `BMCMPR` reader
pub struct R(crate::R<BMCMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCMPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BMCMPR` writer
pub struct W(crate::W<BMCMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCMPR_SPEC>;
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
impl From<crate::W<BMCMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCMPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BMCMP` reader - BMCMP
pub type BMCMP_R = crate::FieldReader<u16, u16>;
///Field `BMCMP` writer - BMCMP
pub type BMCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCMPR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - BMCMP
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - BMCMP
    #[inline(always)]
    #[must_use]
    pub fn bmcmp(&mut self) -> BMCMP_W<0> {
        BMCMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///BMCMPR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmcmpr](index.html) module
pub struct BMCMPR_SPEC;
impl crate::RegisterSpec for BMCMPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bmcmpr::R](R) reader structure
impl crate::Readable for BMCMPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bmcmpr::W](W) writer structure
impl crate::Writable for BMCMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BMCMPR to value 0
impl crate::Resettable for BMCMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
