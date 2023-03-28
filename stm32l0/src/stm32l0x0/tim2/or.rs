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
///Field `ETR_RMP` reader - Timer2 ETR remap
pub type ETR_RMP_R = crate::FieldReader<u8, ETR_RMP_A>;
///Timer2 ETR remap
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETR_RMP_A {
    ///3: TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set
    Hsi = 3,
    ///5: TIM2 ETR input is connected to LSE
    Lse = 5,
    ///6: TIM2 ETR input is connected to COMP2_OUT
    Comp2Out = 6,
    ///7: TIM2 ETR input is connected to COMP1_OUT
    Comp1Out = 7,
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
    pub fn variant(&self) -> Option<ETR_RMP_A> {
        match self.bits {
            3 => Some(ETR_RMP_A::Hsi),
            5 => Some(ETR_RMP_A::Lse),
            6 => Some(ETR_RMP_A::Comp2Out),
            7 => Some(ETR_RMP_A::Comp1Out),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == ETR_RMP_A::Hsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP_A::Lse
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
}
///Field `ETR_RMP` writer - Timer2 ETR remap
pub type ETR_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, ETR_RMP_A, 3, O>;
impl<'a, const O: u8> ETR_RMP_W<'a, O> {
    ///TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Hsi)
    }
    ///TIM2 ETR input is connected to LSE
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Lse)
    }
    ///TIM2 ETR input is connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Comp2Out)
    }
    ///TIM2 ETR input is connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Comp1Out)
    }
}
///Field `TI4_RMP` reader - Internal trigger
pub type TI4_RMP_R = crate::FieldReader<u8, TI4_RMP_A>;
///Internal trigger
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI4_RMP_A {
    ///1: TIM2 TI4 input connected to COMP2_OUT
    Comp2Out = 1,
    ///2: TIM2 TI4 input connected to COMP1_OUT
    Comp1Out = 2,
}
impl From<TI4_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI4_RMP_A) -> Self {
        variant as _
    }
}
impl TI4_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TI4_RMP_A> {
        match self.bits {
            1 => Some(TI4_RMP_A::Comp2Out),
            2 => Some(TI4_RMP_A::Comp1Out),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Comp2Out`
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI4_RMP_A::Comp2Out
    }
    ///Checks if the value of the field is `Comp1Out`
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI4_RMP_A::Comp1Out
    }
}
///Field `TI4_RMP` writer - Internal trigger
pub type TI4_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, TI4_RMP_A, 2, O>;
impl<'a, const O: u8> TI4_RMP_W<'a, O> {
    ///TIM2 TI4 input connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(TI4_RMP_A::Comp2Out)
    }
    ///TIM2 TI4 input connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(TI4_RMP_A::Comp1Out)
    }
}
impl R {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<0> {
        ETR_RMP_W::new(self)
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    #[must_use]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<3> {
        TI4_RMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM2 option register
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
