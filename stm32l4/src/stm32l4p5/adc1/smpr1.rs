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
///Field `SMP0` reader - SMP0
pub type SMP0_R = crate::FieldReader<u8, SMP0_A>;
///SMP0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0_A {
    ///0: 2.5 ADC clock cycles
    Cycles25 = 0,
    ///1: 6.5 ADC clock cycles
    Cycles65 = 1,
    ///2: 12.5 ADC clock cycles
    Cycles125 = 2,
    ///3: 24.5 ADC clock cycles
    Cycles245 = 3,
    ///4: 47.5 ADC clock cycles
    Cycles475 = 4,
    ///5: 92.5 ADC clock cycles
    Cycles925 = 5,
    ///6: 247.5 ADC clock cycles
    Cycles2475 = 6,
    ///7: 640.5 ADC clock cycles
    Cycles6405 = 7,
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
            0 => SMP0_A::Cycles25,
            1 => SMP0_A::Cycles65,
            2 => SMP0_A::Cycles125,
            3 => SMP0_A::Cycles245,
            4 => SMP0_A::Cycles475,
            5 => SMP0_A::Cycles925,
            6 => SMP0_A::Cycles2475,
            7 => SMP0_A::Cycles6405,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Cycles25`
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP0_A::Cycles25
    }
    ///Checks if the value of the field is `Cycles65`
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        *self == SMP0_A::Cycles65
    }
    ///Checks if the value of the field is `Cycles125`
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP0_A::Cycles125
    }
    ///Checks if the value of the field is `Cycles245`
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        *self == SMP0_A::Cycles245
    }
    ///Checks if the value of the field is `Cycles475`
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        *self == SMP0_A::Cycles475
    }
    ///Checks if the value of the field is `Cycles925`
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        *self == SMP0_A::Cycles925
    }
    ///Checks if the value of the field is `Cycles2475`
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        *self == SMP0_A::Cycles2475
    }
    ///Checks if the value of the field is `Cycles6405`
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        *self == SMP0_A::Cycles6405
    }
}
///Field `SMP0` writer - SMP0
pub type SMP0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR1_SPEC, u8, SMP0_A, 3, O>;
impl<'a, const O: u8> SMP0_W<'a, O> {
    ///2.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles25)
    }
    ///6.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles65)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles125)
    }
    ///24.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles245)
    }
    ///47.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles475)
    }
    ///92.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles925)
    }
    ///247.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles2475)
    }
    ///640.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles6405)
    }
}
///Field `SMP1` reader - SMP1
pub use SMP0_R as SMP1_R;
///Field `SMP2` reader - SMP2
pub use SMP0_R as SMP2_R;
///Field `SMP3` reader - SMP3
pub use SMP0_R as SMP3_R;
///Field `SMP4` reader - SMP4
pub use SMP0_R as SMP4_R;
///Field `SMP5` reader - SMP5
pub use SMP0_R as SMP5_R;
///Field `SMP6` reader - SMP6
pub use SMP0_R as SMP6_R;
///Field `SMP7` reader - SMP7
pub use SMP0_R as SMP7_R;
///Field `SMP8` reader - SMP8
pub use SMP0_R as SMP8_R;
///Field `SMP9` reader - SMP9
pub use SMP0_R as SMP9_R;
///Field `SMP1` writer - SMP1
pub use SMP0_W as SMP1_W;
///Field `SMP2` writer - SMP2
pub use SMP0_W as SMP2_W;
///Field `SMP3` writer - SMP3
pub use SMP0_W as SMP3_W;
///Field `SMP4` writer - SMP4
pub use SMP0_W as SMP4_W;
///Field `SMP5` writer - SMP5
pub use SMP0_W as SMP5_W;
///Field `SMP6` writer - SMP6
pub use SMP0_W as SMP6_W;
///Field `SMP7` writer - SMP7
pub use SMP0_W as SMP7_W;
///Field `SMP8` writer - SMP8
pub use SMP0_W as SMP8_W;
///Field `SMP9` writer - SMP9
pub use SMP0_W as SMP9_W;
///Field `SMPPLUS` reader - Addition of one clock cycle to the sampling time
pub type SMPPLUS_R = crate::BitReader<SMPPLUS_A>;
///Addition of one clock cycle to the sampling time
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPPLUS_A {
    ///0: The sampling time remains set to 2.5 ADC clock cycles remains
    KeepCycles = 0,
    ///1: 2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers
    Add1cycle = 1,
}
impl From<SMPPLUS_A> for bool {
    #[inline(always)]
    fn from(variant: SMPPLUS_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPPLUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMPPLUS_A {
        match self.bits {
            false => SMPPLUS_A::KeepCycles,
            true => SMPPLUS_A::Add1cycle,
        }
    }
    ///Checks if the value of the field is `KeepCycles`
    #[inline(always)]
    pub fn is_keep_cycles(&self) -> bool {
        *self == SMPPLUS_A::KeepCycles
    }
    ///Checks if the value of the field is `Add1cycle`
    #[inline(always)]
    pub fn is_add1cycle(&self) -> bool {
        *self == SMPPLUS_A::Add1cycle
    }
}
///Field `SMPPLUS` writer - Addition of one clock cycle to the sampling time
pub type SMPPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR1_SPEC, SMPPLUS_A, O>;
impl<'a, const O: u8> SMPPLUS_W<'a, O> {
    ///The sampling time remains set to 2.5 ADC clock cycles remains
    #[inline(always)]
    pub fn keep_cycles(self) -> &'a mut W {
        self.variant(SMPPLUS_A::KeepCycles)
    }
    ///2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers
    #[inline(always)]
    pub fn add1cycle(self) -> &'a mut W {
        self.variant(SMPPLUS_A::Add1cycle)
    }
}
impl R {
    ///Bits 0:2 - SMP0
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SMP1
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SMP2
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SMP3
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SMP4
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SMP5
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SMP6
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SMP7
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SMP8
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - SMP9
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bit 31 - Addition of one clock cycle to the sampling time
    #[inline(always)]
    pub fn smpplus(&self) -> SMPPLUS_R {
        SMPPLUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - SMP0
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> SMP0_W<0> {
        SMP0_W::new(self)
    }
    ///Bits 3:5 - SMP1
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<3> {
        SMP1_W::new(self)
    }
    ///Bits 6:8 - SMP2
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<6> {
        SMP2_W::new(self)
    }
    ///Bits 9:11 - SMP3
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<9> {
        SMP3_W::new(self)
    }
    ///Bits 12:14 - SMP4
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<12> {
        SMP4_W::new(self)
    }
    ///Bits 15:17 - SMP5
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<15> {
        SMP5_W::new(self)
    }
    ///Bits 18:20 - SMP6
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<18> {
        SMP6_W::new(self)
    }
    ///Bits 21:23 - SMP7
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<21> {
        SMP7_W::new(self)
    }
    ///Bits 24:26 - SMP8
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<24> {
        SMP8_W::new(self)
    }
    ///Bits 27:29 - SMP9
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<27> {
        SMP9_W::new(self)
    }
    ///Bit 31 - Addition of one clock cycle to the sampling time
    #[inline(always)]
    #[must_use]
    pub fn smpplus(&mut self) -> SMPPLUS_W<31> {
        SMPPLUS_W::new(self)
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
