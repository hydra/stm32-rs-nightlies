///Register `SPI_CFG1` reader
pub struct R(crate::R<SPI_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SPI_CFG1` writer
pub struct W(crate::W<SPI_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CFG1_SPEC>;
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
impl From<crate::W<SPI_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DSIZE` reader - DSIZE
pub type DSIZE_R = crate::FieldReader<u8, u8>;
///Field `DSIZE` writer - DSIZE
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG1_SPEC, u8, u8, 5, O>;
///Field `FTHLV` reader - FTHLV
pub type FTHLV_R = crate::FieldReader<u8, u8>;
///Field `FTHLV` writer - FTHLV
pub type FTHLV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG1_SPEC, u8, u8, 4, O>;
///Field `UDRCFG` reader - UDRCFG
pub type UDRCFG_R = crate::FieldReader<u8, u8>;
///Field `UDRCFG` writer - UDRCFG
pub type UDRCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG1_SPEC, u8, u8, 2, O>;
///Field `UDRDET` reader - UDRDET
pub type UDRDET_R = crate::FieldReader<u8, u8>;
///Field `UDRDET` writer - UDRDET
pub type UDRDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG1_SPEC, u8, u8, 2, O>;
///Field `RXDMAEN` reader - RXDMAEN
pub type RXDMAEN_R = crate::BitReader<bool>;
///Field `RXDMAEN` writer - RXDMAEN
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG1_SPEC, bool, O>;
///Field `TXDMAEN` reader - TXDMAEN
pub type TXDMAEN_R = crate::BitReader<bool>;
///Field `TXDMAEN` writer - TXDMAEN
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG1_SPEC, bool, O>;
///Field `CRCSIZE` reader - CRCSIZE
pub type CRCSIZE_R = crate::FieldReader<u8, u8>;
///Field `CRCSIZE` writer - CRCSIZE
pub type CRCSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG1_SPEC, u8, u8, 5, O>;
///Field `CRCEN` reader - CRCEN
pub type CRCEN_R = crate::BitReader<bool>;
///Field `CRCEN` writer - CRCEN
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG1_SPEC, bool, O>;
///Field `MBR` reader - MBR
pub type MBR_R = crate::FieldReader<u8, u8>;
///Field `MBR` writer - MBR
pub type MBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG1_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:4 - DSIZE
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:8 - FTHLV
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:10 - UDRCFG
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - UDRDET
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 14 - RXDMAEN
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - CRCSIZE
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 22 - CRCEN
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 28:30 - MBR
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:4 - DSIZE
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<0> {
        DSIZE_W::new(self)
    }
    ///Bits 5:8 - FTHLV
    #[inline(always)]
    #[must_use]
    pub fn fthlv(&mut self) -> FTHLV_W<5> {
        FTHLV_W::new(self)
    }
    ///Bits 9:10 - UDRCFG
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<9> {
        UDRCFG_W::new(self)
    }
    ///Bits 11:12 - UDRDET
    #[inline(always)]
    #[must_use]
    pub fn udrdet(&mut self) -> UDRDET_W<11> {
        UDRDET_W::new(self)
    }
    ///Bit 14 - RXDMAEN
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<14> {
        RXDMAEN_W::new(self)
    }
    ///Bit 15 - TXDMAEN
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<15> {
        TXDMAEN_W::new(self)
    }
    ///Bits 16:20 - CRCSIZE
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<16> {
        CRCSIZE_W::new(self)
    }
    ///Bit 22 - CRCEN
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<22> {
        CRCEN_W::new(self)
    }
    ///Bits 28:30 - MBR
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<28> {
        MBR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Content of this register is write protected when SPI is enabled
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_cfg1](index.html) module
pub struct SPI_CFG1_SPEC;
impl crate::RegisterSpec for SPI_CFG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_cfg1::R](R) reader structure
impl crate::Readable for SPI_CFG1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [spi_cfg1::W](W) writer structure
impl crate::Writable for SPI_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SPI_CFG1 to value 0x0007_0007
impl crate::Resettable for SPI_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
