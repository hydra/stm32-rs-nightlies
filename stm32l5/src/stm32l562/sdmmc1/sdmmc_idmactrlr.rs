///Register `SDMMC_IDMACTRLR` reader
pub struct R(crate::R<SDMMC_IDMACTRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMACTRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMACTRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMACTRLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDMMC_IDMACTRLR` writer
pub struct W(crate::W<SDMMC_IDMACTRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMACTRLR_SPEC>;
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
impl From<crate::W<SDMMC_IDMACTRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMACTRLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IDMAEN` reader - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMAEN_R = crate::BitReader<bool>;
///Field `IDMAEN` writer - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMACTRLR_SPEC, bool, O>;
///Field `IDMABMODE` reader - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABMODE_R = crate::BitReader<bool>;
///Field `IDMABMODE` writer - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMACTRLR_SPEC, bool, O>;
///Field `IDMABACT` reader - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware.
pub type IDMABACT_R = crate::BitReader<bool>;
///Field `IDMABACT` writer - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware.
pub type IDMABACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMACTRLR_SPEC, bool, O>;
impl R {
    ///Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabmode(&self) -> IDMABMODE_R {
        IDMABMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware.
    #[inline(always)]
    pub fn idmabact(&self) -> IDMABACT_R {
        IDMABACT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    #[must_use]
    pub fn idmaen(&mut self) -> IDMAEN_W<0> {
        IDMAEN_W::new(self)
    }
    ///Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    #[must_use]
    pub fn idmabmode(&mut self) -> IDMABMODE_W<1> {
        IDMABMODE_W::new(self)
    }
    ///Bit 2 - Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware.
    #[inline(always)]
    #[must_use]
    pub fn idmabact(&mut self) -> IDMABACT_W<2> {
        IDMABACT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdmmc_idmactrlr](index.html) module
pub struct SDMMC_IDMACTRLR_SPEC;
impl crate::RegisterSpec for SDMMC_IDMACTRLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_idmactrlr::R](R) reader structure
impl crate::Readable for SDMMC_IDMACTRLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdmmc_idmactrlr::W](W) writer structure
impl crate::Writable for SDMMC_IDMACTRLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDMMC_IDMACTRLR to value 0
impl crate::Resettable for SDMMC_IDMACTRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
