///Register `CH0DLYR` reader
pub struct R(crate::R<CH0DLYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0DLYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0DLYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0DLYR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CH0DLYR` writer
pub struct W(crate::W<CH0DLYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0DLYR_SPEC>;
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
impl From<crate::W<CH0DLYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0DLYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLSSKP` reader - PLSSKP
pub type PLSSKP_R = crate::FieldReader<u8, u8>;
///Field `PLSSKP` writer - PLSSKP
pub type PLSSKP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0DLYR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<0> {
        PLSSKP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0dlyr](index.html) module
pub struct CH0DLYR_SPEC;
impl crate::RegisterSpec for CH0DLYR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ch0dlyr::R](R) reader structure
impl crate::Readable for CH0DLYR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ch0dlyr::W](W) writer structure
impl crate::Writable for CH0DLYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CH0DLYR to value 0
impl crate::Resettable for CH0DLYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
