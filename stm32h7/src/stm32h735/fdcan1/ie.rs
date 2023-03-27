///Register `IE` reader
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IE` writer
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RF0NE` reader - Rx FIFO 0 New Message Enable
pub type RF0NE_R = crate::BitReader<bool>;
///Field `RF0NE` writer - Rx FIFO 0 New Message Enable
pub type RF0NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `RF0WE` reader - Rx FIFO 0 Full Enable
pub type RF0WE_R = crate::BitReader<bool>;
///Field `RF0WE` writer - Rx FIFO 0 Full Enable
pub type RF0WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `RF0FE` reader - Rx FIFO 0 Full Enable
pub type RF0FE_R = crate::BitReader<bool>;
///Field `RF0FE` writer - Rx FIFO 0 Full Enable
pub type RF0FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `RF0LE` reader - Rx FIFO 0 Message Lost Enable
pub type RF0LE_R = crate::BitReader<bool>;
///Field `RF0LE` writer - Rx FIFO 0 Message Lost Enable
pub type RF0LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `RF1NE` reader - Rx FIFO 1 New Message Enable
pub type RF1NE_R = crate::BitReader<bool>;
///Field `RF1NE` writer - Rx FIFO 1 New Message Enable
pub type RF1NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `RF1WE` reader - Rx FIFO 1 Watermark Reached Enable
pub type RF1WE_R = crate::BitReader<bool>;
///Field `RF1WE` writer - Rx FIFO 1 Watermark Reached Enable
pub type RF1WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `RF1FE` reader - Rx FIFO 1 Watermark Reached Enable
pub type RF1FE_R = crate::BitReader<bool>;
///Field `RF1FE` writer - Rx FIFO 1 Watermark Reached Enable
pub type RF1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `RF1LE` reader - Rx FIFO 1 Message Lost Enable
pub type RF1LE_R = crate::BitReader<bool>;
///Field `RF1LE` writer - Rx FIFO 1 Message Lost Enable
pub type RF1LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `HPME` reader - High Priority Message Enable
pub type HPME_R = crate::BitReader<bool>;
///Field `HPME` writer - High Priority Message Enable
pub type HPME_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TCE` reader - Transmission Completed Enable
pub type TCE_R = crate::BitReader<bool>;
///Field `TCE` writer - Transmission Completed Enable
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TCFE` reader - Transmission Cancellation Finished Enable
pub type TCFE_R = crate::BitReader<bool>;
///Field `TCFE` writer - Transmission Cancellation Finished Enable
pub type TCFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TEFE` reader - Tx FIFO Empty Enable
pub type TEFE_R = crate::BitReader<bool>;
///Field `TEFE` writer - Tx FIFO Empty Enable
pub type TEFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TEFNE` reader - Tx Event FIFO New Entry Enable
pub type TEFNE_R = crate::BitReader<bool>;
///Field `TEFNE` writer - Tx Event FIFO New Entry Enable
pub type TEFNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TEFWE` reader - Tx Event FIFO Watermark Reached Enable
pub type TEFWE_R = crate::BitReader<bool>;
///Field `TEFWE` writer - Tx Event FIFO Watermark Reached Enable
pub type TEFWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TEFFE` reader - Tx Event FIFO Full Enable
pub type TEFFE_R = crate::BitReader<bool>;
///Field `TEFFE` writer - Tx Event FIFO Full Enable
pub type TEFFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TEFLE` reader - Tx Event FIFO Element Lost Enable
pub type TEFLE_R = crate::BitReader<bool>;
///Field `TEFLE` writer - Tx Event FIFO Element Lost Enable
pub type TEFLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TSWE` reader - Timestamp Wraparound Enable
pub type TSWE_R = crate::BitReader<bool>;
///Field `TSWE` writer - Timestamp Wraparound Enable
pub type TSWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `MRAFE` reader - Message RAM Access Failure Enable
pub type MRAFE_R = crate::BitReader<bool>;
///Field `MRAFE` writer - Message RAM Access Failure Enable
pub type MRAFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `TOOE` reader - Timeout Occurred Enable
pub type TOOE_R = crate::BitReader<bool>;
///Field `TOOE` writer - Timeout Occurred Enable
pub type TOOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `DRXE` reader - Message stored to Dedicated Rx Buffer Enable
pub type DRXE_R = crate::BitReader<bool>;
///Field `DRXE` writer - Message stored to Dedicated Rx Buffer Enable
pub type DRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `BECE` reader - Bit Error Corrected Interrupt Enable
pub type BECE_R = crate::BitReader<bool>;
///Field `BECE` writer - Bit Error Corrected Interrupt Enable
pub type BECE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `BEUE` reader - Bit Error Uncorrected Interrupt Enable
pub type BEUE_R = crate::BitReader<bool>;
///Field `BEUE` writer - Bit Error Uncorrected Interrupt Enable
pub type BEUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `ELOE` reader - Error Logging Overflow Enable
pub type ELOE_R = crate::BitReader<bool>;
///Field `ELOE` writer - Error Logging Overflow Enable
pub type ELOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `EPE` reader - Error Passive Enable
pub type EPE_R = crate::BitReader<bool>;
///Field `EPE` writer - Error Passive Enable
pub type EPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `EWE` reader - Warning Status Enable
pub type EWE_R = crate::BitReader<bool>;
///Field `EWE` writer - Warning Status Enable
pub type EWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `BOE` reader - Bus_Off Status Enable
pub type BOE_R = crate::BitReader<bool>;
///Field `BOE` writer - Bus_Off Status Enable
pub type BOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `WDIE` reader - Watchdog Interrupt Enable
pub type WDIE_R = crate::BitReader<bool>;
///Field `WDIE` writer - Watchdog Interrupt Enable
pub type WDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `PEAE` reader - Protocol Error in Arbitration Phase Enable
pub type PEAE_R = crate::BitReader<bool>;
///Field `PEAE` writer - Protocol Error in Arbitration Phase Enable
pub type PEAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `PEDE` reader - Protocol Error in Data Phase Enable
pub type PEDE_R = crate::BitReader<bool>;
///Field `PEDE` writer - Protocol Error in Data Phase Enable
pub type PEDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
///Field `ARAE` reader - Access to Reserved Address Enable
pub type ARAE_R = crate::BitReader<bool>;
///Field `ARAE` writer - Access to Reserved Address Enable
pub type ARAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
impl R {
    ///Bit 0 - Rx FIFO 0 New Message Enable
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Full Enable
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Full Enable
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost Enable
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 New Message Enable
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached Enable
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached Enable
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost Enable
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High Priority Message Enable
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission Completed Enable
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission Cancellation Finished Enable
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx FIFO Empty Enable
    #[inline(always)]
    pub fn tefe(&self) -> TEFE_R {
        TEFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx Event FIFO New Entry Enable
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached Enable
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx Event FIFO Full Enable
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx Event FIFO Element Lost Enable
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp Wraparound Enable
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Message RAM Access Failure Enable
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timeout Occurred Enable
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer Enable
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Bit Error Corrected Interrupt Enable
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Bit Error Uncorrected Interrupt Enable
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Error Logging Overflow Enable
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Error Passive Enable
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Warning Status Enable
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bus_Off Status Enable
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Watchdog Interrupt Enable
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase Enable
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Protocol Error in Data Phase Enable
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Access to Reserved Address Enable
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 New Message Enable
    #[inline(always)]
    #[must_use]
    pub fn rf0ne(&mut self) -> RF0NE_W<0> {
        RF0NE_W::new(self)
    }
    ///Bit 1 - Rx FIFO 0 Full Enable
    #[inline(always)]
    #[must_use]
    pub fn rf0we(&mut self) -> RF0WE_W<1> {
        RF0WE_W::new(self)
    }
    ///Bit 2 - Rx FIFO 0 Full Enable
    #[inline(always)]
    #[must_use]
    pub fn rf0fe(&mut self) -> RF0FE_W<2> {
        RF0FE_W::new(self)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost Enable
    #[inline(always)]
    #[must_use]
    pub fn rf0le(&mut self) -> RF0LE_W<3> {
        RF0LE_W::new(self)
    }
    ///Bit 4 - Rx FIFO 1 New Message Enable
    #[inline(always)]
    #[must_use]
    pub fn rf1ne(&mut self) -> RF1NE_W<4> {
        RF1NE_W::new(self)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached Enable
    #[inline(always)]
    #[must_use]
    pub fn rf1we(&mut self) -> RF1WE_W<5> {
        RF1WE_W::new(self)
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached Enable
    #[inline(always)]
    #[must_use]
    pub fn rf1fe(&mut self) -> RF1FE_W<6> {
        RF1FE_W::new(self)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost Enable
    #[inline(always)]
    #[must_use]
    pub fn rf1le(&mut self) -> RF1LE_W<7> {
        RF1LE_W::new(self)
    }
    ///Bit 8 - High Priority Message Enable
    #[inline(always)]
    #[must_use]
    pub fn hpme(&mut self) -> HPME_W<8> {
        HPME_W::new(self)
    }
    ///Bit 9 - Transmission Completed Enable
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<9> {
        TCE_W::new(self)
    }
    ///Bit 10 - Transmission Cancellation Finished Enable
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TCFE_W<10> {
        TCFE_W::new(self)
    }
    ///Bit 11 - Tx FIFO Empty Enable
    #[inline(always)]
    #[must_use]
    pub fn tefe(&mut self) -> TEFE_W<11> {
        TEFE_W::new(self)
    }
    ///Bit 12 - Tx Event FIFO New Entry Enable
    #[inline(always)]
    #[must_use]
    pub fn tefne(&mut self) -> TEFNE_W<12> {
        TEFNE_W::new(self)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached Enable
    #[inline(always)]
    #[must_use]
    pub fn tefwe(&mut self) -> TEFWE_W<13> {
        TEFWE_W::new(self)
    }
    ///Bit 14 - Tx Event FIFO Full Enable
    #[inline(always)]
    #[must_use]
    pub fn teffe(&mut self) -> TEFFE_W<14> {
        TEFFE_W::new(self)
    }
    ///Bit 15 - Tx Event FIFO Element Lost Enable
    #[inline(always)]
    #[must_use]
    pub fn tefle(&mut self) -> TEFLE_W<15> {
        TEFLE_W::new(self)
    }
    ///Bit 16 - Timestamp Wraparound Enable
    #[inline(always)]
    #[must_use]
    pub fn tswe(&mut self) -> TSWE_W<16> {
        TSWE_W::new(self)
    }
    ///Bit 17 - Message RAM Access Failure Enable
    #[inline(always)]
    #[must_use]
    pub fn mrafe(&mut self) -> MRAFE_W<17> {
        MRAFE_W::new(self)
    }
    ///Bit 18 - Timeout Occurred Enable
    #[inline(always)]
    #[must_use]
    pub fn tooe(&mut self) -> TOOE_W<18> {
        TOOE_W::new(self)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer Enable
    #[inline(always)]
    #[must_use]
    pub fn drxe(&mut self) -> DRXE_W<19> {
        DRXE_W::new(self)
    }
    ///Bit 20 - Bit Error Corrected Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn bece(&mut self) -> BECE_W<20> {
        BECE_W::new(self)
    }
    ///Bit 21 - Bit Error Uncorrected Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn beue(&mut self) -> BEUE_W<21> {
        BEUE_W::new(self)
    }
    ///Bit 22 - Error Logging Overflow Enable
    #[inline(always)]
    #[must_use]
    pub fn eloe(&mut self) -> ELOE_W<22> {
        ELOE_W::new(self)
    }
    ///Bit 23 - Error Passive Enable
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EPE_W<23> {
        EPE_W::new(self)
    }
    ///Bit 24 - Warning Status Enable
    #[inline(always)]
    #[must_use]
    pub fn ewe(&mut self) -> EWE_W<24> {
        EWE_W::new(self)
    }
    ///Bit 25 - Bus_Off Status Enable
    #[inline(always)]
    #[must_use]
    pub fn boe(&mut self) -> BOE_W<25> {
        BOE_W::new(self)
    }
    ///Bit 26 - Watchdog Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WDIE_W<26> {
        WDIE_W::new(self)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase Enable
    #[inline(always)]
    #[must_use]
    pub fn peae(&mut self) -> PEAE_W<27> {
        PEAE_W::new(self)
    }
    ///Bit 28 - Protocol Error in Data Phase Enable
    #[inline(always)]
    #[must_use]
    pub fn pede(&mut self) -> PEDE_W<28> {
        PEDE_W::new(self)
    }
    ///Bit 29 - Access to Reserved Address Enable
    #[inline(always)]
    #[must_use]
    pub fn arae(&mut self) -> ARAE_W<29> {
        ARAE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Interrupt Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ie](index.html) module
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
///`read()` method returns [ie::R](R) reader structure
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ie::W](W) writer structure
impl crate::Writable for IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IE to value 0
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
