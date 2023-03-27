///Register `FLASH_ACR` reader
pub struct R(crate::R<FLASH_ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_ACR` writer
pub struct W(crate::W<FLASH_ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ACR_SPEC>;
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
impl From<crate::W<FLASH_ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LATENCY` reader - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
pub type LATENCY_R = crate::FieldReader<u8, u8>;
///Field `LATENCY` writer - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_ACR_SPEC, u8, u8, 4, O>;
///Field `PRFTEN` reader - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
pub type PRFTEN_R = crate::BitReader<bool>;
///Field `PRFTEN` writer - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ACR_SPEC, bool, O>;
///Field `LPM` reader - Low-power read mode This bit puts the Flash memory in low-power read mode.
pub type LPM_R = crate::BitReader<bool>;
///Field `LPM` writer - Low-power read mode This bit puts the Flash memory in low-power read mode.
pub type LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ACR_SPEC, bool, O>;
///Field `PDREQ1` reader - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
pub type PDREQ1_R = crate::BitReader<bool>;
///Field `PDREQ1` writer - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
pub type PDREQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ACR_SPEC, bool, O>;
///Field `PDREQ2` reader - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
pub type PDREQ2_R = crate::BitReader<bool>;
///Field `PDREQ2` writer - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
pub type PDREQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ACR_SPEC, bool, O>;
///Field `SLEEP_PD` reader - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
pub type SLEEP_PD_R = crate::BitReader<bool>;
///Field `SLEEP_PD` writer - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
pub type SLEEP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ACR_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode.
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
    #[inline(always)]
    pub fn pdreq1(&self) -> PDREQ1_R {
        PDREQ1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
    #[inline(always)]
    pub fn pdreq2(&self) -> PDREQ2_R {
        PDREQ2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ...
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    ///Bit 8 - Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory.
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<8> {
        PRFTEN_W::new(self)
    }
    ///Bit 11 - Low-power read mode This bit puts the Flash memory in low-power read mode.
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<11> {
        LPM_W::new(self)
    }
    ///Bit 12 - Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked.
    #[inline(always)]
    #[must_use]
    pub fn pdreq1(&mut self) -> PDREQ1_W<12> {
        PDREQ1_W::new(self)
    }
    ///Bit 13 - Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked.
    #[inline(always)]
    #[must_use]
    pub fn pdreq2(&mut self) -> PDREQ2_W<13> {
        PDREQ2_W::new(self)
    }
    ///Bit 14 - Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going.
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<14> {
        SLEEP_PD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_acr](index.html) module
pub struct FLASH_ACR_SPEC;
impl crate::RegisterSpec for FLASH_ACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_acr::R](R) reader structure
impl crate::Readable for FLASH_ACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_acr::W](W) writer structure
impl crate::Writable for FLASH_ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_ACR to value 0
impl crate::Resettable for FLASH_ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
