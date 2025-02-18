///Register `GICC_BPR` reader
pub struct R(crate::R<GICC_BPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_BPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_BPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_BPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICC_BPR` writer
pub struct W(crate::W<GICC_BPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_BPR_SPEC>;
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
impl From<crate::W<GICC_BPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_BPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BINARY_POINT` reader - BINARY_POINT
pub type BINARY_POINT_R = crate::FieldReader<u8, u8>;
///Field `BINARY_POINT` writer - BINARY_POINT
pub type BINARY_POINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICC_BPR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:2 - BINARY_POINT
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - BINARY_POINT
    #[inline(always)]
    #[must_use]
    pub fn binary_point(&mut self) -> BINARY_POINT_W<0> {
        BINARY_POINT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GICC binary point register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicc_bpr](index.html) module
pub struct GICC_BPR_SPEC;
impl crate::RegisterSpec for GICC_BPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicc_bpr::R](R) reader structure
impl crate::Readable for GICC_BPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicc_bpr::W](W) writer structure
impl crate::Writable for GICC_BPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICC_BPR to value 0x02
impl crate::Resettable for GICC_BPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
