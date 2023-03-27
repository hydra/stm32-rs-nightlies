///Register `ILS` reader
pub struct R(crate::R<ILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ILS` writer
pub struct W(crate::W<ILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILS_SPEC>;
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
impl From<crate::W<ILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RF0NL` reader - Rx FIFO 0 New Message Interrupt Line
pub type RF0NL_R = crate::BitReader<bool>;
///Field `RF0NL` writer - Rx FIFO 0 New Message Interrupt Line
pub type RF0NL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `RF0WL` reader - Rx FIFO 0 Watermark Reached Interrupt Line
pub type RF0WL_R = crate::BitReader<bool>;
///Field `RF0WL` writer - Rx FIFO 0 Watermark Reached Interrupt Line
pub type RF0WL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `RF0FL` reader - Rx FIFO 0 Full Interrupt Line
pub type RF0FL_R = crate::BitReader<bool>;
///Field `RF0FL` writer - Rx FIFO 0 Full Interrupt Line
pub type RF0FL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `RF0LL` reader - Rx FIFO 0 Message Lost Interrupt Line
pub type RF0LL_R = crate::BitReader<bool>;
///Field `RF0LL` writer - Rx FIFO 0 Message Lost Interrupt Line
pub type RF0LL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `RF1NL` reader - Rx FIFO 1 New Message Interrupt Line
pub type RF1NL_R = crate::BitReader<bool>;
///Field `RF1NL` writer - Rx FIFO 1 New Message Interrupt Line
pub type RF1NL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `RF1WL` reader - Rx FIFO 1 Watermark Reached Interrupt Line
pub type RF1WL_R = crate::BitReader<bool>;
///Field `RF1WL` writer - Rx FIFO 1 Watermark Reached Interrupt Line
pub type RF1WL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `RF1FL` reader - Rx FIFO 1 Full Interrupt Line
pub type RF1FL_R = crate::BitReader<bool>;
///Field `RF1FL` writer - Rx FIFO 1 Full Interrupt Line
pub type RF1FL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `RF1LL` reader - Rx FIFO 1 Message Lost Interrupt Line
pub type RF1LL_R = crate::BitReader<bool>;
///Field `RF1LL` writer - Rx FIFO 1 Message Lost Interrupt Line
pub type RF1LL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `HPML` reader - High Priority Message Interrupt Line
pub type HPML_R = crate::BitReader<bool>;
///Field `HPML` writer - High Priority Message Interrupt Line
pub type HPML_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TCL` reader - Transmission Completed Interrupt Line
pub type TCL_R = crate::BitReader<bool>;
///Field `TCL` writer - Transmission Completed Interrupt Line
pub type TCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TCFL` reader - Transmission Cancellation Finished Interrupt Line
pub type TCFL_R = crate::BitReader<bool>;
///Field `TCFL` writer - Transmission Cancellation Finished Interrupt Line
pub type TCFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TEFL` reader - Tx FIFO Empty Interrupt Line
pub type TEFL_R = crate::BitReader<bool>;
///Field `TEFL` writer - Tx FIFO Empty Interrupt Line
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TEFNL` reader - Tx Event FIFO New Entry Interrupt Line
pub type TEFNL_R = crate::BitReader<bool>;
///Field `TEFNL` writer - Tx Event FIFO New Entry Interrupt Line
pub type TEFNL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TEFWL` reader - Tx Event FIFO Watermark Reached Interrupt Line
pub type TEFWL_R = crate::BitReader<bool>;
///Field `TEFWL` writer - Tx Event FIFO Watermark Reached Interrupt Line
pub type TEFWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TEFFL` reader - Tx Event FIFO Full Interrupt Line
pub type TEFFL_R = crate::BitReader<bool>;
///Field `TEFFL` writer - Tx Event FIFO Full Interrupt Line
pub type TEFFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TEFLL` reader - Tx Event FIFO Element Lost Interrupt Line
pub type TEFLL_R = crate::BitReader<bool>;
///Field `TEFLL` writer - Tx Event FIFO Element Lost Interrupt Line
pub type TEFLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TSWL` reader - Timestamp Wraparound Interrupt Line
pub type TSWL_R = crate::BitReader<bool>;
///Field `TSWL` writer - Timestamp Wraparound Interrupt Line
pub type TSWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `MRAFL` reader - Message RAM Access Failure Interrupt Line
pub type MRAFL_R = crate::BitReader<bool>;
///Field `MRAFL` writer - Message RAM Access Failure Interrupt Line
pub type MRAFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `TOOL` reader - Timeout Occurred Interrupt Line
pub type TOOL_R = crate::BitReader<bool>;
///Field `TOOL` writer - Timeout Occurred Interrupt Line
pub type TOOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `DRXL` reader - Message stored to Dedicated Rx Buffer Interrupt Line
pub type DRXL_R = crate::BitReader<bool>;
///Field `DRXL` writer - Message stored to Dedicated Rx Buffer Interrupt Line
pub type DRXL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `BECL` reader - Bit Error Corrected Interrupt Line
pub type BECL_R = crate::BitReader<bool>;
///Field `BECL` writer - Bit Error Corrected Interrupt Line
pub type BECL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `BEUL` reader - Bit Error Uncorrected Interrupt Line
pub type BEUL_R = crate::BitReader<bool>;
///Field `BEUL` writer - Bit Error Uncorrected Interrupt Line
pub type BEUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `ELOL` reader - Error Logging Overflow Interrupt Line
pub type ELOL_R = crate::BitReader<bool>;
///Field `ELOL` writer - Error Logging Overflow Interrupt Line
pub type ELOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `EPL` reader - Error Passive Interrupt Line
pub type EPL_R = crate::BitReader<bool>;
///Field `EPL` writer - Error Passive Interrupt Line
pub type EPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `EWL` reader - Warning Status Interrupt Line
pub type EWL_R = crate::BitReader<bool>;
///Field `EWL` writer - Warning Status Interrupt Line
pub type EWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `BOL` reader - Bus_Off Status
pub type BOL_R = crate::BitReader<bool>;
///Field `BOL` writer - Bus_Off Status
pub type BOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `WDIL` reader - Watchdog Interrupt Line
pub type WDIL_R = crate::BitReader<bool>;
///Field `WDIL` writer - Watchdog Interrupt Line
pub type WDIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `PEAL` reader - Protocol Error in Arbitration Phase Line
pub type PEAL_R = crate::BitReader<bool>;
///Field `PEAL` writer - Protocol Error in Arbitration Phase Line
pub type PEAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `PEDL` reader - Protocol Error in Data Phase Line
pub type PEDL_R = crate::BitReader<bool>;
///Field `PEDL` writer - Protocol Error in Data Phase Line
pub type PEDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
///Field `ARAL` reader - Access to Reserved Address Line
pub type ARAL_R = crate::BitReader<bool>;
///Field `ARAL` writer - Access to Reserved Address Line
pub type ARAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
impl R {
    ///Bit 0 - Rx FIFO 0 New Message Interrupt Line
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Full Interrupt Line
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 New Message Interrupt Line
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx FIFO 1 Full Interrupt Line
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High Priority Message Interrupt Line
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission Completed Interrupt Line
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission Cancellation Finished Interrupt Line
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx FIFO Empty Interrupt Line
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx Event FIFO New Entry Interrupt Line
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx Event FIFO Full Interrupt Line
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx Event FIFO Element Lost Interrupt Line
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp Wraparound Interrupt Line
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Message RAM Access Failure Interrupt Line
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timeout Occurred Interrupt Line
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Bit Error Corrected Interrupt Line
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Bit Error Uncorrected Interrupt Line
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Error Logging Overflow Interrupt Line
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Error Passive Interrupt Line
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Warning Status Interrupt Line
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Watchdog Interrupt Line
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase Line
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Protocol Error in Data Phase Line
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Access to Reserved Address Line
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 New Message Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf0nl(&mut self) -> RF0NL_W<0> {
        RF0NL_W::new(self)
    }
    ///Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf0wl(&mut self) -> RF0WL_W<1> {
        RF0WL_W::new(self)
    }
    ///Bit 2 - Rx FIFO 0 Full Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf0fl(&mut self) -> RF0FL_W<2> {
        RF0FL_W::new(self)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf0ll(&mut self) -> RF0LL_W<3> {
        RF0LL_W::new(self)
    }
    ///Bit 4 - Rx FIFO 1 New Message Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf1nl(&mut self) -> RF1NL_W<4> {
        RF1NL_W::new(self)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf1wl(&mut self) -> RF1WL_W<5> {
        RF1WL_W::new(self)
    }
    ///Bit 6 - Rx FIFO 1 Full Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf1fl(&mut self) -> RF1FL_W<6> {
        RF1FL_W::new(self)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn rf1ll(&mut self) -> RF1LL_W<7> {
        RF1LL_W::new(self)
    }
    ///Bit 8 - High Priority Message Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn hpml(&mut self) -> HPML_W<8> {
        HPML_W::new(self)
    }
    ///Bit 9 - Transmission Completed Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TCL_W<9> {
        TCL_W::new(self)
    }
    ///Bit 10 - Transmission Cancellation Finished Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tcfl(&mut self) -> TCFL_W<10> {
        TCFL_W::new(self)
    }
    ///Bit 11 - Tx FIFO Empty Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<11> {
        TEFL_W::new(self)
    }
    ///Bit 12 - Tx Event FIFO New Entry Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tefnl(&mut self) -> TEFNL_W<12> {
        TEFNL_W::new(self)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tefwl(&mut self) -> TEFWL_W<13> {
        TEFWL_W::new(self)
    }
    ///Bit 14 - Tx Event FIFO Full Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn teffl(&mut self) -> TEFFL_W<14> {
        TEFFL_W::new(self)
    }
    ///Bit 15 - Tx Event FIFO Element Lost Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tefll(&mut self) -> TEFLL_W<15> {
        TEFLL_W::new(self)
    }
    ///Bit 16 - Timestamp Wraparound Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tswl(&mut self) -> TSWL_W<16> {
        TSWL_W::new(self)
    }
    ///Bit 17 - Message RAM Access Failure Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn mrafl(&mut self) -> MRAFL_W<17> {
        MRAFL_W::new(self)
    }
    ///Bit 18 - Timeout Occurred Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn tool(&mut self) -> TOOL_W<18> {
        TOOL_W::new(self)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn drxl(&mut self) -> DRXL_W<19> {
        DRXL_W::new(self)
    }
    ///Bit 20 - Bit Error Corrected Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn becl(&mut self) -> BECL_W<20> {
        BECL_W::new(self)
    }
    ///Bit 21 - Bit Error Uncorrected Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn beul(&mut self) -> BEUL_W<21> {
        BEUL_W::new(self)
    }
    ///Bit 22 - Error Logging Overflow Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn elol(&mut self) -> ELOL_W<22> {
        ELOL_W::new(self)
    }
    ///Bit 23 - Error Passive Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn epl(&mut self) -> EPL_W<23> {
        EPL_W::new(self)
    }
    ///Bit 24 - Warning Status Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn ewl(&mut self) -> EWL_W<24> {
        EWL_W::new(self)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    #[must_use]
    pub fn bol(&mut self) -> BOL_W<25> {
        BOL_W::new(self)
    }
    ///Bit 26 - Watchdog Interrupt Line
    #[inline(always)]
    #[must_use]
    pub fn wdil(&mut self) -> WDIL_W<26> {
        WDIL_W::new(self)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase Line
    #[inline(always)]
    #[must_use]
    pub fn peal(&mut self) -> PEAL_W<27> {
        PEAL_W::new(self)
    }
    ///Bit 28 - Protocol Error in Data Phase Line
    #[inline(always)]
    #[must_use]
    pub fn pedl(&mut self) -> PEDL_W<28> {
        PEDL_W::new(self)
    }
    ///Bit 29 - Access to Reserved Address Line
    #[inline(always)]
    #[must_use]
    pub fn aral(&mut self) -> ARAL_W<29> {
        ARAL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Interrupt Line Select Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ils](index.html) module
pub struct ILS_SPEC;
impl crate::RegisterSpec for ILS_SPEC {
    type Ux = u32;
}
///`read()` method returns [ils::R](R) reader structure
impl crate::Readable for ILS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ils::W](W) writer structure
impl crate::Writable for ILS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ILS to value 0
impl crate::Resettable for ILS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
