///Register `SAI_BDR` reader
pub struct R(crate::R<SAI_BDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_BDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_BDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_BDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SAI_BDR` writer
pub struct W(crate::W<SAI_BDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_BDR_SPEC>;
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
impl From<crate::W<SAI_BDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_BDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u32, u32>;
///Field `DATA` writer - DATA
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BDR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - DATA
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sai_bdr](index.html) module
pub struct SAI_BDR_SPEC;
impl crate::RegisterSpec for SAI_BDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sai_bdr::R](R) reader structure
impl crate::Readable for SAI_BDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sai_bdr::W](W) writer structure
impl crate::Writable for SAI_BDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SAI_BDR to value 0
impl crate::Resettable for SAI_BDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
