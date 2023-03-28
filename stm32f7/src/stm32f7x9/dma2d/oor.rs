///Register `OOR` reader
pub struct R(crate::R<OOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OOR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OOR` writer
pub struct W(crate::W<OOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OOR_SPEC>;
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
impl From<crate::W<OOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OOR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LO` reader - Line Offset
pub type LO_R = crate::FieldReader<u16, u16>;
///Field `LO` writer - Line Offset
pub type LO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OOR_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 0:13 - Line Offset
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Line Offset
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<0> {
        LO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///output offset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oor](index.html) module
pub struct OOR_SPEC;
impl crate::RegisterSpec for OOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [oor::R](R) reader structure
impl crate::Readable for OOR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [oor::W](W) writer structure
impl crate::Writable for OOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OOR to value 0
impl crate::Resettable for OOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
