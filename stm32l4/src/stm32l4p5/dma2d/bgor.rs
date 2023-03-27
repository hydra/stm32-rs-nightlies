///Register `BGOR` reader
pub struct R(crate::R<BGOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGOR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BGOR` writer
pub struct W(crate::W<BGOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGOR_SPEC>;
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
impl From<crate::W<BGOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGOR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LO` reader - Line offset
pub type LO_R = crate::FieldReader<u16, u16>;
///Field `LO` writer - Line offset
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGOR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Line offset
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Line offset
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
///background offset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgor](index.html) module
pub struct BGOR_SPEC;
impl crate::RegisterSpec for BGOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bgor::R](R) reader structure
impl crate::Readable for BGOR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bgor::W](W) writer structure
impl crate::Writable for BGOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BGOR to value 0
impl crate::Resettable for BGOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
