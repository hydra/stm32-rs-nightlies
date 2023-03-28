///Register `CSL` reader
pub struct R(crate::R<CSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSL` writer
pub struct W(crate::W<CSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSL_SPEC>;
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
impl From<crate::W<CSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LENG` reader - code segment length
pub type LENG_R = crate::FieldReader<u16, u16>;
///Field `LENG` writer - code segment length
pub type LENG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSL_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 8:21 - code segment length
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 8:21 - code segment length
    #[inline(always)]
    #[must_use]
    pub fn leng(&mut self) -> LENG_W<8> {
        LENG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Code segment length
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csl](index.html) module
pub struct CSL_SPEC;
impl crate::RegisterSpec for CSL_SPEC {
    type Ux = u32;
}
///`read()` method returns [csl::R](R) reader structure
impl crate::Readable for CSL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csl::W](W) writer structure
impl crate::Writable for CSL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSL to value 0
impl crate::Resettable for CSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
