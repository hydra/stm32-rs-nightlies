///Register `VNPCR` reader
pub struct R(crate::R<VNPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VNPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VNPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VNPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VNPCR` writer
pub struct W(crate::W<VNPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VNPCR_SPEC>;
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
impl From<crate::W<VNPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VNPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NPSIZE` reader - Null Packet Size
pub type NPSIZE_R = crate::FieldReader<u16, u16>;
///Field `NPSIZE` writer - Null Packet Size
pub type NPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VNPCR_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 0:13 - Null Packet Size
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Null Packet Size
    #[inline(always)]
    #[must_use]
    pub fn npsize(&mut self) -> NPSIZE_W<0> {
        NPSIZE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Video Null Packet Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vnpcr](index.html) module
pub struct VNPCR_SPEC;
impl crate::RegisterSpec for VNPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vnpcr::R](R) reader structure
impl crate::Readable for VNPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vnpcr::W](W) writer structure
impl crate::Writable for VNPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VNPCR to value 0
impl crate::Resettable for VNPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
