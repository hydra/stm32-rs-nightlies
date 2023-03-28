///Register `REPDR` reader
pub struct R(crate::R<REPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `REPDR` writer
pub struct W(crate::W<REPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REPDR_SPEC>;
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
impl From<crate::W<REPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REPx` reader - Timerx Repetition counter value
pub type REPX_R = crate::FieldReader<u8, u8>;
///Field `REPx` writer - Timerx Repetition counter value
pub type REPX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, REPDR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<0> {
        REPX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [repdr](index.html) module
pub struct REPDR_SPEC;
impl crate::RegisterSpec for REPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [repdr::R](R) reader structure
impl crate::Readable for REPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [repdr::W](W) writer structure
impl crate::Writable for REPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets REPDR to value 0
impl crate::Resettable for REPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
