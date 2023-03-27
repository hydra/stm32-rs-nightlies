///Register `ADC_AWD3CR` reader
pub struct R(crate::R<ADC_AWD3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_AWD3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_AWD3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_AWD3CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_AWD3CR` writer
pub struct W(crate::W<ADC_AWD3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_AWD3CR_SPEC>;
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
impl From<crate::W<ADC_AWD3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_AWD3CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AWD3CH0` reader - AWD3CH0
pub type AWD3CH0_R = crate::BitReader<bool>;
///Field `AWD3CH0` writer - AWD3CH0
pub type AWD3CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH1` reader - AWD3CH1
pub type AWD3CH1_R = crate::BitReader<bool>;
///Field `AWD3CH1` writer - AWD3CH1
pub type AWD3CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH2` reader - AWD3CH2
pub type AWD3CH2_R = crate::BitReader<bool>;
///Field `AWD3CH2` writer - AWD3CH2
pub type AWD3CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH3` reader - AWD3CH3
pub type AWD3CH3_R = crate::BitReader<bool>;
///Field `AWD3CH3` writer - AWD3CH3
pub type AWD3CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH4` reader - AWD3CH4
pub type AWD3CH4_R = crate::BitReader<bool>;
///Field `AWD3CH4` writer - AWD3CH4
pub type AWD3CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH5` reader - AWD3CH5
pub type AWD3CH5_R = crate::BitReader<bool>;
///Field `AWD3CH5` writer - AWD3CH5
pub type AWD3CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH6` reader - AWD3CH6
pub type AWD3CH6_R = crate::BitReader<bool>;
///Field `AWD3CH6` writer - AWD3CH6
pub type AWD3CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH7` reader - AWD3CH7
pub type AWD3CH7_R = crate::BitReader<bool>;
///Field `AWD3CH7` writer - AWD3CH7
pub type AWD3CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH8` reader - AWD3CH8
pub type AWD3CH8_R = crate::BitReader<bool>;
///Field `AWD3CH8` writer - AWD3CH8
pub type AWD3CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH9` reader - AWD3CH9
pub type AWD3CH9_R = crate::BitReader<bool>;
///Field `AWD3CH9` writer - AWD3CH9
pub type AWD3CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH10` reader - AWD3CH10
pub type AWD3CH10_R = crate::BitReader<bool>;
///Field `AWD3CH10` writer - AWD3CH10
pub type AWD3CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH11` reader - AWD3CH11
pub type AWD3CH11_R = crate::BitReader<bool>;
///Field `AWD3CH11` writer - AWD3CH11
pub type AWD3CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH12` reader - AWD3CH12
pub type AWD3CH12_R = crate::BitReader<bool>;
///Field `AWD3CH12` writer - AWD3CH12
pub type AWD3CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH13` reader - AWD3CH13
pub type AWD3CH13_R = crate::BitReader<bool>;
///Field `AWD3CH13` writer - AWD3CH13
pub type AWD3CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH14` reader - AWD3CH14
pub type AWD3CH14_R = crate::BitReader<bool>;
///Field `AWD3CH14` writer - AWD3CH14
pub type AWD3CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH15` reader - AWD3CH15
pub type AWD3CH15_R = crate::BitReader<bool>;
///Field `AWD3CH15` writer - AWD3CH15
pub type AWD3CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH16` reader - AWD3CH16
pub type AWD3CH16_R = crate::BitReader<bool>;
///Field `AWD3CH16` writer - AWD3CH16
pub type AWD3CH16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH17` reader - AWD3CH17
pub type AWD3CH17_R = crate::BitReader<bool>;
///Field `AWD3CH17` writer - AWD3CH17
pub type AWD3CH17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH18` reader - AWD3CH18
pub type AWD3CH18_R = crate::BitReader<bool>;
///Field `AWD3CH18` writer - AWD3CH18
pub type AWD3CH18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH19` reader - AWD3CH19
pub type AWD3CH19_R = crate::BitReader<bool>;
///Field `AWD3CH19` writer - AWD3CH19
pub type AWD3CH19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH20` reader - AWD3CH20
pub type AWD3CH20_R = crate::BitReader<bool>;
///Field `AWD3CH20` writer - AWD3CH20
pub type AWD3CH20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH21` reader - AWD3CH21
pub type AWD3CH21_R = crate::BitReader<bool>;
///Field `AWD3CH21` writer - AWD3CH21
pub type AWD3CH21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH22` reader - AWD3CH22
pub type AWD3CH22_R = crate::BitReader<bool>;
///Field `AWD3CH22` writer - AWD3CH22
pub type AWD3CH22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
///Field `AWD3CH23` reader - AWD3CH23
pub type AWD3CH23_R = crate::BitReader<bool>;
///Field `AWD3CH23` writer - AWD3CH23
pub type AWD3CH23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_AWD3CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - AWD3CH0
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH0_R {
        AWD3CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AWD3CH1
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH1_R {
        AWD3CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AWD3CH2
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH2_R {
        AWD3CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AWD3CH3
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH3_R {
        AWD3CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AWD3CH4
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH4_R {
        AWD3CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AWD3CH5
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH5_R {
        AWD3CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AWD3CH6
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH6_R {
        AWD3CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AWD3CH7
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH7_R {
        AWD3CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD3CH8
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH8_R {
        AWD3CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD3CH9
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH9_R {
        AWD3CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AWD3CH10
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH10_R {
        AWD3CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AWD3CH11
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH11_R {
        AWD3CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AWD3CH12
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH12_R {
        AWD3CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AWD3CH13
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH13_R {
        AWD3CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AWD3CH14
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH14_R {
        AWD3CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AWD3CH15
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH15_R {
        AWD3CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AWD3CH16
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH16_R {
        AWD3CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AWD3CH17
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH17_R {
        AWD3CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AWD3CH18
    #[inline(always)]
    pub fn awd3ch18(&self) -> AWD3CH18_R {
        AWD3CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AWD3CH19
    #[inline(always)]
    pub fn awd3ch19(&self) -> AWD3CH19_R {
        AWD3CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AWD3CH20
    #[inline(always)]
    pub fn awd3ch20(&self) -> AWD3CH20_R {
        AWD3CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - AWD3CH21
    #[inline(always)]
    pub fn awd3ch21(&self) -> AWD3CH21_R {
        AWD3CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - AWD3CH22
    #[inline(always)]
    pub fn awd3ch22(&self) -> AWD3CH22_R {
        AWD3CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - AWD3CH23
    #[inline(always)]
    pub fn awd3ch23(&self) -> AWD3CH23_R {
        AWD3CH23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AWD3CH0
    #[inline(always)]
    #[must_use]
    pub fn awd3ch0(&mut self) -> AWD3CH0_W<0> {
        AWD3CH0_W::new(self)
    }
    ///Bit 1 - AWD3CH1
    #[inline(always)]
    #[must_use]
    pub fn awd3ch1(&mut self) -> AWD3CH1_W<1> {
        AWD3CH1_W::new(self)
    }
    ///Bit 2 - AWD3CH2
    #[inline(always)]
    #[must_use]
    pub fn awd3ch2(&mut self) -> AWD3CH2_W<2> {
        AWD3CH2_W::new(self)
    }
    ///Bit 3 - AWD3CH3
    #[inline(always)]
    #[must_use]
    pub fn awd3ch3(&mut self) -> AWD3CH3_W<3> {
        AWD3CH3_W::new(self)
    }
    ///Bit 4 - AWD3CH4
    #[inline(always)]
    #[must_use]
    pub fn awd3ch4(&mut self) -> AWD3CH4_W<4> {
        AWD3CH4_W::new(self)
    }
    ///Bit 5 - AWD3CH5
    #[inline(always)]
    #[must_use]
    pub fn awd3ch5(&mut self) -> AWD3CH5_W<5> {
        AWD3CH5_W::new(self)
    }
    ///Bit 6 - AWD3CH6
    #[inline(always)]
    #[must_use]
    pub fn awd3ch6(&mut self) -> AWD3CH6_W<6> {
        AWD3CH6_W::new(self)
    }
    ///Bit 7 - AWD3CH7
    #[inline(always)]
    #[must_use]
    pub fn awd3ch7(&mut self) -> AWD3CH7_W<7> {
        AWD3CH7_W::new(self)
    }
    ///Bit 8 - AWD3CH8
    #[inline(always)]
    #[must_use]
    pub fn awd3ch8(&mut self) -> AWD3CH8_W<8> {
        AWD3CH8_W::new(self)
    }
    ///Bit 9 - AWD3CH9
    #[inline(always)]
    #[must_use]
    pub fn awd3ch9(&mut self) -> AWD3CH9_W<9> {
        AWD3CH9_W::new(self)
    }
    ///Bit 10 - AWD3CH10
    #[inline(always)]
    #[must_use]
    pub fn awd3ch10(&mut self) -> AWD3CH10_W<10> {
        AWD3CH10_W::new(self)
    }
    ///Bit 11 - AWD3CH11
    #[inline(always)]
    #[must_use]
    pub fn awd3ch11(&mut self) -> AWD3CH11_W<11> {
        AWD3CH11_W::new(self)
    }
    ///Bit 12 - AWD3CH12
    #[inline(always)]
    #[must_use]
    pub fn awd3ch12(&mut self) -> AWD3CH12_W<12> {
        AWD3CH12_W::new(self)
    }
    ///Bit 13 - AWD3CH13
    #[inline(always)]
    #[must_use]
    pub fn awd3ch13(&mut self) -> AWD3CH13_W<13> {
        AWD3CH13_W::new(self)
    }
    ///Bit 14 - AWD3CH14
    #[inline(always)]
    #[must_use]
    pub fn awd3ch14(&mut self) -> AWD3CH14_W<14> {
        AWD3CH14_W::new(self)
    }
    ///Bit 15 - AWD3CH15
    #[inline(always)]
    #[must_use]
    pub fn awd3ch15(&mut self) -> AWD3CH15_W<15> {
        AWD3CH15_W::new(self)
    }
    ///Bit 16 - AWD3CH16
    #[inline(always)]
    #[must_use]
    pub fn awd3ch16(&mut self) -> AWD3CH16_W<16> {
        AWD3CH16_W::new(self)
    }
    ///Bit 17 - AWD3CH17
    #[inline(always)]
    #[must_use]
    pub fn awd3ch17(&mut self) -> AWD3CH17_W<17> {
        AWD3CH17_W::new(self)
    }
    ///Bit 18 - AWD3CH18
    #[inline(always)]
    #[must_use]
    pub fn awd3ch18(&mut self) -> AWD3CH18_W<18> {
        AWD3CH18_W::new(self)
    }
    ///Bit 19 - AWD3CH19
    #[inline(always)]
    #[must_use]
    pub fn awd3ch19(&mut self) -> AWD3CH19_W<19> {
        AWD3CH19_W::new(self)
    }
    ///Bit 20 - AWD3CH20
    #[inline(always)]
    #[must_use]
    pub fn awd3ch20(&mut self) -> AWD3CH20_W<20> {
        AWD3CH20_W::new(self)
    }
    ///Bit 21 - AWD3CH21
    #[inline(always)]
    #[must_use]
    pub fn awd3ch21(&mut self) -> AWD3CH21_W<21> {
        AWD3CH21_W::new(self)
    }
    ///Bit 22 - AWD3CH22
    #[inline(always)]
    #[must_use]
    pub fn awd3ch22(&mut self) -> AWD3CH22_W<22> {
        AWD3CH22_W::new(self)
    }
    ///Bit 23 - AWD3CH23
    #[inline(always)]
    #[must_use]
    pub fn awd3ch23(&mut self) -> AWD3CH23_W<23> {
        AWD3CH23_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC Analog Watchdog 3 Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_awd3cr](index.html) module
pub struct ADC_AWD3CR_SPEC;
impl crate::RegisterSpec for ADC_AWD3CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_awd3cr::R](R) reader structure
impl crate::Readable for ADC_AWD3CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_awd3cr::W](W) writer structure
impl crate::Writable for ADC_AWD3CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC_AWD3CR to value 0
impl crate::Resettable for ADC_AWD3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
