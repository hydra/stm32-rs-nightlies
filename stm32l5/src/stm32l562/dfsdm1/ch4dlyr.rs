///Register `CH4DLYR` reader
pub struct R(crate::R<CH4DLYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4DLYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4DLYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4DLYR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CH4DLYR` writer
pub struct W(crate::W<CH4DLYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4DLYR_SPEC>;
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
impl From<crate::W<CH4DLYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4DLYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLSSKP` reader - PLSSKP
pub type PLSSKP_R = crate::FieldReader<u8, u8>;
///Field `PLSSKP` writer - PLSSKP
pub type PLSSKP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH4DLYR_SPEC, u8, u8, 6, O>;
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
///DFSDM channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch4dlyr](index.html) module
pub struct CH4DLYR_SPEC;
impl crate::RegisterSpec for CH4DLYR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ch4dlyr::R](R) reader structure
impl crate::Readable for CH4DLYR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ch4dlyr::W](W) writer structure
impl crate::Writable for CH4DLYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CH4DLYR to value 0
impl crate::Resettable for CH4DLYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
