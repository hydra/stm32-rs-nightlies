///Register `SPI_IFCR` writer
pub struct W(crate::W<SPI_IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_IFCR_SPEC>;
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
impl From<crate::W<SPI_IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_IFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EOTC` writer - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register
pub type EOTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
///Field `TXTFC` writer - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register
pub type TXTFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
///Field `UDRC` writer - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register
pub type UDRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
///Field `OVRC` writer - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register
pub type OVRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
///Field `CRCEC` writer - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register
pub type CRCEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
///Field `TIFREC` writer - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register
pub type TIFREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
///Field `MODFC` writer - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register
pub type MODFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
///Field `SUSPC` writer - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register
pub type SUSPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IFCR_SPEC, bool, O>;
impl W {
    ///Bit 3 - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn eotc(&mut self) -> EOTC_W<3> {
        EOTC_W::new(self)
    }
    ///Bit 4 - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn txtfc(&mut self) -> TXTFC_W<4> {
        TXTFC_W::new(self)
    }
    ///Bit 5 - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn udrc(&mut self) -> UDRC_W<5> {
        UDRC_W::new(self)
    }
    ///Bit 6 - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn ovrc(&mut self) -> OVRC_W<6> {
        OVRC_W::new(self)
    }
    ///Bit 7 - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn crcec(&mut self) -> CRCEC_W<7> {
        CRCEC_W::new(self)
    }
    ///Bit 8 - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn tifrec(&mut self) -> TIFREC_W<8> {
        TIFREC_W::new(self)
    }
    ///Bit 9 - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<9> {
        MODFC_W::new(self)
    }
    ///Bit 11 - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<11> {
        SUSPC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_ifcr](index.html) module
pub struct SPI_IFCR_SPEC;
impl crate::RegisterSpec for SPI_IFCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [spi_ifcr::W](W) writer structure
impl crate::Writable for SPI_IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SPI_IFCR to value 0
impl crate::Resettable for SPI_IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
