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
///Field `RXP` reader - Rx-Packet available
pub type RXP_R = crate::BitReader<RXP_A>;
///Rx-Packet available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP_A {
    ///0: Rx buffer empty
    Empty = 0,
    ///1: Rx buffer not empty
    NotEmpty = 1,
}
impl From<RXP_A> for bool {
    #[inline(always)]
    fn from(variant: RXP_A) -> Self {
        variant as u8 != 0
    }
}
impl RXP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXP_A {
        match self.bits {
            false => RXP_A::Empty,
            true => RXP_A::NotEmpty,
        }
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXP_A::Empty
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXP_A::NotEmpty
    }
}
///Field `TXP` reader - Tx-Packet space available
pub type TXP_R = crate::BitReader<TXP_A>;
///Tx-Packet space available
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP_A {
    ///0: Tx buffer full
    Full = 0,
    ///1: Tx buffer not full
    NotFull = 1,
}
impl From<TXP_A> for bool {
    #[inline(always)]
    fn from(variant: TXP_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXP_A {
        match self.bits {
            false => TXP_A::Full,
            true => TXP_A::NotFull,
        }
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXP_A::Full
    }
    ///Checks if the value of the field is `NotFull`
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXP_A::NotFull
    }
}
///Field `DXP` reader - Duplex Packet
pub type DXP_R = crate::BitReader<DXP_A>;
///Duplex Packet
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXP_A {
    ///0: Duplex packet unavailable: no space for transmission and/or no data received
    Unavailable = 0,
    ///1: Duplex packet available: space for transmission and data received
    Available = 1,
}
impl From<DXP_A> for bool {
    #[inline(always)]
    fn from(variant: DXP_A) -> Self {
        variant as u8 != 0
    }
}
impl DXP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DXP_A {
        match self.bits {
            false => DXP_A::Unavailable,
            true => DXP_A::Available,
        }
    }
    ///Checks if the value of the field is `Unavailable`
    #[inline(always)]
    pub fn is_unavailable(&self) -> bool {
        *self == DXP_A::Unavailable
    }
    ///Checks if the value of the field is `Available`
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == DXP_A::Available
    }
}
///Field `EOT` reader - End Of Transfer
pub type EOT_R = crate::BitReader<EOT_A>;
///End Of Transfer
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT_A {
    ///0: Transfer ongoing or not started
    NotCompleted = 0,
    ///1: Transfer complete
    Completed = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
impl EOT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::NotCompleted,
            true => EOT_A::Completed,
        }
    }
    ///Checks if the value of the field is `NotCompleted`
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == EOT_A::NotCompleted
    }
    ///Checks if the value of the field is `Completed`
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == EOT_A::Completed
    }
}
///Field `TXTF` reader - Transmission Transfer Filled
pub type TXTF_R = crate::BitReader<TXTF_A>;
///Transmission Transfer Filled
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTF_A {
    ///0: Transmission buffer incomplete
    NotCompleted = 0,
    ///1: Transmission buffer filled with at least one transfer
    Completed = 1,
}
impl From<TXTF_A> for bool {
    #[inline(always)]
    fn from(variant: TXTF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXTF_A {
        match self.bits {
            false => TXTF_A::NotCompleted,
            true => TXTF_A::Completed,
        }
    }
    ///Checks if the value of the field is `NotCompleted`
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TXTF_A::NotCompleted
    }
    ///Checks if the value of the field is `Completed`
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXTF_A::Completed
    }
}
///Field `UDR` reader - Underrun at slave transmission mode
pub type UDR_R = crate::BitReader<UDR_A>;
///Underrun at slave transmission mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR_A {
    ///0: No underrun occurred
    NoUnderrun = 0,
    ///1: Underrun occurred
    Underrun = 1,
}
impl From<UDR_A> for bool {
    #[inline(always)]
    fn from(variant: UDR_A) -> Self {
        variant as u8 != 0
    }
}
impl UDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDR_A {
        match self.bits {
            false => UDR_A::NoUnderrun,
            true => UDR_A::Underrun,
        }
    }
    ///Checks if the value of the field is `NoUnderrun`
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDR_A::NoUnderrun
    }
    ///Checks if the value of the field is `Underrun`
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDR_A::Underrun
    }
}
///Field `OVR` reader - Overrun
pub type OVR_R = crate::BitReader<OVR_A>;
///Overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_A {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NoOverrun,
            true => OVR_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::Overrun
    }
}
///Field `CRCE` reader - CRC Error
pub type CRCE_R = crate::BitReader<CRCE_A>;
///CRC Error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE_A {
    ///0: No CRC error detected
    NoError = 0,
    ///1: CRC error detected
    Error = 1,
}
impl From<CRCE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCE_A {
        match self.bits {
            false => CRCE_A::NoError,
            true => CRCE_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == CRCE_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == CRCE_A::Error
    }
}
///Field `TIFRE` reader - TI frame format error
pub type TIFRE_R = crate::BitReader<TIFRE_A>;
///TI frame format error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFRE_A {
    ///0: TI frame format error detected
    NoError = 0,
    ///1: TI frame format error detected
    Error = 1,
}
impl From<TIFRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIFRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIFRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIFRE_A {
        match self.bits {
            false => TIFRE_A::NoError,
            true => TIFRE_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TIFRE_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TIFRE_A::Error
    }
}
///Field `MODF` reader - Mode Fault
pub type MODF_R = crate::BitReader<MODF_A>;
///Mode Fault
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF_A {
    ///0: No mode fault detected
    NoFault = 0,
    ///1: Mode fault detected
    Fault = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::NoFault,
            true => MODF_A::Fault,
        }
    }
    ///Checks if the value of the field is `NoFault`
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODF_A::NoFault
    }
    ///Checks if the value of the field is `Fault`
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODF_A::Fault
    }
}
///Field `TSERF` reader - Additional number of SPI data to be transacted was reload
pub type TSERF_R = crate::BitReader<TSERF_A>;
///Additional number of SPI data to be transacted was reload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSERF_A {
    ///0: Additional number of SPI data to be transacted not yet loaded
    NotLoaded = 0,
    ///1: Additional number of SPI data to be transacted was reloaded
    Loaded = 1,
}
impl From<TSERF_A> for bool {
    #[inline(always)]
    fn from(variant: TSERF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSERF_A {
        match self.bits {
            false => TSERF_A::NotLoaded,
            true => TSERF_A::Loaded,
        }
    }
    ///Checks if the value of the field is `NotLoaded`
    #[inline(always)]
    pub fn is_not_loaded(&self) -> bool {
        *self == TSERF_A::NotLoaded
    }
    ///Checks if the value of the field is `Loaded`
    #[inline(always)]
    pub fn is_loaded(&self) -> bool {
        *self == TSERF_A::Loaded
    }
}
///Field `SUSP` reader - SUSPend
pub type SUSP_R = crate::BitReader<SUSP_A>;
///SUSPend
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    ///0: Master not suspended
    NotSuspended = 0,
    ///1: Master suspended
    Suspended = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::NotSuspended,
            true => SUSP_A::Suspended,
        }
    }
    ///Checks if the value of the field is `NotSuspended`
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP_A::NotSuspended
    }
    ///Checks if the value of the field is `Suspended`
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP_A::Suspended
    }
}
///Field `TXC` reader - TxFIFO transmission complete
pub type TXC_R = crate::BitReader<TXC_A>;
///TxFIFO transmission complete
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXC_A {
    ///0: Transmission ongoing
    Ongoing = 0,
    ///1: Transmission completed
    Completed = 1,
}
impl From<TXC_A> for bool {
    #[inline(always)]
    fn from(variant: TXC_A) -> Self {
        variant as u8 != 0
    }
}
impl TXC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXC_A {
        match self.bits {
            false => TXC_A::Ongoing,
            true => TXC_A::Completed,
        }
    }
    ///Checks if the value of the field is `Ongoing`
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == TXC_A::Ongoing
    }
    ///Checks if the value of the field is `Completed`
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXC_A::Completed
    }
}
///Field `RXPLVL` reader - RxFIFO Packing LeVeL
pub type RXPLVL_R = crate::FieldReader<u8, RXPLVL_A>;
///RxFIFO Packing LeVeL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPLVL_A {
    ///0: Zero frames beyond packing ratio available
    ZeroFrames = 0,
    ///1: One frame beyond packing ratio available
    OneFrame = 1,
    ///2: Two frame beyond packing ratio available
    TwoFrames = 2,
    ///3: Three frame beyond packing ratio available
    ThreeFrames = 3,
}
impl From<RXPLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPLVL_A) -> Self {
        variant as _
    }
}
impl RXPLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXPLVL_A {
        match self.bits {
            0 => RXPLVL_A::ZeroFrames,
            1 => RXPLVL_A::OneFrame,
            2 => RXPLVL_A::TwoFrames,
            3 => RXPLVL_A::ThreeFrames,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `ZeroFrames`
    #[inline(always)]
    pub fn is_zero_frames(&self) -> bool {
        *self == RXPLVL_A::ZeroFrames
    }
    ///Checks if the value of the field is `OneFrame`
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == RXPLVL_A::OneFrame
    }
    ///Checks if the value of the field is `TwoFrames`
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == RXPLVL_A::TwoFrames
    }
    ///Checks if the value of the field is `ThreeFrames`
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == RXPLVL_A::ThreeFrames
    }
}
///Field `RXWNE` reader - RxFIFO Word Not Empty
pub type RXWNE_R = crate::BitReader<RXWNE_A>;
///RxFIFO Word Not Empty
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXWNE_A {
    ///0: Less than 32-bit data frame received
    LessThan32 = 0,
    ///1: At least 32-bit data frame received
    AtLeast32 = 1,
}
impl From<RXWNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXWNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXWNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXWNE_A {
        match self.bits {
            false => RXWNE_A::LessThan32,
            true => RXWNE_A::AtLeast32,
        }
    }
    ///Checks if the value of the field is `LessThan32`
    #[inline(always)]
    pub fn is_less_than32(&self) -> bool {
        *self == RXWNE_A::LessThan32
    }
    ///Checks if the value of the field is `AtLeast32`
    #[inline(always)]
    pub fn is_at_least32(&self) -> bool {
        *self == RXWNE_A::AtLeast32
    }
}
///Field `CTSIZE` reader - Number of data frames remaining in current TSIZE session
pub type CTSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bit 0 - Rx-Packet available
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx-Packet space available
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Duplex Packet
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End Of Transfer
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmission Transfer Filled
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Underrun at slave transmission mode
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC Error
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TI frame format error
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Mode Fault
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Additional number of SPI data to be transacted was reload
    #[inline(always)]
    pub fn tserf(&self) -> TSERF_R {
        TSERF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SUSPend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TxFIFO transmission complete
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - RxFIFO Packing LeVeL
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - RxFIFO Word Not Empty
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - Number of data frames remaining in current TSIZE session
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///Status Register
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
///`reset()` method sets SR to value 0x1002
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1002;
}
