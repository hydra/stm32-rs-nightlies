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
///Field `SMP10` reader - SMP10
pub type SMP10_R = crate::FieldReader<u8, SMP10_A>;
///SMP10
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10_A {
    ///0: 1.5 ADC clock cycles
    Cycles15 = 0,
    ///1: 2.5 ADC clock cycles
    Cycles25 = 1,
    ///2: 4.5 ADC clock cycles
    Cycles45 = 2,
    ///3: 7.5 ADC clock cycles
    Cycles75 = 3,
    ///4: 19.5 ADC clock cycles
    Cycles195 = 4,
    ///5: 61.5 ADC clock cycles
    Cycles615 = 5,
    ///6: 181.5 ADC clock cycles
    Cycles1815 = 6,
    ///7: 601.5 ADC clock cycles
    Cycles6015 = 7,
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
            1 => SMP10_A::Cycles25,
            2 => SMP10_A::Cycles45,
            3 => SMP10_A::Cycles75,
            4 => SMP10_A::Cycles195,
            5 => SMP10_A::Cycles615,
            6 => SMP10_A::Cycles1815,
            7 => SMP10_A::Cycles6015,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Cycles15`
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP10_A::Cycles15
    }
    ///Checks if the value of the field is `Cycles25`
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP10_A::Cycles25
    }
    ///Checks if the value of the field is `Cycles45`
    #[inline(always)]
    pub fn is_cycles4_5(&self) -> bool {
        *self == SMP10_A::Cycles45
    }
    ///Checks if the value of the field is `Cycles75`
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP10_A::Cycles75
    }
    ///Checks if the value of the field is `Cycles195`
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP10_A::Cycles195
    }
    ///Checks if the value of the field is `Cycles615`
    #[inline(always)]
    pub fn is_cycles61_5(&self) -> bool {
        *self == SMP10_A::Cycles615
    }
    ///Checks if the value of the field is `Cycles1815`
    #[inline(always)]
    pub fn is_cycles181_5(&self) -> bool {
        *self == SMP10_A::Cycles1815
    }
    ///Checks if the value of the field is `Cycles6015`
    #[inline(always)]
    pub fn is_cycles601_5(&self) -> bool {
        *self == SMP10_A::Cycles6015
    }
}
///Field `SMP10` writer - SMP10
pub type SMP10_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR2_SPEC, u8, SMP10_A, 3, O>;
impl<'a, const O: u8> SMP10_W<'a, O> {
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles15)
    }
    ///2.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles25)
    }
    ///4.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles4_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles45)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles75)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles195)
    }
    ///61.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles61_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles615)
    }
    ///181.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles181_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles1815)
    }
    ///601.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles601_5(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles6015)
    }
}
///Field `SMP11` reader - SMP11
pub use SMP10_R as SMP11_R;
///Field `SMP12` reader - SMP12
pub use SMP10_R as SMP12_R;
///Field `SMP13` reader - SMP13
pub use SMP10_R as SMP13_R;
///Field `SMP14` reader - SMP14
pub use SMP10_R as SMP14_R;
///Field `SMP15` reader - SMP15
pub use SMP10_R as SMP15_R;
///Field `SMP16` reader - SMP16
pub use SMP10_R as SMP16_R;
///Field `SMP17` reader - SMP17
pub use SMP10_R as SMP17_R;
///Field `SMP18` reader - SMP18
pub use SMP10_R as SMP18_R;
///Field `SMP11` writer - SMP11
pub use SMP10_W as SMP11_W;
///Field `SMP12` writer - SMP12
pub use SMP10_W as SMP12_W;
///Field `SMP13` writer - SMP13
pub use SMP10_W as SMP13_W;
///Field `SMP14` writer - SMP14
pub use SMP10_W as SMP14_W;
///Field `SMP15` writer - SMP15
pub use SMP10_W as SMP15_W;
///Field `SMP16` writer - SMP16
pub use SMP10_W as SMP16_W;
///Field `SMP17` writer - SMP17
pub use SMP10_W as SMP17_W;
///Field `SMP18` writer - SMP18
pub use SMP10_W as SMP18_W;
impl R {
    ///Bits 0:2 - SMP10
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SMP11
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SMP12
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SMP13
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SMP14
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SMP15
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SMP16
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SMP17
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SMP18
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - SMP10
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<0> {
        SMP10_W::new(self)
    }
    ///Bits 3:5 - SMP11
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<3> {
        SMP11_W::new(self)
    }
    ///Bits 6:8 - SMP12
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<6> {
        SMP12_W::new(self)
    }
    ///Bits 9:11 - SMP13
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<9> {
        SMP13_W::new(self)
    }
    ///Bits 12:14 - SMP14
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<12> {
        SMP14_W::new(self)
    }
    ///Bits 15:17 - SMP15
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<15> {
        SMP15_W::new(self)
    }
    ///Bits 18:20 - SMP16
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<18> {
        SMP16_W::new(self)
    }
    ///Bits 21:23 - SMP17
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<21> {
        SMP17_W::new(self)
    }
    ///Bits 24:26 - SMP18
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<24> {
        SMP18_W::new(self)
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
