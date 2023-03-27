///Register `ADC_OFR4` reader
pub struct R(crate::R<ADC_OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_OFR4` writer
pub struct W(crate::W<ADC_OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR4_SPEC>;
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
impl From<crate::W<ADC_OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET` reader - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
///bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
///bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
///bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
///= 4 and OFFSET2_CH\[4:0\]
///= 4, this is OFFSET1\[25:0\]
///that is subtracted when converting channel 4.
pub type OFFSET_R = crate::FieldReader<u32, u32>;
///Field `OFFSET` writer - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
///bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
///bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
///bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
///= 4 and OFFSET2_CH\[4:0\]
///= 4, this is OFFSET1\[25:0\]
///that is subtracted when converting channel 4.
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR4_SPEC, u32, u32, 24, O>;
///Field `POSOFF` reader - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type POSOFF_R = crate::BitReader<bool>;
///Field `POSOFF` writer - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type POSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_OFR4_SPEC, bool, O>;
///Field `USAT` reader - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type USAT_R = crate::BitReader<bool>;
///Field `USAT` writer - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type USAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_OFR4_SPEC, bool, O>;
///Field `SSAT` reader - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SSAT_R = crate::BitReader<bool>;
///Field `SSAT` writer - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SSAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_OFR4_SPEC, bool, O>;
///Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
///bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
pub type OFFSET_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
///bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
pub type OFFSET_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR4_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
    ///bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
    ///bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
    ///bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
    ///= 4 and OFFSET2_CH\[4:0\]
    ///= 4, this is OFFSET1\[25:0\]
    ///that is subtracted when converting channel 4.
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn posoff(&self) -> POSOFF_R {
        POSOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn usat(&self) -> USAT_R {
        USAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ssat(&self) -> SSAT_R {
        SSAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
    ///bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
    ///bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
    ///bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
    ///bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
    ///= 4 and OFFSET2_CH\[4:0\]
    ///= 4, this is OFFSET1\[25:0\]
    ///that is subtracted when converting channel 4.
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<0> {
        OFFSET_W::new(self)
    }
    ///Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn posoff(&mut self) -> POSOFF_W<24> {
        POSOFF_W::new(self)
    }
    ///Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn usat(&mut self) -> USAT_W<25> {
        USAT_W::new(self)
    }
    ///Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn ssat(&mut self) -> SSAT_W<26> {
        SSAT_W::new(self)
    }
    ///Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
    ///bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
    #[inline(always)]
    #[must_use]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<27> {
        OFFSET_CH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC offset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_ofr4](index.html) module
pub struct ADC_OFR4_SPEC;
impl crate::RegisterSpec for ADC_OFR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_ofr4::R](R) reader structure
impl crate::Readable for ADC_OFR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_ofr4::W](W) writer structure
impl crate::Writable for ADC_OFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_OFR4 to value 0
impl crate::Resettable for ADC_OFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
