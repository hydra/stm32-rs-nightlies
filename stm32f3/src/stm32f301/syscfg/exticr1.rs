///Register `EXTICR1` reader
pub struct R(crate::R<EXTICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR1` writer
pub struct W(crate::W<EXTICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR1_SPEC>;
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
impl From<crate::W<EXTICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EXTI0` reader - EXTI 0 configuration bits
pub type EXTI0_R = crate::FieldReader<u8, EXTI0_A>;
///EXTI 0 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0_A {
    ///0: Select PA0 as the source input for the EXTI0 external interrupt
    Pa0 = 0,
    ///1: Select PB0 as the source input for the EXTI0 external interrupt
    Pb0 = 1,
    ///2: Select PC0 as the source input for the EXTI0 external interrupt
    Pc0 = 2,
    ///5: Select PF0 as the source input for the EXTI0 external interrupt
    Pf0 = 5,
}
impl From<EXTI0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_A) -> Self {
        variant as _
    }
}
impl EXTI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI0_A> {
        match self.bits {
            0 => Some(EXTI0_A::Pa0),
            1 => Some(EXTI0_A::Pb0),
            2 => Some(EXTI0_A::Pc0),
            5 => Some(EXTI0_A::Pf0),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa0`
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0_A::Pa0
    }
    ///Checks if the value of the field is `Pb0`
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0_A::Pb0
    }
    ///Checks if the value of the field is `Pc0`
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0_A::Pc0
    }
    ///Checks if the value of the field is `Pf0`
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == EXTI0_A::Pf0
    }
}
///Field `EXTI0` writer - EXTI 0 configuration bits
pub type EXTI0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, EXTI0_A, 4, O>;
impl<'a, const O: u8> EXTI0_W<'a, O> {
    ///Select PA0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(EXTI0_A::Pa0)
    }
    ///Select PB0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(EXTI0_A::Pb0)
    }
    ///Select PC0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pc0(self) -> &'a mut W {
        self.variant(EXTI0_A::Pc0)
    }
    ///Select PF0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pf0(self) -> &'a mut W {
        self.variant(EXTI0_A::Pf0)
    }
}
///Field `EXTI1` reader - EXTI 1 configuration bits
pub type EXTI1_R = crate::FieldReader<u8, EXTI1_A>;
///EXTI 1 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1_A {
    ///0: Select PA1 as the source input for the EXTI1 external interrupt
    Pa1 = 0,
    ///1: Select PB1 as the source input for the EXTI1 external interrupt
    Pb1 = 1,
    ///2: Select PC1 as the source input for the EXTI1 external interrupt
    Pc1 = 2,
    ///5: Select PF1 as the source input for the EXTI1 external interrupt
    Pf1 = 5,
}
impl From<EXTI1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1_A) -> Self {
        variant as _
    }
}
impl EXTI1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI1_A> {
        match self.bits {
            0 => Some(EXTI1_A::Pa1),
            1 => Some(EXTI1_A::Pb1),
            2 => Some(EXTI1_A::Pc1),
            5 => Some(EXTI1_A::Pf1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa1`
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1_A::Pa1
    }
    ///Checks if the value of the field is `Pb1`
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1_A::Pb1
    }
    ///Checks if the value of the field is `Pc1`
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1_A::Pc1
    }
    ///Checks if the value of the field is `Pf1`
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == EXTI1_A::Pf1
    }
}
///Field `EXTI1` writer - EXTI 1 configuration bits
pub type EXTI1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, EXTI1_A, 4, O>;
impl<'a, const O: u8> EXTI1_W<'a, O> {
    ///Select PA1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(EXTI1_A::Pa1)
    }
    ///Select PB1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(EXTI1_A::Pb1)
    }
    ///Select PC1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pc1(self) -> &'a mut W {
        self.variant(EXTI1_A::Pc1)
    }
    ///Select PF1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pf1(self) -> &'a mut W {
        self.variant(EXTI1_A::Pf1)
    }
}
///Field `EXTI2` reader - EXTI 2 configuration bits
pub type EXTI2_R = crate::FieldReader<u8, EXTI2_A>;
///EXTI 2 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2_A {
    ///0: Select PA2 as the source input for the EXTI2 external interrupt
    Pa2 = 0,
    ///1: Select PB2 as the source input for the EXTI2 external interrupt
    Pb2 = 1,
    ///2: Select PC2 as the source input for the EXTI2 external interrupt
    Pc2 = 2,
    ///3: Select PD2 as the source input for the EXTI2 external interrupt
    Pd2 = 3,
}
impl From<EXTI2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2_A) -> Self {
        variant as _
    }
}
impl EXTI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI2_A> {
        match self.bits {
            0 => Some(EXTI2_A::Pa2),
            1 => Some(EXTI2_A::Pb2),
            2 => Some(EXTI2_A::Pc2),
            3 => Some(EXTI2_A::Pd2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa2`
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2_A::Pa2
    }
    ///Checks if the value of the field is `Pb2`
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2_A::Pb2
    }
    ///Checks if the value of the field is `Pc2`
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2_A::Pc2
    }
    ///Checks if the value of the field is `Pd2`
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2_A::Pd2
    }
}
///Field `EXTI2` writer - EXTI 2 configuration bits
pub type EXTI2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, EXTI2_A, 4, O>;
impl<'a, const O: u8> EXTI2_W<'a, O> {
    ///Select PA2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(EXTI2_A::Pa2)
    }
    ///Select PB2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(EXTI2_A::Pb2)
    }
    ///Select PC2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pc2(self) -> &'a mut W {
        self.variant(EXTI2_A::Pc2)
    }
    ///Select PD2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pd2(self) -> &'a mut W {
        self.variant(EXTI2_A::Pd2)
    }
}
///Field `EXTI3` reader - EXTI 3 configuration bits
pub type EXTI3_R = crate::FieldReader<u8, EXTI3_A>;
///EXTI 3 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3_A {
    ///0: Select PA3 as the source input for the EXTI3 external interrupt
    Pa3 = 0,
    ///1: Select PB3 as the source input for the EXTI3 external interrupt
    Pb3 = 1,
    ///2: Select PC3 as the source input for the EXTI3 external interrupt
    Pc3 = 2,
}
impl From<EXTI3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3_A) -> Self {
        variant as _
    }
}
impl EXTI3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI3_A> {
        match self.bits {
            0 => Some(EXTI3_A::Pa3),
            1 => Some(EXTI3_A::Pb3),
            2 => Some(EXTI3_A::Pc3),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa3`
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3_A::Pa3
    }
    ///Checks if the value of the field is `Pb3`
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3_A::Pb3
    }
    ///Checks if the value of the field is `Pc3`
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3_A::Pc3
    }
}
///Field `EXTI3` writer - EXTI 3 configuration bits
pub type EXTI3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, EXTI3_A, 4, O>;
impl<'a, const O: u8> EXTI3_W<'a, O> {
    ///Select PA3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(EXTI3_A::Pa3)
    }
    ///Select PB3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(EXTI3_A::Pb3)
    }
    ///Select PC3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pc3(self) -> &'a mut W {
        self.variant(EXTI3_A::Pc3)
    }
}
impl R {
    ///Bits 0:3 - EXTI 0 configuration bits
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI 1 configuration bits
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI 2 configuration bits
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI 3 configuration bits
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - EXTI 0 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<0> {
        EXTI0_W::new(self)
    }
    ///Bits 4:7 - EXTI 1 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<4> {
        EXTI1_W::new(self)
    }
    ///Bits 8:11 - EXTI 2 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<8> {
        EXTI2_W::new(self)
    }
    ///Bits 12:15 - EXTI 3 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<12> {
        EXTI3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///external interrupt configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr1](index.html) module
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr1::R](R) reader structure
impl crate::Readable for EXTICR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr1::W](W) writer structure
impl crate::Writable for EXTICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTICR1 to value 0
impl crate::Resettable for EXTICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
