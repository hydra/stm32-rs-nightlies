///Register `DAC_DHR12L1` reader
pub struct R(crate::R<DAC_DHR12L1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DHR12L1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DHR12L1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DHR12L1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAC_DHR12L1` writer
pub struct W(crate::W<DAC_DHR12L1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DHR12L1_SPEC>;
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
impl From<crate::W<DAC_DHR12L1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DHR12L1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data
pub type DACC1DHR_R = crate::FieldReader<u16, u16>;
///Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data
pub type DACC1DHR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12L1_SPEC, u16, u16, 12, O>;
///Field `DACC1DHRB` reader - DAC channel1 12-bit left-aligned data B
pub type DACC1DHRB_R = crate::FieldReader<u16, u16>;
///Field `DACC1DHRB` writer - DAC channel1 12-bit left-aligned data B
pub type DACC1DHRB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12L1_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 20:31 - DAC channel1 12-bit left-aligned data B
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<4> {
        DACC1DHR_W::new(self)
    }
    ///Bits 20:31 - DAC channel1 12-bit left-aligned data B
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<20> {
        DACC1DHRB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC channel1 12-bit left aligned data holding register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_dhr12l1](index.html) module
pub struct DAC_DHR12L1_SPEC;
impl crate::RegisterSpec for DAC_DHR12L1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_dhr12l1::R](R) reader structure
impl crate::Readable for DAC_DHR12L1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dac_dhr12l1::W](W) writer structure
impl crate::Writable for DAC_DHR12L1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_DHR12L1 to value 0
impl crate::Resettable for DAC_DHR12L1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
