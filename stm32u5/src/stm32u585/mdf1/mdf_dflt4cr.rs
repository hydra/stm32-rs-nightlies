///Register `MDF_DFLT4CR` reader
pub struct R(crate::R<MDF_DFLT4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_DFLT4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_DFLT4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_DFLT4CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_DFLT4CR` writer
pub struct W(crate::W<MDF_DFLT4CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_DFLT4CR_SPEC>;
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
impl From<crate::W<MDF_DFLT4CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_DFLT4CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DFLTEN` writer - Digital Filter Enable Set and cleared by software. This bit is used to control the start of acquisition of the corresponding digital filter path. The behavior of this bit depends on ACQMOD and external events. or the acquisition starts when the proper trigger event occurs if ACQMOD = 01x . The serial or parallel interface delivering the samples shall be enabled as well. - 0: The acquisition is stopped immediately - 1: The acquisition is immediately started if ACQMOD = 00x or 1xx ,
pub type DFLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT4CR_SPEC, bool, O>;
///Field `DMAEN` reader - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DMAEN_R = crate::BitReader<bool>;
///Field `DMAEN` writer - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT4CR_SPEC, bool, O>;
///Field `FTH` reader - RXFIFO Threshold selection Set and cleared by software. This bit is used to select the RXFIFO threshold. This bit is not significant for RXFIFOs working in a interleaved transfer mode. Refer to Section 1.4.13.4: Using the interleaved transfer mode for details. - 0: RXFIFO threshold event generated when the RXFIFO is not empty - 1: RXFIFO threshold event generated when the RXFIFO is half-full This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type FTH_R = crate::BitReader<bool>;
///Field `FTH` writer - RXFIFO Threshold selection Set and cleared by software. This bit is used to select the RXFIFO threshold. This bit is not significant for RXFIFOs working in a interleaved transfer mode. Refer to Section 1.4.13.4: Using the interleaved transfer mode for details. - 0: RXFIFO threshold event generated when the RXFIFO is not empty - 1: RXFIFO threshold event generated when the RXFIFO is half-full This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type FTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT4CR_SPEC, bool, O>;
///Field `ACQMOD` reader - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACQMOD_R = crate::FieldReader<u8, u8>;
///Field `ACQMOD` writer - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type ACQMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT4CR_SPEC, u8, u8, 3, O>;
///Field `TRGSENS` reader - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSENS_R = crate::BitReader<bool>;
///Field `TRGSENS` writer - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT4CR_SPEC, bool, O>;
///Field `TRGSRC` reader - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\]
///is selected ... - 1111: mdf_trg\[13\]
///is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSRC_R = crate::FieldReader<u8, u8>;
///Field `TRGSRC` writer - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\]
///is selected ... - 1111: mdf_trg\[13\]
///is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type TRGSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT4CR_SPEC, u8, u8, 4, O>;
///Field `SNPSFMT` reader - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\]
///of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SNPSFMT_R = crate::BitReader<bool>;
///Field `SNPSFMT` writer - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\]
///of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SNPSFMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT4CR_SPEC, bool, O>;
///Field `NBDIS` reader - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type NBDIS_R = crate::FieldReader<u8, u8>;
///Field `NBDIS` writer - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type NBDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT4CR_SPEC, u8, u8, 8, O>;
///Field `DFLTRUN` reader - Digital filter Run Status Flag Set and cleared by hardware. This bit indicates if the digital filter is running or not. - 0: The digital filter is not running, and ready to accept a new trigger event - 1: The digital filter is running
pub type DFLTRUN_R = crate::BitReader<bool>;
///Field `DFLTACTIVE` reader - Digital filter Active Flag Set and cleared by hardware. This bit indicates if the digital filter is active: can be running or waiting for events. - 0: The digital filter is not active, and can be re-enabled again (via DFLTEN bit) if needed - 1: The digital filter is active
pub type DFLTACTIVE_R = crate::BitReader<bool>;
impl R {
    ///Bit 1 - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXFIFO Threshold selection Set and cleared by software. This bit is used to select the RXFIFO threshold. This bit is not significant for RXFIFOs working in a interleaved transfer mode. Refer to Section 1.4.13.4: Using the interleaved transfer mode for details. - 0: RXFIFO threshold event generated when the RXFIFO is not empty - 1: RXFIFO threshold event generated when the RXFIFO is half-full This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn acqmod(&self) -> ACQMOD_R {
        ACQMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\]
    ///is selected ... - 1111: mdf_trg\[13\]
    ///is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\]
    ///of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn snpsfmt(&self) -> SNPSFMT_R {
        SNPSFMT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:27 - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn nbdis(&self) -> NBDIS_R {
        NBDIS_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - Digital filter Run Status Flag Set and cleared by hardware. This bit indicates if the digital filter is running or not. - 0: The digital filter is not running, and ready to accept a new trigger event - 1: The digital filter is running
    #[inline(always)]
    pub fn dfltrun(&self) -> DFLTRUN_R {
        DFLTRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Digital filter Active Flag Set and cleared by hardware. This bit indicates if the digital filter is active: can be running or waiting for events. - 0: The digital filter is not active, and can be re-enabled again (via DFLTEN bit) if needed - 1: The digital filter is active
    #[inline(always)]
    pub fn dfltactive(&self) -> DFLTACTIVE_R {
        DFLTACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Digital Filter Enable Set and cleared by software. This bit is used to control the start of acquisition of the corresponding digital filter path. The behavior of this bit depends on ACQMOD and external events. or the acquisition starts when the proper trigger event occurs if ACQMOD = 01x . The serial or parallel interface delivering the samples shall be enabled as well. - 0: The acquisition is stopped immediately - 1: The acquisition is immediately started if ACQMOD = 00x or 1xx ,
    #[inline(always)]
    #[must_use]
    pub fn dflten(&mut self) -> DFLTEN_W<0> {
        DFLTEN_W::new(self)
    }
    ///Bit 1 - DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<1> {
        DMAEN_W::new(self)
    }
    ///Bit 2 - RXFIFO Threshold selection Set and cleared by software. This bit is used to select the RXFIFO threshold. This bit is not significant for RXFIFOs working in a interleaved transfer mode. Refer to Section 1.4.13.4: Using the interleaved transfer mode for details. - 0: RXFIFO threshold event generated when the RXFIFO is not empty - 1: RXFIFO threshold event generated when the RXFIFO is half-full This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<2> {
        FTH_W::new(self)
    }
    ///Bits 4:6 - Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn acqmod(&mut self) -> ACQMOD_W<4> {
        ACQMOD_W::new(self)
    }
    ///Bit 8 - Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn trgsens(&mut self) -> TRGSENS_W<8> {
        TRGSENS_W::new(self)
    }
    ///Bits 12:15 - Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\[0\]
    ///is selected ... - 1111: mdf_trg\[13\]
    ///is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<12> {
        TRGSRC_W::new(self)
    }
    ///Bit 16 - Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \[15:9\]
    ///of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn snpsfmt(&mut self) -> SNPSFMT_W<16> {
        SNPSFMT_W::new(self)
    }
    ///Bits 20:27 - Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn nbdis(&mut self) -> NBDIS_W<20> {
        NBDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the digital filter 4.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_dflt4cr](index.html) module
pub struct MDF_DFLT4CR_SPEC;
impl crate::RegisterSpec for MDF_DFLT4CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_dflt4cr::R](R) reader structure
impl crate::Readable for MDF_DFLT4CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_dflt4cr::W](W) writer structure
impl crate::Writable for MDF_DFLT4CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_DFLT4CR to value 0
impl crate::Resettable for MDF_DFLT4CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
