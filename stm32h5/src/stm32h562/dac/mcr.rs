///Register `MCR` reader
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCR` writer
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MODE1` reader - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1 = 0.
pub type MODE1_R = crate::FieldReader<u8, u8>;
///Field `MODE1` writer - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1 = 0.
pub type MODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, u8, 3, O>;
///Field `DMADOUBLE1` reader - DAC channel1 DMA double data mode This bit is set and cleared by software.
pub type DMADOUBLE1_R = crate::BitReader<bool>;
///Field `DMADOUBLE1` writer - DAC channel1 DMA double data mode This bit is set and cleared by software.
pub type DMADOUBLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `SINFORMAT1` reader - Enable signed format for DAC channel1 This bit is set and cleared by software.
pub type SINFORMAT1_R = crate::BitReader<bool>;
///Field `SINFORMAT1` writer - Enable signed format for DAC channel1 This bit is set and cleared by software.
pub type SINFORMAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `HFSEL0` reader - High frequency interface mode selection
pub type HFSEL0_R = crate::BitReader<bool>;
///Field `HFSEL0` writer - High frequency interface mode selection
pub type HFSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `HFSEL1` reader - High frequency interface mode selection
pub type HFSEL1_R = crate::BitReader<bool>;
///Field `HFSEL1` writer - High frequency interface mode selection
pub type HFSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `MODE2` reader - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
pub type MODE2_R = crate::FieldReader<u8, u8>;
///Field `MODE2` writer - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
pub type MODE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, u8, 3, O>;
///Field `DMADOUBLE2` reader - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type DMADOUBLE2_R = crate::BitReader<bool>;
///Field `DMADOUBLE2` writer - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type DMADOUBLE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `SINFORMAT2` reader - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type SINFORMAT2_R = crate::BitReader<bool>;
///Field `SINFORMAT2` writer - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type SINFORMAT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1 = 0.
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - DAC channel1 DMA double data mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmadouble1(&self) -> DMADOUBLE1_R {
        DMADOUBLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable signed format for DAC channel1 This bit is set and cleared by software.
    #[inline(always)]
    pub fn sinformat1(&self) -> SINFORMAT1_R {
        SINFORMAT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel0(&self) -> HFSEL0_R {
        HFSEL0_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel1(&self) -> HFSEL1_R {
        HFSEL1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 24 - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn dmadouble2(&self) -> DMADOUBLE2_R {
        DMADOUBLE2_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn sinformat2(&self) -> SINFORMAT2_R {
        SINFORMAT2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1 = 0.
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<0> {
        MODE1_W::new(self)
    }
    ///Bit 8 - DAC channel1 DMA double data mode This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dmadouble1(&mut self) -> DMADOUBLE1_W<8> {
        DMADOUBLE1_W::new(self)
    }
    ///Bit 9 - Enable signed format for DAC channel1 This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sinformat1(&mut self) -> SINFORMAT1_W<9> {
        SINFORMAT1_W::new(self)
    }
    ///Bit 14 - High frequency interface mode selection
    #[inline(always)]
    #[must_use]
    pub fn hfsel0(&mut self) -> HFSEL0_W<14> {
        HFSEL0_W::new(self)
    }
    ///Bit 15 - High frequency interface mode selection
    #[inline(always)]
    #[must_use]
    pub fn hfsel1(&mut self) -> HFSEL1_W<15> {
        HFSEL1_W::new(self)
    }
    ///Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<16> {
        MODE2_W::new(self)
    }
    ///Bit 24 - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    #[must_use]
    pub fn dmadouble2(&mut self) -> DMADOUBLE2_W<24> {
        DMADOUBLE2_W::new(self)
    }
    ///Bit 25 - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    #[must_use]
    pub fn sinformat2(&mut self) -> SINFORMAT2_W<25> {
        SINFORMAT2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](index.html) module
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcr::R](R) reader structure
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcr::W](W) writer structure
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
