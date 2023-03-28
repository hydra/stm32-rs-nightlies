///Register `IR` reader
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IR` writer
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RF0N` reader - Rx FIFO 0 New Message
pub type RF0N_R = crate::BitReader<bool>;
///Field `RF0N` writer - Rx FIFO 0 New Message
pub type RF0N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `RF0W` reader - Rx FIFO 0 Full
pub type RF0W_R = crate::BitReader<bool>;
///Field `RF0W` writer - Rx FIFO 0 Full
pub type RF0W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `RF0F` reader - Rx FIFO 0 Full
pub type RF0F_R = crate::BitReader<bool>;
///Field `RF0F` writer - Rx FIFO 0 Full
pub type RF0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `RF0L` reader - Rx FIFO 0 Message Lost
pub type RF0L_R = crate::BitReader<bool>;
///Field `RF0L` writer - Rx FIFO 0 Message Lost
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `RF1N` reader - Rx FIFO 1 New Message
pub type RF1N_R = crate::BitReader<bool>;
///Field `RF1N` writer - Rx FIFO 1 New Message
pub type RF1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `RF1W` reader - Rx FIFO 1 Watermark Reached
pub type RF1W_R = crate::BitReader<bool>;
///Field `RF1W` writer - Rx FIFO 1 Watermark Reached
pub type RF1W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `RF1F` reader - Rx FIFO 1 Watermark Reached
pub type RF1F_R = crate::BitReader<bool>;
///Field `RF1F` writer - Rx FIFO 1 Watermark Reached
pub type RF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `RF1L` reader - Rx FIFO 1 Message Lost
pub type RF1L_R = crate::BitReader<bool>;
///Field `RF1L` writer - Rx FIFO 1 Message Lost
pub type RF1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `HPM` reader - High Priority Message
pub type HPM_R = crate::BitReader<bool>;
///Field `HPM` writer - High Priority Message
pub type HPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TC` reader - Transmission Completed
pub type TC_R = crate::BitReader<bool>;
///Field `TC` writer - Transmission Completed
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TCF` reader - Transmission Cancellation Finished
pub type TCF_R = crate::BitReader<bool>;
///Field `TCF` writer - Transmission Cancellation Finished
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TEF` reader - Tx FIFO Empty
pub type TEF_R = crate::BitReader<bool>;
///Field `TEF` writer - Tx FIFO Empty
pub type TEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TEFN` reader - Tx Event FIFO New Entry
pub type TEFN_R = crate::BitReader<bool>;
///Field `TEFN` writer - Tx Event FIFO New Entry
pub type TEFN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TEFW` reader - Tx Event FIFO Watermark Reached
pub type TEFW_R = crate::BitReader<bool>;
///Field `TEFW` writer - Tx Event FIFO Watermark Reached
pub type TEFW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TEFF` reader - Tx Event FIFO Full
pub type TEFF_R = crate::BitReader<bool>;
///Field `TEFF` writer - Tx Event FIFO Full
pub type TEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TEFL` reader - Tx Event FIFO Element Lost
pub type TEFL_R = crate::BitReader<bool>;
///Field `TEFL` writer - Tx Event FIFO Element Lost
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TSW` reader - Timestamp Wraparound
pub type TSW_R = crate::BitReader<bool>;
///Field `TSW` writer - Timestamp Wraparound
pub type TSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `MRAF` reader - Message RAM Access Failure
pub type MRAF_R = crate::BitReader<bool>;
///Field `MRAF` writer - Message RAM Access Failure
pub type MRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `TOO` reader - Timeout Occurred
pub type TOO_R = crate::BitReader<bool>;
///Field `TOO` writer - Timeout Occurred
pub type TOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `DRX` reader - Message stored to Dedicated Rx Buffer
pub type DRX_R = crate::BitReader<bool>;
///Field `DRX` writer - Message stored to Dedicated Rx Buffer
pub type DRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `ELO` reader - Error Logging Overflow
pub type ELO_R = crate::BitReader<bool>;
///Field `ELO` writer - Error Logging Overflow
pub type ELO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `EP` reader - Error Passive
pub type EP_R = crate::BitReader<bool>;
///Field `EP` writer - Error Passive
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `EW` reader - Warning Status
pub type EW_R = crate::BitReader<bool>;
///Field `EW` writer - Warning Status
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `BO` reader - Bus_Off Status
pub type BO_R = crate::BitReader<bool>;
///Field `BO` writer - Bus_Off Status
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `WDI` reader - Watchdog Interrupt
pub type WDI_R = crate::BitReader<bool>;
///Field `WDI` writer - Watchdog Interrupt
pub type WDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
pub type PEA_R = crate::BitReader<bool>;
///Field `PEA` writer - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
pub type PEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used)
pub type PED_R = crate::BitReader<bool>;
///Field `PED` writer - Protocol Error in Data Phase (Data Bit Time is used)
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `ARA` reader - Access to Reserved Address
pub type ARA_R = crate::BitReader<bool>;
///Field `ARA` writer - Access to Reserved Address
pub type ARA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Rx FIFO 0 New Message
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 New Message
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High Priority Message
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission Completed
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission Cancellation Finished
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx FIFO Empty
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx Event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx Event FIFO Full
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx Event FIFO Element Lost
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp Wraparound
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Message RAM Access Failure
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timeout Occurred
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Error Logging Overflow
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Error Passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Warning Status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Watchdog Interrupt
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Access to Reserved Address
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 New Message
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<0> {
        RF0N_W::new(self)
    }
    ///Bit 1 - Rx FIFO 0 Full
    #[inline(always)]
    #[must_use]
    pub fn rf0w(&mut self) -> RF0W_W<1> {
        RF0W_W::new(self)
    }
    ///Bit 2 - Rx FIFO 0 Full
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<2> {
        RF0F_W::new(self)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<3> {
        RF0L_W::new(self)
    }
    ///Bit 4 - Rx FIFO 1 New Message
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<4> {
        RF1N_W::new(self)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    #[must_use]
    pub fn rf1w(&mut self) -> RF1W_W<5> {
        RF1W_W::new(self)
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<6> {
        RF1F_W::new(self)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<7> {
        RF1L_W::new(self)
    }
    ///Bit 8 - High Priority Message
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<8> {
        HPM_W::new(self)
    }
    ///Bit 9 - Transmission Completed
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<9> {
        TC_W::new(self)
    }
    ///Bit 10 - Transmission Cancellation Finished
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<10> {
        TCF_W::new(self)
    }
    ///Bit 11 - Tx FIFO Empty
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<11> {
        TEF_W::new(self)
    }
    ///Bit 12 - Tx Event FIFO New Entry
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<12> {
        TEFN_W::new(self)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached
    #[inline(always)]
    #[must_use]
    pub fn tefw(&mut self) -> TEFW_W<13> {
        TEFW_W::new(self)
    }
    ///Bit 14 - Tx Event FIFO Full
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<14> {
        TEFF_W::new(self)
    }
    ///Bit 15 - Tx Event FIFO Element Lost
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<15> {
        TEFL_W::new(self)
    }
    ///Bit 16 - Timestamp Wraparound
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<16> {
        TSW_W::new(self)
    }
    ///Bit 17 - Message RAM Access Failure
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<17> {
        MRAF_W::new(self)
    }
    ///Bit 18 - Timeout Occurred
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<18> {
        TOO_W::new(self)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DRX_W<19> {
        DRX_W::new(self)
    }
    ///Bit 22 - Error Logging Overflow
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<22> {
        ELO_W::new(self)
    }
    ///Bit 23 - Error Passive
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<23> {
        EP_W::new(self)
    }
    ///Bit 24 - Warning Status
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<24> {
        EW_W::new(self)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<25> {
        BO_W::new(self)
    }
    ///Bit 26 - Watchdog Interrupt
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<26> {
        WDI_W::new(self)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<27> {
        PEA_W::new(self)
    }
    ///Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<28> {
        PED_W::new(self)
    }
    ///Bit 29 - Access to Reserved Address
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> ARA_W<29> {
        ARA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Interrupt Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ir](index.html) module
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ir::R](R) reader structure
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ir::W](W) writer structure
impl crate::Writable for IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
