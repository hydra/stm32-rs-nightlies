///Register `PTPSSIR` reader
pub struct R(crate::R<PTPSSIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPSSIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPSSIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPSSIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PTPSSIR` writer
pub struct W(crate::W<PTPSSIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPSSIR_SPEC>;
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
impl From<crate::W<PTPSSIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPSSIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `STSSI` reader - System time subsecond increment
pub type STSSI_R = crate::FieldReader<u8, u8>;
///Field `STSSI` writer - System time subsecond increment
pub type STSSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPSSIR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - System time subsecond increment
    #[inline(always)]
    pub fn stssi(&self) -> STSSI_R {
        STSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - System time subsecond increment
    #[inline(always)]
    #[must_use]
    pub fn stssi(&mut self) -> STSSI_W<0> {
        STSSI_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet PTP subsecond increment register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptpssir](index.html) module
pub struct PTPSSIR_SPEC;
impl crate::RegisterSpec for PTPSSIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ptpssir::R](R) reader structure
impl crate::Readable for PTPSSIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ptpssir::W](W) writer structure
impl crate::Writable for PTPSSIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PTPSSIR to value 0
impl crate::Resettable for PTPSSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
