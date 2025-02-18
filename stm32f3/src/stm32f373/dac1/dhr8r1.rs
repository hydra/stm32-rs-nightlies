///Register `DHR8R1` reader
pub struct R(crate::R<DHR8R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR8R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR8R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR8R1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DHR8R1` writer
pub struct W(crate::W<DHR8R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR8R1_SPEC>;
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
impl From<crate::W<DHR8R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR8R1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data
pub type DACC1DHR_R = crate::FieldReader<u8, u8>;
///Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data
pub type DACC1DHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHR8R1_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<0> {
        DACC1DHR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel1 8-bit right aligned data holding register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dhr8r1](index.html) module
pub struct DHR8R1_SPEC;
impl crate::RegisterSpec for DHR8R1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dhr8r1::R](R) reader structure
impl crate::Readable for DHR8R1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dhr8r1::W](W) writer structure
impl crate::Writable for DHR8R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DHR8R1 to value 0
impl crate::Resettable for DHR8R1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
