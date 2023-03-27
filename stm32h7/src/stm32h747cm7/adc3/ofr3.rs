///Register `OFR3` reader
pub struct R(crate::R<OFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OFR3` writer
pub struct W(crate::W<OFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR3_SPEC>;
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
impl From<crate::W<OFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET3` reader - ADC offset number 1 offset level
pub type OFFSET3_R = crate::FieldReader<u32, u32>;
///Field `OFFSET3` writer - ADC offset number 1 offset level
pub type OFFSET3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR3_SPEC, u32, u32, 26, O>;
///Field `OFFSET3_CH` reader - ADC offset number 1 channel selection
pub type OFFSET3_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET3_CH` writer - ADC offset number 1 channel selection
pub type OFFSET3_CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR3_SPEC, u8, u8, 5, O>;
///Field `SSATE` reader - Signed saturation enable
pub type SSATE_R = crate::BitReader<SSATE_A>;
///Signed saturation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSATE_A {
    ///0: Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)
    Disabled = 0,
    ///1: Offset is subtracted and result is saturated to maintain result size
    Enabled = 1,
}
impl From<SSATE_A> for bool {
    #[inline(always)]
    fn from(variant: SSATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSATE_A {
        match self.bits {
            false => SSATE_A::Disabled,
            true => SSATE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSATE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSATE_A::Enabled
    }
}
///Field `SSATE` writer - Signed saturation enable
pub type SSATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR3_SPEC, SSATE_A, O>;
impl<'a, const O: u8> SSATE_W<'a, O> {
    ///Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSATE_A::Disabled)
    }
    ///Offset is subtracted and result is saturated to maintain result size
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSATE_A::Enabled)
    }
}
impl R {
    ///Bits 0:25 - ADC offset number 1 offset level
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:30 - ADC offset number 1 channel selection
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Signed saturation enable
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:25 - ADC offset number 1 offset level
    #[inline(always)]
    #[must_use]
    pub fn offset3(&mut self) -> OFFSET3_W<0> {
        OFFSET3_W::new(self)
    }
    ///Bits 26:30 - ADC offset number 1 channel selection
    #[inline(always)]
    #[must_use]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W<26> {
        OFFSET3_CH_W::new(self)
    }
    ///Bit 31 - Signed saturation enable
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<31> {
        SSATE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC offset number 3 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr3](index.html) module
pub struct OFR3_SPEC;
impl crate::RegisterSpec for OFR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ofr3::R](R) reader structure
impl crate::Readable for OFR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ofr3::W](W) writer structure
impl crate::Writable for OFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OFR3 to value 0
impl crate::Resettable for OFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
