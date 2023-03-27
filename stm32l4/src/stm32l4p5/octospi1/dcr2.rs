///Register `DCR2` reader
pub struct R(crate::R<DCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCR2` writer
pub struct W(crate::W<DCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR2_SPEC>;
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
impl From<crate::W<DCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRESCALER` reader - Clock prescaler
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
///Field `PRESCALER` writer - Clock prescaler
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCR2_SPEC, u8, u8, 8, O>;
///Field `WRAPSIZE` reader - Wrap size
pub type WRAPSIZE_R = crate::FieldReader<u8, WRAPSIZE_A>;
///Wrap size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRAPSIZE_A {
    ///0: Wrapped reads are not supported by the memory
    NoWrappingSupport = 0,
    ///2: External memory supports wrap size of 16 bytes
    WrappingSize16 = 2,
    ///3: External memory supports wrap size of 16 bytes
    WrappingSize32 = 3,
    ///4: External memory supports wrap size of 16 bytes
    WrappingSize64 = 4,
    ///5: External memory supports wrap size of 16 bytes
    WrappingSize128 = 5,
}
impl From<WRAPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: WRAPSIZE_A) -> Self {
        variant as _
    }
}
impl WRAPSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WRAPSIZE_A> {
        match self.bits {
            0 => Some(WRAPSIZE_A::NoWrappingSupport),
            2 => Some(WRAPSIZE_A::WrappingSize16),
            3 => Some(WRAPSIZE_A::WrappingSize32),
            4 => Some(WRAPSIZE_A::WrappingSize64),
            5 => Some(WRAPSIZE_A::WrappingSize128),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoWrappingSupport`
    #[inline(always)]
    pub fn is_no_wrapping_support(&self) -> bool {
        *self == WRAPSIZE_A::NoWrappingSupport
    }
    ///Checks if the value of the field is `WrappingSize16`
    #[inline(always)]
    pub fn is_wrapping_size16(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize16
    }
    ///Checks if the value of the field is `WrappingSize32`
    #[inline(always)]
    pub fn is_wrapping_size32(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize32
    }
    ///Checks if the value of the field is `WrappingSize64`
    #[inline(always)]
    pub fn is_wrapping_size64(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize64
    }
    ///Checks if the value of the field is `WrappingSize128`
    #[inline(always)]
    pub fn is_wrapping_size128(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize128
    }
}
///Field `WRAPSIZE` writer - Wrap size
pub type WRAPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR2_SPEC, u8, WRAPSIZE_A, 3, O>;
impl<'a, const O: u8> WRAPSIZE_W<'a, O> {
    ///Wrapped reads are not supported by the memory
    #[inline(always)]
    pub fn no_wrapping_support(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::NoWrappingSupport)
    }
    ///External memory supports wrap size of 16 bytes
    #[inline(always)]
    pub fn wrapping_size16(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize16)
    }
    ///External memory supports wrap size of 16 bytes
    #[inline(always)]
    pub fn wrapping_size32(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize32)
    }
    ///External memory supports wrap size of 16 bytes
    #[inline(always)]
    pub fn wrapping_size64(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize64)
    }
    ///External memory supports wrap size of 16 bytes
    #[inline(always)]
    pub fn wrapping_size128(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize128)
    }
}
impl R {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    #[must_use]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W<16> {
        WRAPSIZE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///device configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr2](index.html) module
pub struct DCR2_SPEC;
impl crate::RegisterSpec for DCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcr2::R](R) reader structure
impl crate::Readable for DCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dcr2::W](W) writer structure
impl crate::Writable for DCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCR2 to value 0
impl crate::Resettable for DCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
