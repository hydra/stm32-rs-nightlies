///Register `CNDTR5` reader
pub struct R(crate::R<CNDTR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNDTR5` writer
pub struct W(crate::W<CNDTR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR5_SPEC>;
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
impl From<crate::W<CNDTR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NDT` reader - Number of data to transfer
pub type NDT_R = crate::FieldReader<u16, u16>;
///Field `NDT` writer - Number of data to transfer
pub type NDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CNDTR5_SPEC, u16, u16, 16, O>;
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
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr5](index.html) module
pub struct CNDTR5_SPEC;
impl crate::RegisterSpec for CNDTR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [cndtr5::R](R) reader structure
impl crate::Readable for CNDTR5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cndtr5::W](W) writer structure
impl crate::Writable for CNDTR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNDTR5 to value 0
impl crate::Resettable for CNDTR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
