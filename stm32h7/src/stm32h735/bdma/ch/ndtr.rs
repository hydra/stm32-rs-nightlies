///Register `NDTR` reader
pub struct R(crate::R<NDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `NDTR` writer
pub struct W(crate::W<NDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDTR_SPEC>;
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
impl From<crate::W<NDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NDT` reader - Number of data to transfer
pub type NDT_R = crate::FieldReader<u16, u16>;
///Field `NDT` writer - Number of data to transfer
pub type NDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, NDTR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Number of data to transfer
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel x number of data to transfer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ndtr](index.html) module
pub struct NDTR_SPEC;
impl crate::RegisterSpec for NDTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ndtr::R](R) reader structure
impl crate::Readable for NDTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ndtr::W](W) writer structure
impl crate::Writable for NDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets NDTR to value 0
impl crate::Resettable for NDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
