///Register `SMPR1` reader
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMPR1` writer
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMP10` reader - Channel 10 sample time selection
pub type SMP10_R = crate::FieldReader<u8, SMP10_A>;
///Channel 10 sample time selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10_A {
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
impl From<SMP10_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP10_A) -> Self {
        variant as _
    }
}
impl SMP10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMP10_A {
        match self.bits {
            0 => SMP10_A::Cycles15,
            1 => SMP10_A::Cycles75,
            2 => SMP10_A::Cycles135,
            3 => SMP10_A::Cycles285,
            4 => SMP10_A::Cycles415,
            5 => SMP10_A::Cycles555,
            6 => SMP10_A::Cycles715,
            7 => SMP10_A::Cycles2395,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Cycles15`
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP10_A::Cycles15
    }
    ///Checks if the value of the field is `Cycles75`
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP10_A::Cycles75
    }
    ///Checks if the value of the field is `Cycles135`
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMP10_A::Cycles135
    }
    ///Checks if the value of the field is `Cycles285`
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMP10_A::Cycles285
    }
    ///Checks if the value of the field is `Cycles415`
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMP10_A::Cycles415
    }
    ///Checks if the value of the field is `Cycles555`
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMP10_A::Cycles555
    }
    ///Checks if the value of the field is `Cycles715`
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMP10_A::Cycles715
    }
    ///Checks if the value of the field is `Cycles2395`
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMP10_A::Cycles2395
    }
}
///Field `SMP10` writer - Channel 10 sample time selection
pub type SMP10_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR1_SPEC, u8, SMP10_A, 3, O>;
impl<'a, const O: u8> SMP10_W<'a, O> {
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles15)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles75)
    }
    ///13.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles135)
    }
    ///28.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles285)
    }
    ///41.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles415)
    }
    ///55.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles555)
    }
    ///71.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles715)
    }
    ///239.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles2395)
    }
}
///Field `SMP11` reader - Channel 11 sample time selection
pub use SMP10_R as SMP11_R;
///Field `SMP12` reader - Channel 12 sample time selection
pub use SMP10_R as SMP12_R;
///Field `SMP13` reader - Channel 13 sample time selection
pub use SMP10_R as SMP13_R;
///Field `SMP14` reader - Channel 14 sample time selection
pub use SMP10_R as SMP14_R;
///Field `SMP15` reader - Channel 15 sample time selection
pub use SMP10_R as SMP15_R;
///Field `SMP16` reader - Channel 16 sample time selection
pub use SMP10_R as SMP16_R;
///Field `SMP17` reader - Channel 17 sample time selection
pub use SMP10_R as SMP17_R;
///Field `SMP11` writer - Channel 11 sample time selection
pub use SMP10_W as SMP11_W;
///Field `SMP12` writer - Channel 12 sample time selection
pub use SMP10_W as SMP12_W;
///Field `SMP13` writer - Channel 13 sample time selection
pub use SMP10_W as SMP13_W;
///Field `SMP14` writer - Channel 14 sample time selection
pub use SMP10_W as SMP14_W;
///Field `SMP15` writer - Channel 15 sample time selection
pub use SMP10_W as SMP15_W;
///Field `SMP16` writer - Channel 16 sample time selection
pub use SMP10_W as SMP16_W;
///Field `SMP17` writer - Channel 17 sample time selection
pub use SMP10_W as SMP17_W;
impl R {
    ///Bits 0:2 - Channel 10 sample time selection
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 11 sample time selection
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 12 sample time selection
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 13 sample time selection
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 14 sample time selection
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 15 sample time selection
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 16 sample time selection
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 17 sample time selection
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Channel 10 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<0> {
        SMP10_W::new(self)
    }
    ///Bits 3:5 - Channel 11 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<3> {
        SMP11_W::new(self)
    }
    ///Bits 6:8 - Channel 12 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<6> {
        SMP12_W::new(self)
    }
    ///Bits 9:11 - Channel 13 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<9> {
        SMP13_W::new(self)
    }
    ///Bits 12:14 - Channel 14 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<12> {
        SMP14_W::new(self)
    }
    ///Bits 15:17 - Channel 15 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<15> {
        SMP15_W::new(self)
    }
    ///Bits 18:20 - Channel 16 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<18> {
        SMP16_W::new(self)
    }
    ///Bits 21:23 - Channel 17 sample time selection
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<21> {
        SMP17_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///sample time register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr1](index.html) module
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [smpr1::R](R) reader structure
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smpr1::W](W) writer structure
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
