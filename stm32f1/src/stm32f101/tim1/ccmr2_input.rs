///Register `CCMR2_Input` reader
pub struct R(crate::R<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR2_Input` writer
pub struct W(crate::W<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_INPUT_SPEC>;
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
impl From<crate::W<CCMR2_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC3S` reader - Capture/compare 3 selection
pub type CC3S_R = crate::FieldReader<u8, CC3S_A>;
///Capture/compare 3 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S_A {
    ///1: CC3 channel is configured as input, IC3 is mapped on TI3
    Ti3 = 1,
    ///2: CC3 channel is configured as input, IC3 is mapped on TI4
    Ti4 = 2,
    ///3: CC3 channel is configured as input, IC3 is mapped on TRC
    Trc = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
impl CC3S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC3S_A> {
        match self.bits {
            1 => Some(CC3S_A::Ti3),
            2 => Some(CC3S_A::Ti4),
            3 => Some(CC3S_A::Trc),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Ti3`
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        *self == CC3S_A::Ti3
    }
    ///Checks if the value of the field is `Ti4`
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        *self == CC3S_A::Ti4
    }
    ///Checks if the value of the field is `Trc`
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC3S_A::Trc
    }
}
///Field `CC3S` writer - Capture/compare 3 selection
pub type CC3S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_INPUT_SPEC, u8, CC3S_A, 2, O>;
impl<'a, const O: u8> CC3S_W<'a, O> {
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC3S_A::Ti3)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC3S_A::Ti4)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC3S_A::Trc)
    }
}
///Field `IC3PSC` reader - Input capture 3 prescaler
pub type IC3PSC_R = crate::FieldReader<u8, u8>;
///Field `IC3PSC` writer - Input capture 3 prescaler
pub type IC3PSC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR2_INPUT_SPEC, u8, u8, 2, O>;
///Field `IC3F` reader - Input capture 3 filter
pub type IC3F_R = crate::FieldReader<u8, u8>;
///Field `IC3F` writer - Input capture 3 filter
pub type IC3F_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCMR2_INPUT_SPEC, u8, u8, 4, O>;
///Field `CC4S` reader - Capture/Compare 4 selection
pub type CC4S_R = crate::FieldReader<u8, CC4S_A>;
///Capture/Compare 4 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC4S_A {
    ///1: CC4 channel is configured as input, IC4 is mapped on TI4
    Ti4 = 1,
    ///2: CC4 channel is configured as input, IC4 is mapped on TI3
    Ti3 = 2,
    ///3: CC4 channel is configured as input, IC4 is mapped on TRC
    Trc = 3,
}
impl From<CC4S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC4S_A) -> Self {
        variant as _
    }
}
impl CC4S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC4S_A> {
        match self.bits {
            1 => Some(CC4S_A::Ti4),
            2 => Some(CC4S_A::Ti3),
            3 => Some(CC4S_A::Trc),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Ti4`
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        *self == CC4S_A::Ti4
    }
    ///Checks if the value of the field is `Ti3`
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        *self == CC4S_A::Ti3
    }
    ///Checks if the value of the field is `Trc`
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC4S_A::Trc
    }
}
///Field `CC4S` writer - Capture/Compare 4 selection
pub type CC4S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_INPUT_SPEC, u8, CC4S_A, 2, O>;
impl<'a, const O: u8> CC4S_W<'a, O> {
    ///CC4 channel is configured as input, IC4 is mapped on TI4
    #[inline(always)]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC4S_A::Ti4)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI3
    #[inline(always)]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC4S_A::Ti3)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC4S_A::Trc)
    }
}
///Field `IC4PSC` reader - Input capture 4 prescaler
pub type IC4PSC_R = crate::FieldReader<u8, u8>;
///Field `IC4PSC` writer - Input capture 4 prescaler
pub type IC4PSC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR2_INPUT_SPEC, u8, u8, 2, O>;
///Field `IC4F` reader - Input capture 4 filter
pub type IC4F_R = crate::FieldReader<u8, u8>;
///Field `IC4F` writer - Input capture 4 filter
pub type IC4F_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCMR2_INPUT_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:1 - Capture/compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 15:18 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Capture/compare 3 selection
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<0> {
        CC3S_W::new(self)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> IC3PSC_W<2> {
        IC3PSC_W::new(self)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> IC3F_W<4> {
        IC3F_W::new(self)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<8> {
        CC4S_W::new(self)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> IC4PSC_W<10> {
        IC4PSC_W::new(self)
    }
    ///Bits 15:18 - Input capture 4 filter
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> IC4F_W<15> {
        IC4F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 2 (input mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr2_input](index.html) module
pub struct CCMR2_INPUT_SPEC;
impl crate::RegisterSpec for CCMR2_INPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr2_input::R](R) reader structure
impl crate::Readable for CCMR2_INPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr2_input::W](W) writer structure
impl crate::Writable for CCMR2_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR2_Input to value 0
impl crate::Resettable for CCMR2_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
