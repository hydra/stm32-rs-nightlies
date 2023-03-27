///Register `FCR1` writer
pub struct W(crate::W<FCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR1_SPEC>;
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
impl From<crate::W<FCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSPI3F` writer - clear the illegal access flag for SPI3
pub type CSPI3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CLPUART1F` writer - clear the illegal access flag for LPUART1
pub type CLPUART1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CI2C3F` writer - clear the illegal access flag for I2C3
pub type CI2C3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CLPTIM1F` writer - clear the illegal access flag for LPTIM1
pub type CLPTIM1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CLPTIM3F` writer - clear the illegal access flag for LPTIM3
pub type CLPTIM3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CLPTIM4F` writer - clear the illegal access flag for LPTIM4
pub type CLPTIM4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `COPAMPF` writer - clear the illegal access flag for OPAMP
pub type COPAMPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CCOMPF` writer - clear the illegal access flag for COMP
pub type CCOMPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CADC4F` writer - clear the illegal access flag for ADC4
pub type CADC4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CVREFBUFF` writer - clear the illegal access flag for VREFBUF
pub type CVREFBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CDAC1F` writer - clear the illegal access flag for DAC1
pub type CDAC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
///Field `CADF1F` writer - clear the illegal access flag for ADF1
pub type CADF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
impl W {
    ///Bit 0 - clear the illegal access flag for SPI3
    #[inline(always)]
    #[must_use]
    pub fn cspi3f(&mut self) -> CSPI3F_W<0> {
        CSPI3F_W::new(self)
    }
    ///Bit 1 - clear the illegal access flag for LPUART1
    #[inline(always)]
    #[must_use]
    pub fn clpuart1f(&mut self) -> CLPUART1F_W<1> {
        CLPUART1F_W::new(self)
    }
    ///Bit 2 - clear the illegal access flag for I2C3
    #[inline(always)]
    #[must_use]
    pub fn ci2c3f(&mut self) -> CI2C3F_W<2> {
        CI2C3F_W::new(self)
    }
    ///Bit 3 - clear the illegal access flag for LPTIM1
    #[inline(always)]
    #[must_use]
    pub fn clptim1f(&mut self) -> CLPTIM1F_W<3> {
        CLPTIM1F_W::new(self)
    }
    ///Bit 4 - clear the illegal access flag for LPTIM3
    #[inline(always)]
    #[must_use]
    pub fn clptim3f(&mut self) -> CLPTIM3F_W<4> {
        CLPTIM3F_W::new(self)
    }
    ///Bit 5 - clear the illegal access flag for LPTIM4
    #[inline(always)]
    #[must_use]
    pub fn clptim4f(&mut self) -> CLPTIM4F_W<5> {
        CLPTIM4F_W::new(self)
    }
    ///Bit 6 - clear the illegal access flag for OPAMP
    #[inline(always)]
    #[must_use]
    pub fn copampf(&mut self) -> COPAMPF_W<6> {
        COPAMPF_W::new(self)
    }
    ///Bit 7 - clear the illegal access flag for COMP
    #[inline(always)]
    #[must_use]
    pub fn ccompf(&mut self) -> CCOMPF_W<7> {
        CCOMPF_W::new(self)
    }
    ///Bit 8 - clear the illegal access flag for ADC4
    #[inline(always)]
    #[must_use]
    pub fn cadc4f(&mut self) -> CADC4F_W<8> {
        CADC4F_W::new(self)
    }
    ///Bit 9 - clear the illegal access flag for VREFBUF
    #[inline(always)]
    #[must_use]
    pub fn cvrefbuff(&mut self) -> CVREFBUFF_W<9> {
        CVREFBUFF_W::new(self)
    }
    ///Bit 11 - clear the illegal access flag for DAC1
    #[inline(always)]
    #[must_use]
    pub fn cdac1f(&mut self) -> CDAC1F_W<11> {
        CDAC1F_W::new(self)
    }
    ///Bit 12 - clear the illegal access flag for ADF1
    #[inline(always)]
    #[must_use]
    pub fn cadf1f(&mut self) -> CADF1F_W<12> {
        CADF1F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC flag clear register 1
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr1](index.html) module
pub struct FCR1_SPEC;
impl crate::RegisterSpec for FCR1_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [fcr1::W](W) writer structure
impl crate::Writable for FCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
