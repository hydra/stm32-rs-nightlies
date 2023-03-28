///Register `SDMMC_FIFOR13` reader
pub struct R(crate::R<SDMMC_FIFOR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_FIFOR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_FIFOR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_FIFOR13_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDMMC_FIFOR13` writer
pub struct W(crate::W<SDMMC_FIFOR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_FIFOR13_SPEC>;
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
impl From<crate::W<SDMMC_FIFOR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_FIFOR13_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FIFODATA` reader - FIFODATA
pub type FIFODATA_R = crate::FieldReader<u32, u32>;
///Field `FIFODATA` writer - FIFODATA
pub type FIFODATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_FIFOR13_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - FIFODATA
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - FIFODATA
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<0> {
        FIFODATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdmmc_fifor13](index.html) module
pub struct SDMMC_FIFOR13_SPEC;
impl crate::RegisterSpec for SDMMC_FIFOR13_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_fifor13::R](R) reader structure
impl crate::Readable for SDMMC_FIFOR13_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdmmc_fifor13::W](W) writer structure
impl crate::Writable for SDMMC_FIFOR13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDMMC_FIFOR13 to value 0
impl crate::Resettable for SDMMC_FIFOR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
