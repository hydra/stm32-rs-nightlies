///Register `DHR12L2` reader
pub struct R(crate::R<DHR12L2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR12L2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR12L2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR12L2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DHR12L2` writer
pub struct W(crate::W<DHR12L2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR12L2_SPEC>;
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
impl From<crate::W<DHR12L2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR12L2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2.
pub type DACC2DHR_R = crate::FieldReader<u16, u16>;
///Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2.
pub type DACC2DHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHR12L2_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 4:15 - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2.
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 4:15 - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2.
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<4> {
        DACC2DHR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC channel2 12-bit left aligned data holding register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dhr12l2](index.html) module
pub struct DHR12L2_SPEC;
impl crate::RegisterSpec for DHR12L2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dhr12l2::R](R) reader structure
impl crate::Readable for DHR12L2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dhr12l2::W](W) writer structure
impl crate::Writable for DHR12L2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DHR12L2 to value 0
impl crate::Resettable for DHR12L2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
