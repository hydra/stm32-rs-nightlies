///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TEF` reader - Transfer error flag This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode. It is cleared by writing 1 to CTEF.
pub type TEF_R = crate::BitReader<bool>;
///Field `TCF` reader - Transfer complete flag This bit is set in Indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF.
pub type TCF_R = crate::BitReader<bool>;
///Field `FTF` reader - FIFO threshold flag In Indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In Automatic status-polling mode, this bit is set every time the status register is read, and the bit is cleared when the data register is read.
pub type FTF_R = crate::BitReader<bool>;
///Field `SMF` reader - Status match flag This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR). It is cleared by writing 1 to CSMF.
pub type SMF_R = crate::BitReader<bool>;
///Field `TOF` reader - Timeout flag This bit is set when timeout occurs. It is cleared by writing 1 to CTOF.
pub type TOF_R = crate::BitReader<bool>;
///Field `BUSY` reader - Busy This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty.
pub type BUSY_R = crate::BitReader<bool>;
///Field `FLEVEL` reader - FIFO level This field gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 32 when it is full. In Automatic status-polling mode, FLEVEL is zero.
pub type FLEVEL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - Transfer error flag This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode. It is cleared by writing 1 to CTEF.
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete flag This bit is set in Indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF.
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO threshold flag In Indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In Automatic status-polling mode, this bit is set every time the status register is read, and the bit is cleared when the data register is read.
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Status match flag This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR). It is cleared by writing 1 to CSMF.
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timeout flag This bit is set when timeout occurs. It is cleared by writing 1 to CTOF.
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Busy This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:13 - FIFO level This field gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 32 when it is full. In Automatic status-polling mode, FLEVEL is zero.
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
///OCTOSPI status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
