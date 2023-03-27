///Register `MDF_DFLT0ISR` reader
pub struct R(crate::R<MDF_DFLT0ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_DFLT0ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_DFLT0ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_DFLT0ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_DFLT0ISR` writer
pub struct W(crate::W<MDF_DFLT0ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_DFLT0ISR_SPEC>;
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
impl From<crate::W<MDF_DFLT0ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_DFLT0ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FTHF` reader - FTHF
pub type FTHF_R = crate::BitReader<bool>;
///Field `DOVRF` reader - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
pub type DOVRF_R = crate::BitReader<bool>;
///Field `DOVRF` writer - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
pub type DOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
///Field `SSDRF` reader - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
pub type SSDRF_R = crate::BitReader<bool>;
///Field `SSDRF` writer - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
pub type SSDRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
///Field `RXNEF` reader - RXFIFO Not Empty flag Set and cleared by hardware according to the RXFIFO level. - 0: Reading 0 means that the RXFIFO is empty. - 1: Reading 1 means that the RXFIFO is not empty.
pub type RXNEF_R = crate::BitReader<bool>;
///Field `OLDF` reader - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
pub type OLDF_R = crate::BitReader<bool>;
///Field `OLDF` writer - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
pub type OLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
///Field `THLF` reader - Low threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the low threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was higher than OLDTHL when the last OLD event occurred. - 1: The signal was lower than OLDTHL when the last OLD event occurred.
pub type THLF_R = crate::BitReader<bool>;
///Field `THHF` reader - High threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the high threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was lower than OLDTHH when the last OLD event occurred. - 1: The signal was higher than OLDTHH when the last OLD event occurred.
pub type THHF_R = crate::BitReader<bool>;
///Field `SSOVRF` reader - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
pub type SSOVRF_R = crate::BitReader<bool>;
///Field `SSOVRF` writer - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
pub type SSOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
///Field `SCDF` reader - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
pub type SCDF_R = crate::BitReader<bool>;
///Field `SCDF` writer - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
pub type SCDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
///Field `SATF` reader - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
pub type SATF_R = crate::BitReader<bool>;
///Field `SATF` writer - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
pub type SATF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
///Field `CKABF` reader - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
pub type CKABF_R = crate::BitReader<bool>;
///Field `CKABF` writer - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
pub type CKABF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
///Field `RFOVRF` reader - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
pub type RFOVRF_R = crate::BitReader<bool>;
///Field `RFOVRF` writer - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
pub type RFOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT0ISR_SPEC, bool, O>;
impl R {
    ///Bit 0 - FTHF
    #[inline(always)]
    pub fn fthf(&self) -> FTHF_R {
        FTHF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn dovrf(&self) -> DOVRF_R {
        DOVRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
    #[inline(always)]
    pub fn ssdrf(&self) -> SSDRF_R {
        SSDRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RXFIFO Not Empty flag Set and cleared by hardware according to the RXFIFO level. - 0: Reading 0 means that the RXFIFO is empty. - 1: Reading 1 means that the RXFIFO is not empty.
    #[inline(always)]
    pub fn rxnef(&self) -> RXNEF_R {
        RXNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
    #[inline(always)]
    pub fn oldf(&self) -> OLDF_R {
        OLDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Low threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the low threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was higher than OLDTHL when the last OLD event occurred. - 1: The signal was lower than OLDTHL when the last OLD event occurred.
    #[inline(always)]
    pub fn thlf(&self) -> THLF_R {
        THLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - High threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the high threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was lower than OLDTHH when the last OLD event occurred. - 1: The signal was higher than OLDTHH when the last OLD event occurred.
    #[inline(always)]
    pub fn thhf(&self) -> THHF_R {
        THHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn ssovrf(&self) -> SSOVRF_R {
        SSOVRF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn satf(&self) -> SATF_R {
        SATF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
    #[inline(always)]
    pub fn rfovrf(&self) -> RFOVRF_R {
        RFOVRF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag.
    #[inline(always)]
    #[must_use]
    pub fn dovrf(&mut self) -> DOVRF_W<1> {
        DOVRF_W::new(self)
    }
    ///Bit 2 - Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag.
    #[inline(always)]
    #[must_use]
    pub fn ssdrf(&mut self) -> SSDRF_W<2> {
        SSDRF_W::new(self)
    }
    ///Bit 4 - Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags.
    #[inline(always)]
    #[must_use]
    pub fn oldf(&mut self) -> OLDF_W<4> {
        OLDF_W::new(self)
    }
    ///Bit 7 - Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag.
    #[inline(always)]
    #[must_use]
    pub fn ssovrf(&mut self) -> SSOVRF_W<7> {
        SSOVRF_W::new(self)
    }
    ///Bit 8 - Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag.
    #[inline(always)]
    #[must_use]
    pub fn scdf(&mut self) -> SCDF_W<8> {
        SCDF_W::new(self)
    }
    ///Bit 9 - Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag.
    #[inline(always)]
    #[must_use]
    pub fn satf(&mut self) -> SATF_W<9> {
        SATF_W::new(self)
    }
    ///Bit 10 - Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag.
    #[inline(always)]
    #[must_use]
    pub fn ckabf(&mut self) -> CKABF_W<10> {
        CKABF_W::new(self)
    }
    ///Bit 11 - Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag.
    #[inline(always)]
    #[must_use]
    pub fn rfovrf(&mut self) -> RFOVRF_W<11> {
        RFOVRF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDF DFLT0 interrupt status register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_dflt0isr](index.html) module
pub struct MDF_DFLT0ISR_SPEC;
impl crate::RegisterSpec for MDF_DFLT0ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_dflt0isr::R](R) reader structure
impl crate::Readable for MDF_DFLT0ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_dflt0isr::W](W) writer structure
impl crate::Writable for MDF_DFLT0ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_DFLT0ISR to value 0
impl crate::Resettable for MDF_DFLT0ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
