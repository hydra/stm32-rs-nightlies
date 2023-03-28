///Register `PTPTTLR` reader
pub struct R(crate::R<PTPTTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTTLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PTPTTLR` writer
pub struct W(crate::W<PTPTTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTTLR_SPEC>;
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
impl From<crate::W<PTPTTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTTLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TTSL` reader - Target time stamp low
pub type TTSL_R = crate::FieldReader<u32, u32>;
///Field `TTSL` writer - Target time stamp low
pub type TTSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPTTLR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Target time stamp low
    #[inline(always)]
    pub fn ttsl(&self) -> TTSL_R {
        TTSL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Target time stamp low
    #[inline(always)]
    #[must_use]
    pub fn ttsl(&mut self) -> TTSL_W<0> {
        TTSL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet PTP target time low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptpttlr](index.html) module
pub struct PTPTTLR_SPEC;
impl crate::RegisterSpec for PTPTTLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ptpttlr::R](R) reader structure
impl crate::Readable for PTPTTLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ptpttlr::W](W) writer structure
impl crate::Writable for PTPTTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PTPTTLR to value 0
impl crate::Resettable for PTPTTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
