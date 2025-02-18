///Register `RCC_MP_AHB6LPENSETR` reader
pub struct R(crate::R<RCC_MP_AHB6LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB6LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB6LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB6LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_AHB6LPENSETR` writer
pub struct W(crate::W<RCC_MP_AHB6LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB6LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB6LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB6LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDMALPEN` reader - MDMALPEN
pub type MDMALPEN_R = crate::BitReader<bool>;
///Field `MDMALPEN` writer - MDMALPEN
pub type MDMALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `GPULPEN` reader - GPULPEN
pub type GPULPEN_R = crate::BitReader<bool>;
///Field `GPULPEN` writer - GPULPEN
pub type GPULPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `ETHCKLPEN` reader - ETHCKLPEN
pub type ETHCKLPEN_R = crate::BitReader<bool>;
///Field `ETHCKLPEN` writer - ETHCKLPEN
pub type ETHCKLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `ETHTXLPEN` reader - ETHTXLPEN
pub type ETHTXLPEN_R = crate::BitReader<bool>;
///Field `ETHTXLPEN` writer - ETHTXLPEN
pub type ETHTXLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `ETHRXLPEN` reader - ETHRXLPEN
pub type ETHRXLPEN_R = crate::BitReader<bool>;
///Field `ETHRXLPEN` writer - ETHRXLPEN
pub type ETHRXLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `ETHMACLPEN` reader - ETHMACLPEN
pub type ETHMACLPEN_R = crate::BitReader<bool>;
///Field `ETHMACLPEN` writer - ETHMACLPEN
pub type ETHMACLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `ETHSTPEN` reader - ETHSTPEN
pub type ETHSTPEN_R = crate::BitReader<bool>;
///Field `ETHSTPEN` writer - ETHSTPEN
pub type ETHSTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `FMCLPEN` reader - FMCLPEN
pub type FMCLPEN_R = crate::BitReader<bool>;
///Field `FMCLPEN` writer - FMCLPEN
pub type FMCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `QSPILPEN` reader - QSPILPEN
pub type QSPILPEN_R = crate::BitReader<bool>;
///Field `QSPILPEN` writer - QSPILPEN
pub type QSPILPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `SDMMC1LPEN` reader - SDMMC1LPEN
pub type SDMMC1LPEN_R = crate::BitReader<bool>;
///Field `SDMMC1LPEN` writer - SDMMC1LPEN
pub type SDMMC1LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `SDMMC2LPEN` reader - SDMMC2LPEN
pub type SDMMC2LPEN_R = crate::BitReader<bool>;
///Field `SDMMC2LPEN` writer - SDMMC2LPEN
pub type SDMMC2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `CRC1LPEN` reader - CRC1LPEN
pub type CRC1LPEN_R = crate::BitReader<bool>;
///Field `CRC1LPEN` writer - CRC1LPEN
pub type CRC1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
///Field `USBHLPEN` reader - USBHLPEN
pub type USBHLPEN_R = crate::BitReader<bool>;
///Field `USBHLPEN` writer - USBHLPEN
pub type USBHLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB6LPENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - GPULPEN
    #[inline(always)]
    pub fn gpulpen(&self) -> GPULPEN_R {
        GPULPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - ETHCKLPEN
    #[inline(always)]
    pub fn ethcklpen(&self) -> ETHCKLPEN_R {
        ETHCKLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ETHTXLPEN
    #[inline(always)]
    pub fn ethtxlpen(&self) -> ETHTXLPEN_R {
        ETHTXLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ETHRXLPEN
    #[inline(always)]
    pub fn ethrxlpen(&self) -> ETHRXLPEN_R {
        ETHRXLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ETHMACLPEN
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ETHSTPEN
    #[inline(always)]
    pub fn ethstpen(&self) -> ETHSTPEN_R {
        ETHSTPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - FMCLPEN
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QSPILPEN
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1LPEN
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SDMMC2LPEN
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - CRC1LPEN
    #[inline(always)]
    pub fn crc1lpen(&self) -> CRC1LPEN_R {
        CRC1LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - USBHLPEN
    #[inline(always)]
    pub fn usbhlpen(&self) -> USBHLPEN_R {
        USBHLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<0> {
        MDMALPEN_W::new(self)
    }
    ///Bit 5 - GPULPEN
    #[inline(always)]
    #[must_use]
    pub fn gpulpen(&mut self) -> GPULPEN_W<5> {
        GPULPEN_W::new(self)
    }
    ///Bit 7 - ETHCKLPEN
    #[inline(always)]
    #[must_use]
    pub fn ethcklpen(&mut self) -> ETHCKLPEN_W<7> {
        ETHCKLPEN_W::new(self)
    }
    ///Bit 8 - ETHTXLPEN
    #[inline(always)]
    #[must_use]
    pub fn ethtxlpen(&mut self) -> ETHTXLPEN_W<8> {
        ETHTXLPEN_W::new(self)
    }
    ///Bit 9 - ETHRXLPEN
    #[inline(always)]
    #[must_use]
    pub fn ethrxlpen(&mut self) -> ETHRXLPEN_W<9> {
        ETHRXLPEN_W::new(self)
    }
    ///Bit 10 - ETHMACLPEN
    #[inline(always)]
    #[must_use]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W<10> {
        ETHMACLPEN_W::new(self)
    }
    ///Bit 11 - ETHSTPEN
    #[inline(always)]
    #[must_use]
    pub fn ethstpen(&mut self) -> ETHSTPEN_W<11> {
        ETHSTPEN_W::new(self)
    }
    ///Bit 12 - FMCLPEN
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<12> {
        FMCLPEN_W::new(self)
    }
    ///Bit 14 - QSPILPEN
    #[inline(always)]
    #[must_use]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<14> {
        QSPILPEN_W::new(self)
    }
    ///Bit 16 - SDMMC1LPEN
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<16> {
        SDMMC1LPEN_W::new(self)
    }
    ///Bit 17 - SDMMC2LPEN
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<17> {
        SDMMC2LPEN_W::new(self)
    }
    ///Bit 20 - CRC1LPEN
    #[inline(always)]
    #[must_use]
    pub fn crc1lpen(&mut self) -> CRC1LPEN_W<20> {
        CRC1LPEN_W::new(self)
    }
    ///Bit 24 - USBHLPEN
    #[inline(always)]
    #[must_use]
    pub fn usbhlpen(&mut self) -> USBHLPEN_W<24> {
        USBHLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used by the MCU in order to clear the PERxLPEN bits
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_ahb6lpensetr](index.html) module
pub struct RCC_MP_AHB6LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB6LPENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_ahb6lpensetr::R](R) reader structure
impl crate::Readable for RCC_MP_AHB6LPENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_ahb6lpensetr::W](W) writer structure
impl crate::Writable for RCC_MP_AHB6LPENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_AHB6LPENSETR to value 0x0113_57a1
impl crate::Resettable for RCC_MP_AHB6LPENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0113_57a1;
}
