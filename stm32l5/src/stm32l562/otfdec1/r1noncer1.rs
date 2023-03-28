///Register `R1NONCER1` reader
pub struct R(crate::R<R1NONCER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R1NONCER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R1NONCER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R1NONCER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `R1NONCER1` writer
pub struct W(crate::W<R1NONCER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R1NONCER1_SPEC>;
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
impl From<crate::W<R1NONCER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R1NONCER1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REGx_NONCE` reader - Region nonce
pub type REGX_NONCE_R = crate::FieldReader<u32, u32>;
///Field `REGx_NONCE` writer - Region nonce
pub type REGX_NONCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R1NONCER1_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Region nonce
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Region nonce
    #[inline(always)]
    #[must_use]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<0> {
        REGX_NONCE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTFDEC region x nonce register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r1noncer1](index.html) module
pub struct R1NONCER1_SPEC;
impl crate::RegisterSpec for R1NONCER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [r1noncer1::R](R) reader structure
impl crate::Readable for R1NONCER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [r1noncer1::W](W) writer structure
impl crate::Writable for R1NONCER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets R1NONCER1 to value 0
impl crate::Resettable for R1NONCER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
