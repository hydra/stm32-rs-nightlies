///Register `ADC_CFGR2` reader
pub struct R(crate::R<ADC_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_CFGR2` writer
pub struct W(crate::W<ADC_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFGR2_SPEC>;
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
impl From<crate::W<ADC_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OVSE` reader - OVSE
pub type OVSE_R = crate::BitReader<bool>;
///Field `OVSE` writer - OVSE
pub type OVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
///Field `OVSR` reader - OVSR
pub type OVSR_R = crate::FieldReader<u8, u8>;
///Field `OVSR` writer - OVSR
pub type OVSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_CFGR2_SPEC, u8, u8, 3, O>;
///Field `OVSS` reader - OVSS
pub type OVSS_R = crate::FieldReader<u8, u8>;
///Field `OVSS` writer - OVSS
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_CFGR2_SPEC, u8, u8, 4, O>;
///Field `TOVS` reader - TOVS
pub type TOVS_R = crate::BitReader<bool>;
///Field `TOVS` writer - TOVS
pub type TOVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
///Field `LFTRIG` reader - LFTRIG
pub type LFTRIG_R = crate::BitReader<bool>;
///Field `LFTRIG` writer - LFTRIG
pub type LFTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - OVSE
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - OVSR
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - TOVS
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - LFTRIG
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - OVSE
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<0> {
        OVSE_W::new(self)
    }
    ///Bits 2:4 - OVSR
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<2> {
        OVSR_W::new(self)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    ///Bit 9 - TOVS
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<9> {
        TOVS_W::new(self)
    }
    ///Bit 29 - LFTRIG
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<29> {
        LFTRIG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_cfgr2](index.html) module
pub struct ADC_CFGR2_SPEC;
impl crate::RegisterSpec for ADC_CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_cfgr2::R](R) reader structure
impl crate::Readable for ADC_CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_cfgr2::W](W) writer structure
impl crate::Writable for ADC_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_CFGR2 to value 0
impl crate::Resettable for ADC_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
