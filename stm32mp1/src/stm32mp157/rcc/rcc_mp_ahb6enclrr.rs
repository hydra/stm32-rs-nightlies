///Register `RCC_MP_AHB6ENCLRR` reader
pub struct R(crate::R<RCC_MP_AHB6ENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB6ENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB6ENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB6ENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_AHB6ENCLRR` writer
pub struct W(crate::W<RCC_MP_AHB6ENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB6ENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB6ENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB6ENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDMAEN` reader - MDMAEN
pub type MDMAEN_R = crate::BitReader<bool>;
///Field `MDMAEN` writer - MDMAEN
pub type MDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `GPUEN` reader - GPUEN
pub type GPUEN_R = crate::BitReader<bool>;
///Field `GPUEN` writer - GPUEN
pub type GPUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `ETHCKEN` reader - ETHCKEN
pub type ETHCKEN_R = crate::BitReader<bool>;
///Field `ETHCKEN` writer - ETHCKEN
pub type ETHCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `ETHTXEN` reader - ETHTXEN
pub type ETHTXEN_R = crate::BitReader<bool>;
///Field `ETHTXEN` writer - ETHTXEN
pub type ETHTXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `ETHRXEN` reader - ETHRXEN
pub type ETHRXEN_R = crate::BitReader<bool>;
///Field `ETHRXEN` writer - ETHRXEN
pub type ETHRXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `ETHMACEN` reader - ETHMACEN
pub type ETHMACEN_R = crate::BitReader<bool>;
///Field `ETHMACEN` writer - ETHMACEN
pub type ETHMACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `FMCEN` reader - FMCEN
pub type FMCEN_R = crate::BitReader<bool>;
///Field `FMCEN` writer - FMCEN
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `QSPIEN` reader - QSPIEN
pub type QSPIEN_R = crate::BitReader<bool>;
///Field `QSPIEN` writer - QSPIEN
pub type QSPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `SDMMC1EN` reader - SDMMC1EN
pub type SDMMC1EN_R = crate::BitReader<bool>;
///Field `SDMMC1EN` writer - SDMMC1EN
pub type SDMMC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `SDMMC2EN` reader - SDMMC2EN
pub type SDMMC2EN_R = crate::BitReader<bool>;
///Field `SDMMC2EN` writer - SDMMC2EN
pub type SDMMC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `CRC1EN` reader - CRC1EN
pub type CRC1EN_R = crate::BitReader<bool>;
///Field `CRC1EN` writer - CRC1EN
pub type CRC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
///Field `USBHEN` reader - USBHEN
pub type USBHEN_R = crate::BitReader<bool>;
///Field `USBHEN` writer - USBHEN
pub type USBHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6ENCLRR_SPEC, bool, O>;
impl R {
    ///Bit 0 - MDMAEN
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - GPUEN
    #[inline(always)]
    pub fn gpuen(&self) -> GPUEN_R {
        GPUEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - ETHCKEN
    #[inline(always)]
    pub fn ethcken(&self) -> ETHCKEN_R {
        ETHCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ETHTXEN
    #[inline(always)]
    pub fn ethtxen(&self) -> ETHTXEN_R {
        ETHTXEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ETHRXEN
    #[inline(always)]
    pub fn ethrxen(&self) -> ETHRXEN_R {
        ETHRXEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ETHMACEN
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - FMCEN
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QSPIEN
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1EN
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SDMMC2EN
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - CRC1EN
    #[inline(always)]
    pub fn crc1en(&self) -> CRC1EN_R {
        CRC1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - USBHEN
    #[inline(always)]
    pub fn usbhen(&self) -> USBHEN_R {
        USBHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MDMAEN
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<0> {
        MDMAEN_W::new(self)
    }
    ///Bit 5 - GPUEN
    #[inline(always)]
    #[must_use]
    pub fn gpuen(&mut self) -> GPUEN_W<5> {
        GPUEN_W::new(self)
    }
    ///Bit 7 - ETHCKEN
    #[inline(always)]
    #[must_use]
    pub fn ethcken(&mut self) -> ETHCKEN_W<7> {
        ETHCKEN_W::new(self)
    }
    ///Bit 8 - ETHTXEN
    #[inline(always)]
    #[must_use]
    pub fn ethtxen(&mut self) -> ETHTXEN_W<8> {
        ETHTXEN_W::new(self)
    }
    ///Bit 9 - ETHRXEN
    #[inline(always)]
    #[must_use]
    pub fn ethrxen(&mut self) -> ETHRXEN_W<9> {
        ETHRXEN_W::new(self)
    }
    ///Bit 10 - ETHMACEN
    #[inline(always)]
    #[must_use]
    pub fn ethmacen(&mut self) -> ETHMACEN_W<10> {
        ETHMACEN_W::new(self)
    }
    ///Bit 12 - FMCEN
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<12> {
        FMCEN_W::new(self)
    }
    ///Bit 14 - QSPIEN
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<14> {
        QSPIEN_W::new(self)
    }
    ///Bit 16 - SDMMC1EN
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<16> {
        SDMMC1EN_W::new(self)
    }
    ///Bit 17 - SDMMC2EN
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<17> {
        SDMMC2EN_W::new(self)
    }
    ///Bit 20 - CRC1EN
    #[inline(always)]
    #[must_use]
    pub fn crc1en(&mut self) -> CRC1EN_W<20> {
        CRC1EN_W::new(self)
    }
    ///Bit 24 - USBHEN
    #[inline(always)]
    #[must_use]
    pub fn usbhen(&mut self) -> USBHEN_W<24> {
        USBHEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_ahb6enclrr](index.html) module
pub struct RCC_MP_AHB6ENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB6ENCLRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_ahb6enclrr::R](R) reader structure
impl crate::Readable for RCC_MP_AHB6ENCLRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_ahb6enclrr::W](W) writer structure
impl crate::Writable for RCC_MP_AHB6ENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_AHB6ENCLRR to value 0
impl crate::Resettable for RCC_MP_AHB6ENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
