///Register `ADC_SMPR` reader
pub struct R(crate::R<ADC_SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SMPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_SMPR` writer
pub struct W(crate::W<ADC_SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SMPR_SPEC>;
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
impl From<crate::W<ADC_SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SMPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMP1` reader - SMP1
pub type SMP1_R = crate::FieldReader<u8, u8>;
///Field `SMP1` writer - SMP1
pub type SMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_SMPR_SPEC, u8, u8, 3, O>;
///Field `SMP2` reader - SMP2
pub type SMP2_R = crate::FieldReader<u8, u8>;
///Field `SMP2` writer - SMP2
pub type SMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_SMPR_SPEC, u8, u8, 3, O>;
///Field `SMPSEL0` reader - SMPSEL0
pub type SMPSEL0_R = crate::BitReader<bool>;
///Field `SMPSEL0` writer - SMPSEL0
pub type SMPSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL1` reader - SMPSEL1
pub type SMPSEL1_R = crate::BitReader<bool>;
///Field `SMPSEL1` writer - SMPSEL1
pub type SMPSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL2` reader - SMPSEL2
pub type SMPSEL2_R = crate::BitReader<bool>;
///Field `SMPSEL2` writer - SMPSEL2
pub type SMPSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL3` reader - SMPSEL3
pub type SMPSEL3_R = crate::BitReader<bool>;
///Field `SMPSEL3` writer - SMPSEL3
pub type SMPSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL4` reader - SMPSEL4
pub type SMPSEL4_R = crate::BitReader<bool>;
///Field `SMPSEL4` writer - SMPSEL4
pub type SMPSEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL5` reader - SMPSEL5
pub type SMPSEL5_R = crate::BitReader<bool>;
///Field `SMPSEL5` writer - SMPSEL5
pub type SMPSEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL6` reader - SMPSEL6
pub type SMPSEL6_R = crate::BitReader<bool>;
///Field `SMPSEL6` writer - SMPSEL6
pub type SMPSEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL7` reader - SMPSEL7
pub type SMPSEL7_R = crate::BitReader<bool>;
///Field `SMPSEL7` writer - SMPSEL7
pub type SMPSEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL8` reader - SMPSEL8
pub type SMPSEL8_R = crate::BitReader<bool>;
///Field `SMPSEL8` writer - SMPSEL8
pub type SMPSEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL9` reader - SMPSEL9
pub type SMPSEL9_R = crate::BitReader<bool>;
///Field `SMPSEL9` writer - SMPSEL9
pub type SMPSEL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL10` reader - SMPSEL10
pub type SMPSEL10_R = crate::BitReader<bool>;
///Field `SMPSEL10` writer - SMPSEL10
pub type SMPSEL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL11` reader - SMPSEL11
pub type SMPSEL11_R = crate::BitReader<bool>;
///Field `SMPSEL11` writer - SMPSEL11
pub type SMPSEL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL12` reader - SMPSEL12
pub type SMPSEL12_R = crate::BitReader<bool>;
///Field `SMPSEL12` writer - SMPSEL12
pub type SMPSEL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL13` reader - SMPSEL13
pub type SMPSEL13_R = crate::BitReader<bool>;
///Field `SMPSEL13` writer - SMPSEL13
pub type SMPSEL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL14` reader - SMPSEL14
pub type SMPSEL14_R = crate::BitReader<bool>;
///Field `SMPSEL14` writer - SMPSEL14
pub type SMPSEL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL15` reader - SMPSEL15
pub type SMPSEL15_R = crate::BitReader<bool>;
///Field `SMPSEL15` writer - SMPSEL15
pub type SMPSEL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL16` reader - SMPSEL16
pub type SMPSEL16_R = crate::BitReader<bool>;
///Field `SMPSEL16` writer - SMPSEL16
pub type SMPSEL16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL17` reader - SMPSEL17
pub type SMPSEL17_R = crate::BitReader<bool>;
///Field `SMPSEL17` writer - SMPSEL17
pub type SMPSEL17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL18` reader - SMPSEL18
pub type SMPSEL18_R = crate::BitReader<bool>;
///Field `SMPSEL18` writer - SMPSEL18
pub type SMPSEL18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL19` reader - SMPSEL19
pub type SMPSEL19_R = crate::BitReader<bool>;
///Field `SMPSEL19` writer - SMPSEL19
pub type SMPSEL19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL20` reader - SMPSEL20
pub type SMPSEL20_R = crate::BitReader<bool>;
///Field `SMPSEL20` writer - SMPSEL20
pub type SMPSEL20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL21` reader - SMPSEL21
pub type SMPSEL21_R = crate::BitReader<bool>;
///Field `SMPSEL21` writer - SMPSEL21
pub type SMPSEL21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL22` reader - SMPSEL22
pub type SMPSEL22_R = crate::BitReader<bool>;
///Field `SMPSEL22` writer - SMPSEL22
pub type SMPSEL22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
///Field `SMPSEL23` reader - SMPSEL23
pub type SMPSEL23_R = crate::BitReader<bool>;
///Field `SMPSEL23` writer - SMPSEL23
pub type SMPSEL23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SMPR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - SMP1
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - SMP2
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - SMPSEL0
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SMPSEL1
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SMPSEL2
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SMPSEL3
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SMPSEL4
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SMPSEL5
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SMPSEL6
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SMPSEL7
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SMPSEL8
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SMPSEL9
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SMPSEL10
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SMPSEL11
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SMPSEL12
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SMPSEL13
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SMPSEL14
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SMPSEL15
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SMPSEL16
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SMPSEL17
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SMPSEL18
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SMPSEL19
    #[inline(always)]
    pub fn smpsel19(&self) -> SMPSEL19_R {
        SMPSEL19_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SMPSEL20
    #[inline(always)]
    pub fn smpsel20(&self) -> SMPSEL20_R {
        SMPSEL20_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SMPSEL21
    #[inline(always)]
    pub fn smpsel21(&self) -> SMPSEL21_R {
        SMPSEL21_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SMPSEL22
    #[inline(always)]
    pub fn smpsel22(&self) -> SMPSEL22_R {
        SMPSEL22_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SMPSEL23
    #[inline(always)]
    pub fn smpsel23(&self) -> SMPSEL23_R {
        SMPSEL23_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - SMP1
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<0> {
        SMP1_W::new(self)
    }
    ///Bits 4:6 - SMP2
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<4> {
        SMP2_W::new(self)
    }
    ///Bit 8 - SMPSEL0
    #[inline(always)]
    #[must_use]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<8> {
        SMPSEL0_W::new(self)
    }
    ///Bit 9 - SMPSEL1
    #[inline(always)]
    #[must_use]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<9> {
        SMPSEL1_W::new(self)
    }
    ///Bit 10 - SMPSEL2
    #[inline(always)]
    #[must_use]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<10> {
        SMPSEL2_W::new(self)
    }
    ///Bit 11 - SMPSEL3
    #[inline(always)]
    #[must_use]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<11> {
        SMPSEL3_W::new(self)
    }
    ///Bit 12 - SMPSEL4
    #[inline(always)]
    #[must_use]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<12> {
        SMPSEL4_W::new(self)
    }
    ///Bit 13 - SMPSEL5
    #[inline(always)]
    #[must_use]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<13> {
        SMPSEL5_W::new(self)
    }
    ///Bit 14 - SMPSEL6
    #[inline(always)]
    #[must_use]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<14> {
        SMPSEL6_W::new(self)
    }
    ///Bit 15 - SMPSEL7
    #[inline(always)]
    #[must_use]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<15> {
        SMPSEL7_W::new(self)
    }
    ///Bit 16 - SMPSEL8
    #[inline(always)]
    #[must_use]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<16> {
        SMPSEL8_W::new(self)
    }
    ///Bit 17 - SMPSEL9
    #[inline(always)]
    #[must_use]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<17> {
        SMPSEL9_W::new(self)
    }
    ///Bit 18 - SMPSEL10
    #[inline(always)]
    #[must_use]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<18> {
        SMPSEL10_W::new(self)
    }
    ///Bit 19 - SMPSEL11
    #[inline(always)]
    #[must_use]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<19> {
        SMPSEL11_W::new(self)
    }
    ///Bit 20 - SMPSEL12
    #[inline(always)]
    #[must_use]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<20> {
        SMPSEL12_W::new(self)
    }
    ///Bit 21 - SMPSEL13
    #[inline(always)]
    #[must_use]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<21> {
        SMPSEL13_W::new(self)
    }
    ///Bit 22 - SMPSEL14
    #[inline(always)]
    #[must_use]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<22> {
        SMPSEL14_W::new(self)
    }
    ///Bit 23 - SMPSEL15
    #[inline(always)]
    #[must_use]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<23> {
        SMPSEL15_W::new(self)
    }
    ///Bit 24 - SMPSEL16
    #[inline(always)]
    #[must_use]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<24> {
        SMPSEL16_W::new(self)
    }
    ///Bit 25 - SMPSEL17
    #[inline(always)]
    #[must_use]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<25> {
        SMPSEL17_W::new(self)
    }
    ///Bit 26 - SMPSEL18
    #[inline(always)]
    #[must_use]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<26> {
        SMPSEL18_W::new(self)
    }
    ///Bit 27 - SMPSEL19
    #[inline(always)]
    #[must_use]
    pub fn smpsel19(&mut self) -> SMPSEL19_W<27> {
        SMPSEL19_W::new(self)
    }
    ///Bit 28 - SMPSEL20
    #[inline(always)]
    #[must_use]
    pub fn smpsel20(&mut self) -> SMPSEL20_W<28> {
        SMPSEL20_W::new(self)
    }
    ///Bit 29 - SMPSEL21
    #[inline(always)]
    #[must_use]
    pub fn smpsel21(&mut self) -> SMPSEL21_W<29> {
        SMPSEL21_W::new(self)
    }
    ///Bit 30 - SMPSEL22
    #[inline(always)]
    #[must_use]
    pub fn smpsel22(&mut self) -> SMPSEL22_W<30> {
        SMPSEL22_W::new(self)
    }
    ///Bit 31 - SMPSEL23
    #[inline(always)]
    #[must_use]
    pub fn smpsel23(&mut self) -> SMPSEL23_W<31> {
        SMPSEL23_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC sample time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_smpr](index.html) module
pub struct ADC_SMPR_SPEC;
impl crate::RegisterSpec for ADC_SMPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_smpr::R](R) reader structure
impl crate::Readable for ADC_SMPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_smpr::W](W) writer structure
impl crate::Writable for ADC_SMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_SMPR to value 0
impl crate::Resettable for ADC_SMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
