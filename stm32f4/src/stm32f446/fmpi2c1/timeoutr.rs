///Register `TIMEOUTR` reader
pub struct R(crate::R<TIMEOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMEOUTR` writer
pub struct W(crate::W<TIMEOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUTR_SPEC>;
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
impl From<crate::W<TIMEOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIMEOUTA` reader - Bus timeout A
pub type TIMEOUTA_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUTA` writer - Bus timeout A
pub type TIMEOUTA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMEOUTR_SPEC, u16, u16, 12, O>;
///Field `TIDLE` reader - Idle clock timeout detection
pub type TIDLE_R = crate::BitReader<TIDLE_A>;
///Idle clock timeout detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIDLE_A {
    ///0: TIMEOUTA is used to detect SCL low timeout
    Disabled = 0,
    ///1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
    Enabled = 1,
}
impl From<TIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: TIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIDLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIDLE_A {
        match self.bits {
            false => TIDLE_A::Disabled,
            true => TIDLE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIDLE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIDLE_A::Enabled
    }
}
///Field `TIDLE` writer - Idle clock timeout detection
pub type TIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, TIDLE_A, O>;
impl<'a, const O: u8> TIDLE_W<'a, O> {
    ///TIMEOUTA is used to detect SCL low timeout
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIDLE_A::Disabled)
    }
    ///TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIDLE_A::Enabled)
    }
}
///Field `TIMOUTEN` reader - Clock timeout enable
pub type TIMOUTEN_R = crate::BitReader<TIMOUTEN_A>;
///Clock timeout enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTEN_A {
    ///0: SCL timeout detection is disabled
    Disabled = 0,
    ///1: SCL timeout detection is enabled
    Enabled = 1,
}
impl From<TIMOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMOUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMOUTEN_A {
        match self.bits {
            false => TIMOUTEN_A::Disabled,
            true => TIMOUTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUTEN_A::Enabled
    }
}
///Field `TIMOUTEN` writer - Clock timeout enable
pub type TIMOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, TIMOUTEN_A, O>;
impl<'a, const O: u8> TIMOUTEN_W<'a, O> {
    ///SCL timeout detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMOUTEN_A::Disabled)
    }
    ///SCL timeout detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMOUTEN_A::Enabled)
    }
}
///Field `TIMEOUTB` reader - Bus timeout B
pub type TIMEOUTB_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUTB` writer - Bus timeout B
pub type TIMEOUTB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMEOUTR_SPEC, u16, u16, 12, O>;
///Field `TEXTEN` reader - Extended clock timeout enable
pub type TEXTEN_R = crate::BitReader<TEXTEN_A>;
///Extended clock timeout enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTEN_A {
    ///0: Extended clock timeout detection is disabled
    Disabled = 0,
    ///1: Extended clock timeout detection is enabled
    Enabled = 1,
}
impl From<TEXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEXTEN_A {
        match self.bits {
            false => TEXTEN_A::Disabled,
            true => TEXTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEXTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEXTEN_A::Enabled
    }
}
///Field `TEXTEN` writer - Extended clock timeout enable
pub type TEXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, TEXTEN_A, O>;
impl<'a, const O: u8> TEXTEN_W<'a, O> {
    ///Extended clock timeout detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEXTEN_A::Disabled)
    }
    ///Extended clock timeout detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEXTEN_A::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<0> {
        TIMEOUTA_W::new(self)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<12> {
        TIDLE_W::new(self)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TIMOUTEN_W<15> {
        TIMOUTEN_W::new(self)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<16> {
        TIMEOUTB_W::new(self)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    #[must_use]
    pub fn texten(&mut self) -> TEXTEN_W<31> {
        TEXTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timeout register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timeoutr](index.html) module
pub struct TIMEOUTR_SPEC;
impl crate::RegisterSpec for TIMEOUTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [timeoutr::R](R) reader structure
impl crate::Readable for TIMEOUTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timeoutr::W](W) writer structure
impl crate::Writable for TIMEOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMEOUTR to value 0
impl crate::Resettable for TIMEOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
