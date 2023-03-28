///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADON` reader - A/D Converter ON / OFF
pub type ADON_R = crate::BitReader<bool>;
///Field `ADON` writer - A/D Converter ON / OFF
pub type ADON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `CONT` reader - Continuous conversion
pub type CONT_R = crate::BitReader<bool>;
///Field `CONT` writer - Continuous conversion
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `ADC_CFG` reader - ADC configuration
pub type ADC_CFG_R = crate::BitReader<bool>;
///Field `ADC_CFG` writer - ADC configuration
pub type ADC_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `DELS` reader - Delay selection
pub type DELS_R = crate::FieldReader<u8, u8>;
///Field `DELS` writer - Delay selection
pub type DELS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
///Field `DMA` reader - Direct memory access mode
pub type DMA_R = crate::BitReader<bool>;
///Field `DMA` writer - Direct memory access mode
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `DDS` reader - DMA disable selection
pub type DDS_R = crate::BitReader<bool>;
///Field `DDS` writer - DMA disable selection
pub type DDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `EOCS` reader - End of conversion selection
pub type EOCS_R = crate::BitReader<bool>;
///Field `EOCS` writer - End of conversion selection
pub type EOCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `ALIGN` reader - Data alignment
pub type ALIGN_R = crate::BitReader<bool>;
///Field `ALIGN` writer - Data alignment
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `JEXTSEL` reader - External event select for injected group
pub type JEXTSEL_R = crate::FieldReader<u8, u8>;
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
///Field `JEXTEN` reader - External trigger enable for injected channels
pub type JEXTEN_R = crate::FieldReader<u8, u8>;
///Field `JEXTEN` writer - External trigger enable for injected channels
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
///Field `JSWSTART` reader - Start conversion of injected channels
pub type JSWSTART_R = crate::BitReader<bool>;
///Field `JSWSTART` writer - Start conversion of injected channels
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `EXTSEL` reader - External event select for regular group
pub type EXTSEL_R = crate::FieldReader<u8, u8>;
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
///Field `EXTEN` reader - External trigger enable for regular channels
pub type EXTEN_R = crate::FieldReader<u8, u8>;
///Field `EXTEN` writer - External trigger enable for regular channels
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
///Field `SWSTART` reader - Start conversion of regular channels
pub type SWSTART_R = crate::BitReader<bool>;
///Field `SWSTART` writer - Start conversion of regular channels
pub type SWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC configuration
    #[inline(always)]
    pub fn adc_cfg(&self) -> ADC_CFG_R {
        ADC_CFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - Delay selection
    #[inline(always)]
    pub fn dels(&self) -> DELS_R {
        DELS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA disable selection
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&self) -> EOCS_R {
        EOCS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> ADON_W<0> {
        ADON_W::new(self)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<1> {
        CONT_W::new(self)
    }
    ///Bit 2 - ADC configuration
    #[inline(always)]
    #[must_use]
    pub fn adc_cfg(&mut self) -> ADC_CFG_W<2> {
        ADC_CFG_W::new(self)
    }
    ///Bits 4:6 - Delay selection
    #[inline(always)]
    #[must_use]
    pub fn dels(&mut self) -> DELS_W<4> {
        DELS_W::new(self)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<8> {
        DMA_W::new(self)
    }
    ///Bit 9 - DMA disable selection
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<9> {
        DDS_W::new(self)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    #[must_use]
    pub fn eocs(&mut self) -> EOCS_W<10> {
        EOCS_W::new(self)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<11> {
        ALIGN_W::new(self)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<16> {
        JEXTSEL_W::new(self)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<20> {
        JEXTEN_W::new(self)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<22> {
        JSWSTART_W::new(self)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<24> {
        EXTSEL_W::new(self)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<28> {
        EXTEN_W::new(self)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    #[must_use]
    pub fn swstart(&mut self) -> SWSTART_W<30> {
        SWSTART_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
