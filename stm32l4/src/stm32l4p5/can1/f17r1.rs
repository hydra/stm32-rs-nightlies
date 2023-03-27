///Register `F17R1` reader
pub struct R(crate::R<F17R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F17R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F17R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F17R1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `F17R1` writer
pub struct W(crate::W<F17R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F17R1_SPEC>;
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
impl From<crate::W<F17R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F17R1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FB` reader - Filter bits
pub type FB_R = crate::FieldReader<u32, u32>;
///Field `FB` writer - Filter bits
pub type FB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F17R1_SPEC, u32, u32, 32, O>;
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
///Filter bank 17 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [f17r1](index.html) module
pub struct F17R1_SPEC;
impl crate::RegisterSpec for F17R1_SPEC {
    type Ux = u32;
}
///`read()` method returns [f17r1::R](R) reader structure
impl crate::Readable for F17R1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [f17r1::W](W) writer structure
impl crate::Writable for F17R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets F17R1 to value 0
impl crate::Resettable for F17R1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
