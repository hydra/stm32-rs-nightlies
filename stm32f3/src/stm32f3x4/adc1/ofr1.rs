///Register `OFR1` reader
pub struct R(crate::R<OFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OFR1` writer
pub struct W(crate::W<OFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR1_SPEC>;
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
impl From<crate::W<OFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET1` reader - OFFSET1
pub type OFFSET1_R = crate::FieldReader<u16, u16>;
///Field `OFFSET1` writer - OFFSET1
pub type OFFSET1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR1_SPEC, u16, u16, 12, O>;
///Field `OFFSET1_CH` reader - OFFSET1_CH
pub type OFFSET1_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET1_CH` writer - OFFSET1_CH
pub type OFFSET1_CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR1_SPEC, u8, u8, 5, O>;
///Field `OFFSET1_EN` reader - OFFSET1_EN
pub type OFFSET1_EN_R = crate::BitReader<OFFSET1_EN_A>;
///OFFSET1_EN
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
impl OFFSET1_EN_R {
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
///Field `OFFSET1_EN` writer - OFFSET1_EN
pub type OFFSET1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR1_SPEC, OFFSET1_EN_A, O>;
impl<'a, const O: u8> OFFSET1_EN_W<'a, O> {
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
    ///Bits 0:11 - OFFSET1
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - OFFSET1_EN
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - OFFSET1
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<0> {
        OFFSET1_W::new(self)
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    #[must_use]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<26> {
        OFFSET1_CH_W::new(self)
    }
    ///Bit 31 - OFFSET1_EN
    #[inline(always)]
    #[must_use]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W<31> {
        OFFSET1_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///offset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr1](index.html) module
pub struct OFR1_SPEC;
impl crate::RegisterSpec for OFR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ofr1::R](R) reader structure
impl crate::Readable for OFR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ofr1::W](W) writer structure
impl crate::Writable for OFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OFR1 to value 0
impl crate::Resettable for OFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
