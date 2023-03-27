///Register `OFR2` reader
pub struct R(crate::R<OFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OFR2` writer
pub struct W(crate::W<OFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR2_SPEC>;
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
impl From<crate::W<OFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET2` reader - Data offset 2 for the channel programmed into bits OFFSET2_CH
pub type OFFSET2_R = crate::FieldReader<u16, u16>;
///Field `OFFSET2` writer - Data offset 2 for the channel programmed into bits OFFSET2_CH
pub type OFFSET2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR2_SPEC, u16, u16, 12, O>;
///Field `OFFSETPOS` reader - Positive offset
pub type OFFSETPOS_R = crate::BitReader<bool>;
///Field `OFFSETPOS` writer - Positive offset
pub type OFFSETPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR2_SPEC, bool, O>;
///Field `SATEN` reader - Saturation enable
pub type SATEN_R = crate::BitReader<bool>;
///Field `SATEN` writer - Saturation enable
pub type SATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR2_SPEC, bool, O>;
///Field `OFFSET2_CH` reader - Channel selection for the data offset 2
pub type OFFSET2_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET2_CH` writer - Channel selection for the data offset 2
pub type OFFSET2_CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR2_SPEC, u8, u8, 5, O>;
///Field `OFFSET2_EN` reader - Offset 2 Enable
pub type OFFSET2_EN_R = crate::BitReader<OFFSET1_EN_A>;
///Offset 2 Enable
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
impl OFFSET2_EN_R {
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
///Field `OFFSET2_EN` writer - Offset 2 Enable
pub type OFFSET2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR2_SPEC, OFFSET1_EN_A, O>;
impl<'a, const O: u8> OFFSET2_EN_W<'a, O> {
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
    ///Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
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
    ///Bits 26:30 - Channel selection for the data offset 2
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Offset 2 Enable
    #[inline(always)]
    pub fn offset2_en(&self) -> OFFSET2_EN_R {
        OFFSET2_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<0> {
        OFFSET2_W::new(self)
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
    ///Bits 26:30 - Channel selection for the data offset 2
    #[inline(always)]
    #[must_use]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<26> {
        OFFSET2_CH_W::new(self)
    }
    ///Bit 31 - Offset 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn offset2_en(&mut self) -> OFFSET2_EN_W<31> {
        OFFSET2_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///offset register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr2](index.html) module
pub struct OFR2_SPEC;
impl crate::RegisterSpec for OFR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ofr2::R](R) reader structure
impl crate::Readable for OFR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ofr2::W](W) writer structure
impl crate::Writable for OFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OFR2 to value 0
impl crate::Resettable for OFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
