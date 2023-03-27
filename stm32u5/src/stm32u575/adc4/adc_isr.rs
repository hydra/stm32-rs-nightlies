///Register `ADC_ISR` reader
pub struct R(crate::R<ADC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_ISR` writer
pub struct W(crate::W<ADC_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ISR_SPEC>;
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
impl From<crate::W<ADC_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADRDY` reader - ADRDY
pub type ADRDY_R = crate::BitReader<bool>;
///Field `ADRDY` writer - ADRDY
pub type ADRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `EOSMP` reader - EOSMP
pub type EOSMP_R = crate::BitReader<bool>;
///Field `EOSMP` writer - EOSMP
pub type EOSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `EOC` reader - EOC
pub type EOC_R = crate::BitReader<bool>;
///Field `EOC` writer - EOC
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `EOS` reader - EOS
pub type EOS_R = crate::BitReader<bool>;
///Field `EOS` writer - EOS
pub type EOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `OVR` reader - OVR
pub type OVR_R = crate::BitReader<bool>;
///Field `OVR` writer - OVR
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `AWD1` reader - AWD1
pub type AWD1_R = crate::BitReader<bool>;
///Field `AWD1` writer - AWD1
pub type AWD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `AWD2` reader - AWD2
pub type AWD2_R = crate::BitReader<bool>;
///Field `AWD2` writer - AWD2
pub type AWD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `AWD3` reader - AWD3
pub type AWD3_R = crate::BitReader<bool>;
///Field `AWD3` writer - AWD3
pub type AWD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `EOCAL` reader - EOCAL
pub type EOCAL_R = crate::BitReader<bool>;
///Field `EOCAL` writer - EOCAL
pub type EOCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
///Field `LDORDY` reader - LDORDY
pub type LDORDY_R = crate::BitReader<bool>;
///Field `LDORDY` writer - LDORDY
pub type LDORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_ISR_SPEC, bool, O>;
impl R {
    ///Bit 0 - ADRDY
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - AWD1
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD2
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD3
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - EOCAL
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LDORDY
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ADRDY
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<0> {
        ADRDY_W::new(self)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<1> {
        EOSMP_W::new(self)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<2> {
        EOC_W::new(self)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<3> {
        EOS_W::new(self)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<4> {
        OVR_W::new(self)
    }
    ///Bit 7 - AWD1
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<7> {
        AWD1_W::new(self)
    }
    ///Bit 8 - AWD2
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<8> {
        AWD2_W::new(self)
    }
    ///Bit 9 - AWD3
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<9> {
        AWD3_W::new(self)
    }
    ///Bit 11 - EOCAL
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<11> {
        EOCAL_W::new(self)
    }
    ///Bit 12 - LDORDY
    #[inline(always)]
    #[must_use]
    pub fn ldordy(&mut self) -> LDORDY_W<12> {
        LDORDY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_isr](index.html) module
pub struct ADC_ISR_SPEC;
impl crate::RegisterSpec for ADC_ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_isr::R](R) reader structure
impl crate::Readable for ADC_ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_isr::W](W) writer structure
impl crate::Writable for ADC_ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_ISR to value 0
impl crate::Resettable for ADC_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
