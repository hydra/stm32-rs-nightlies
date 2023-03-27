///Register `FCR3` writer
pub struct W(crate::W<FCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR3_SPEC>;
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
impl From<crate::W<FCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMDF1F` writer - clear the illegal access flag for MDF1
pub type CMDF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CCORDICF` writer - clear the illegal access flag for CORDIC
pub type CCORDICF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CFMACF` writer - clear the illegal access flag for FMAC
pub type CFMACF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CCRCF` writer - clear the illegal access flag for CRC
pub type CCRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CTSCF` writer - clear the illegal access flag for TSC
pub type CTSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CDMA2DF` writer - clear the illegal access flag for register of DMA2D
pub type CDMA2DF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CICACHEF` writer - clear the illegal access flag for ICACHE registers
pub type CICACHEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CDCACHEF` writer - clear the illegal access flag for DCACHE registers
pub type CDCACHEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CADC1F` writer - clear the illegal access flag for ADC1
pub type CADC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CDCMIF` writer - clear the illegal access flag for DCMI
pub type CDCMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `COTGFSF` writer - clear the illegal access flag for OTG_FS
pub type COTGFSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CAESF` writer - clear the illegal access flag for AES
pub type CAESF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CHASHF` writer - clear the illegal access flag for HASH
pub type CHASHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CRNGF` writer - clear the illegal access flag for RNG
pub type CRNGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CPKAF` writer - clear the illegal access flag for PKA
pub type CPKAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CSAESF` writer - clear the illegal access flag for SAES
pub type CSAESF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `COCTOSPIMF` writer - clear the illegal access flag for OCTOSPIM
pub type COCTOSPIMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CSDMMC1F` writer - clear the illegal access flag for SDMMC2
pub type CSDMMC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CSDMMC2F` writer - clear the illegal access flag for SDMMC1
pub type CSDMMC2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CFSMCF` writer - clear the illegal access flag for FSMC registers
pub type CFSMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `COCTOSPI1F` writer - clear the illegal access flag for OCTOSPI1 registers
pub type COCTOSPI1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `COCTOSPI2F` writer - clear the illegal access flag for OCTOSPI2 registers
pub type COCTOSPI2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
///Field `CRAMCFGF` writer - clear the illegal access flag for RAMCFG
pub type CRAMCFGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR3_SPEC, bool, O>;
impl W {
    ///Bit 0 - clear the illegal access flag for MDF1
    #[inline(always)]
    #[must_use]
    pub fn cmdf1f(&mut self) -> CMDF1F_W<0> {
        CMDF1F_W::new(self)
    }
    ///Bit 1 - clear the illegal access flag for CORDIC
    #[inline(always)]
    #[must_use]
    pub fn ccordicf(&mut self) -> CCORDICF_W<1> {
        CCORDICF_W::new(self)
    }
    ///Bit 2 - clear the illegal access flag for FMAC
    #[inline(always)]
    #[must_use]
    pub fn cfmacf(&mut self) -> CFMACF_W<2> {
        CFMACF_W::new(self)
    }
    ///Bit 3 - clear the illegal access flag for CRC
    #[inline(always)]
    #[must_use]
    pub fn ccrcf(&mut self) -> CCRCF_W<3> {
        CCRCF_W::new(self)
    }
    ///Bit 4 - clear the illegal access flag for TSC
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<4> {
        CTSCF_W::new(self)
    }
    ///Bit 5 - clear the illegal access flag for register of DMA2D
    #[inline(always)]
    #[must_use]
    pub fn cdma2df(&mut self) -> CDMA2DF_W<5> {
        CDMA2DF_W::new(self)
    }
    ///Bit 6 - clear the illegal access flag for ICACHE registers
    #[inline(always)]
    #[must_use]
    pub fn cicachef(&mut self) -> CICACHEF_W<6> {
        CICACHEF_W::new(self)
    }
    ///Bit 7 - clear the illegal access flag for DCACHE registers
    #[inline(always)]
    #[must_use]
    pub fn cdcachef(&mut self) -> CDCACHEF_W<7> {
        CDCACHEF_W::new(self)
    }
    ///Bit 8 - clear the illegal access flag for ADC1
    #[inline(always)]
    #[must_use]
    pub fn cadc1f(&mut self) -> CADC1F_W<8> {
        CADC1F_W::new(self)
    }
    ///Bit 9 - clear the illegal access flag for DCMI
    #[inline(always)]
    #[must_use]
    pub fn cdcmif(&mut self) -> CDCMIF_W<9> {
        CDCMIF_W::new(self)
    }
    ///Bit 10 - clear the illegal access flag for OTG_FS
    #[inline(always)]
    #[must_use]
    pub fn cotgfsf(&mut self) -> COTGFSF_W<10> {
        COTGFSF_W::new(self)
    }
    ///Bit 11 - clear the illegal access flag for AES
    #[inline(always)]
    #[must_use]
    pub fn caesf(&mut self) -> CAESF_W<11> {
        CAESF_W::new(self)
    }
    ///Bit 12 - clear the illegal access flag for HASH
    #[inline(always)]
    #[must_use]
    pub fn chashf(&mut self) -> CHASHF_W<12> {
        CHASHF_W::new(self)
    }
    ///Bit 13 - clear the illegal access flag for RNG
    #[inline(always)]
    #[must_use]
    pub fn crngf(&mut self) -> CRNGF_W<13> {
        CRNGF_W::new(self)
    }
    ///Bit 14 - clear the illegal access flag for PKA
    #[inline(always)]
    #[must_use]
    pub fn cpkaf(&mut self) -> CPKAF_W<14> {
        CPKAF_W::new(self)
    }
    ///Bit 15 - clear the illegal access flag for SAES
    #[inline(always)]
    #[must_use]
    pub fn csaesf(&mut self) -> CSAESF_W<15> {
        CSAESF_W::new(self)
    }
    ///Bit 16 - clear the illegal access flag for OCTOSPIM
    #[inline(always)]
    #[must_use]
    pub fn coctospimf(&mut self) -> COCTOSPIMF_W<16> {
        COCTOSPIMF_W::new(self)
    }
    ///Bit 17 - clear the illegal access flag for SDMMC2
    #[inline(always)]
    #[must_use]
    pub fn csdmmc1f(&mut self) -> CSDMMC1F_W<17> {
        CSDMMC1F_W::new(self)
    }
    ///Bit 18 - clear the illegal access flag for SDMMC1
    #[inline(always)]
    #[must_use]
    pub fn csdmmc2f(&mut self) -> CSDMMC2F_W<18> {
        CSDMMC2F_W::new(self)
    }
    ///Bit 19 - clear the illegal access flag for FSMC registers
    #[inline(always)]
    #[must_use]
    pub fn cfsmcf(&mut self) -> CFSMCF_W<19> {
        CFSMCF_W::new(self)
    }
    ///Bit 20 - clear the illegal access flag for OCTOSPI1 registers
    #[inline(always)]
    #[must_use]
    pub fn coctospi1f(&mut self) -> COCTOSPI1F_W<20> {
        COCTOSPI1F_W::new(self)
    }
    ///Bit 21 - clear the illegal access flag for OCTOSPI2 registers
    #[inline(always)]
    #[must_use]
    pub fn coctospi2f(&mut self) -> COCTOSPI2F_W<21> {
        COCTOSPI2F_W::new(self)
    }
    ///Bit 22 - clear the illegal access flag for RAMCFG
    #[inline(always)]
    #[must_use]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<22> {
        CRAMCFGF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC flag clear register 3
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr3](index.html) module
pub struct FCR3_SPEC;
impl crate::RegisterSpec for FCR3_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [fcr3::W](W) writer structure
impl crate::Writable for FCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR3 to value 0
impl crate::Resettable for FCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
