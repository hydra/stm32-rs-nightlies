///Register `REPER` reader
pub struct R(crate::R<REPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REPER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `REPER` writer
pub struct W(crate::W<REPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REPER_SPEC>;
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
impl From<crate::W<REPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REPER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REPx` reader - Timerx Repetition counter value
pub type REPX_R = crate::FieldReader<u8, u8>;
///Field `REPx` writer - Timerx Repetition counter value
pub type REPX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, REPER_SPEC, u8, u8, 8, O>;
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
///For information about available fields see [reper](index.html) module
pub struct REPER_SPEC;
impl crate::RegisterSpec for REPER_SPEC {
    type Ux = u32;
}
///`read()` method returns [reper::R](R) reader structure
impl crate::Readable for REPER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [reper::W](W) writer structure
impl crate::Writable for REPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets REPER to value 0
impl crate::Resettable for REPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
