///Register `I2C_CR1` reader
pub struct R(crate::R<I2C_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2C_CR1` writer
pub struct W(crate::W<I2C_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CR1_SPEC>;
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
impl From<crate::W<I2C_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PE` reader - PE
pub type PE_R = crate::BitReader<bool>;
///Field `PE` writer - PE
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `TXIE` reader - TXIE
pub type TXIE_R = crate::BitReader<bool>;
///Field `TXIE` writer - TXIE
pub type TXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `RXIE` reader - RXIE
pub type RXIE_R = crate::BitReader<bool>;
///Field `RXIE` writer - RXIE
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `ADDRIE` reader - ADDRIE
pub type ADDRIE_R = crate::BitReader<bool>;
///Field `ADDRIE` writer - ADDRIE
pub type ADDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `NACKIE` reader - NACKIE
pub type NACKIE_R = crate::BitReader<bool>;
///Field `NACKIE` writer - NACKIE
pub type NACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `STOPIE` reader - STOPIE
pub type STOPIE_R = crate::BitReader<bool>;
///Field `STOPIE` writer - STOPIE
pub type STOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader<bool>;
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `DNF` reader - DNF
pub type DNF_R = crate::FieldReader<u8, u8>;
///Field `DNF` writer - DNF
pub type DNF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_CR1_SPEC, u8, u8, 4, O>;
///Field `ANFOFF` reader - ANFOFF
pub type ANFOFF_R = crate::BitReader<bool>;
///Field `ANFOFF` writer - ANFOFF
pub type ANFOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `TXDMAEN` reader - TXDMAEN
pub type TXDMAEN_R = crate::BitReader<bool>;
///Field `TXDMAEN` writer - TXDMAEN
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `RXDMAEN` reader - RXDMAEN
pub type RXDMAEN_R = crate::BitReader<bool>;
///Field `RXDMAEN` writer - RXDMAEN
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `SBC` reader - SBC
pub type SBC_R = crate::BitReader<bool>;
///Field `SBC` writer - SBC
pub type SBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `NOSTRETCH` reader - NOSTRETCH
pub type NOSTRETCH_R = crate::BitReader<bool>;
///Field `NOSTRETCH` writer - NOSTRETCH
pub type NOSTRETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `WUPEN` reader - WUPEN
pub type WUPEN_R = crate::BitReader<bool>;
///Field `WUPEN` writer - WUPEN
pub type WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `GCEN` reader - GCEN
pub type GCEN_R = crate::BitReader<bool>;
///Field `GCEN` writer - GCEN
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `SMBHEN` reader - SMBHEN
pub type SMBHEN_R = crate::BitReader<bool>;
///Field `SMBHEN` writer - SMBHEN
pub type SMBHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `SMBDEN` reader - SMBDEN
pub type SMBDEN_R = crate::BitReader<bool>;
///Field `SMBDEN` writer - SMBDEN
pub type SMBDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `ALERTEN` reader - ALERTEN
pub type ALERTEN_R = crate::BitReader<bool>;
///Field `ALERTEN` writer - ALERTEN
pub type ALERTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
///Field `PECEN` reader - PECEN
pub type PECEN_R = crate::BitReader<bool>;
///Field `PECEN` writer - PECEN
pub type PECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXIE
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXIE
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADDRIE
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACKIE
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STOPIE
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - DNF
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - ANFOFF
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RXDMAEN
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SBC
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NOSTRETCH
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - WUPEN
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GCEN
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SMBHEN
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SMBDEN
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ALERTEN
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PECEN
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PE
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    ///Bit 1 - TXIE
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<1> {
        TXIE_W::new(self)
    }
    ///Bit 2 - RXIE
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<2> {
        RXIE_W::new(self)
    }
    ///Bit 3 - ADDRIE
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> ADDRIE_W<3> {
        ADDRIE_W::new(self)
    }
    ///Bit 4 - NACKIE
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NACKIE_W<4> {
        NACKIE_W::new(self)
    }
    ///Bit 5 - STOPIE
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> STOPIE_W<5> {
        STOPIE_W::new(self)
    }
    ///Bit 6 - TCIE
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    ///Bit 7 - ERRIE
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<7> {
        ERRIE_W::new(self)
    }
    ///Bits 8:11 - DNF
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<8> {
        DNF_W::new(self)
    }
    ///Bit 12 - ANFOFF
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> ANFOFF_W<12> {
        ANFOFF_W::new(self)
    }
    ///Bit 14 - TXDMAEN
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<14> {
        TXDMAEN_W::new(self)
    }
    ///Bit 15 - RXDMAEN
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<15> {
        RXDMAEN_W::new(self)
    }
    ///Bit 16 - SBC
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<16> {
        SBC_W::new(self)
    }
    ///Bit 17 - NOSTRETCH
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<17> {
        NOSTRETCH_W::new(self)
    }
    ///Bit 18 - WUPEN
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<18> {
        WUPEN_W::new(self)
    }
    ///Bit 19 - GCEN
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<19> {
        GCEN_W::new(self)
    }
    ///Bit 20 - SMBHEN
    #[inline(always)]
    #[must_use]
    pub fn smbhen(&mut self) -> SMBHEN_W<20> {
        SMBHEN_W::new(self)
    }
    ///Bit 21 - SMBDEN
    #[inline(always)]
    #[must_use]
    pub fn smbden(&mut self) -> SMBDEN_W<21> {
        SMBDEN_W::new(self)
    }
    ///Bit 22 - ALERTEN
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> ALERTEN_W<22> {
        ALERTEN_W::new(self)
    }
    ///Bit 23 - PECEN
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<23> {
        PECEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2c_cr1](index.html) module
pub struct I2C_CR1_SPEC;
impl crate::RegisterSpec for I2C_CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2c_cr1::R](R) reader structure
impl crate::Readable for I2C_CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2c_cr1::W](W) writer structure
impl crate::Writable for I2C_CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I2C_CR1 to value 0
impl crate::Resettable for I2C_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
