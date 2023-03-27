///Register `PWR_CR2` reader
pub struct R(crate::R<PWR_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_CR2` writer
pub struct W(crate::W<PWR_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR2_SPEC>;
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
impl From<crate::W<PWR_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SRAM1PDS1` reader - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS1_R = crate::BitReader<bool>;
///Field `SRAM1PDS1` writer - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM1PDS2` reader - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS2_R = crate::BitReader<bool>;
///Field `SRAM1PDS2` writer - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM1PDS3` reader - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS3_R = crate::BitReader<bool>;
///Field `SRAM1PDS3` writer - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM2PDS1` reader - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
pub type SRAM2PDS1_R = crate::BitReader<bool>;
///Field `SRAM2PDS1` writer - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
pub type SRAM2PDS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM2PDS2` reader - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
pub type SRAM2PDS2_R = crate::BitReader<bool>;
///Field `SRAM2PDS2` writer - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
pub type SRAM2PDS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM4PDS` reader - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM4PDS_R = crate::BitReader<bool>;
///Field `SRAM4PDS` writer - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM4PDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `ICRAMPDS` reader - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type ICRAMPDS_R = crate::BitReader<bool>;
///Field `ICRAMPDS` writer - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type ICRAMPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `DC1RAMPDS` reader - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DC1RAMPDS_R = crate::BitReader<bool>;
///Field `DC1RAMPDS` writer - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DC1RAMPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `DMA2DRAMPDS` reader - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DMA2DRAMPDS_R = crate::BitReader<bool>;
///Field `DMA2DRAMPDS` writer - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DMA2DRAMPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `PRAMPDS` reader - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type PRAMPDS_R = crate::BitReader<bool>;
///Field `PRAMPDS` writer - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type PRAMPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `PKARAMPDS` reader - PKA SRAM power-down
pub type PKARAMPDS_R = crate::BitReader<bool>;
///Field `PKARAMPDS` writer - PKA SRAM power-down
pub type PKARAMPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM4FWU` reader - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
pub type SRAM4FWU_R = crate::BitReader<bool>;
///Field `SRAM4FWU` writer - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
pub type SRAM4FWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `FLASHFWU` reader - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_R = crate::BitReader<bool>;
///Field `FLASHFWU` writer - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS1` reader - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS1_R = crate::BitReader<bool>;
///Field `SRAM3PDS1` writer - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS2` reader - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS2_R = crate::BitReader<bool>;
///Field `SRAM3PDS2` writer - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS3` reader - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS3_R = crate::BitReader<bool>;
///Field `SRAM3PDS3` writer - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS4` reader - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS4_R = crate::BitReader<bool>;
///Field `SRAM3PDS4` writer - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS5` reader - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS5_R = crate::BitReader<bool>;
///Field `SRAM3PDS5` writer - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS6` reader - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS6_R = crate::BitReader<bool>;
///Field `SRAM3PDS6` writer - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS7` reader - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS7_R = crate::BitReader<bool>;
///Field `SRAM3PDS7` writer - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRAM3PDS8` reader - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS8_R = crate::BitReader<bool>;
///Field `SRAM3PDS8` writer - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM3PDS8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
///Field `SRDRUN` reader - SmartRun domain in Run mode
pub type SRDRUN_R = crate::BitReader<bool>;
///Field `SRDRUN` writer - SmartRun domain in Run mode
pub type SRDRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds1(&self) -> SRAM1PDS1_R {
        SRAM1PDS1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds2(&self) -> SRAM1PDS2_R {
        SRAM1PDS2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds3(&self) -> SRAM1PDS3_R {
        SRAM1PDS3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds1(&self) -> SRAM2PDS1_R {
        SRAM2PDS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds2(&self) -> SRAM2PDS2_R {
        SRAM2PDS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram4pds(&self) -> SRAM4PDS_R {
        SRAM4PDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn icrampds(&self) -> ICRAMPDS_R {
        ICRAMPDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn dc1rampds(&self) -> DC1RAMPDS_R {
        DC1RAMPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn dma2drampds(&self) -> DMA2DRAMPDS_R {
        DMA2DRAMPDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn prampds(&self) -> PRAMPDS_R {
        PRAMPDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PKA SRAM power-down
    #[inline(always)]
    pub fn pkarampds(&self) -> PKARAMPDS_R {
        PKARAMPDS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
    #[inline(always)]
    pub fn sram4fwu(&self) -> SRAM4FWU_R {
        SRAM4FWU_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    pub fn flashfwu(&self) -> FLASHFWU_R {
        FLASHFWU_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds1(&self) -> SRAM3PDS1_R {
        SRAM3PDS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds2(&self) -> SRAM3PDS2_R {
        SRAM3PDS2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds3(&self) -> SRAM3PDS3_R {
        SRAM3PDS3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds4(&self) -> SRAM3PDS4_R {
        SRAM3PDS4_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds5(&self) -> SRAM3PDS5_R {
        SRAM3PDS5_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds6(&self) -> SRAM3PDS6_R {
        SRAM3PDS6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds7(&self) -> SRAM3PDS7_R {
        SRAM3PDS7_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram3pds8(&self) -> SRAM3PDS8_R {
        SRAM3PDS8_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - SmartRun domain in Run mode
    #[inline(always)]
    pub fn srdrun(&self) -> SRDRUN_R {
        SRDRUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram1pds1(&mut self) -> SRAM1PDS1_W<0> {
        SRAM1PDS1_W::new(self)
    }
    ///Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram1pds2(&mut self) -> SRAM1PDS2_W<1> {
        SRAM1PDS2_W::new(self)
    }
    ///Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram1pds3(&mut self) -> SRAM1PDS3_W<2> {
        SRAM1PDS3_W::new(self)
    }
    ///Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
    #[inline(always)]
    #[must_use]
    pub fn sram2pds1(&mut self) -> SRAM2PDS1_W<4> {
        SRAM2PDS1_W::new(self)
    }
    ///Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
    #[inline(always)]
    #[must_use]
    pub fn sram2pds2(&mut self) -> SRAM2PDS2_W<5> {
        SRAM2PDS2_W::new(self)
    }
    ///Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram4pds(&mut self) -> SRAM4PDS_W<6> {
        SRAM4PDS_W::new(self)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn icrampds(&mut self) -> ICRAMPDS_W<8> {
        ICRAMPDS_W::new(self)
    }
    ///Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn dc1rampds(&mut self) -> DC1RAMPDS_W<9> {
        DC1RAMPDS_W::new(self)
    }
    ///Bit 10 - DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn dma2drampds(&mut self) -> DMA2DRAMPDS_W<10> {
        DMA2DRAMPDS_W::new(self)
    }
    ///Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn prampds(&mut self) -> PRAMPDS_W<11> {
        PRAMPDS_W::new(self)
    }
    ///Bit 12 - PKA SRAM power-down
    #[inline(always)]
    #[must_use]
    pub fn pkarampds(&mut self) -> PKARAMPDS_W<12> {
        PKARAMPDS_W::new(self)
    }
    ///Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn sram4fwu(&mut self) -> SRAM4FWU_W<13> {
        SRAM4FWU_W::new(self)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    #[must_use]
    pub fn flashfwu(&mut self) -> FLASHFWU_W<14> {
        FLASHFWU_W::new(self)
    }
    ///Bit 16 - SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds1(&mut self) -> SRAM3PDS1_W<16> {
        SRAM3PDS1_W::new(self)
    }
    ///Bit 17 - SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds2(&mut self) -> SRAM3PDS2_W<17> {
        SRAM3PDS2_W::new(self)
    }
    ///Bit 18 - SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds3(&mut self) -> SRAM3PDS3_W<18> {
        SRAM3PDS3_W::new(self)
    }
    ///Bit 19 - SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds4(&mut self) -> SRAM3PDS4_W<19> {
        SRAM3PDS4_W::new(self)
    }
    ///Bit 20 - SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds5(&mut self) -> SRAM3PDS5_W<20> {
        SRAM3PDS5_W::new(self)
    }
    ///Bit 21 - SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds6(&mut self) -> SRAM3PDS6_W<21> {
        SRAM3PDS6_W::new(self)
    }
    ///Bit 22 - SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds7(&mut self) -> SRAM3PDS7_W<22> {
        SRAM3PDS7_W::new(self)
    }
    ///Bit 23 - SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram3pds8(&mut self) -> SRAM3PDS8_W<23> {
        SRAM3PDS8_W::new(self)
    }
    ///Bit 31 - SmartRun domain in Run mode
    #[inline(always)]
    #[must_use]
    pub fn srdrun(&mut self) -> SRDRUN_W<31> {
        SRDRUN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_cr2](index.html) module
pub struct PWR_CR2_SPEC;
impl crate::RegisterSpec for PWR_CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_cr2::R](R) reader structure
impl crate::Readable for PWR_CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_cr2::W](W) writer structure
impl crate::Writable for PWR_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_CR2 to value 0
impl crate::Resettable for PWR_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
