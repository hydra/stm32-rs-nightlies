///Register `IER4` reader
pub struct R(crate::R<IER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER4` writer
pub struct W(crate::W<IER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER4_SPEC>;
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
impl From<crate::W<IER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPDMA1IE` reader - illegal access interrupt enable for GPDMA1
pub type GPDMA1IE_R = crate::BitReader<bool>;
///Field `GPDMA1IE` writer - illegal access interrupt enable for GPDMA1
pub type GPDMA1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `FLASHIE` reader - illegal access interrupt enable for FLASH memory
pub type FLASHIE_R = crate::BitReader<bool>;
///Field `FLASHIE` writer - illegal access interrupt enable for FLASH memory
pub type FLASHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `FLASH_REGIE` reader - illegal access interrupt enable for FLASH registers
pub type FLASH_REGIE_R = crate::BitReader<bool>;
///Field `FLASH_REGIE` writer - illegal access interrupt enable for FLASH registers
pub type FLASH_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `OTFDEC1IE` reader - illegal access interrupt enable for OTFDEC1
pub type OTFDEC1IE_R = crate::BitReader<bool>;
///Field `OTFDEC1IE` writer - illegal access interrupt enable for OTFDEC1
pub type OTFDEC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `OTFDEC2IE` reader - illegal access interrupt enable for OTFDEC2
pub type OTFDEC2IE_R = crate::BitReader<bool>;
///Field `OTFDEC2IE` writer - illegal access interrupt enable for OTFDEC2
pub type OTFDEC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `TZSC1IE` reader - illegal access interrupt enable for GTZC1 TZSC registers
pub type TZSC1IE_R = crate::BitReader<bool>;
///Field `TZSC1IE` writer - illegal access interrupt enable for GTZC1 TZSC registers
pub type TZSC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `TZIC1IE` reader - illegal access interrupt enable for GTZC1 TZIC registers
pub type TZIC1IE_R = crate::BitReader<bool>;
///Field `TZIC1IE` writer - illegal access interrupt enable for GTZC1 TZIC registers
pub type TZIC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `OCTOSPI1_MEMIE` reader - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
pub type OCTOSPI1_MEMIE_R = crate::BitReader<bool>;
///Field `OCTOSPI1_MEMIE` writer - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
pub type OCTOSPI1_MEMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `FSMC_MEMIE` reader - illegal access interrupt enable for MPCWM2 (FSMC NAND) and MPCWM3
pub type FSMC_MEMIE_R = crate::BitReader<bool>;
///Field `FSMC_MEMIE` writer - illegal access interrupt enable for MPCWM2 (FSMC NAND) and MPCWM3
pub type FSMC_MEMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `BKPSRAMIE` reader - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
pub type BKPSRAMIE_R = crate::BitReader<bool>;
///Field `BKPSRAMIE` writer - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
pub type BKPSRAMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `OCTOSPI2_MEMIE` reader - illegal access interrupt enable for OCTOSPI2 memory bank
pub type OCTOSPI2_MEMIE_R = crate::BitReader<bool>;
///Field `OCTOSPI2_MEMIE` writer - illegal access interrupt enable for OCTOSPI2 memory bank
pub type OCTOSPI2_MEMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `SRAM1IE` reader - illegal access interrupt enable for SRAM1
pub type SRAM1IE_R = crate::BitReader<bool>;
///Field `SRAM1IE` writer - illegal access interrupt enable for SRAM1
pub type SRAM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `MPCBB1_REGIE` reader - illegal access interrupt enable for MPCBB1 registers
pub type MPCBB1_REGIE_R = crate::BitReader<bool>;
///Field `MPCBB1_REGIE` writer - illegal access interrupt enable for MPCBB1 registers
pub type MPCBB1_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `SRAM2IE` reader - illegal access interrupt enable for SRAM2
pub type SRAM2IE_R = crate::BitReader<bool>;
///Field `SRAM2IE` writer - illegal access interrupt enable for SRAM2
pub type SRAM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `MPCBB2_REGIE` reader - illegal access interrupt enable for MPCBB2 registers
pub type MPCBB2_REGIE_R = crate::BitReader<bool>;
///Field `MPCBB2_REGIE` writer - illegal access interrupt enable for MPCBB2 registers
pub type MPCBB2_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `SRAM3IE` reader - illegal access interrupt enable for SRAM3
pub type SRAM3IE_R = crate::BitReader<bool>;
///Field `SRAM3IE` writer - illegal access interrupt enable for SRAM3
pub type SRAM3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
///Field `MPCBB3_REGIE` reader - illegal access interrupt enable for MPCBB3 registers
pub type MPCBB3_REGIE_R = crate::BitReader<bool>;
///Field `MPCBB3_REGIE` writer - illegal access interrupt enable for MPCBB3 registers
pub type MPCBB3_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER4_SPEC, bool, O>;
impl R {
    ///Bit 0 - illegal access interrupt enable for GPDMA1
    #[inline(always)]
    pub fn gpdma1ie(&self) -> GPDMA1IE_R {
        GPDMA1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for FLASH memory
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for FLASH registers
    #[inline(always)]
    pub fn flash_regie(&self) -> FLASH_REGIE_R {
        FLASH_REGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for OTFDEC1
    #[inline(always)]
    pub fn otfdec1ie(&self) -> OTFDEC1IE_R {
        OTFDEC1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for OTFDEC2
    #[inline(always)]
    pub fn otfdec2ie(&self) -> OTFDEC2IE_R {
        OTFDEC2IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC1 TZSC registers
    #[inline(always)]
    pub fn tzsc1ie(&self) -> TZSC1IE_R {
        TZSC1IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC1 TZIC registers
    #[inline(always)]
    pub fn tzic1ie(&self) -> TZIC1IE_R {
        TZIC1IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    pub fn octospi1_memie(&self) -> OCTOSPI1_MEMIE_R {
        OCTOSPI1_MEMIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for MPCWM2 (FSMC NAND) and MPCWM3
    #[inline(always)]
    pub fn fsmc_memie(&self) -> FSMC_MEMIE_R {
        FSMC_MEMIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
    #[inline(always)]
    pub fn bkpsramie(&self) -> BKPSRAMIE_R {
        BKPSRAMIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for OCTOSPI2 memory bank
    #[inline(always)]
    pub fn octospi2_memie(&self) -> OCTOSPI2_MEMIE_R {
        OCTOSPI2_MEMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM1
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB1 registers
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for SRAM2
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for MPCBB2 registers
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for SRAM3
    #[inline(always)]
    pub fn sram3ie(&self) -> SRAM3IE_R {
        SRAM3IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for MPCBB3 registers
    #[inline(always)]
    pub fn mpcbb3_regie(&self) -> MPCBB3_REGIE_R {
        MPCBB3_REGIE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for GPDMA1
    #[inline(always)]
    #[must_use]
    pub fn gpdma1ie(&mut self) -> GPDMA1IE_W<0> {
        GPDMA1IE_W::new(self)
    }
    ///Bit 1 - illegal access interrupt enable for FLASH memory
    #[inline(always)]
    #[must_use]
    pub fn flashie(&mut self) -> FLASHIE_W<1> {
        FLASHIE_W::new(self)
    }
    ///Bit 2 - illegal access interrupt enable for FLASH registers
    #[inline(always)]
    #[must_use]
    pub fn flash_regie(&mut self) -> FLASH_REGIE_W<2> {
        FLASH_REGIE_W::new(self)
    }
    ///Bit 3 - illegal access interrupt enable for OTFDEC1
    #[inline(always)]
    #[must_use]
    pub fn otfdec1ie(&mut self) -> OTFDEC1IE_W<3> {
        OTFDEC1IE_W::new(self)
    }
    ///Bit 4 - illegal access interrupt enable for OTFDEC2
    #[inline(always)]
    #[must_use]
    pub fn otfdec2ie(&mut self) -> OTFDEC2IE_W<4> {
        OTFDEC2IE_W::new(self)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC1 TZSC registers
    #[inline(always)]
    #[must_use]
    pub fn tzsc1ie(&mut self) -> TZSC1IE_W<14> {
        TZSC1IE_W::new(self)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC1 TZIC registers
    #[inline(always)]
    #[must_use]
    pub fn tzic1ie(&mut self) -> TZIC1IE_W<15> {
        TZIC1IE_W::new(self)
    }
    ///Bit 16 - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    #[must_use]
    pub fn octospi1_memie(&mut self) -> OCTOSPI1_MEMIE_W<16> {
        OCTOSPI1_MEMIE_W::new(self)
    }
    ///Bit 17 - illegal access interrupt enable for MPCWM2 (FSMC NAND) and MPCWM3
    #[inline(always)]
    #[must_use]
    pub fn fsmc_memie(&mut self) -> FSMC_MEMIE_W<17> {
        FSMC_MEMIE_W::new(self)
    }
    ///Bit 18 - illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
    #[inline(always)]
    #[must_use]
    pub fn bkpsramie(&mut self) -> BKPSRAMIE_W<18> {
        BKPSRAMIE_W::new(self)
    }
    ///Bit 19 - illegal access interrupt enable for OCTOSPI2 memory bank
    #[inline(always)]
    #[must_use]
    pub fn octospi2_memie(&mut self) -> OCTOSPI2_MEMIE_W<19> {
        OCTOSPI2_MEMIE_W::new(self)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM1
    #[inline(always)]
    #[must_use]
    pub fn sram1ie(&mut self) -> SRAM1IE_W<24> {
        SRAM1IE_W::new(self)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB1 registers
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W<25> {
        MPCBB1_REGIE_W::new(self)
    }
    ///Bit 26 - illegal access interrupt enable for SRAM2
    #[inline(always)]
    #[must_use]
    pub fn sram2ie(&mut self) -> SRAM2IE_W<26> {
        SRAM2IE_W::new(self)
    }
    ///Bit 27 - illegal access interrupt enable for MPCBB2 registers
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W<27> {
        MPCBB2_REGIE_W::new(self)
    }
    ///Bit 28 - illegal access interrupt enable for SRAM3
    #[inline(always)]
    #[must_use]
    pub fn sram3ie(&mut self) -> SRAM3IE_W<28> {
        SRAM3IE_W::new(self)
    }
    ///Bit 29 - illegal access interrupt enable for MPCBB3 registers
    #[inline(always)]
    #[must_use]
    pub fn mpcbb3_regie(&mut self) -> MPCBB3_REGIE_W<29> {
        MPCBB3_REGIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZIC interrupt enable register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier4](index.html) module
pub struct IER4_SPEC;
impl crate::RegisterSpec for IER4_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier4::R](R) reader structure
impl crate::Readable for IER4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier4::W](W) writer structure
impl crate::Writable for IER4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER4 to value 0
impl crate::Resettable for IER4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
