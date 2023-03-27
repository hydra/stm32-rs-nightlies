///Register `DMASR` reader
pub struct R(crate::R<DMASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMASR` writer
pub struct W(crate::W<DMASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASR_SPEC>;
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
impl From<crate::W<DMASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TS` reader - Transmit status
pub type TS_R = crate::BitReader<bool>;
///Field `TS` writer - Transmit status
pub type TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TPSS` reader - Transmit process stopped status
pub type TPSS_R = crate::BitReader<bool>;
///Field `TPSS` writer - Transmit process stopped status
pub type TPSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TBUS` reader - Transmit buffer unavailable status
pub type TBUS_R = crate::BitReader<bool>;
///Field `TBUS` writer - Transmit buffer unavailable status
pub type TBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TJTS` reader - Transmit jabber timeout status
pub type TJTS_R = crate::BitReader<bool>;
///Field `TJTS` writer - Transmit jabber timeout status
pub type TJTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `ROS` reader - Receive overflow status
pub type ROS_R = crate::BitReader<bool>;
///Field `ROS` writer - Receive overflow status
pub type ROS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `TUS` reader - Transmit underflow status
pub type TUS_R = crate::BitReader<bool>;
///Field `TUS` writer - Transmit underflow status
pub type TUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RS` reader - Receive status
pub type RS_R = crate::BitReader<bool>;
///Field `RS` writer - Receive status
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RBUS` reader - Receive buffer unavailable status
pub type RBUS_R = crate::BitReader<bool>;
///Field `RBUS` writer - Receive buffer unavailable status
pub type RBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RPSS` reader - Receive process stopped status
pub type RPSS_R = crate::BitReader<bool>;
///Field `RPSS` writer - Receive process stopped status
pub type RPSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `PWTS` reader - PWTS
pub type PWTS_R = crate::BitReader<bool>;
///Field `PWTS` writer - PWTS
pub type PWTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `ETS` reader - Early transmit status
pub type ETS_R = crate::BitReader<bool>;
///Field `ETS` writer - Early transmit status
pub type ETS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `FBES` reader - Fatal bus error status
pub type FBES_R = crate::BitReader<bool>;
///Field `FBES` writer - Fatal bus error status
pub type FBES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `ERS` reader - Early receive status
pub type ERS_R = crate::BitReader<bool>;
///Field `ERS` writer - Early receive status
pub type ERS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `AIS` reader - Abnormal interrupt summary
pub type AIS_R = crate::BitReader<bool>;
///Field `AIS` writer - Abnormal interrupt summary
pub type AIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `NIS` reader - Normal interrupt summary
pub type NIS_R = crate::BitReader<bool>;
///Field `NIS` writer - Normal interrupt summary
pub type NIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, O>;
///Field `RPS` reader - Receive process state
pub type RPS_R = crate::FieldReader<u8, RPS_A>;
///Receive process state
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPS_A {
    ///0: Stopped, reset or Stop Receive command issued
    Stopped = 0,
    ///1: Running, fetching receive transfer descriptor
    RunningFetching = 1,
    ///3: Running, waiting for receive packet
    RunningWaiting = 3,
    ///4: Suspended, receive descriptor unavailable
    Suspended = 4,
    ///7: Running, writing data to host memory buffer
    RunningWriting = 7,
}
impl From<RPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RPS_A) -> Self {
        variant as _
    }
}
impl RPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RPS_A> {
        match self.bits {
            0 => Some(RPS_A::Stopped),
            1 => Some(RPS_A::RunningFetching),
            3 => Some(RPS_A::RunningWaiting),
            4 => Some(RPS_A::Suspended),
            7 => Some(RPS_A::RunningWriting),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Stopped`
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == RPS_A::Stopped
    }
    ///Checks if the value of the field is `RunningFetching`
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == RPS_A::RunningFetching
    }
    ///Checks if the value of the field is `RunningWaiting`
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == RPS_A::RunningWaiting
    }
    ///Checks if the value of the field is `Suspended`
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == RPS_A::Suspended
    }
    ///Checks if the value of the field is `RunningWriting`
    #[inline(always)]
    pub fn is_running_writing(&self) -> bool {
        *self == RPS_A::RunningWriting
    }
}
///Field `TPS` reader - Transmit process state
pub type TPS_R = crate::FieldReader<u8, TPS_A>;
///Transmit process state
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPS_A {
    ///0: Stopped, Reset or Stop Transmit command issued
    Stopped = 0,
    ///1: Running, fetching transmit transfer descriptor
    RunningFetching = 1,
    ///2: Running, waiting for status
    RunningWaiting = 2,
    ///3: Running, reading data from host memory buffer
    RunningReading = 3,
    ///6: Suspended, transmit descriptor unavailable or transmit buffer underflow
    Suspended = 6,
    ///7: Running, closing transmit descriptor
    Running = 7,
}
impl From<TPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TPS_A) -> Self {
        variant as _
    }
}
impl TPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TPS_A> {
        match self.bits {
            0 => Some(TPS_A::Stopped),
            1 => Some(TPS_A::RunningFetching),
            2 => Some(TPS_A::RunningWaiting),
            3 => Some(TPS_A::RunningReading),
            6 => Some(TPS_A::Suspended),
            7 => Some(TPS_A::Running),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Stopped`
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == TPS_A::Stopped
    }
    ///Checks if the value of the field is `RunningFetching`
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == TPS_A::RunningFetching
    }
    ///Checks if the value of the field is `RunningWaiting`
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == TPS_A::RunningWaiting
    }
    ///Checks if the value of the field is `RunningReading`
    #[inline(always)]
    pub fn is_running_reading(&self) -> bool {
        *self == TPS_A::RunningReading
    }
    ///Checks if the value of the field is `Suspended`
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == TPS_A::Suspended
    }
    ///Checks if the value of the field is `Running`
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == TPS_A::Running
    }
}
///Field `EBS` reader - Error bits status
pub type EBS_R = crate::FieldReader<u8, u8>;
///Field `MMCS` reader - MMC status
pub type MMCS_R = crate::BitReader<bool>;
///Field `PMTS` reader - PMT status
pub type PMTS_R = crate::BitReader<bool>;
///Field `TSTS` reader - Time stamp trigger status
pub type TSTS_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Transmit status
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit process stopped status
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit buffer unavailable status
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit jabber timeout status
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive overflow status
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit underflow status
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive status
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive buffer unavailable status
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Receive process stopped status
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Early transmit status
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Fatal bus error status
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Early receive status
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Abnormal interrupt summary
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Normal interrupt summary
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Receive process state
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22 - Transmit process state
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - Error bits status
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 27 - MMC status
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PMT status
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Time stamp trigger status
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit status
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<0> {
        TS_W::new(self)
    }
    ///Bit 1 - Transmit process stopped status
    #[inline(always)]
    #[must_use]
    pub fn tpss(&mut self) -> TPSS_W<1> {
        TPSS_W::new(self)
    }
    ///Bit 2 - Transmit buffer unavailable status
    #[inline(always)]
    #[must_use]
    pub fn tbus(&mut self) -> TBUS_W<2> {
        TBUS_W::new(self)
    }
    ///Bit 3 - Transmit jabber timeout status
    #[inline(always)]
    #[must_use]
    pub fn tjts(&mut self) -> TJTS_W<3> {
        TJTS_W::new(self)
    }
    ///Bit 4 - Receive overflow status
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> ROS_W<4> {
        ROS_W::new(self)
    }
    ///Bit 5 - Transmit underflow status
    #[inline(always)]
    #[must_use]
    pub fn tus(&mut self) -> TUS_W<5> {
        TUS_W::new(self)
    }
    ///Bit 6 - Receive status
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<6> {
        RS_W::new(self)
    }
    ///Bit 7 - Receive buffer unavailable status
    #[inline(always)]
    #[must_use]
    pub fn rbus(&mut self) -> RBUS_W<7> {
        RBUS_W::new(self)
    }
    ///Bit 8 - Receive process stopped status
    #[inline(always)]
    #[must_use]
    pub fn rpss(&mut self) -> RPSS_W<8> {
        RPSS_W::new(self)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    #[must_use]
    pub fn pwts(&mut self) -> PWTS_W<9> {
        PWTS_W::new(self)
    }
    ///Bit 10 - Early transmit status
    #[inline(always)]
    #[must_use]
    pub fn ets(&mut self) -> ETS_W<10> {
        ETS_W::new(self)
    }
    ///Bit 13 - Fatal bus error status
    #[inline(always)]
    #[must_use]
    pub fn fbes(&mut self) -> FBES_W<13> {
        FBES_W::new(self)
    }
    ///Bit 14 - Early receive status
    #[inline(always)]
    #[must_use]
    pub fn ers(&mut self) -> ERS_W<14> {
        ERS_W::new(self)
    }
    ///Bit 15 - Abnormal interrupt summary
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<15> {
        AIS_W::new(self)
    }
    ///Bit 16 - Normal interrupt summary
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<16> {
        NIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmasr](index.html) module
pub struct DMASR_SPEC;
impl crate::RegisterSpec for DMASR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmasr::R](R) reader structure
impl crate::Readable for DMASR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmasr::W](W) writer structure
impl crate::Writable for DMASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMASR to value 0
impl crate::Resettable for DMASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
