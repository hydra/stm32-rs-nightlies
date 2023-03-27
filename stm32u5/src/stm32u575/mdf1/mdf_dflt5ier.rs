///Register `MDF_DFLT5IER` reader
pub struct R(crate::R<MDF_DFLT5IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_DFLT5IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_DFLT5IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_DFLT5IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_DFLT5IER` writer
pub struct W(crate::W<MDF_DFLT5IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_DFLT5IER_SPEC>;
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
impl From<crate::W<MDF_DFLT5IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_DFLT5IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FTHIE` reader - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
pub type FTHIE_R = crate::BitReader<bool>;
///Field `FTHIE` writer - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
pub type FTHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `DOVRIE` reader - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
pub type DOVRIE_R = crate::BitReader<bool>;
///Field `DOVRIE` writer - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
pub type DOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `SSDRIE` reader - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
pub type SSDRIE_R = crate::BitReader<bool>;
///Field `SSDRIE` writer - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
pub type SSDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `OLDIE` reader - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
pub type OLDIE_R = crate::BitReader<bool>;
///Field `OLDIE` writer - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
pub type OLDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `SSOVRIE` reader - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
pub type SSOVRIE_R = crate::BitReader<bool>;
///Field `SSOVRIE` writer - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
pub type SSOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `SCDIE` reader - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
pub type SCDIE_R = crate::BitReader<bool>;
///Field `SCDIE` writer - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
pub type SCDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `SATIE` reader - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
pub type SATIE_R = crate::BitReader<bool>;
///Field `SATIE` writer - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
pub type SATIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `CKABIE` reader - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
pub type CKABIE_R = crate::BitReader<bool>;
///Field `CKABIE` writer - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
pub type CKABIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
///Field `RFOVRIE` reader - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
pub type RFOVRIE_R = crate::BitReader<bool>;
///Field `RFOVRIE` writer - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
pub type RFOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_DFLT5IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
    #[inline(always)]
    pub fn fthie(&self) -> FTHIE_R {
        FTHIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
    #[inline(always)]
    pub fn dovrie(&self) -> DOVRIE_R {
        DOVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
    #[inline(always)]
    pub fn ssdrie(&self) -> SSDRIE_R {
        SSDRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
    #[inline(always)]
    pub fn oldie(&self) -> OLDIE_R {
        OLDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
    #[inline(always)]
    pub fn ssovrie(&self) -> SSOVRIE_R {
        SSOVRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
    #[inline(always)]
    pub fn satie(&self) -> SATIE_R {
        SATIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
    #[inline(always)]
    pub fn rfovrie(&self) -> RFOVRIE_R {
        RFOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn fthie(&mut self) -> FTHIE_W<0> {
        FTHIE_W::new(self)
    }
    ///Bit 1 - Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn dovrie(&mut self) -> DOVRIE_W<1> {
        DOVRIE_W::new(self)
    }
    ///Bit 2 - Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn ssdrie(&mut self) -> SSDRIE_W<2> {
        SSDRIE_W::new(self)
    }
    ///Bit 4 - Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn oldie(&mut self) -> OLDIE_W<4> {
        OLDIE_W::new(self)
    }
    ///Bit 7 - Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn ssovrie(&mut self) -> SSOVRIE_W<7> {
        SSOVRIE_W::new(self)
    }
    ///Bit 8 - Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn scdie(&mut self) -> SCDIE_W<8> {
        SCDIE_W::new(self)
    }
    ///Bit 9 - Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn satie(&mut self) -> SATIE_W<9> {
        SATIE_W::new(self)
    }
    ///Bit 10 - Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<10> {
        CKABIE_W::new(self)
    }
    ///Bit 11 - Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn rfovrie(&mut self) -> RFOVRIE_W<11> {
        RFOVRIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDF DFLTx interrupt enable register x
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_dflt5ier](index.html) module
pub struct MDF_DFLT5IER_SPEC;
impl crate::RegisterSpec for MDF_DFLT5IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_dflt5ier::R](R) reader structure
impl crate::Readable for MDF_DFLT5IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_dflt5ier::W](W) writer structure
impl crate::Writable for MDF_DFLT5IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_DFLT5IER to value 0
impl crate::Resettable for MDF_DFLT5IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
