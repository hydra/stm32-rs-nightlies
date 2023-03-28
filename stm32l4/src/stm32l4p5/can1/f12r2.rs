///Register `F12R2` reader
pub struct R(crate::R<F12R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F12R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F12R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F12R2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `F12R2` writer
pub struct W(crate::W<F12R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F12R2_SPEC>;
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
impl From<crate::W<F12R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F12R2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FB` reader - Filter bits
pub type FB_R = crate::FieldReader<u32, u32>;
///Field `FB` writer - Filter bits
pub type FB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F12R2_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Filter bits
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Filter bits
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<0> {
        FB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Filter bank 12 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [f12r2](index.html) module
pub struct F12R2_SPEC;
impl crate::RegisterSpec for F12R2_SPEC {
    type Ux = u32;
}
///`read()` method returns [f12r2::R](R) reader structure
impl crate::Readable for F12R2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [f12r2::W](W) writer structure
impl crate::Writable for F12R2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets F12R2 to value 0
impl crate::Resettable for F12R2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
