///Register `PR` reader
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PR` writer
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PR` reader - Prescaler divider
pub type PR_R = crate::FieldReader<u8, PR_A>;
///Prescaler divider
///
///Value on reset: 7
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    ///0: Divider /4
    DivideBy4 = 0,
    ///1: Divider /8
    DivideBy8 = 1,
    ///2: Divider /16
    DivideBy16 = 2,
    ///3: Divider /32
    DivideBy32 = 3,
    ///4: Divider /64
    DivideBy64 = 4,
    ///5: Divider /128
    DivideBy128 = 5,
    ///6: Divider /256
    DivideBy256 = 6,
    ///7: Divider /256
    DivideBy256bis = 7,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl PR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::DivideBy4,
            1 => PR_A::DivideBy8,
            2 => PR_A::DivideBy16,
            3 => PR_A::DivideBy32,
            4 => PR_A::DivideBy64,
            5 => PR_A::DivideBy128,
            6 => PR_A::DivideBy256,
            7 => PR_A::DivideBy256bis,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DivideBy4`
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == PR_A::DivideBy4
    }
    ///Checks if the value of the field is `DivideBy8`
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == PR_A::DivideBy8
    }
    ///Checks if the value of the field is `DivideBy16`
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == PR_A::DivideBy16
    }
    ///Checks if the value of the field is `DivideBy32`
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == PR_A::DivideBy32
    }
    ///Checks if the value of the field is `DivideBy64`
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == PR_A::DivideBy64
    }
    ///Checks if the value of the field is `DivideBy128`
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == PR_A::DivideBy128
    }
    ///Checks if the value of the field is `DivideBy256`
    #[inline(always)]
    pub fn is_divide_by256(&self) -> bool {
        *self == PR_A::DivideBy256
    }
    ///Checks if the value of the field is `DivideBy256bis`
    #[inline(always)]
    pub fn is_divide_by256bis(&self) -> bool {
        *self == PR_A::DivideBy256bis
    }
}
///Field `PR` writer - Prescaler divider
pub type PR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PR_SPEC, u8, PR_A, 3, O>;
impl<'a, const O: u8> PR_W<'a, O> {
    ///Divider /4
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut W {
        self.variant(PR_A::DivideBy4)
    }
    ///Divider /8
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut W {
        self.variant(PR_A::DivideBy8)
    }
    ///Divider /16
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut W {
        self.variant(PR_A::DivideBy16)
    }
    ///Divider /32
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut W {
        self.variant(PR_A::DivideBy32)
    }
    ///Divider /64
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut W {
        self.variant(PR_A::DivideBy64)
    }
    ///Divider /128
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut W {
        self.variant(PR_A::DivideBy128)
    }
    ///Divider /256
    #[inline(always)]
    pub fn divide_by256(self) -> &'a mut W {
        self.variant(PR_A::DivideBy256)
    }
    ///Divider /256
    #[inline(always)]
    pub fn divide_by256bis(self) -> &'a mut W {
        self.variant(PR_A::DivideBy256bis)
    }
}
impl R {
    ///Bits 0:2 - Prescaler divider
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Prescaler divider
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr](index.html) module
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pr::R](R) reader structure
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pr::W](W) writer structure
impl crate::Writable for PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PR to value 0x07
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
