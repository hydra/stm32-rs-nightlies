///Register `FLTR` reader
pub struct R(crate::R<FLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLTR` writer
pub struct W(crate::W<FLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTR_SPEC>;
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
impl From<crate::W<FLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DNF` reader - Digital noise filter
pub type DNF_R = crate::FieldReader<u8, DNF_A>;
///Digital noise filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF_A {
    ///0: Digital filter disabled
    NoFilter = 0,
    ///1: Digital filter enabled and filtering capability up to 1 tI2CCLK
    Filter1 = 1,
    ///2: Digital filter enabled and filtering capability up to 2 tI2CCLK
    Filter2 = 2,
    ///3: Digital filter enabled and filtering capability up to 3 tI2CCLK
    Filter3 = 3,
    ///4: Digital filter enabled and filtering capability up to 4 tI2CCLK
    Filter4 = 4,
    ///5: Digital filter enabled and filtering capability up to 5 tI2CCLK
    Filter5 = 5,
    ///6: Digital filter enabled and filtering capability up to 6 tI2CCLK
    Filter6 = 6,
    ///7: Digital filter enabled and filtering capability up to 7 tI2CCLK
    Filter7 = 7,
    ///8: Digital filter enabled and filtering capability up to 8 tI2CCLK
    Filter8 = 8,
    ///9: Digital filter enabled and filtering capability up to 9 tI2CCLK
    Filter9 = 9,
    ///10: Digital filter enabled and filtering capability up to 10 tI2CCLK
    Filter10 = 10,
    ///11: Digital filter enabled and filtering capability up to 11 tI2CCLK
    Filter11 = 11,
    ///12: Digital filter enabled and filtering capability up to 12 tI2CCLK
    Filter12 = 12,
    ///13: Digital filter enabled and filtering capability up to 13 tI2CCLK
    Filter13 = 13,
    ///14: Digital filter enabled and filtering capability up to 14 tI2CCLK
    Filter14 = 14,
    ///15: Digital filter enabled and filtering capability up to 15 tI2CCLK
    Filter15 = 15,
}
impl From<DNF_A> for u8 {
    #[inline(always)]
    fn from(variant: DNF_A) -> Self {
        variant as _
    }
}
impl DNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DNF_A {
        match self.bits {
            0 => DNF_A::NoFilter,
            1 => DNF_A::Filter1,
            2 => DNF_A::Filter2,
            3 => DNF_A::Filter3,
            4 => DNF_A::Filter4,
            5 => DNF_A::Filter5,
            6 => DNF_A::Filter6,
            7 => DNF_A::Filter7,
            8 => DNF_A::Filter8,
            9 => DNF_A::Filter9,
            10 => DNF_A::Filter10,
            11 => DNF_A::Filter11,
            12 => DNF_A::Filter12,
            13 => DNF_A::Filter13,
            14 => DNF_A::Filter14,
            15 => DNF_A::Filter15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoFilter`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == DNF_A::NoFilter
    }
    ///Checks if the value of the field is `Filter1`
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DNF_A::Filter1
    }
    ///Checks if the value of the field is `Filter2`
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DNF_A::Filter2
    }
    ///Checks if the value of the field is `Filter3`
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DNF_A::Filter3
    }
    ///Checks if the value of the field is `Filter4`
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DNF_A::Filter4
    }
    ///Checks if the value of the field is `Filter5`
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DNF_A::Filter5
    }
    ///Checks if the value of the field is `Filter6`
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DNF_A::Filter6
    }
    ///Checks if the value of the field is `Filter7`
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DNF_A::Filter7
    }
    ///Checks if the value of the field is `Filter8`
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DNF_A::Filter8
    }
    ///Checks if the value of the field is `Filter9`
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == DNF_A::Filter9
    }
    ///Checks if the value of the field is `Filter10`
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == DNF_A::Filter10
    }
    ///Checks if the value of the field is `Filter11`
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == DNF_A::Filter11
    }
    ///Checks if the value of the field is `Filter12`
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == DNF_A::Filter12
    }
    ///Checks if the value of the field is `Filter13`
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == DNF_A::Filter13
    }
    ///Checks if the value of the field is `Filter14`
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == DNF_A::Filter14
    }
    ///Checks if the value of the field is `Filter15`
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == DNF_A::Filter15
    }
}
///Field `DNF` writer - Digital noise filter
pub type DNF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FLTR_SPEC, u8, DNF_A, 4, O>;
impl<'a, const O: u8> DNF_W<'a, O> {
    ///Digital filter disabled
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(DNF_A::NoFilter)
    }
    ///Digital filter enabled and filtering capability up to 1 tI2CCLK
    #[inline(always)]
    pub fn filter1(self) -> &'a mut W {
        self.variant(DNF_A::Filter1)
    }
    ///Digital filter enabled and filtering capability up to 2 tI2CCLK
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(DNF_A::Filter2)
    }
    ///Digital filter enabled and filtering capability up to 3 tI2CCLK
    #[inline(always)]
    pub fn filter3(self) -> &'a mut W {
        self.variant(DNF_A::Filter3)
    }
    ///Digital filter enabled and filtering capability up to 4 tI2CCLK
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(DNF_A::Filter4)
    }
    ///Digital filter enabled and filtering capability up to 5 tI2CCLK
    #[inline(always)]
    pub fn filter5(self) -> &'a mut W {
        self.variant(DNF_A::Filter5)
    }
    ///Digital filter enabled and filtering capability up to 6 tI2CCLK
    #[inline(always)]
    pub fn filter6(self) -> &'a mut W {
        self.variant(DNF_A::Filter6)
    }
    ///Digital filter enabled and filtering capability up to 7 tI2CCLK
    #[inline(always)]
    pub fn filter7(self) -> &'a mut W {
        self.variant(DNF_A::Filter7)
    }
    ///Digital filter enabled and filtering capability up to 8 tI2CCLK
    #[inline(always)]
    pub fn filter8(self) -> &'a mut W {
        self.variant(DNF_A::Filter8)
    }
    ///Digital filter enabled and filtering capability up to 9 tI2CCLK
    #[inline(always)]
    pub fn filter9(self) -> &'a mut W {
        self.variant(DNF_A::Filter9)
    }
    ///Digital filter enabled and filtering capability up to 10 tI2CCLK
    #[inline(always)]
    pub fn filter10(self) -> &'a mut W {
        self.variant(DNF_A::Filter10)
    }
    ///Digital filter enabled and filtering capability up to 11 tI2CCLK
    #[inline(always)]
    pub fn filter11(self) -> &'a mut W {
        self.variant(DNF_A::Filter11)
    }
    ///Digital filter enabled and filtering capability up to 12 tI2CCLK
    #[inline(always)]
    pub fn filter12(self) -> &'a mut W {
        self.variant(DNF_A::Filter12)
    }
    ///Digital filter enabled and filtering capability up to 13 tI2CCLK
    #[inline(always)]
    pub fn filter13(self) -> &'a mut W {
        self.variant(DNF_A::Filter13)
    }
    ///Digital filter enabled and filtering capability up to 14 tI2CCLK
    #[inline(always)]
    pub fn filter14(self) -> &'a mut W {
        self.variant(DNF_A::Filter14)
    }
    ///Digital filter enabled and filtering capability up to 15 tI2CCLK
    #[inline(always)]
    pub fn filter15(self) -> &'a mut W {
        self.variant(DNF_A::Filter15)
    }
}
///Field `ANOFF` reader - Analog noise filter OFF
pub type ANOFF_R = crate::BitReader<ANOFF_A>;
///Analog noise filter OFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANOFF_A {
    ///0: Analog noise filter enabled
    Enabled = 0,
    ///1: Analog noise filter disabled
    Disabled = 1,
}
impl From<ANOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl ANOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ANOFF_A {
        match self.bits {
            false => ANOFF_A::Enabled,
            true => ANOFF_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ANOFF_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANOFF_A::Disabled
    }
}
///Field `ANOFF` writer - Analog noise filter OFF
pub type ANOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTR_SPEC, ANOFF_A, O>;
impl<'a, const O: u8> ANOFF_W<'a, O> {
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ANOFF_A::Enabled)
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ANOFF_A::Disabled)
    }
}
impl R {
    ///Bits 0:3 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Analog noise filter OFF
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Digital noise filter
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<0> {
        DNF_W::new(self)
    }
    ///Bit 4 - Analog noise filter OFF
    #[inline(always)]
    #[must_use]
    pub fn anoff(&mut self) -> ANOFF_W<4> {
        ANOFF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I2C FLTR register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltr](index.html) module
pub struct FLTR_SPEC;
impl crate::RegisterSpec for FLTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fltr::R](R) reader structure
impl crate::Readable for FLTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fltr::W](W) writer structure
impl crate::Writable for FLTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLTR to value 0
impl crate::Resettable for FLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
