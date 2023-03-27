///Register `OR1` reader
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR1` writer
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability
pub type TIM1_ETR_ADC1_RMP_R = crate::FieldReader<u8, TIM1_ETR_ADC1_RMP_A>;
///TIM1_ETR_ADC1 remapping capability
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIM1_ETR_ADC1_RMP_A {
    ///0: TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)
    Select = 0,
    ///1: TIM1_ETR is connected to ADC AWD1
    AdcAwd1 = 1,
    ///2: TIM1_ETR is connected to ADC AWD2
    AdcAwd2 = 2,
    ///3: TIM1_ETR is connected to ADC AWD3
    AdcAwd3 = 3,
}
impl From<TIM1_ETR_ADC1_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TIM1_ETR_ADC1_RMP_A) -> Self {
        variant as _
    }
}
impl TIM1_ETR_ADC1_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM1_ETR_ADC1_RMP_A {
        match self.bits {
            0 => TIM1_ETR_ADC1_RMP_A::Select,
            1 => TIM1_ETR_ADC1_RMP_A::AdcAwd1,
            2 => TIM1_ETR_ADC1_RMP_A::AdcAwd2,
            3 => TIM1_ETR_ADC1_RMP_A::AdcAwd3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Select`
    #[inline(always)]
    pub fn is_select(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP_A::Select
    }
    ///Checks if the value of the field is `AdcAwd1`
    #[inline(always)]
    pub fn is_adc_awd1(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP_A::AdcAwd1
    }
    ///Checks if the value of the field is `AdcAwd2`
    #[inline(always)]
    pub fn is_adc_awd2(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP_A::AdcAwd2
    }
    ///Checks if the value of the field is `AdcAwd3`
    #[inline(always)]
    pub fn is_adc_awd3(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP_A::AdcAwd3
    }
}
///Field `TIM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability
pub type TIM1_ETR_ADC1_RMP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OR1_SPEC, u8, TIM1_ETR_ADC1_RMP_A, 2, O>;
impl<'a, const O: u8> TIM1_ETR_ADC1_RMP_W<'a, O> {
    ///TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)
    #[inline(always)]
    pub fn select(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::Select)
    }
    ///TIM1_ETR is connected to ADC AWD1
    #[inline(always)]
    pub fn adc_awd1(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::AdcAwd1)
    }
    ///TIM1_ETR is connected to ADC AWD2
    #[inline(always)]
    pub fn adc_awd2(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::AdcAwd2)
    }
    ///TIM1_ETR is connected to ADC AWD3
    #[inline(always)]
    pub fn adc_awd3(self) -> &'a mut W {
        self.variant(TIM1_ETR_ADC1_RMP_A::AdcAwd3)
    }
}
///Field `TI1_RMP` reader - Input Capture 1 remap
pub type TI1_RMP_R = crate::BitReader<TI1_RMP_A>;
///Input Capture 1 remap
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1_RMP_A {
    ///0: TIM1 input capture 1 is connected to I/O
    Io = 0,
    ///1: TIM1 input capture 1 is connected to COMP1 output
    Comp1 = 1,
}
impl From<TI1_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TI1_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TI1_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TI1_RMP_A {
        match self.bits {
            false => TI1_RMP_A::Io,
            true => TI1_RMP_A::Comp1,
        }
    }
    ///Checks if the value of the field is `Io`
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == TI1_RMP_A::Io
    }
    ///Checks if the value of the field is `Comp1`
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == TI1_RMP_A::Comp1
    }
}
///Field `TI1_RMP` writer - Input Capture 1 remap
pub type TI1_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, TI1_RMP_A, O>;
impl<'a, const O: u8> TI1_RMP_W<'a, O> {
    ///TIM1 input capture 1 is connected to I/O
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Io)
    }
    ///TIM1 input capture 1 is connected to COMP1 output
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Comp1)
    }
}
impl R {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> TIM1_ETR_ADC1_RMP_R {
        TIM1_ETR_ADC1_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    #[must_use]
    pub fn tim1_etr_adc1_rmp(&mut self) -> TIM1_ETR_ADC1_RMP_W<0> {
        TIM1_ETR_ADC1_RMP_W::new(self)
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<4> {
        TI1_RMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///option register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or1](index.html) module
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [or1::R](R) reader structure
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or1::W](W) writer structure
impl crate::Writable for OR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
