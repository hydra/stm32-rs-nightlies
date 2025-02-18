///Register `I2C_ISR` reader
pub struct R(crate::R<I2C_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2C_ISR` writer
pub struct W(crate::W<I2C_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ISR_SPEC>;
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
impl From<crate::W<I2C_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXE` reader - TXE
pub type TXE_R = crate::BitReader<bool>;
///Field `TXE` writer - TXE
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ISR_SPEC, bool, O>;
///Field `TXIS` reader - TXIS
pub type TXIS_R = crate::BitReader<bool>;
///Field `TXIS` writer - TXIS
pub type TXIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ISR_SPEC, bool, O>;
///Field `RXNE` reader - RXNE
pub type RXNE_R = crate::BitReader<bool>;
///Field `ADDR` reader - ADDR
pub type ADDR_R = crate::BitReader<bool>;
///Field `NACKF` reader - NACKF
pub type NACKF_R = crate::BitReader<bool>;
///Field `STOPF` reader - STOPF
pub type STOPF_R = crate::BitReader<bool>;
///Field `TC` reader - TC
pub type TC_R = crate::BitReader<bool>;
///Field `TCR` reader - TCR
pub type TCR_R = crate::BitReader<bool>;
///Field `BERR` reader - BERR
pub type BERR_R = crate::BitReader<bool>;
///Field `ARLO` reader - ARLO
pub type ARLO_R = crate::BitReader<bool>;
///Field `OVR` reader - OVR
pub type OVR_R = crate::BitReader<bool>;
///Field `PECERR` reader - PECERR
pub type PECERR_R = crate::BitReader<bool>;
///Field `TIMEOUT` reader - TIMEOUT
pub type TIMEOUT_R = crate::BitReader<bool>;
///Field `ALERT` reader - ALERT
pub type ALERT_R = crate::BitReader<bool>;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader<bool>;
///Field `DIR` reader - DIR
pub type DIR_R = crate::BitReader<bool>;
///Field `ADDCODE` reader - ADDCODE
pub type ADDCODE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - TXE
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXIS
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADDR
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACKF
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STOPF
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TCR
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BERR
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ARLO
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - OVR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PECERR
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TIMEOUT
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ALERT
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DIR
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:23 - ADDCODE
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    ///Bit 0 - TXE
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<0> {
        TXE_W::new(self)
    }
    ///Bit 1 - TXIS
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<1> {
        TXIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Access: No wait states
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2c_isr](index.html) module
pub struct I2C_ISR_SPEC;
impl crate::RegisterSpec for I2C_ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2c_isr::R](R) reader structure
impl crate::Readable for I2C_ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2c_isr::W](W) writer structure
impl crate::Writable for I2C_ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I2C_ISR to value 0x01
impl crate::Resettable for I2C_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
