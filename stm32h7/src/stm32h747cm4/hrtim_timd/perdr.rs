///Register `PERDR` reader
pub struct R(crate::R<PERDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PERDR` writer
pub struct W(crate::W<PERDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERDR_SPEC>;
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
impl From<crate::W<PERDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PERx` reader - Timerx Period value
pub type PERX_R = crate::FieldReader<u16, u16>;
///Field `PERx` writer - Timerx Period value
pub type PERX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERDR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PERX_W<0> {
        PERX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [perdr](index.html) module
pub struct PERDR_SPEC;
impl crate::RegisterSpec for PERDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [perdr::R](R) reader structure
impl crate::Readable for PERDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [perdr::W](W) writer structure
impl crate::Writable for PERDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PERDR to value 0xffff
impl crate::Resettable for PERDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
