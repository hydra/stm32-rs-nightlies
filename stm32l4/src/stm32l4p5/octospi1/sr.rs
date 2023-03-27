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
///Field `TEF` reader - Transfer error flag
pub type TEF_R = crate::BitReader<TEF_A>;
///Transfer error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEF_A {
    ///0: This bit is cleared by writing 1 to CTEF
    Cleared = 0,
    ///1: This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode
    InvalidAddressAccessed = 1,
}
impl From<TEF_A> for bool {
    #[inline(always)]
    fn from(variant: TEF_A) -> Self {
        variant as u8 != 0
    }
}
impl TEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEF_A {
        match self.bits {
            false => TEF_A::Cleared,
            true => TEF_A::InvalidAddressAccessed,
        }
    }
    ///Checks if the value of the field is `Cleared`
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TEF_A::Cleared
    }
    ///Checks if the value of the field is `InvalidAddressAccessed`
    #[inline(always)]
    pub fn is_invalid_address_accessed(&self) -> bool {
        *self == TEF_A::InvalidAddressAccessed
    }
}
///Field `TEF` writer - Transfer error flag
pub type TEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TEF_A, O>;
impl<'a, const O: u8> TEF_W<'a, O> {
    ///This bit is cleared by writing 1 to CTEF
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(TEF_A::Cleared)
    }
    ///This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode
    #[inline(always)]
    pub fn invalid_address_accessed(self) -> &'a mut W {
        self.variant(TEF_A::InvalidAddressAccessed)
    }
}
///Field `TCF` reader - Transfer complete flag
pub type TCF_R = crate::BitReader<TCF_A>;
///Transfer complete flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    ///0: This bit is cleared by writing 1 to CTCF
    Cleared = 0,
    ///1: This bit is set when the programmed number of data has been transferred
    TransferCompleted = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::Cleared,
            true => TCF_A::TransferCompleted,
        }
    }
    ///Checks if the value of the field is `Cleared`
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TCF_A::Cleared
    }
    ///Checks if the value of the field is `TransferCompleted`
    #[inline(always)]
    pub fn is_transfer_completed(&self) -> bool {
        *self == TCF_A::TransferCompleted
    }
}
///Field `TCF` writer - Transfer complete flag
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TCF_A, O>;
impl<'a, const O: u8> TCF_W<'a, O> {
    ///This bit is cleared by writing 1 to CTCF
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(TCF_A::Cleared)
    }
    ///This bit is set when the programmed number of data has been transferred
    #[inline(always)]
    pub fn transfer_completed(self) -> &'a mut W {
        self.variant(TCF_A::TransferCompleted)
    }
}
///Field `FTF` reader - FIFO threshold flag
pub type FTF_R = crate::BitReader<FTF_A>;
///FIFO threshold flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF_A {
    ///0: It is cleared automatically as soon as the threshold condition is no longer true
    Cleared = 0,
    ///1: This bit is set when the FIFO threshold has been reached
    ThresholdReached = 1,
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
    pub fn variant(&self) -> FTF_A {
        match self.bits {
            false => FTF_A::Cleared,
            true => FTF_A::ThresholdReached,
        }
    }
    ///Checks if the value of the field is `Cleared`
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == FTF_A::Cleared
    }
    ///Checks if the value of the field is `ThresholdReached`
    #[inline(always)]
    pub fn is_threshold_reached(&self) -> bool {
        *self == FTF_A::ThresholdReached
    }
}
///Field `FTF` writer - FIFO threshold flag
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, FTF_A, O>;
impl<'a, const O: u8> FTF_W<'a, O> {
    ///It is cleared automatically as soon as the threshold condition is no longer true
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(FTF_A::Cleared)
    }
    ///This bit is set when the FIFO threshold has been reached
    #[inline(always)]
    pub fn threshold_reached(self) -> &'a mut W {
        self.variant(FTF_A::ThresholdReached)
    }
}
///Field `SMF` reader - Status match flag
pub type SMF_R = crate::BitReader<SMF_A>;
///Status match flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMF_A {
    ///0: It is cleared by writing 1 to CSMF
    Cleared = 0,
    ///1: This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)
    Matched = 1,
}
impl From<SMF_A> for bool {
    #[inline(always)]
    fn from(variant: SMF_A) -> Self {
        variant as u8 != 0
    }
}
impl SMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMF_A {
        match self.bits {
            false => SMF_A::Cleared,
            true => SMF_A::Matched,
        }
    }
    ///Checks if the value of the field is `Cleared`
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SMF_A::Cleared
    }
    ///Checks if the value of the field is `Matched`
    #[inline(always)]
    pub fn is_matched(&self) -> bool {
        *self == SMF_A::Matched
    }
}
///Field `SMF` writer - Status match flag
pub type SMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, SMF_A, O>;
impl<'a, const O: u8> SMF_W<'a, O> {
    ///It is cleared by writing 1 to CSMF
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(SMF_A::Cleared)
    }
    ///This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)
    #[inline(always)]
    pub fn matched(self) -> &'a mut W {
        self.variant(SMF_A::Matched)
    }
}
///Field `TOF` reader - Timeout flag
pub type TOF_R = crate::BitReader<TOF_A>;
///Timeout flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF_A {
    ///0: This bit is cleared by writing 1 to CTOF
    Cleared = 0,
    ///1: This bit is set when timeout occurs
    Timeout = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::Cleared,
            true => TOF_A::Timeout,
        }
    }
    ///Checks if the value of the field is `Cleared`
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TOF_A::Cleared
    }
    ///Checks if the value of the field is `Timeout`
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TOF_A::Timeout
    }
}
///Field `TOF` writer - Timeout flag
pub type TOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TOF_A, O>;
impl<'a, const O: u8> TOF_W<'a, O> {
    ///This bit is cleared by writing 1 to CTOF
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(TOF_A::Cleared)
    }
    ///This bit is set when timeout occurs
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(TOF_A::Timeout)
    }
}
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader<BUSY_A>;
///BUSY
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    ///0: This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty
    Cleared = 0,
    ///1: This bit is set when an operation is ongoing
    Busy = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::Cleared,
            true => BUSY_A::Busy,
        }
    }
    ///Checks if the value of the field is `Cleared`
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == BUSY_A::Cleared
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::Busy
    }
}
///Field `BUSY` writer - BUSY
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, BUSY_A, O>;
impl<'a, const O: u8> BUSY_W<'a, O> {
    ///This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(BUSY_A::Cleared)
    }
    ///This bit is set when an operation is ongoing
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(BUSY_A::Busy)
    }
}
///Field `FLEVEL` reader - FIFO level
pub type FLEVEL_R = crate::FieldReader<u8, u8>;
///Field `FLEVEL` writer - FIFO level
pub type FLEVEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:13 - FIFO level
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<0> {
        TEF_W::new(self)
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<1> {
        TCF_W::new(self)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<2> {
        FTF_W::new(self)
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    #[must_use]
    pub fn smf(&mut self) -> SMF_W<3> {
        SMF_W::new(self)
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<4> {
        TOF_W::new(self)
    }
    ///Bit 5 - BUSY
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<5> {
        BUSY_W::new(self)
    }
    ///Bits 8:13 - FIFO level
    #[inline(always)]
    #[must_use]
    pub fn flevel(&mut self) -> FLEVEL_W<8> {
        FLEVEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
