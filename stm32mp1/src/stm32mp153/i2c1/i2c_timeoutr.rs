///Register `I2C_TIMEOUTR` reader
pub struct R(crate::R<I2C_TIMEOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TIMEOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TIMEOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TIMEOUTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2C_TIMEOUTR` writer
pub struct W(crate::W<I2C_TIMEOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TIMEOUTR_SPEC>;
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
impl From<crate::W<I2C_TIMEOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TIMEOUTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIMEOUTA` reader - TIMEOUTA
pub type TIMEOUTA_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUTA` writer - TIMEOUTA
pub type TIMEOUTA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_TIMEOUTR_SPEC, u16, u16, 12, O>;
///Field `TIDLE` reader - TIDLE
pub type TIDLE_R = crate::BitReader<bool>;
///Field `TIDLE` writer - TIDLE
pub type TIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_TIMEOUTR_SPEC, bool, O>;
///Field `TIMOUTEN` reader - TIMOUTEN
pub type TIMOUTEN_R = crate::BitReader<bool>;
///Field `TIMOUTEN` writer - TIMOUTEN
pub type TIMOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_TIMEOUTR_SPEC, bool, O>;
///Field `TIMEOUTB` reader - TIMEOUTB
pub type TIMEOUTB_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUTB` writer - TIMEOUTB
pub type TIMEOUTB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_TIMEOUTR_SPEC, u16, u16, 12, O>;
///Field `TEXTEN` reader - TEXTEN
pub type TEXTEN_R = crate::BitReader<bool>;
///Field `TEXTEN` writer - TEXTEN
pub type TEXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_TIMEOUTR_SPEC, bool, O>;
impl R {
    ///Bits 0:11 - TIMEOUTA
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - TIDLE
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - TIMOUTEN
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - TIMEOUTB
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - TEXTEN
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - TIMEOUTA
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<0> {
        TIMEOUTA_W::new(self)
    }
    ///Bit 12 - TIDLE
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<12> {
        TIDLE_W::new(self)
    }
    ///Bit 15 - TIMOUTEN
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TIMOUTEN_W<15> {
        TIMOUTEN_W::new(self)
    }
    ///Bits 16:27 - TIMEOUTB
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<16> {
        TIMEOUTB_W::new(self)
    }
    ///Bit 31 - TEXTEN
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
///Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2c_timeoutr](index.html) module
pub struct I2C_TIMEOUTR_SPEC;
impl crate::RegisterSpec for I2C_TIMEOUTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2c_timeoutr::R](R) reader structure
impl crate::Readable for I2C_TIMEOUTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2c_timeoutr::W](W) writer structure
impl crate::Writable for I2C_TIMEOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I2C_TIMEOUTR to value 0
impl crate::Resettable for I2C_TIMEOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
