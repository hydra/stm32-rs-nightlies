///Register `CMP1CCR` reader
pub struct R(crate::R<CMP1CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP1CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP1CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP1CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMP1CCR` writer
pub struct W(crate::W<CMP1CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP1CCR_SPEC>;
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
impl From<crate::W<CMP1CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP1CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMP1x` reader - Timerx Compare 1 value
pub type CMP1X_R = crate::FieldReader<u16, u16>;
///Field `CMP1x` writer - Timerx Compare 1 value
pub type CMP1X_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMP1CCR_SPEC, u16, u16, 16, O>;
///Field `REPx` reader - Timerx Repetition value (aliased from HRTIM_REPx register)
pub type REPX_R = crate::FieldReader<u8, u8>;
///Field `REPx` writer - Timerx Repetition value (aliased from HRTIM_REPx register)
pub type REPX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMP1CCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> CMP1X_W<0> {
        CMP1X_W::new(self)
    }
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<16> {
        REPX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Compare 1 Compound Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1ccr](index.html) module
pub struct CMP1CCR_SPEC;
impl crate::RegisterSpec for CMP1CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmp1ccr::R](R) reader structure
impl crate::Readable for CMP1CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmp1ccr::W](W) writer structure
impl crate::Writable for CMP1CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CMP1CCR to value 0
impl crate::Resettable for CMP1CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
