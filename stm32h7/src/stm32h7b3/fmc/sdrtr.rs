///Register `SDRTR` reader
pub struct R(crate::R<SDRTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDRTR` writer
pub struct W(crate::W<SDRTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRTR_SPEC>;
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
impl From<crate::W<SDRTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRE` writer - Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register.
pub type CRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRTR_SPEC, bool, O>;
///Field `COUNT` reader - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
pub type COUNT_R = crate::FieldReader<u16, u16>;
///Field `COUNT` writer - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRTR_SPEC, u16, u16, 13, O>;
///Field `REIE` reader - RES Interrupt Enable
pub type REIE_R = crate::BitReader<bool>;
///Field `REIE` writer - RES Interrupt Enable
pub type REIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRTR_SPEC, bool, O>;
impl R {
    ///Bits 1:13 - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register.
    #[inline(always)]
    #[must_use]
    pub fn cre(&mut self) -> CRE_W<0> {
        CRE_W::new(self)
    }
    ///Bits 1:13 - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<1> {
        COUNT_W::new(self)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn reie(&mut self) -> REIE_W<14> {
        REIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register sets the refresh rate in number of SDCLK clock cycles between the refresh cycles by configuring the Refresh Timer Count value.Examplewhere 64 ms is the SDRAM refresh period.The refresh rate must be increased by 20 SDRAM clock cycles (as in the above example) to obtain a safe margin if an internal refresh request occurs when a read request has been accepted. It corresponds to a COUNT value of 0000111000000 (448). This 13-bit field is loaded into a timer which is decremented using the SDRAM clock. This timer generates a refresh pulse when zero is reached. The COUNT value must be set at least to 41 SDRAM clock cycles.As soon as the FMC_SDRTR register is programmed, the timer starts counting. If the value programmed in the register is 0, no refresh is carried out. This register must not be reprogrammed after the initialization procedure to avoid modifying the refresh rate.Each time a refresh pulse is generated, this 13-bit COUNT field is reloaded into the counter.If a memory access is in progress, the Auto-refresh request is delayed. However, if the memory access and Auto-refresh requests are generated simultaneously, the Auto-refresh takes precedence. If the memory access occurs during a refresh operation, the request is buffered to be processed when the refresh is complete.This register is common to SDRAM bank 1 and bank 2.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdrtr](index.html) module
pub struct SDRTR_SPEC;
impl crate::RegisterSpec for SDRTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdrtr::R](R) reader structure
impl crate::Readable for SDRTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdrtr::W](W) writer structure
impl crate::Writable for SDRTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDRTR to value 0
impl crate::Resettable for SDRTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
