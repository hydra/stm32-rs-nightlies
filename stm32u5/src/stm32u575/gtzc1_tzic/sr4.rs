///Register `SR4` reader
pub struct R(crate::R<SR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `GPDMA1F` reader - illegal access flag for GPDMA1
pub type GPDMA1F_R = crate::BitReader<bool>;
///Field `FLASHF` reader - illegal access flag for FLASH memory
pub type FLASHF_R = crate::BitReader<bool>;
///Field `FLASH_REGF` reader - illegal access flag for FLASH registers
pub type FLASH_REGF_R = crate::BitReader<bool>;
///Field `OTFDEC1F` reader - illegal access flag for OTFDEC1
pub type OTFDEC1F_R = crate::BitReader<bool>;
///Field `OTFDEC2F` reader - illegal access flag for OTFDEC2
pub type OTFDEC2F_R = crate::BitReader<bool>;
///Field `TZSC1F` reader - illegal access flag for GTZC1 TZSC registers
pub type TZSC1F_R = crate::BitReader<bool>;
///Field `TZIC1F` reader - illegal access flag for GTZC1 TZIC registers
pub type TZIC1F_R = crate::BitReader<bool>;
///Field `OCTOSPI1_MEMF` reader - illegal access flag for MPCWM1 (OCTOSPI1) memory bank
pub type OCTOSPI1_MEMF_R = crate::BitReader<bool>;
///Field `FSMC_MEMF` reader - illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3 (FSMC NOR)
pub type FSMC_MEMF_R = crate::BitReader<bool>;
///Field `BKPSRAMF` reader - illegal access flag for MPCWM3 (BKPSRAM) memory bank
pub type BKPSRAMF_R = crate::BitReader<bool>;
///Field `OCTOSPI2_MEMF` reader - illegal access flag for OCTOSPI2 memory bank
pub type OCTOSPI2_MEMF_R = crate::BitReader<bool>;
///Field `SRAM1F` reader - illegal access flag for SRAM1
pub type SRAM1F_R = crate::BitReader<bool>;
///Field `MPCBB1_REGF` reader - illegal access flag for MPCBB1 registers
pub type MPCBB1_REGF_R = crate::BitReader<bool>;
///Field `SRAM2F` reader - illegal access flag for SRAM2
pub type SRAM2F_R = crate::BitReader<bool>;
///Field `MPCBB2_REGF` reader - illegal access flag for MPCBB2 registers
pub type MPCBB2_REGF_R = crate::BitReader<bool>;
///Field `SRAM3F` reader - illegal access flag for SRAM3
pub type SRAM3F_R = crate::BitReader<bool>;
///Field `MPCBB3_REGF` reader - illegal access flag for MPCBB3 registers
pub type MPCBB3_REGF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - illegal access flag for GPDMA1
    #[inline(always)]
    pub fn gpdma1f(&self) -> GPDMA1F_R {
        GPDMA1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for FLASH memory
    #[inline(always)]
    pub fn flashf(&self) -> FLASHF_R {
        FLASHF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for FLASH registers
    #[inline(always)]
    pub fn flash_regf(&self) -> FLASH_REGF_R {
        FLASH_REGF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for OTFDEC1
    #[inline(always)]
    pub fn otfdec1f(&self) -> OTFDEC1F_R {
        OTFDEC1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for OTFDEC2
    #[inline(always)]
    pub fn otfdec2f(&self) -> OTFDEC2F_R {
        OTFDEC2F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for GTZC1 TZSC registers
    #[inline(always)]
    pub fn tzsc1f(&self) -> TZSC1F_R {
        TZSC1F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for GTZC1 TZIC registers
    #[inline(always)]
    pub fn tzic1f(&self) -> TZIC1F_R {
        TZIC1F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    pub fn octospi1_memf(&self) -> OCTOSPI1_MEMF_R {
        OCTOSPI1_MEMF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3 (FSMC NOR)
    #[inline(always)]
    pub fn fsmc_memf(&self) -> FSMC_MEMF_R {
        FSMC_MEMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for MPCWM3 (BKPSRAM) memory bank
    #[inline(always)]
    pub fn bkpsramf(&self) -> BKPSRAMF_R {
        BKPSRAMF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access flag for OCTOSPI2 memory bank
    #[inline(always)]
    pub fn octospi2_memf(&self) -> OCTOSPI2_MEMF_R {
        OCTOSPI2_MEMF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for SRAM1
    #[inline(always)]
    pub fn sram1f(&self) -> SRAM1F_R {
        SRAM1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access flag for MPCBB1 registers
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access flag for SRAM2
    #[inline(always)]
    pub fn sram2f(&self) -> SRAM2F_R {
        SRAM2F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access flag for MPCBB2 registers
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access flag for SRAM3
    #[inline(always)]
    pub fn sram3f(&self) -> SRAM3F_R {
        SRAM3F_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access flag for MPCBB3 registers
    #[inline(always)]
    pub fn mpcbb3_regf(&self) -> MPCBB3_REGF_R {
        MPCBB3_REGF_R::new(((self.bits >> 29) & 1) != 0)
    }
}
///TZIC status register 4
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr4](index.html) module
pub struct SR4_SPEC;
impl crate::RegisterSpec for SR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr4::R](R) reader structure
impl crate::Readable for SR4_SPEC {
    type Reader = R;
}
///`reset()` method sets SR4 to value 0
impl crate::Resettable for SR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
