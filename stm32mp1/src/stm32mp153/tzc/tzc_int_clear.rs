///Register `TZC_INT_CLEAR` reader
pub struct R(crate::R<TZC_INT_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_INT_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_INT_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_INT_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZC_INT_CLEAR` writer
pub struct W(crate::W<TZC_INT_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_INT_CLEAR_SPEC>;
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
impl From<crate::W<TZC_INT_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_INT_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CLEAR` writer - CLEAR
pub type CLEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TZC_INT_CLEAR_SPEC, u8, u8, 2, O>;
impl W {
    ///Bits 0:1 - CLEAR
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt clear for each filter.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_int_clear](index.html) module
pub struct TZC_INT_CLEAR_SPEC;
impl crate::RegisterSpec for TZC_INT_CLEAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_int_clear::R](R) reader structure
impl crate::Readable for TZC_INT_CLEAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzc_int_clear::W](W) writer structure
impl crate::Writable for TZC_INT_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZC_INT_CLEAR to value 0
impl crate::Resettable for TZC_INT_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
