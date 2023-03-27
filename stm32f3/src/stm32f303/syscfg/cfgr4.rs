///Register `CFGR4` reader
pub struct R(crate::R<CFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR4` writer
pub struct W(crate::W<CFGR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR4_SPEC>;
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
impl From<crate::W<CFGR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADC12_EXT2_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT2
pub type ADC12_EXT2_RMP_R = crate::BitReader<ADC12_EXT2_RMP_A>;
///Controls the Input trigger of ADC12 regular channel EXT2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT2_RMP_A {
    ///0: Trigger source is TIM3_CC3
    Tim1 = 0,
    ///1: rigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC12_EXT2_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT2_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_EXT2_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT2_RMP_A {
        match self.bits {
            false => ADC12_EXT2_RMP_A::Tim1,
            true => ADC12_EXT2_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim1`
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC12_EXT2_RMP_A::Tim1
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT2_RMP_A::Tim20
    }
}
///Field `ADC12_EXT2_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT2
pub type ADC12_EXT2_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_EXT2_RMP_A, O>;
impl<'a, const O: u8> ADC12_EXT2_RMP_W<'a, O> {
    ///Trigger source is TIM3_CC3
    #[inline(always)]
    pub fn tim1(self) -> &'a mut W {
        self.variant(ADC12_EXT2_RMP_A::Tim1)
    }
    ///rigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT2_RMP_A::Tim20)
    }
}
///Field `ADC12_EXT3_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT3
pub type ADC12_EXT3_RMP_R = crate::BitReader<ADC12_EXT3_RMP_A>;
///Controls the Input trigger of ADC12 regular channel EXT3
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT3_RMP_A {
    ///0: Trigger source is TIM2_CC2
    Tim2 = 0,
    ///1: rigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC12_EXT3_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT3_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_EXT3_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT3_RMP_A {
        match self.bits {
            false => ADC12_EXT3_RMP_A::Tim2,
            true => ADC12_EXT3_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim2`
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_EXT3_RMP_A::Tim2
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT3_RMP_A::Tim20
    }
}
///Field `ADC12_EXT3_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT3
pub type ADC12_EXT3_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_EXT3_RMP_A, O>;
impl<'a, const O: u8> ADC12_EXT3_RMP_W<'a, O> {
    ///Trigger source is TIM2_CC2
    #[inline(always)]
    pub fn tim2(self) -> &'a mut W {
        self.variant(ADC12_EXT3_RMP_A::Tim2)
    }
    ///rigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT3_RMP_A::Tim20)
    }
}
///Field `ADC12_EXT5_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT5
pub type ADC12_EXT5_RMP_R = crate::BitReader<ADC12_EXT5_RMP_A>;
///Controls the Input trigger of ADC12 regular channel EXT5
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT5_RMP_A {
    ///0: Trigger source is TIM4_CC4
    Tim4 = 0,
    ///1: Trigger source is TIM20_CC1
    Tim20 = 1,
}
impl From<ADC12_EXT5_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT5_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_EXT5_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT5_RMP_A {
        match self.bits {
            false => ADC12_EXT5_RMP_A::Tim4,
            true => ADC12_EXT5_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim4`
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC12_EXT5_RMP_A::Tim4
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT5_RMP_A::Tim20
    }
}
///Field `ADC12_EXT5_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT5
pub type ADC12_EXT5_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_EXT5_RMP_A, O>;
impl<'a, const O: u8> ADC12_EXT5_RMP_W<'a, O> {
    ///Trigger source is TIM4_CC4
    #[inline(always)]
    pub fn tim4(self) -> &'a mut W {
        self.variant(ADC12_EXT5_RMP_A::Tim4)
    }
    ///Trigger source is TIM20_CC1
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT5_RMP_A::Tim20)
    }
}
///Field `ADC12_EXT13_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT13
pub type ADC12_EXT13_RMP_R = crate::BitReader<ADC12_EXT13_RMP_A>;
///Controls the Input trigger of ADC12 regular channel EXT13
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT13_RMP_A {
    ///0: Trigger source is TIM6_TRGO
    Tim6 = 0,
    ///1: Trigger source is TIM20_CC2
    Tim20 = 1,
}
impl From<ADC12_EXT13_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT13_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_EXT13_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT13_RMP_A {
        match self.bits {
            false => ADC12_EXT13_RMP_A::Tim6,
            true => ADC12_EXT13_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim6`
    #[inline(always)]
    pub fn is_tim6(&self) -> bool {
        *self == ADC12_EXT13_RMP_A::Tim6
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT13_RMP_A::Tim20
    }
}
///Field `ADC12_EXT13_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT13
pub type ADC12_EXT13_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_EXT13_RMP_A, O>;
impl<'a, const O: u8> ADC12_EXT13_RMP_W<'a, O> {
    ///Trigger source is TIM6_TRGO
    #[inline(always)]
    pub fn tim6(self) -> &'a mut W {
        self.variant(ADC12_EXT13_RMP_A::Tim6)
    }
    ///Trigger source is TIM20_CC2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT13_RMP_A::Tim20)
    }
}
///Field `ADC12_EXT15_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT15
pub type ADC12_EXT15_RMP_R = crate::BitReader<ADC12_EXT15_RMP_A>;
///Controls the Input trigger of ADC12 regular channel EXT15
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT15_RMP_A {
    ///0: Trigger source is TIM3_CC4
    Tim3 = 0,
    ///1: Trigger source is TIM20_CC3
    Tim20 = 1,
}
impl From<ADC12_EXT15_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT15_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_EXT15_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT15_RMP_A {
        match self.bits {
            false => ADC12_EXT15_RMP_A::Tim3,
            true => ADC12_EXT15_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim3`
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_EXT15_RMP_A::Tim3
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT15_RMP_A::Tim20
    }
}
///Field `ADC12_EXT15_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT15
pub type ADC12_EXT15_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_EXT15_RMP_A, O>;
impl<'a, const O: u8> ADC12_EXT15_RMP_W<'a, O> {
    ///Trigger source is TIM3_CC4
    #[inline(always)]
    pub fn tim3(self) -> &'a mut W {
        self.variant(ADC12_EXT15_RMP_A::Tim3)
    }
    ///Trigger source is TIM20_CC3
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT15_RMP_A::Tim20)
    }
}
///Field `ADC12_JEXT3_RMP` reader - Controls the Input trigger of ADC12 injected channel JEXT3
pub type ADC12_JEXT3_RMP_R = crate::BitReader<ADC12_JEXT3_RMP_A>;
///Controls the Input trigger of ADC12 injected channel JEXT3
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT3_RMP_A {
    ///0: Trigger source is TIM2_CC1
    Tim2 = 0,
    ///1: Trigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC12_JEXT3_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT3_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_JEXT3_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_JEXT3_RMP_A {
        match self.bits {
            false => ADC12_JEXT3_RMP_A::Tim2,
            true => ADC12_JEXT3_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim2`
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_JEXT3_RMP_A::Tim2
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT3_RMP_A::Tim20
    }
}
///Field `ADC12_JEXT3_RMP` writer - Controls the Input trigger of ADC12 injected channel JEXT3
pub type ADC12_JEXT3_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_JEXT3_RMP_A, O>;
impl<'a, const O: u8> ADC12_JEXT3_RMP_W<'a, O> {
    ///Trigger source is TIM2_CC1
    #[inline(always)]
    pub fn tim2(self) -> &'a mut W {
        self.variant(ADC12_JEXT3_RMP_A::Tim2)
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_JEXT3_RMP_A::Tim20)
    }
}
///Field `ADC12_JEXT6_RMP` reader - Controls the Input trigger of ADC12 injected channel JEXT6
pub type ADC12_JEXT6_RMP_R = crate::BitReader<ADC12_JEXT6_RMP_A>;
///Controls the Input trigger of ADC12 injected channel JEXT6
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT6_RMP_A {
    ///0: Trigger source is EXTI line 15
    Exti15 = 0,
    ///1: Trigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC12_JEXT6_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT6_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_JEXT6_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_JEXT6_RMP_A {
        match self.bits {
            false => ADC12_JEXT6_RMP_A::Exti15,
            true => ADC12_JEXT6_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Exti15`
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == ADC12_JEXT6_RMP_A::Exti15
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT6_RMP_A::Tim20
    }
}
///Field `ADC12_JEXT6_RMP` writer - Controls the Input trigger of ADC12 injected channel JEXT6
pub type ADC12_JEXT6_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_JEXT6_RMP_A, O>;
impl<'a, const O: u8> ADC12_JEXT6_RMP_W<'a, O> {
    ///Trigger source is EXTI line 15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(ADC12_JEXT6_RMP_A::Exti15)
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_JEXT6_RMP_A::Tim20)
    }
}
///Field `ADC12_JEXT13_RMP` reader - Controls the Input trigger of ADC12 injected channel JEXT13
pub type ADC12_JEXT13_RMP_R = crate::BitReader<ADC12_JEXT13_RMP_A>;
///Controls the Input trigger of ADC12 injected channel JEXT13
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT13_RMP_A {
    ///0: Trigger source is TIM3_CC1
    Tim3 = 0,
    ///1: Trigger source is TIM20_CC4
    Tim20 = 1,
}
impl From<ADC12_JEXT13_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT13_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12_JEXT13_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC12_JEXT13_RMP_A {
        match self.bits {
            false => ADC12_JEXT13_RMP_A::Tim3,
            true => ADC12_JEXT13_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim3`
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_JEXT13_RMP_A::Tim3
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT13_RMP_A::Tim20
    }
}
///Field `ADC12_JEXT13_RMP` writer - Controls the Input trigger of ADC12 injected channel JEXT13
pub type ADC12_JEXT13_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC12_JEXT13_RMP_A, O>;
impl<'a, const O: u8> ADC12_JEXT13_RMP_W<'a, O> {
    ///Trigger source is TIM3_CC1
    #[inline(always)]
    pub fn tim3(self) -> &'a mut W {
        self.variant(ADC12_JEXT13_RMP_A::Tim3)
    }
    ///Trigger source is TIM20_CC4
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_JEXT13_RMP_A::Tim20)
    }
}
///Field `ADC34_EXT5_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT5
pub type ADC34_EXT5_RMP_R = crate::BitReader<ADC34_EXT5_RMP_A>;
///Controls the Input trigger of ADC34 regular channel EXT5
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT5_RMP_A {
    ///0: Trigger source is EXTI line 2 when reset at 0
    Exti2 = 0,
    ///1: Trigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC34_EXT5_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT5_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC34_EXT5_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC34_EXT5_RMP_A {
        match self.bits {
            false => ADC34_EXT5_RMP_A::Exti2,
            true => ADC34_EXT5_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Exti2`
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == ADC34_EXT5_RMP_A::Exti2
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT5_RMP_A::Tim20
    }
}
///Field `ADC34_EXT5_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT5
pub type ADC34_EXT5_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC34_EXT5_RMP_A, O>;
impl<'a, const O: u8> ADC34_EXT5_RMP_W<'a, O> {
    ///Trigger source is EXTI line 2 when reset at 0
    #[inline(always)]
    pub fn exti2(self) -> &'a mut W {
        self.variant(ADC34_EXT5_RMP_A::Exti2)
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_EXT5_RMP_A::Tim20)
    }
}
///Field `ADC34_EXT6_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT6
pub type ADC34_EXT6_RMP_R = crate::BitReader<ADC34_EXT6_RMP_A>;
///Controls the Input trigger of ADC34 regular channel EXT6
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT6_RMP_A {
    ///0: Trigger source is TIM4_CC1
    Tim4 = 0,
    ///1: Trigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC34_EXT6_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT6_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC34_EXT6_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC34_EXT6_RMP_A {
        match self.bits {
            false => ADC34_EXT6_RMP_A::Tim4,
            true => ADC34_EXT6_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim4`
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_EXT6_RMP_A::Tim4
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT6_RMP_A::Tim20
    }
}
///Field `ADC34_EXT6_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT6
pub type ADC34_EXT6_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC34_EXT6_RMP_A, O>;
impl<'a, const O: u8> ADC34_EXT6_RMP_W<'a, O> {
    ///Trigger source is TIM4_CC1
    #[inline(always)]
    pub fn tim4(self) -> &'a mut W {
        self.variant(ADC34_EXT6_RMP_A::Tim4)
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_EXT6_RMP_A::Tim20)
    }
}
///Field `ADC34_EXT15_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT15
pub type ADC34_EXT15_RMP_R = crate::BitReader<ADC34_EXT15_RMP_A>;
///Controls the Input trigger of ADC34 regular channel EXT15
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT15_RMP_A {
    ///0: Trigger source is TIM2_CC1
    Tim2 = 0,
    ///1: Trigger source is TIM20_CC1
    Tim20 = 1,
}
impl From<ADC34_EXT15_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT15_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC34_EXT15_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC34_EXT15_RMP_A {
        match self.bits {
            false => ADC34_EXT15_RMP_A::Tim2,
            true => ADC34_EXT15_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim2`
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC34_EXT15_RMP_A::Tim2
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT15_RMP_A::Tim20
    }
}
///Field `ADC34_EXT15_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT15
pub type ADC34_EXT15_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC34_EXT15_RMP_A, O>;
impl<'a, const O: u8> ADC34_EXT15_RMP_W<'a, O> {
    ///Trigger source is TIM2_CC1
    #[inline(always)]
    pub fn tim2(self) -> &'a mut W {
        self.variant(ADC34_EXT15_RMP_A::Tim2)
    }
    ///Trigger source is TIM20_CC1
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_EXT15_RMP_A::Tim20)
    }
}
///Field `ADC34_JEXT5_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT5
pub type ADC34_JEXT5_RMP_R = crate::BitReader<ADC34_JEXT5_RMP_A>;
///Controls the Input trigger of ADC34 injected channel JEXT5
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT5_RMP_A {
    ///0: Trigger source is TIM4_CC3
    Tim4 = 0,
    ///1: Trigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC34_JEXT5_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT5_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC34_JEXT5_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC34_JEXT5_RMP_A {
        match self.bits {
            false => ADC34_JEXT5_RMP_A::Tim4,
            true => ADC34_JEXT5_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim4`
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_JEXT5_RMP_A::Tim4
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT5_RMP_A::Tim20
    }
}
///Field `ADC34_JEXT5_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT5
pub type ADC34_JEXT5_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC34_JEXT5_RMP_A, O>;
impl<'a, const O: u8> ADC34_JEXT5_RMP_W<'a, O> {
    ///Trigger source is TIM4_CC3
    #[inline(always)]
    pub fn tim4(self) -> &'a mut W {
        self.variant(ADC34_JEXT5_RMP_A::Tim4)
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_JEXT5_RMP_A::Tim20)
    }
}
///Field `ADC34_JEXT11_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT11
pub type ADC34_JEXT11_RMP_R = crate::BitReader<ADC34_JEXT11_RMP_A>;
///Controls the Input trigger of ADC34 injected channel JEXT11
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT11_RMP_A {
    ///0: Trigger source is TIM1_CC3
    Tim1 = 0,
    ///1: Trigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC34_JEXT11_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT11_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC34_JEXT11_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC34_JEXT11_RMP_A {
        match self.bits {
            false => ADC34_JEXT11_RMP_A::Tim1,
            true => ADC34_JEXT11_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim1`
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC34_JEXT11_RMP_A::Tim1
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT11_RMP_A::Tim20
    }
}
///Field `ADC34_JEXT11_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT11
pub type ADC34_JEXT11_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC34_JEXT11_RMP_A, O>;
impl<'a, const O: u8> ADC34_JEXT11_RMP_W<'a, O> {
    ///Trigger source is TIM1_CC3
    #[inline(always)]
    pub fn tim1(self) -> &'a mut W {
        self.variant(ADC34_JEXT11_RMP_A::Tim1)
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_JEXT11_RMP_A::Tim20)
    }
}
///Field `ADC34_JEXT14_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT14
pub type ADC34_JEXT14_RMP_R = crate::BitReader<ADC34_JEXT14_RMP_A>;
///Controls the Input trigger of ADC34 injected channel JEXT14
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT14_RMP_A {
    ///0: Trigger source is TIM7_TRGO
    Tim7 = 0,
    ///1: Trigger source is TIM20_CC2
    Tim20 = 1,
}
impl From<ADC34_JEXT14_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT14_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC34_JEXT14_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADC34_JEXT14_RMP_A {
        match self.bits {
            false => ADC34_JEXT14_RMP_A::Tim7,
            true => ADC34_JEXT14_RMP_A::Tim20,
        }
    }
    ///Checks if the value of the field is `Tim7`
    #[inline(always)]
    pub fn is_tim7(&self) -> bool {
        *self == ADC34_JEXT14_RMP_A::Tim7
    }
    ///Checks if the value of the field is `Tim20`
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT14_RMP_A::Tim20
    }
}
///Field `ADC34_JEXT14_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT14
pub type ADC34_JEXT14_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR4_SPEC, ADC34_JEXT14_RMP_A, O>;
impl<'a, const O: u8> ADC34_JEXT14_RMP_W<'a, O> {
    ///Trigger source is TIM7_TRGO
    #[inline(always)]
    pub fn tim7(self) -> &'a mut W {
        self.variant(ADC34_JEXT14_RMP_A::Tim7)
    }
    ///Trigger source is TIM20_CC2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_JEXT14_RMP_A::Tim20)
    }
}
impl R {
    ///Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2
    #[inline(always)]
    pub fn adc12_ext2_rmp(&self) -> ADC12_EXT2_RMP_R {
        ADC12_EXT2_RMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3
    #[inline(always)]
    pub fn adc12_ext3_rmp(&self) -> ADC12_EXT3_RMP_R {
        ADC12_EXT3_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5
    #[inline(always)]
    pub fn adc12_ext5_rmp(&self) -> ADC12_EXT5_RMP_R {
        ADC12_EXT5_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13
    #[inline(always)]
    pub fn adc12_ext13_rmp(&self) -> ADC12_EXT13_RMP_R {
        ADC12_EXT13_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15
    #[inline(always)]
    pub fn adc12_ext15_rmp(&self) -> ADC12_EXT15_RMP_R {
        ADC12_EXT15_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Controls the Input trigger of ADC12 injected channel JEXT3
    #[inline(always)]
    pub fn adc12_jext3_rmp(&self) -> ADC12_JEXT3_RMP_R {
        ADC12_JEXT3_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Controls the Input trigger of ADC12 injected channel JEXT6
    #[inline(always)]
    pub fn adc12_jext6_rmp(&self) -> ADC12_JEXT6_RMP_R {
        ADC12_JEXT6_RMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Controls the Input trigger of ADC12 injected channel JEXT13
    #[inline(always)]
    pub fn adc12_jext13_rmp(&self) -> ADC12_JEXT13_RMP_R {
        ADC12_JEXT13_RMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5
    #[inline(always)]
    pub fn adc34_ext5_rmp(&self) -> ADC34_EXT5_RMP_R {
        ADC34_EXT5_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6
    #[inline(always)]
    pub fn adc34_ext6_rmp(&self) -> ADC34_EXT6_RMP_R {
        ADC34_EXT6_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15
    #[inline(always)]
    pub fn adc34_ext15_rmp(&self) -> ADC34_EXT15_RMP_R {
        ADC34_EXT15_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5
    #[inline(always)]
    pub fn adc34_jext5_rmp(&self) -> ADC34_JEXT5_RMP_R {
        ADC34_JEXT5_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11
    #[inline(always)]
    pub fn adc34_jext11_rmp(&self) -> ADC34_JEXT11_RMP_R {
        ADC34_JEXT11_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14
    #[inline(always)]
    pub fn adc34_jext14_rmp(&self) -> ADC34_JEXT14_RMP_R {
        ADC34_JEXT14_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext2_rmp(&mut self) -> ADC12_EXT2_RMP_W<0> {
        ADC12_EXT2_RMP_W::new(self)
    }
    ///Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext3_rmp(&mut self) -> ADC12_EXT3_RMP_W<1> {
        ADC12_EXT3_RMP_W::new(self)
    }
    ///Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext5_rmp(&mut self) -> ADC12_EXT5_RMP_W<2> {
        ADC12_EXT5_RMP_W::new(self)
    }
    ///Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext13_rmp(&mut self) -> ADC12_EXT13_RMP_W<3> {
        ADC12_EXT13_RMP_W::new(self)
    }
    ///Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext15_rmp(&mut self) -> ADC12_EXT15_RMP_W<4> {
        ADC12_EXT15_RMP_W::new(self)
    }
    ///Bit 5 - Controls the Input trigger of ADC12 injected channel JEXT3
    #[inline(always)]
    #[must_use]
    pub fn adc12_jext3_rmp(&mut self) -> ADC12_JEXT3_RMP_W<5> {
        ADC12_JEXT3_RMP_W::new(self)
    }
    ///Bit 6 - Controls the Input trigger of ADC12 injected channel JEXT6
    #[inline(always)]
    #[must_use]
    pub fn adc12_jext6_rmp(&mut self) -> ADC12_JEXT6_RMP_W<6> {
        ADC12_JEXT6_RMP_W::new(self)
    }
    ///Bit 7 - Controls the Input trigger of ADC12 injected channel JEXT13
    #[inline(always)]
    #[must_use]
    pub fn adc12_jext13_rmp(&mut self) -> ADC12_JEXT13_RMP_W<7> {
        ADC12_JEXT13_RMP_W::new(self)
    }
    ///Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5
    #[inline(always)]
    #[must_use]
    pub fn adc34_ext5_rmp(&mut self) -> ADC34_EXT5_RMP_W<8> {
        ADC34_EXT5_RMP_W::new(self)
    }
    ///Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6
    #[inline(always)]
    #[must_use]
    pub fn adc34_ext6_rmp(&mut self) -> ADC34_EXT6_RMP_W<9> {
        ADC34_EXT6_RMP_W::new(self)
    }
    ///Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15
    #[inline(always)]
    #[must_use]
    pub fn adc34_ext15_rmp(&mut self) -> ADC34_EXT15_RMP_W<10> {
        ADC34_EXT15_RMP_W::new(self)
    }
    ///Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5
    #[inline(always)]
    #[must_use]
    pub fn adc34_jext5_rmp(&mut self) -> ADC34_JEXT5_RMP_W<11> {
        ADC34_JEXT5_RMP_W::new(self)
    }
    ///Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11
    #[inline(always)]
    #[must_use]
    pub fn adc34_jext11_rmp(&mut self) -> ADC34_JEXT11_RMP_W<12> {
        ADC34_JEXT11_RMP_W::new(self)
    }
    ///Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14
    #[inline(always)]
    #[must_use]
    pub fn adc34_jext14_rmp(&mut self) -> ADC34_JEXT14_RMP_W<13> {
        ADC34_JEXT14_RMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr4](index.html) module
pub struct CFGR4_SPEC;
impl crate::RegisterSpec for CFGR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr4::R](R) reader structure
impl crate::Readable for CFGR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr4::W](W) writer structure
impl crate::Writable for CFGR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR4 to value 0
impl crate::Resettable for CFGR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
