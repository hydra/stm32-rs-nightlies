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
///Field `OFFSET3` reader - Data offset 3 for the channel programmed into bits OFFSET3_CH
pub type OFFSET3_R = crate::FieldReader<u16, u16>;
///Field `OFFSET3` writer - Data offset 3 for the channel programmed into bits OFFSET3_CH
pub type OFFSET3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR3_SPEC, u16, u16, 12, O>;
///Field `OFFSETPOS` reader - Positive offset
pub type OFFSETPOS_R = crate::BitReader<bool>;
///Field `OFFSETPOS` writer - Positive offset
pub type OFFSETPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR3_SPEC, bool, O>;
///Field `SATEN` reader - Saturation enable
pub type SATEN_R = crate::BitReader<bool>;
///Field `SATEN` writer - Saturation enable
pub type SATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR3_SPEC, bool, O>;
///Field `OFFSET3_CH` reader - Channel selection for the data offset 3
pub type OFFSET3_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET3_CH` writer - Channel selection for the data offset 3
pub type OFFSET3_CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR3_SPEC, u8, u8, 5, O>;
///Field `OFFSET3_EN` reader - Offset 3 Enable
pub type OFFSET3_EN_R = crate::BitReader<OFFSET1_EN_A>;
///Offset 3 Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET1_EN_A {
    ///0: Offset disabled
    Disabled = 0,
    ///1: Offset enabled
    Enabled = 1,
}
impl From<OFFSET1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET1_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFSET3_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OFFSET1_EN_A {
        match self.bits {
            false => OFFSET1_EN_A::Disabled,
            true => OFFSET1_EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET1_EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET1_EN_A::Enabled
    }
}
///Field `OFFSET3_EN` writer - Offset 3 Enable
pub type OFFSET3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR3_SPEC, OFFSET1_EN_A, O>;
impl<'a, const O: u8> OFFSET3_EN_W<'a, O> {
    ///Offset disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OFFSET1_EN_A::Disabled)
    }
    ///Offset enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OFFSET1_EN_A::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Data offset 3 for the channel programmed into bits OFFSET3_CH
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - Channel selection for the data offset 3
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Offset 3 Enable
    #[inline(always)]
    pub fn offset3_en(&self) -> OFFSET3_EN_R {
        OFFSET3_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Data offset 3 for the channel programmed into bits OFFSET3_CH
    #[inline(always)]
    #[must_use]
    pub fn offset3(&mut self) -> OFFSET3_W<0> {
        OFFSET3_W::new(self)
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    #[must_use]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<24> {
        OFFSETPOS_W::new(self)
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    #[must_use]
    pub fn saten(&mut self) -> SATEN_W<25> {
        SATEN_W::new(self)
    }
    ///Bits 26:30 - Channel selection for the data offset 3
    #[inline(always)]
    #[must_use]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W<26> {
        OFFSET3_CH_W::new(self)
    }
    ///Bit 31 - Offset 3 Enable
    #[inline(always)]
    #[must_use]
    pub fn offset3_en(&mut self) -> OFFSET3_EN_W<31> {
        OFFSET3_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///offset register 3
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
