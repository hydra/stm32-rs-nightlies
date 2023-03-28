///Register `SMPR2` reader
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMPR2` writer
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMP0` reader - Channel 0 sampling time selection
pub type SMP0_R = crate::FieldReader<u8, SMP0_A>;
///Channel 0 sampling time selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0_A {
    ///0: 1.5 ADC clock cycles
    Cycles15 = 0,
    ///1: 7.5 ADC clock cycles
    Cycles75 = 1,
    ///2: 13.5 ADC clock cycles
    Cycles135 = 2,
    ///3: 28.5 ADC clock cycles
    Cycles285 = 3,
    ///4: 41.5 ADC clock cycles
    Cycles415 = 4,
    ///5: 55.5 ADC clock cycles
    Cycles555 = 5,
    ///6: 71.5 ADC clock cycles
    Cycles715 = 6,
    ///7: 239.5 ADC clock cycles
    Cycles2395 = 7,
}
impl From<SMP0_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP0_A) -> Self {
        variant as _
    }
}
impl SMP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMP0_A {
        match self.bits {
            0 => SMP0_A::Cycles15,
            1 => SMP0_A::Cycles75,
            2 => SMP0_A::Cycles135,
            3 => SMP0_A::Cycles285,
            4 => SMP0_A::Cycles415,
            5 => SMP0_A::Cycles555,
            6 => SMP0_A::Cycles715,
            7 => SMP0_A::Cycles2395,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Cycles15`
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP0_A::Cycles15
    }
    ///Checks if the value of the field is `Cycles75`
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP0_A::Cycles75
    }
    ///Checks if the value of the field is `Cycles135`
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMP0_A::Cycles135
    }
    ///Checks if the value of the field is `Cycles285`
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMP0_A::Cycles285
    }
    ///Checks if the value of the field is `Cycles415`
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMP0_A::Cycles415
    }
    ///Checks if the value of the field is `Cycles555`
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMP0_A::Cycles555
    }
    ///Checks if the value of the field is `Cycles715`
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMP0_A::Cycles715
    }
    ///Checks if the value of the field is `Cycles2395`
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMP0_A::Cycles2395
    }
}
///Field `SMP0` writer - Channel 0 sampling time selection
pub type SMP0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR2_SPEC, u8, SMP0_A, 3, O>;
impl<'a, const O: u8> SMP0_W<'a, O> {
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles15)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles75)
    }
    ///13.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles135)
    }
    ///28.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles285)
    }
    ///41.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles415)
    }
    ///55.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles555)
    }
    ///71.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles715)
    }
    ///239.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles2395)
    }
}
///Field `SMP1` reader - Channel 1 sampling time selection
pub use SMP0_R as SMP1_R;
///Field `SMP2` reader - Channel 2 sampling time selection
pub use SMP0_R as SMP2_R;
///Field `SMP3` reader - Channel 3 sampling time selection
pub use SMP0_R as SMP3_R;
///Field `SMP4` reader - Channel 4 sampling time selection
pub use SMP0_R as SMP4_R;
///Field `SMP5` reader - Channel 5 sampling time selection
pub use SMP0_R as SMP5_R;
///Field `SMP6` reader - Channel 6 sampling time selection
pub use SMP0_R as SMP6_R;
///Field `SMP7` reader - Channel 7 sampling time selection
pub use SMP0_R as SMP7_R;
///Field `SMP8` reader - Channel 8 sampling time selection
pub use SMP0_R as SMP8_R;
///Field `SMP9` reader - Channel 9 sampling time selection
pub use SMP0_R as SMP9_R;
///Field `SMP1` writer - Channel 1 sampling time selection
pub use SMP0_W as SMP1_W;
///Field `SMP2` writer - Channel 2 sampling time selection
pub use SMP0_W as SMP2_W;
///Field `SMP3` writer - Channel 3 sampling time selection
pub use SMP0_W as SMP3_W;
///Field `SMP4` writer - Channel 4 sampling time selection
pub use SMP0_W as SMP4_W;
///Field `SMP5` writer - Channel 5 sampling time selection
pub use SMP0_W as SMP5_W;
///Field `SMP6` writer - Channel 6 sampling time selection
pub use SMP0_W as SMP6_W;
///Field `SMP7` writer - Channel 7 sampling time selection
pub use SMP0_W as SMP7_W;
///Field `SMP8` writer - Channel 8 sampling time selection
pub use SMP0_W as SMP8_W;
///Field `SMP9` writer - Channel 9 sampling time selection
pub use SMP0_W as SMP9_W;
impl R {
    ///Bits 0:2 - Channel 0 sampling time selection
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 1 sampling time selection
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 2 sampling time selection
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 3 sampling time selection
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 4 sampling time selection
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 5 sampling time selection
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 6 sampling time selection
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 7 sampling time selection
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 8 sampling time selection
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 9 sampling time selection
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Channel 0 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> SMP0_W<0> {
        SMP0_W::new(self)
    }
    ///Bits 3:5 - Channel 1 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<3> {
        SMP1_W::new(self)
    }
    ///Bits 6:8 - Channel 2 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<6> {
        SMP2_W::new(self)
    }
    ///Bits 9:11 - Channel 3 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<9> {
        SMP3_W::new(self)
    }
    ///Bits 12:14 - Channel 4 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<12> {
        SMP4_W::new(self)
    }
    ///Bits 15:17 - Channel 5 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<15> {
        SMP5_W::new(self)
    }
    ///Bits 18:20 - Channel 6 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<18> {
        SMP6_W::new(self)
    }
    ///Bits 21:23 - Channel 7 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<21> {
        SMP7_W::new(self)
    }
    ///Bits 24:26 - Channel 8 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<24> {
        SMP8_W::new(self)
    }
    ///Bits 27:29 - Channel 9 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<27> {
        SMP9_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///sample time register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr2](index.html) module
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [smpr2::R](R) reader structure
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smpr2::W](W) writer structure
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMPR2 to value 0
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
