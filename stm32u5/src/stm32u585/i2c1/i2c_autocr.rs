///Register `I2C_AUTOCR` reader
pub struct R(crate::R<I2C_AUTOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_AUTOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_AUTOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_AUTOCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2C_AUTOCR` writer
pub struct W(crate::W<I2C_AUTOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_AUTOCR_SPEC>;
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
impl From<crate::W<I2C_AUTOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_AUTOCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TCDMAEN` reader - DMA request enable on Transfer Complete event
pub type TCDMAEN_R = crate::BitReader<bool>;
///Field `TCDMAEN` writer - DMA request enable on Transfer Complete event
pub type TCDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_AUTOCR_SPEC, bool, O>;
///Field `TCRDMAEN` reader - DMA request enable on Transfer Complete Reload event
pub type TCRDMAEN_R = crate::BitReader<bool>;
///Field `TCRDMAEN` writer - DMA request enable on Transfer Complete Reload event
pub type TCRDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_AUTOCR_SPEC, bool, O>;
///Field `TRIGSEL` reader - Trigger selection
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
///Field `TRIGSEL` writer - Trigger selection
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_AUTOCR_SPEC, u8, u8, 4, O>;
///Field `TRIGPOL` reader - Trigger polarity
pub type TRIGPOL_R = crate::BitReader<bool>;
///Field `TRIGPOL` writer - Trigger polarity
pub type TRIGPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_AUTOCR_SPEC, bool, O>;
///Field `TRIGEN` reader - Trigger enable
pub type TRIGEN_R = crate::BitReader<bool>;
///Field `TRIGEN` writer - Trigger enable
pub type TRIGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_AUTOCR_SPEC, bool, O>;
impl R {
    ///Bit 6 - DMA request enable on Transfer Complete event
    #[inline(always)]
    pub fn tcdmaen(&self) -> TCDMAEN_R {
        TCDMAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA request enable on Transfer Complete Reload event
    #[inline(always)]
    pub fn tcrdmaen(&self) -> TCRDMAEN_R {
        TCRDMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:19 - Trigger selection
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - Trigger polarity
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Trigger enable
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - DMA request enable on Transfer Complete event
    #[inline(always)]
    #[must_use]
    pub fn tcdmaen(&mut self) -> TCDMAEN_W<6> {
        TCDMAEN_W::new(self)
    }
    ///Bit 7 - DMA request enable on Transfer Complete Reload event
    #[inline(always)]
    #[must_use]
    pub fn tcrdmaen(&mut self) -> TCRDMAEN_W<7> {
        TCRDMAEN_W::new(self)
    }
    ///Bits 16:19 - Trigger selection
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<16> {
        TRIGSEL_W::new(self)
    }
    ///Bit 20 - Trigger polarity
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<20> {
        TRIGPOL_W::new(self)
    }
    ///Bit 21 - Trigger enable
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<21> {
        TRIGEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I2C Autonomous mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2c_autocr](index.html) module
pub struct I2C_AUTOCR_SPEC;
impl crate::RegisterSpec for I2C_AUTOCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2c_autocr::R](R) reader structure
impl crate::Readable for I2C_AUTOCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2c_autocr::W](W) writer structure
impl crate::Writable for I2C_AUTOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I2C_AUTOCR to value 0
impl crate::Resettable for I2C_AUTOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
