///Register `DMAOMR` reader
pub struct R(crate::R<DMAOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAOMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMAOMR` writer
pub struct W(crate::W<DMAOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAOMR_SPEC>;
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
impl From<crate::W<DMAOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAOMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SR` reader - Start/stop receive
pub type SR_R = crate::BitReader<SR_A>;
///Start/stop receive
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR_A {
    ///0: Reception is stopped after transfer of the current frame
    Stopped = 0,
    ///1: Reception is placed in the Running state
    Started = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
impl SR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::Stopped,
            true => SR_A::Started,
        }
    }
    ///Checks if the value of the field is `Stopped`
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == SR_A::Stopped
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SR_A::Started
    }
}
///Field `SR` writer - Start/stop receive
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, SR_A, O>;
impl<'a, const O: u8> SR_W<'a, O> {
    ///Reception is stopped after transfer of the current frame
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(SR_A::Stopped)
    }
    ///Reception is placed in the Running state
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(SR_A::Started)
    }
}
///Field `OSF` reader - Operate on second frame
pub type OSF_R = crate::BitReader<bool>;
///Field `OSF` writer - Operate on second frame
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
///Field `RTC` reader - Receive threshold control
pub type RTC_R = crate::FieldReader<u8, RTC_A>;
///Receive threshold control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC_A {
    ///0: 64 bytes
    Rtc64 = 0,
    ///1: 32 bytes
    Rtc32 = 1,
    ///2: 96 bytes
    Rtc96 = 2,
    ///3: 128 bytes
    Rtc128 = 3,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as _
    }
}
impl RTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::Rtc64,
            1 => RTC_A::Rtc32,
            2 => RTC_A::Rtc96,
            3 => RTC_A::Rtc128,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Rtc64`
    #[inline(always)]
    pub fn is_rtc64(&self) -> bool {
        *self == RTC_A::Rtc64
    }
    ///Checks if the value of the field is `Rtc32`
    #[inline(always)]
    pub fn is_rtc32(&self) -> bool {
        *self == RTC_A::Rtc32
    }
    ///Checks if the value of the field is `Rtc96`
    #[inline(always)]
    pub fn is_rtc96(&self) -> bool {
        *self == RTC_A::Rtc96
    }
    ///Checks if the value of the field is `Rtc128`
    #[inline(always)]
    pub fn is_rtc128(&self) -> bool {
        *self == RTC_A::Rtc128
    }
}
///Field `RTC` writer - Receive threshold control
pub type RTC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DMAOMR_SPEC, u8, RTC_A, 2, O>;
impl<'a, const O: u8> RTC_W<'a, O> {
    ///64 bytes
    #[inline(always)]
    pub fn rtc64(self) -> &'a mut W {
        self.variant(RTC_A::Rtc64)
    }
    ///32 bytes
    #[inline(always)]
    pub fn rtc32(self) -> &'a mut W {
        self.variant(RTC_A::Rtc32)
    }
    ///96 bytes
    #[inline(always)]
    pub fn rtc96(self) -> &'a mut W {
        self.variant(RTC_A::Rtc96)
    }
    ///128 bytes
    #[inline(always)]
    pub fn rtc128(self) -> &'a mut W {
        self.variant(RTC_A::Rtc128)
    }
}
///Field `FUGF` reader - Forward undersized good frames
pub type FUGF_R = crate::BitReader<FUGF_A>;
///Forward undersized good frames
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUGF_A {
    ///0: Rx FIFO drops all frames of less than 64 bytes
    Drop = 0,
    ///1: Rx FIFO forwards undersized frames
    Forward = 1,
}
impl From<FUGF_A> for bool {
    #[inline(always)]
    fn from(variant: FUGF_A) -> Self {
        variant as u8 != 0
    }
}
impl FUGF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FUGF_A {
        match self.bits {
            false => FUGF_A::Drop,
            true => FUGF_A::Forward,
        }
    }
    ///Checks if the value of the field is `Drop`
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FUGF_A::Drop
    }
    ///Checks if the value of the field is `Forward`
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FUGF_A::Forward
    }
}
///Field `FUGF` writer - Forward undersized good frames
pub type FUGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, FUGF_A, O>;
impl<'a, const O: u8> FUGF_W<'a, O> {
    ///Rx FIFO drops all frames of less than 64 bytes
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(FUGF_A::Drop)
    }
    ///Rx FIFO forwards undersized frames
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(FUGF_A::Forward)
    }
}
///Field `FEF` reader - Forward error frames
pub type FEF_R = crate::BitReader<FEF_A>;
///Forward error frames
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    ///0: Rx FIFO drops frames with error status
    Drop = 0,
    ///1: All frames except runt error frames are forwarded to the DMA
    Forward = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
impl FEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::Drop,
            true => FEF_A::Forward,
        }
    }
    ///Checks if the value of the field is `Drop`
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FEF_A::Drop
    }
    ///Checks if the value of the field is `Forward`
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FEF_A::Forward
    }
}
///Field `FEF` writer - Forward error frames
pub type FEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, FEF_A, O>;
impl<'a, const O: u8> FEF_W<'a, O> {
    ///Rx FIFO drops frames with error status
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(FEF_A::Drop)
    }
    ///All frames except runt error frames are forwarded to the DMA
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(FEF_A::Forward)
    }
}
///Field `ST` reader - Start/stop transmission
pub type ST_R = crate::BitReader<ST_A>;
///Start/stop transmission
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    ///0: Transmission is placed in the Stopped state
    Stopped = 0,
    ///1: Transmission is placed in Running state
    Started = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
impl ST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::Stopped,
            true => ST_A::Started,
        }
    }
    ///Checks if the value of the field is `Stopped`
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == ST_A::Stopped
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == ST_A::Started
    }
}
///Field `ST` writer - Start/stop transmission
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, ST_A, O>;
impl<'a, const O: u8> ST_W<'a, O> {
    ///Transmission is placed in the Stopped state
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(ST_A::Stopped)
    }
    ///Transmission is placed in Running state
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(ST_A::Started)
    }
}
///Field `TTC` reader - Transmit threshold control
pub type TTC_R = crate::FieldReader<u8, TTC_A>;
///Transmit threshold control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTC_A {
    ///0: 64 bytes
    Ttc64 = 0,
    ///1: 128 bytes
    Ttc128 = 1,
    ///2: 192 bytes
    Ttc192 = 2,
    ///3: 256 bytes
    Ttc256 = 3,
    ///4: 40 bytes
    Ttc40 = 4,
    ///5: 32 bytes
    Ttc32 = 5,
    ///6: 24 bytes
    Ttc24 = 6,
    ///7: 16 bytes
    Ttc16 = 7,
}
impl From<TTC_A> for u8 {
    #[inline(always)]
    fn from(variant: TTC_A) -> Self {
        variant as _
    }
}
impl TTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TTC_A {
        match self.bits {
            0 => TTC_A::Ttc64,
            1 => TTC_A::Ttc128,
            2 => TTC_A::Ttc192,
            3 => TTC_A::Ttc256,
            4 => TTC_A::Ttc40,
            5 => TTC_A::Ttc32,
            6 => TTC_A::Ttc24,
            7 => TTC_A::Ttc16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Ttc64`
    #[inline(always)]
    pub fn is_ttc64(&self) -> bool {
        *self == TTC_A::Ttc64
    }
    ///Checks if the value of the field is `Ttc128`
    #[inline(always)]
    pub fn is_ttc128(&self) -> bool {
        *self == TTC_A::Ttc128
    }
    ///Checks if the value of the field is `Ttc192`
    #[inline(always)]
    pub fn is_ttc192(&self) -> bool {
        *self == TTC_A::Ttc192
    }
    ///Checks if the value of the field is `Ttc256`
    #[inline(always)]
    pub fn is_ttc256(&self) -> bool {
        *self == TTC_A::Ttc256
    }
    ///Checks if the value of the field is `Ttc40`
    #[inline(always)]
    pub fn is_ttc40(&self) -> bool {
        *self == TTC_A::Ttc40
    }
    ///Checks if the value of the field is `Ttc32`
    #[inline(always)]
    pub fn is_ttc32(&self) -> bool {
        *self == TTC_A::Ttc32
    }
    ///Checks if the value of the field is `Ttc24`
    #[inline(always)]
    pub fn is_ttc24(&self) -> bool {
        *self == TTC_A::Ttc24
    }
    ///Checks if the value of the field is `Ttc16`
    #[inline(always)]
    pub fn is_ttc16(&self) -> bool {
        *self == TTC_A::Ttc16
    }
}
///Field `TTC` writer - Transmit threshold control
pub type TTC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DMAOMR_SPEC, u8, TTC_A, 3, O>;
impl<'a, const O: u8> TTC_W<'a, O> {
    ///64 bytes
    #[inline(always)]
    pub fn ttc64(self) -> &'a mut W {
        self.variant(TTC_A::Ttc64)
    }
    ///128 bytes
    #[inline(always)]
    pub fn ttc128(self) -> &'a mut W {
        self.variant(TTC_A::Ttc128)
    }
    ///192 bytes
    #[inline(always)]
    pub fn ttc192(self) -> &'a mut W {
        self.variant(TTC_A::Ttc192)
    }
    ///256 bytes
    #[inline(always)]
    pub fn ttc256(self) -> &'a mut W {
        self.variant(TTC_A::Ttc256)
    }
    ///40 bytes
    #[inline(always)]
    pub fn ttc40(self) -> &'a mut W {
        self.variant(TTC_A::Ttc40)
    }
    ///32 bytes
    #[inline(always)]
    pub fn ttc32(self) -> &'a mut W {
        self.variant(TTC_A::Ttc32)
    }
    ///24 bytes
    #[inline(always)]
    pub fn ttc24(self) -> &'a mut W {
        self.variant(TTC_A::Ttc24)
    }
    ///16 bytes
    #[inline(always)]
    pub fn ttc16(self) -> &'a mut W {
        self.variant(TTC_A::Ttc16)
    }
}
///Field `FTF` reader - Flush transmit FIFO
pub type FTF_R = crate::BitReader<FTF_A>;
///Flush transmit FIFO
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF_A {
    ///1: Transmit FIFO controller logic is reset to its default values. Cleared automatically
    Flush = 1,
}
impl From<FTF_A> for bool {
    #[inline(always)]
    fn from(variant: FTF_A) -> Self {
        variant as u8 != 0
    }
}
impl FTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FTF_A> {
        match self.bits {
            true => Some(FTF_A::Flush),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Flush`
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FTF_A::Flush
    }
}
///Field `FTF` writer - Flush transmit FIFO
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, FTF_A, O>;
impl<'a, const O: u8> FTF_W<'a, O> {
    ///Transmit FIFO controller logic is reset to its default values. Cleared automatically
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FTF_A::Flush)
    }
}
///Field `TSF` reader - Transmit store and forward
pub type TSF_R = crate::BitReader<TSF_A>;
///Transmit store and forward
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSF_A {
    ///0: Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold
    CutThrough = 0,
    ///1: Transmission starts when a full frame is in the Tx FIFO
    StoreForward = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSF_A {
        match self.bits {
            false => TSF_A::CutThrough,
            true => TSF_A::StoreForward,
        }
    }
    ///Checks if the value of the field is `CutThrough`
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == TSF_A::CutThrough
    }
    ///Checks if the value of the field is `StoreForward`
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == TSF_A::StoreForward
    }
}
///Field `TSF` writer - Transmit store and forward
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, TSF_A, O>;
impl<'a, const O: u8> TSF_W<'a, O> {
    ///Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(TSF_A::CutThrough)
    }
    ///Transmission starts when a full frame is in the Tx FIFO
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(TSF_A::StoreForward)
    }
}
///Field `DFRF` reader - Disable flushing of received frames
pub type DFRF_R = crate::BitReader<bool>;
///Field `DFRF` writer - Disable flushing of received frames
pub type DFRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
///Field `RSF` reader - Receive store and forward
pub type RSF_R = crate::BitReader<RSF_A>;
///Receive store and forward
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSF_A {
    ///0: Rx FIFO operates in cut-through mode, subject to RTC bits
    CutThrough = 0,
    ///1: Frames are read from Rx FIFO after complete frame has been written
    StoreForward = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::CutThrough,
            true => RSF_A::StoreForward,
        }
    }
    ///Checks if the value of the field is `CutThrough`
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == RSF_A::CutThrough
    }
    ///Checks if the value of the field is `StoreForward`
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == RSF_A::StoreForward
    }
}
///Field `RSF` writer - Receive store and forward
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, RSF_A, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    ///Rx FIFO operates in cut-through mode, subject to RTC bits
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(RSF_A::CutThrough)
    }
    ///Frames are read from Rx FIFO after complete frame has been written
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(RSF_A::StoreForward)
    }
}
///Field `DTCEFD` reader - Dropping of TCP/IP checksum error frames disable
pub type DTCEFD_R = crate::BitReader<DTCEFD_A>;
///Dropping of TCP/IP checksum error frames disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCEFD_A {
    ///0: Drop frames with errors only in the receive checksum offload engine
    Enabled = 0,
    ///1: Do not drop frames that only have errors in the receive checksum offload engine
    Disabled = 1,
}
impl From<DTCEFD_A> for bool {
    #[inline(always)]
    fn from(variant: DTCEFD_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCEFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTCEFD_A {
        match self.bits {
            false => DTCEFD_A::Enabled,
            true => DTCEFD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTCEFD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTCEFD_A::Disabled
    }
}
///Field `DTCEFD` writer - Dropping of TCP/IP checksum error frames disable
pub type DTCEFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, DTCEFD_A, O>;
impl<'a, const O: u8> DTCEFD_W<'a, O> {
    ///Drop frames with errors only in the receive checksum offload engine
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTCEFD_A::Enabled)
    }
    ///Do not drop frames that only have errors in the receive checksum offload engine
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTCEFD_A::Disabled)
    }
}
impl R {
    ///Bit 1 - Start/stop receive
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Operate on second frame
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Receive threshold control
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 6 - Forward undersized good frames
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Forward error frames
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - Start/stop transmission
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:16 - Transmit threshold control
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bit 20 - Flush transmit FIFO
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Transmit store and forward
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Disable flushing of received frames
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Receive store and forward
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Dropping of TCP/IP checksum error frames disable
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Start/stop receive
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<1> {
        SR_W::new(self)
    }
    ///Bit 2 - Operate on second frame
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<2> {
        OSF_W::new(self)
    }
    ///Bits 3:4 - Receive threshold control
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<3> {
        RTC_W::new(self)
    }
    ///Bit 6 - Forward undersized good frames
    #[inline(always)]
    #[must_use]
    pub fn fugf(&mut self) -> FUGF_W<6> {
        FUGF_W::new(self)
    }
    ///Bit 7 - Forward error frames
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<7> {
        FEF_W::new(self)
    }
    ///Bit 13 - Start/stop transmission
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<13> {
        ST_W::new(self)
    }
    ///Bits 14:16 - Transmit threshold control
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<14> {
        TTC_W::new(self)
    }
    ///Bit 20 - Flush transmit FIFO
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<20> {
        FTF_W::new(self)
    }
    ///Bit 21 - Transmit store and forward
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<21> {
        TSF_W::new(self)
    }
    ///Bit 24 - Disable flushing of received frames
    #[inline(always)]
    #[must_use]
    pub fn dfrf(&mut self) -> DFRF_W<24> {
        DFRF_W::new(self)
    }
    ///Bit 25 - Receive store and forward
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<25> {
        RSF_W::new(self)
    }
    ///Bit 26 - Dropping of TCP/IP checksum error frames disable
    #[inline(always)]
    #[must_use]
    pub fn dtcefd(&mut self) -> DTCEFD_W<26> {
        DTCEFD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA operation mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaomr](index.html) module
pub struct DMAOMR_SPEC;
impl crate::RegisterSpec for DMAOMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaomr::R](R) reader structure
impl crate::Readable for DMAOMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmaomr::W](W) writer structure
impl crate::Writable for DMAOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMAOMR to value 0
impl crate::Resettable for DMAOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
