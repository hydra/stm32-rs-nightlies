///Register `CFG1` reader
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFG1` writer
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DSIZE` reader - Number of bits in at single SPI data frame
pub type DSIZE_R = crate::FieldReader<u8, u8>;
///Field `DSIZE` writer - Number of bits in at single SPI data frame
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
///Field `FTHLV` reader - threshold level
pub type FTHLV_R = crate::FieldReader<u8, FTHLV_A>;
///threshold level
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTHLV_A {
    ///0: 1 frame
    OneFrame = 0,
    ///1: 2 frames
    TwoFrames = 1,
    ///2: 3 frames
    ThreeFrames = 2,
    ///3: 4 frames
    FourFrames = 3,
    ///4: 5 frames
    FiveFrames = 4,
    ///5: 6 frames
    SixFrames = 5,
    ///6: 7 frames
    SevenFrames = 6,
    ///7: 8 frames
    EightFrames = 7,
    ///8: 9 frames
    NineFrames = 8,
    ///9: 10 frames
    TenFrames = 9,
    ///10: 11 frames
    ElevenFrames = 10,
    ///11: 12 frames
    TwelveFrames = 11,
    ///12: 13 frames
    ThirteenFrames = 12,
    ///13: 14 frames
    FourteenFrames = 13,
    ///14: 15 frames
    FifteenFrames = 14,
    ///15: 16 frames
    SixteenFrames = 15,
}
impl From<FTHLV_A> for u8 {
    #[inline(always)]
    fn from(variant: FTHLV_A) -> Self {
        variant as _
    }
}
impl FTHLV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FTHLV_A {
        match self.bits {
            0 => FTHLV_A::OneFrame,
            1 => FTHLV_A::TwoFrames,
            2 => FTHLV_A::ThreeFrames,
            3 => FTHLV_A::FourFrames,
            4 => FTHLV_A::FiveFrames,
            5 => FTHLV_A::SixFrames,
            6 => FTHLV_A::SevenFrames,
            7 => FTHLV_A::EightFrames,
            8 => FTHLV_A::NineFrames,
            9 => FTHLV_A::TenFrames,
            10 => FTHLV_A::ElevenFrames,
            11 => FTHLV_A::TwelveFrames,
            12 => FTHLV_A::ThirteenFrames,
            13 => FTHLV_A::FourteenFrames,
            14 => FTHLV_A::FifteenFrames,
            15 => FTHLV_A::SixteenFrames,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `OneFrame`
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == FTHLV_A::OneFrame
    }
    ///Checks if the value of the field is `TwoFrames`
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == FTHLV_A::TwoFrames
    }
    ///Checks if the value of the field is `ThreeFrames`
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == FTHLV_A::ThreeFrames
    }
    ///Checks if the value of the field is `FourFrames`
    #[inline(always)]
    pub fn is_four_frames(&self) -> bool {
        *self == FTHLV_A::FourFrames
    }
    ///Checks if the value of the field is `FiveFrames`
    #[inline(always)]
    pub fn is_five_frames(&self) -> bool {
        *self == FTHLV_A::FiveFrames
    }
    ///Checks if the value of the field is `SixFrames`
    #[inline(always)]
    pub fn is_six_frames(&self) -> bool {
        *self == FTHLV_A::SixFrames
    }
    ///Checks if the value of the field is `SevenFrames`
    #[inline(always)]
    pub fn is_seven_frames(&self) -> bool {
        *self == FTHLV_A::SevenFrames
    }
    ///Checks if the value of the field is `EightFrames`
    #[inline(always)]
    pub fn is_eight_frames(&self) -> bool {
        *self == FTHLV_A::EightFrames
    }
    ///Checks if the value of the field is `NineFrames`
    #[inline(always)]
    pub fn is_nine_frames(&self) -> bool {
        *self == FTHLV_A::NineFrames
    }
    ///Checks if the value of the field is `TenFrames`
    #[inline(always)]
    pub fn is_ten_frames(&self) -> bool {
        *self == FTHLV_A::TenFrames
    }
    ///Checks if the value of the field is `ElevenFrames`
    #[inline(always)]
    pub fn is_eleven_frames(&self) -> bool {
        *self == FTHLV_A::ElevenFrames
    }
    ///Checks if the value of the field is `TwelveFrames`
    #[inline(always)]
    pub fn is_twelve_frames(&self) -> bool {
        *self == FTHLV_A::TwelveFrames
    }
    ///Checks if the value of the field is `ThirteenFrames`
    #[inline(always)]
    pub fn is_thirteen_frames(&self) -> bool {
        *self == FTHLV_A::ThirteenFrames
    }
    ///Checks if the value of the field is `FourteenFrames`
    #[inline(always)]
    pub fn is_fourteen_frames(&self) -> bool {
        *self == FTHLV_A::FourteenFrames
    }
    ///Checks if the value of the field is `FifteenFrames`
    #[inline(always)]
    pub fn is_fifteen_frames(&self) -> bool {
        *self == FTHLV_A::FifteenFrames
    }
    ///Checks if the value of the field is `SixteenFrames`
    #[inline(always)]
    pub fn is_sixteen_frames(&self) -> bool {
        *self == FTHLV_A::SixteenFrames
    }
}
///Field `FTHLV` writer - threshold level
pub type FTHLV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, FTHLV_A, 4, O>;
impl<'a, const O: u8> FTHLV_W<'a, O> {
    ///1 frame
    #[inline(always)]
    pub fn one_frame(self) -> &'a mut W {
        self.variant(FTHLV_A::OneFrame)
    }
    ///2 frames
    #[inline(always)]
    pub fn two_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TwoFrames)
    }
    ///3 frames
    #[inline(always)]
    pub fn three_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::ThreeFrames)
    }
    ///4 frames
    #[inline(always)]
    pub fn four_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FourFrames)
    }
    ///5 frames
    #[inline(always)]
    pub fn five_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FiveFrames)
    }
    ///6 frames
    #[inline(always)]
    pub fn six_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SixFrames)
    }
    ///7 frames
    #[inline(always)]
    pub fn seven_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SevenFrames)
    }
    ///8 frames
    #[inline(always)]
    pub fn eight_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::EightFrames)
    }
    ///9 frames
    #[inline(always)]
    pub fn nine_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::NineFrames)
    }
    ///10 frames
    #[inline(always)]
    pub fn ten_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TenFrames)
    }
    ///11 frames
    #[inline(always)]
    pub fn eleven_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::ElevenFrames)
    }
    ///12 frames
    #[inline(always)]
    pub fn twelve_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TwelveFrames)
    }
    ///13 frames
    #[inline(always)]
    pub fn thirteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::ThirteenFrames)
    }
    ///14 frames
    #[inline(always)]
    pub fn fourteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FourteenFrames)
    }
    ///15 frames
    #[inline(always)]
    pub fn fifteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FifteenFrames)
    }
    ///16 frames
    #[inline(always)]
    pub fn sixteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SixteenFrames)
    }
}
///Field `UDRCFG` reader - Behavior of slave transmitter at underrun condition
pub type UDRCFG_R = crate::FieldReader<u8, UDRCFG_A>;
///Behavior of slave transmitter at underrun condition
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDRCFG_A {
    ///0: Slave sends a constant underrun pattern
    Constant = 0,
    ///1: Slave repeats last received data frame from master
    RepeatReceived = 1,
    ///2: Slave repeats last transmitted data frame
    RepeatTransmitted = 2,
}
impl From<UDRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: UDRCFG_A) -> Self {
        variant as _
    }
}
impl UDRCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<UDRCFG_A> {
        match self.bits {
            0 => Some(UDRCFG_A::Constant),
            1 => Some(UDRCFG_A::RepeatReceived),
            2 => Some(UDRCFG_A::RepeatTransmitted),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Constant`
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == UDRCFG_A::Constant
    }
    ///Checks if the value of the field is `RepeatReceived`
    #[inline(always)]
    pub fn is_repeat_received(&self) -> bool {
        *self == UDRCFG_A::RepeatReceived
    }
    ///Checks if the value of the field is `RepeatTransmitted`
    #[inline(always)]
    pub fn is_repeat_transmitted(&self) -> bool {
        *self == UDRCFG_A::RepeatTransmitted
    }
}
///Field `UDRCFG` writer - Behavior of slave transmitter at underrun condition
pub type UDRCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, UDRCFG_A, 2, O>;
impl<'a, const O: u8> UDRCFG_W<'a, O> {
    ///Slave sends a constant underrun pattern
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(UDRCFG_A::Constant)
    }
    ///Slave repeats last received data frame from master
    #[inline(always)]
    pub fn repeat_received(self) -> &'a mut W {
        self.variant(UDRCFG_A::RepeatReceived)
    }
    ///Slave repeats last transmitted data frame
    #[inline(always)]
    pub fn repeat_transmitted(self) -> &'a mut W {
        self.variant(UDRCFG_A::RepeatTransmitted)
    }
}
///Field `UDRDET` reader - Detection of underrun condition at slave transmitter
pub type UDRDET_R = crate::FieldReader<u8, UDRDET_A>;
///Detection of underrun condition at slave transmitter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDRDET_A {
    ///0: Underrun is detected at begin of data frame
    StartOfFrame = 0,
    ///1: Underrun is detected at end of last data frame
    EndOfFrame = 1,
    ///2: Underrun is detected at begin of active SS signal
    StartOfSlaveSelect = 2,
}
impl From<UDRDET_A> for u8 {
    #[inline(always)]
    fn from(variant: UDRDET_A) -> Self {
        variant as _
    }
}
impl UDRDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<UDRDET_A> {
        match self.bits {
            0 => Some(UDRDET_A::StartOfFrame),
            1 => Some(UDRDET_A::EndOfFrame),
            2 => Some(UDRDET_A::StartOfSlaveSelect),
            _ => None,
        }
    }
    ///Checks if the value of the field is `StartOfFrame`
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == UDRDET_A::StartOfFrame
    }
    ///Checks if the value of the field is `EndOfFrame`
    #[inline(always)]
    pub fn is_end_of_frame(&self) -> bool {
        *self == UDRDET_A::EndOfFrame
    }
    ///Checks if the value of the field is `StartOfSlaveSelect`
    #[inline(always)]
    pub fn is_start_of_slave_select(&self) -> bool {
        *self == UDRDET_A::StartOfSlaveSelect
    }
}
///Field `UDRDET` writer - Detection of underrun condition at slave transmitter
pub type UDRDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, UDRDET_A, 2, O>;
impl<'a, const O: u8> UDRDET_W<'a, O> {
    ///Underrun is detected at begin of data frame
    #[inline(always)]
    pub fn start_of_frame(self) -> &'a mut W {
        self.variant(UDRDET_A::StartOfFrame)
    }
    ///Underrun is detected at end of last data frame
    #[inline(always)]
    pub fn end_of_frame(self) -> &'a mut W {
        self.variant(UDRDET_A::EndOfFrame)
    }
    ///Underrun is detected at begin of active SS signal
    #[inline(always)]
    pub fn start_of_slave_select(self) -> &'a mut W {
        self.variant(UDRDET_A::StartOfSlaveSelect)
    }
}
///Field `RXDMAEN` reader - Rx DMA stream enable
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
///Rx DMA stream enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    ///0: Rx buffer DMA disabled
    Disabled = 0,
    ///1: Rx buffer DMA enabled
    Enabled = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::Disabled,
            true => RXDMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::Enabled
    }
}
///Field `RXDMAEN` writer - Rx DMA stream enable
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, RXDMAEN_A, O>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Disabled)
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Enabled)
    }
}
///Field `TXDMAEN` reader - Tx DMA stream enable
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
///Tx DMA stream enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    ///0: Tx buffer DMA disabled
    Disabled = 0,
    ///1: Tx buffer DMA enabled
    Enabled = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::Disabled,
            true => TXDMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::Enabled
    }
}
///Field `TXDMAEN` writer - Tx DMA stream enable
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, TXDMAEN_A, O>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Disabled)
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Enabled)
    }
}
///Field `CRCSIZE` reader - Length of CRC frame to be transacted and compared
pub type CRCSIZE_R = crate::FieldReader<u8, u8>;
///Field `CRCSIZE` writer - Length of CRC frame to be transacted and compared
pub type CRCSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
///Field `CRCEN` reader - Hardware CRC computation enable
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
///Hardware CRC computation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    ///0: CRC calculation disabled
    Disabled = 0,
    ///1: CRC calculation enabled
    Enabled = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::Disabled,
            true => CRCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::Enabled
    }
}
///Field `CRCEN` writer - Hardware CRC computation enable
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, CRCEN_A, O>;
impl<'a, const O: u8> CRCEN_W<'a, O> {
    ///CRC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Disabled)
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Enabled)
    }
}
///Field `MBR` reader - Master baud rate
pub type MBR_R = crate::FieldReader<u8, MBR_A>;
///Master baud rate
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBR_A {
    ///0: f_spi_ker_ck / 2
    Div2 = 0,
    ///1: f_spi_ker_ck / 4
    Div4 = 1,
    ///2: f_spi_ker_ck / 8
    Div8 = 2,
    ///3: f_spi_ker_ck / 16
    Div16 = 3,
    ///4: f_spi_ker_ck / 32
    Div32 = 4,
    ///5: f_spi_ker_ck / 64
    Div64 = 5,
    ///6: f_spi_ker_ck / 128
    Div128 = 6,
    ///7: f_spi_ker_ck / 256
    Div256 = 7,
}
impl From<MBR_A> for u8 {
    #[inline(always)]
    fn from(variant: MBR_A) -> Self {
        variant as _
    }
}
impl MBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MBR_A {
        match self.bits {
            0 => MBR_A::Div2,
            1 => MBR_A::Div4,
            2 => MBR_A::Div8,
            3 => MBR_A::Div16,
            4 => MBR_A::Div32,
            5 => MBR_A::Div64,
            6 => MBR_A::Div128,
            7 => MBR_A::Div256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MBR_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MBR_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MBR_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MBR_A::Div16
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MBR_A::Div32
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MBR_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MBR_A::Div128
    }
    ///Checks if the value of the field is `Div256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == MBR_A::Div256
    }
}
///Field `MBR` writer - Master baud rate
pub type MBR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, MBR_A, 3, O>;
impl<'a, const O: u8> MBR_W<'a, O> {
    ///f_spi_ker_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MBR_A::Div2)
    }
    ///f_spi_ker_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MBR_A::Div4)
    }
    ///f_spi_ker_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MBR_A::Div8)
    }
    ///f_spi_ker_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MBR_A::Div16)
    }
    ///f_spi_ker_ck / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(MBR_A::Div32)
    }
    ///f_spi_ker_ck / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(MBR_A::Div64)
    }
    ///f_spi_ker_ck / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(MBR_A::Div128)
    }
    ///f_spi_ker_ck / 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(MBR_A::Div256)
    }
}
impl R {
    ///Bits 0:4 - Number of bits in at single SPI data frame
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:8 - threshold level
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:10 - Behavior of slave transmitter at underrun condition
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - Detection of underrun condition at slave transmitter
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 14 - Rx DMA stream enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx DMA stream enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - Length of CRC frame to be transacted and compared
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 22 - Hardware CRC computation enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 28:30 - Master baud rate
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:4 - Number of bits in at single SPI data frame
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<0> {
        DSIZE_W::new(self)
    }
    ///Bits 5:8 - threshold level
    #[inline(always)]
    #[must_use]
    pub fn fthlv(&mut self) -> FTHLV_W<5> {
        FTHLV_W::new(self)
    }
    ///Bits 9:10 - Behavior of slave transmitter at underrun condition
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<9> {
        UDRCFG_W::new(self)
    }
    ///Bits 11:12 - Detection of underrun condition at slave transmitter
    #[inline(always)]
    #[must_use]
    pub fn udrdet(&mut self) -> UDRDET_W<11> {
        UDRDET_W::new(self)
    }
    ///Bit 14 - Rx DMA stream enable
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<14> {
        RXDMAEN_W::new(self)
    }
    ///Bit 15 - Tx DMA stream enable
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<15> {
        TXDMAEN_W::new(self)
    }
    ///Bits 16:20 - Length of CRC frame to be transacted and compared
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<16> {
        CRCSIZE_W::new(self)
    }
    ///Bit 22 - Hardware CRC computation enable
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<22> {
        CRCEN_W::new(self)
    }
    ///Bits 28:30 - Master baud rate
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<28> {
        MBR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfg1](index.html) module
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfg1::R](R) reader structure
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfg1::W](W) writer structure
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFG1 to value 0x0007_0007
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
