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
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXNE` reader - Receive buffer not empty
pub type RXNE_R = crate::BitReader<RXNE_A>;
///Receive buffer not empty
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE_A {
    ///0: Rx buffer empty
    Empty = 0,
    ///1: Rx buffer not empty
    NotEmpty = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::Empty,
            true => RXNE_A::NotEmpty,
        }
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::Empty
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NotEmpty
    }
}
///Field `TXE` reader - Transmit buffer empty
pub type TXE_R = crate::BitReader<TXE_A>;
///Transmit buffer empty
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_A {
    ///0: Tx buffer not empty
    NotEmpty = 0,
    ///1: Tx buffer empty
    Empty = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NotEmpty,
            true => TXE_A::Empty,
        }
    }
    ///Checks if the value of the field is `NotEmpty`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NotEmpty
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::Empty
    }
}
///Field `CHSIDE` reader - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
pub type CHSIDE_R = crate::BitReader<bool>;
///Field `UDR` reader - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence. Note: This bit is not used in SPI mode.
pub type UDR_R = crate::BitReader<bool>;
///Field `CRCERR` reader - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
pub type CRCERR_R = crate::BitReader<CRCERRR_A>;
///CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRR_A {
    ///0: CRC value received matches the SPIx_RXCRCR value
    Match = 0,
    ///1: CRC value received does not match the SPIx_RXCRCR value
    NoMatch = 1,
}
impl From<CRCERRR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCERRR_A {
        match self.bits {
            false => CRCERRR_A::Match,
            true => CRCERRR_A::NoMatch,
        }
    }
    ///Checks if the value of the field is `Match`
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CRCERRR_A::Match
    }
    ///Checks if the value of the field is `NoMatch`
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERRR_A::NoMatch
    }
}
///CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CRCERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCERR` writer - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
pub type CRCERR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, CRCERRW_AW, O>;
impl<'a, const O: u8> CRCERR_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRCERRW_AW::Clear)
    }
}
///Field `MODF` reader - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on pageÂ 1031 for the software sequence. Note: This bit is not used in I2S mode.
pub type MODF_R = crate::BitReader<MODFR_A>;
///Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on pageÂ 1031 for the software sequence. Note: This bit is not used in I2S mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFR_A {
    ///0: No mode fault occurred
    NoFault = 0,
    ///1: Mode fault occurred
    Fault = 1,
}
impl From<MODFR_A> for bool {
    #[inline(always)]
    fn from(variant: MODFR_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODFR_A {
        match self.bits {
            false => MODFR_A::NoFault,
            true => MODFR_A::Fault,
        }
    }
    ///Checks if the value of the field is `NoFault`
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODFR_A::NoFault
    }
    ///Checks if the value of the field is `Fault`
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODFR_A::Fault
    }
}
///Field `OVR` reader - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence.
pub type OVR_R = crate::BitReader<OVRR_A>;
///Overrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR_A {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRR_A {
        match self.bits {
            false => OVRR_A::NoOverrun,
            true => OVRR_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR_A::Overrun
    }
}
///Field `BSY` reader - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .
pub type BSY_R = crate::BitReader<BSYR_A>;
///Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR_A {
    ///0: SPI not busy
    NotBusy = 0,
    ///1: SPI busy
    Busy = 1,
}
impl From<BSYR_A> for bool {
    #[inline(always)]
    fn from(variant: BSYR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BSYR_A {
        match self.bits {
            false => BSYR_A::NotBusy,
            true => BSYR_A::Busy,
        }
    }
    ///Checks if the value of the field is `NotBusy`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR_A::NotBusy
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR_A::Busy
    }
}
///Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software.
pub type FRE_R = crate::BitReader<FRER_A>;
///Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRER_A {
    ///0: No frame format error
    NoError = 0,
    ///1: A frame format error occurred
    Error = 1,
}
impl From<FRER_A> for bool {
    #[inline(always)]
    fn from(variant: FRER_A) -> Self {
        variant as u8 != 0
    }
}
impl FRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRER_A {
        match self.bits {
            false => FRER_A::NoError,
            true => FRER_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FRER_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FRER_A::Error
    }
}
///Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in IÂ²S mode and in SPI receive-only mode while CRC calculation is enabled.
pub type FRLVL_R = crate::FieldReader<u8, FRLVLR_A>;
///FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in IÂ²S mode and in SPI receive-only mode while CRC calculation is enabled.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRLVLR_A {
    ///0: Rx FIFO Empty
    Empty = 0,
    ///1: Rx 1/4 FIFO
    Quarter = 1,
    ///2: Rx 1/2 FIFO
    Half = 2,
    ///3: Rx FIFO full
    Full = 3,
}
impl From<FRLVLR_A> for u8 {
    #[inline(always)]
    fn from(variant: FRLVLR_A) -> Self {
        variant as _
    }
}
impl FRLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRLVLR_A {
        match self.bits {
            0 => FRLVLR_A::Empty,
            1 => FRLVLR_A::Quarter,
            2 => FRLVLR_A::Half,
            3 => FRLVLR_A::Full,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FRLVLR_A::Empty
    }
    ///Checks if the value of the field is `Quarter`
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FRLVLR_A::Quarter
    }
    ///Checks if the value of the field is `Half`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FRLVLR_A::Half
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FRLVLR_A::Full
    }
}
///Field `FTLVL` reader - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.
pub type FTLVL_R = crate::FieldReader<u8, FTLVLR_A>;
///FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTLVLR_A {
    ///0: Tx FIFO Empty
    Empty = 0,
    ///1: Tx 1/4 FIFO
    Quarter = 1,
    ///2: Tx 1/2 FIFO
    Half = 2,
    ///3: Tx FIFO full
    Full = 3,
}
impl From<FTLVLR_A> for u8 {
    #[inline(always)]
    fn from(variant: FTLVLR_A) -> Self {
        variant as _
    }
}
impl FTLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FTLVLR_A {
        match self.bits {
            0 => FTLVLR_A::Empty,
            1 => FTLVLR_A::Quarter,
            2 => FTLVLR_A::Half,
            3 => FTLVLR_A::Full,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FTLVLR_A::Empty
    }
    ///Checks if the value of the field is `Quarter`
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FTLVLR_A::Quarter
    }
    ///Checks if the value of the field is `Half`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FTLVLR_A::Half
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTLVLR_A::Full
    }
}
impl R {
    ///Bit 0 - Receive buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence. Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on pageÂ 1031 for the software sequence. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software.
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in IÂ²S mode and in SPI receive-only mode while CRC calculation is enabled.
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<4> {
        CRCERR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPI status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
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
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x10;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0x02
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
