///Register `OR` reader
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR` writer
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETR_RMP` reader - Timer21 ETR remap
pub type ETR_RMP_R = crate::FieldReader<u8, ETR_RMP_A>;
///Timer21 ETR remap
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETR_RMP_A {
    ///0: TIM2x ETR input connected to GPIO
    Gpio = 0,
    ///1: TIM2x ETR input connected to COMP2_OUT
    Comp2Out = 1,
    ///2: TIM2x ETR input connected to COMP1_OUT
    Comp1Out = 2,
    ///3: TIM2x ETR input connected to LSE clock
    Lse = 3,
}
impl From<ETR_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: ETR_RMP_A) -> Self {
        variant as _
    }
}
impl ETR_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETR_RMP_A {
        match self.bits {
            0 => ETR_RMP_A::Gpio,
            1 => ETR_RMP_A::Comp2Out,
            2 => ETR_RMP_A::Comp1Out,
            3 => ETR_RMP_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Gpio`
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == ETR_RMP_A::Gpio
    }
    ///Checks if the value of the field is `Comp2Out`
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == ETR_RMP_A::Comp2Out
    }
    ///Checks if the value of the field is `Comp1Out`
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == ETR_RMP_A::Comp1Out
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP_A::Lse
    }
}
///Field `ETR_RMP` writer - Timer21 ETR remap
pub type ETR_RMP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OR_SPEC, u8, ETR_RMP_A, 2, O>;
impl<'a, const O: u8> ETR_RMP_W<'a, O> {
    ///TIM2x ETR input connected to GPIO
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Gpio)
    }
    ///TIM2x ETR input connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Comp2Out)
    }
    ///TIM2x ETR input connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Comp1Out)
    }
    ///TIM2x ETR input connected to LSE clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Lse)
    }
}
///Field `TI1_RMP` reader - Timer21 TI1
pub type TI1_RMP_R = crate::FieldReader<u8, TI1_RMP_A>;
///Timer21 TI1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1_RMP_A {
    ///0: TIM2x TI1 input connected to GPIO
    Gpio = 0,
    ///1: TIM2x TI1 input connected to COMP2_OUT
    Comp2Out = 1,
    ///2: TIM2x TI1 input connected to COMP1_OUT
    Comp1Out = 2,
}
impl From<TI1_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI1_RMP_A) -> Self {
        variant as _
    }
}
impl TI1_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TI1_RMP_A> {
        match self.bits {
            0 => Some(TI1_RMP_A::Gpio),
            1 => Some(TI1_RMP_A::Comp2Out),
            2 => Some(TI1_RMP_A::Comp1Out),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Gpio`
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI1_RMP_A::Gpio
    }
    ///Checks if the value of the field is `Comp2Out`
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI1_RMP_A::Comp2Out
    }
    ///Checks if the value of the field is `Comp1Out`
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI1_RMP_A::Comp1Out
    }
}
///Field `TI1_RMP` writer - Timer21 TI1
pub type TI1_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, TI1_RMP_A, 3, O>;
impl<'a, const O: u8> TI1_RMP_W<'a, O> {
    ///TIM2x TI1 input connected to GPIO
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Gpio)
    }
    ///TIM2x TI1 input connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Comp2Out)
    }
    ///TIM2x TI1 input connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Comp1Out)
    }
}
///Field `TI2_RMP` reader - Timer21 TI2
pub type TI2_RMP_R = crate::BitReader<TI2_RMP_A>;
///Timer21 TI2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI2_RMP_A {
    ///0: TIM2x TI2 input connected to GPIO
    Gpio = 0,
    ///1: TIM2x TI2 input connected to COMP2_OUT
    Comp2Out = 1,
}
impl From<TI2_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TI2_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TI2_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TI2_RMP_A {
        match self.bits {
            false => TI2_RMP_A::Gpio,
            true => TI2_RMP_A::Comp2Out,
        }
    }
    ///Checks if the value of the field is `Gpio`
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI2_RMP_A::Gpio
    }
    ///Checks if the value of the field is `Comp2Out`
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI2_RMP_A::Comp2Out
    }
}
///Field `TI2_RMP` writer - Timer21 TI2
pub type TI2_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, TI2_RMP_A, O>;
impl<'a, const O: u8> TI2_RMP_W<'a, O> {
    ///TIM2x TI2 input connected to GPIO
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TI2_RMP_A::Gpio)
    }
    ///TIM2x TI2 input connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(TI2_RMP_A::Comp2Out)
    }
}
impl R {
    ///Bits 0:1 - Timer21 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - Timer21 TI1
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bit 5 - Timer21 TI2
    #[inline(always)]
    pub fn ti2_rmp(&self) -> TI2_RMP_R {
        TI2_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Timer21 ETR remap
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<0> {
        ETR_RMP_W::new(self)
    }
    ///Bits 2:4 - Timer21 TI1
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<2> {
        TI1_RMP_W::new(self)
    }
    ///Bit 5 - Timer21 TI2
    #[inline(always)]
    #[must_use]
    pub fn ti2_rmp(&mut self) -> TI2_RMP_W<5> {
        TI2_RMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM21 option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or](index.html) module
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
///`read()` method returns [or::R](R) reader structure
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or::W](W) writer structure
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
