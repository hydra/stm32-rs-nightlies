///Register `EXTICR4` reader
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR4` writer
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EXTI12` reader - EXTI 12 configuration bits
pub type EXTI12_R = crate::FieldReader<u8, EXTI12_A>;
///EXTI 12 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12_A {
    ///0: Select PA12 as the source input for the EXTI12 external interrupt
    Pa12 = 0,
    ///1: Select PB12 as the source input for the EXTI12 external interrupt
    Pb12 = 1,
    ///2: Select PC12 as the source input for the EXTI12 external interrupt
    Pc12 = 2,
    ///3: Select PD12 as the source input for the EXTI12 external interrupt
    Pd12 = 3,
    ///4: Select PE12 as the source input for the EXTI12 external interrupt
    Pe12 = 4,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
impl EXTI12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI12_A> {
        match self.bits {
            0 => Some(EXTI12_A::Pa12),
            1 => Some(EXTI12_A::Pb12),
            2 => Some(EXTI12_A::Pc12),
            3 => Some(EXTI12_A::Pd12),
            4 => Some(EXTI12_A::Pe12),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa12`
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12_A::Pa12
    }
    ///Checks if the value of the field is `Pb12`
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == EXTI12_A::Pb12
    }
    ///Checks if the value of the field is `Pc12`
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == EXTI12_A::Pc12
    }
    ///Checks if the value of the field is `Pd12`
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == EXTI12_A::Pd12
    }
    ///Checks if the value of the field is `Pe12`
    #[inline(always)]
    pub fn is_pe12(&self) -> bool {
        *self == EXTI12_A::Pe12
    }
}
///Field `EXTI12` writer - EXTI 12 configuration bits
pub type EXTI12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI12_A, 4, O>;
impl<'a, const O: u8> EXTI12_W<'a, O> {
    ///Select PA12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pa12)
    }
    ///Select PB12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pb12)
    }
    ///Select PC12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pc12)
    }
    ///Select PD12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pd12)
    }
    ///Select PE12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pe12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pe12)
    }
}
///Field `EXTI13` reader - EXTI 13 configuration bits
pub type EXTI13_R = crate::FieldReader<u8, EXTI13_A>;
///EXTI 13 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI13_A {
    ///0: Select PA13 as the source input for the EXTI13 external interrupt
    Pa13 = 0,
    ///1: Select PB13 as the source input for the EXTI13 external interrupt
    Pb13 = 1,
    ///2: Select PC13 as the source input for the EXTI13 external interrupt
    Pc13 = 2,
    ///3: Select PD13 as the source input for the EXTI13 external interrupt
    Pd13 = 3,
    ///4: Select PE13 as the source input for the EXTI13 external interrupt
    Pe13 = 4,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
impl EXTI13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI13_A> {
        match self.bits {
            0 => Some(EXTI13_A::Pa13),
            1 => Some(EXTI13_A::Pb13),
            2 => Some(EXTI13_A::Pc13),
            3 => Some(EXTI13_A::Pd13),
            4 => Some(EXTI13_A::Pe13),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa13`
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13_A::Pa13
    }
    ///Checks if the value of the field is `Pb13`
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == EXTI13_A::Pb13
    }
    ///Checks if the value of the field is `Pc13`
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13_A::Pc13
    }
    ///Checks if the value of the field is `Pd13`
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == EXTI13_A::Pd13
    }
    ///Checks if the value of the field is `Pe13`
    #[inline(always)]
    pub fn is_pe13(&self) -> bool {
        *self == EXTI13_A::Pe13
    }
}
///Field `EXTI13` writer - EXTI 13 configuration bits
pub type EXTI13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI13_A, 4, O>;
impl<'a, const O: u8> EXTI13_W<'a, O> {
    ///Select PA13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pa13)
    }
    ///Select PB13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pb13)
    }
    ///Select PC13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pc13)
    }
    ///Select PD13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pd13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pd13)
    }
    ///Select PE13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pe13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pe13)
    }
}
///Field `EXTI14` reader - EXTI 14 configuration bits
pub type EXTI14_R = crate::FieldReader<u8, EXTI14_A>;
///EXTI 14 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI14_A {
    ///0: Select PA14 as the source input for the EXTI14 external interrupt
    Pa14 = 0,
    ///1: Select PB14 as the source input for the EXTI14 external interrupt
    Pb14 = 1,
    ///2: Select PC14 as the source input for the EXTI14 external interrupt
    Pc14 = 2,
    ///3: Select PD14 as the source input for the EXTI14 external interrupt
    Pd14 = 3,
    ///4: Select PE14 as the source input for the EXTI14 external interrupt
    Pe14 = 4,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
impl EXTI14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI14_A> {
        match self.bits {
            0 => Some(EXTI14_A::Pa14),
            1 => Some(EXTI14_A::Pb14),
            2 => Some(EXTI14_A::Pc14),
            3 => Some(EXTI14_A::Pd14),
            4 => Some(EXTI14_A::Pe14),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa14`
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14_A::Pa14
    }
    ///Checks if the value of the field is `Pb14`
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14_A::Pb14
    }
    ///Checks if the value of the field is `Pc14`
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14_A::Pc14
    }
    ///Checks if the value of the field is `Pd14`
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == EXTI14_A::Pd14
    }
    ///Checks if the value of the field is `Pe14`
    #[inline(always)]
    pub fn is_pe14(&self) -> bool {
        *self == EXTI14_A::Pe14
    }
}
///Field `EXTI14` writer - EXTI 14 configuration bits
pub type EXTI14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI14_A, 4, O>;
impl<'a, const O: u8> EXTI14_W<'a, O> {
    ///Select PA14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pa14)
    }
    ///Select PB14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pb14)
    }
    ///Select PC14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pc14)
    }
    ///Select PD14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pd14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pd14)
    }
    ///Select PE14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pe14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pe14)
    }
}
///Field `EXTI15` reader - EXTI 15 configuration bits
pub type EXTI15_R = crate::FieldReader<u8, EXTI15_A>;
///EXTI 15 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI15_A {
    ///0: Select PA15 as the source input for the EXTI15 external interrupt
    Pa15 = 0,
    ///1: Select PB15 as the source input for the EXTI15 external interrupt
    Pb15 = 1,
    ///2: Select PC15 as the source input for the EXTI15 external interrupt
    Pc15 = 2,
    ///3: Select PD15 as the source input for the EXTI15 external interrupt
    Pd15 = 3,
    ///4: Select PE15 as the source input for the EXTI15 external interrupt
    Pe15 = 4,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
impl EXTI15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI15_A> {
        match self.bits {
            0 => Some(EXTI15_A::Pa15),
            1 => Some(EXTI15_A::Pb15),
            2 => Some(EXTI15_A::Pc15),
            3 => Some(EXTI15_A::Pd15),
            4 => Some(EXTI15_A::Pe15),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pa15`
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15_A::Pa15
    }
    ///Checks if the value of the field is `Pb15`
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15_A::Pb15
    }
    ///Checks if the value of the field is `Pc15`
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15_A::Pc15
    }
    ///Checks if the value of the field is `Pd15`
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == EXTI15_A::Pd15
    }
    ///Checks if the value of the field is `Pe15`
    #[inline(always)]
    pub fn is_pe15(&self) -> bool {
        *self == EXTI15_A::Pe15
    }
}
///Field `EXTI15` writer - EXTI 15 configuration bits
pub type EXTI15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI15_A, 4, O>;
impl<'a, const O: u8> EXTI15_W<'a, O> {
    ///Select PA15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pa15)
    }
    ///Select PB15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pb15)
    }
    ///Select PC15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pc15)
    }
    ///Select PD15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pd15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pd15)
    }
    ///Select PE15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pe15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pe15)
    }
}
impl R {
    ///Bits 0:3 - EXTI 12 configuration bits
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI 13 configuration bits
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI 14 configuration bits
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI 15 configuration bits
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - EXTI 12 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<0> {
        EXTI12_W::new(self)
    }
    ///Bits 4:7 - EXTI 13 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<4> {
        EXTI13_W::new(self)
    }
    ///Bits 8:11 - EXTI 14 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<8> {
        EXTI14_W::new(self)
    }
    ///Bits 12:15 - EXTI 15 configuration bits
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<12> {
        EXTI15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///external interrupt configuration register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr4](index.html) module
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr4::R](R) reader structure
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr4::W](W) writer structure
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
