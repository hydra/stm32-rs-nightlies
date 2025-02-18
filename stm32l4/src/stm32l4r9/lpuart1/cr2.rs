///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_R = crate::BitReader<ADDM7_A>;
///7-bit Address Detection/4-bit Address Detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7_A {
    ///0: 4-bit address detection
    Bit4 = 0,
    ///1: 7-bit address detection
    Bit7 = 1,
}
impl From<ADDM7_A> for bool {
    #[inline(always)]
    fn from(variant: ADDM7_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDM7_A {
        match self.bits {
            false => ADDM7_A::Bit4,
            true => ADDM7_A::Bit7,
        }
    }
    ///Checks if the value of the field is `Bit4`
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7_A::Bit4
    }
    ///Checks if the value of the field is `Bit7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7_A::Bit7
    }
}
///Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ADDM7_A, O>;
impl<'a, const O: u8> ADDM7_W<'a, O> {
    ///4-bit address detection
    #[inline(always)]
    pub fn bit4(self) -> &'a mut W {
        self.variant(ADDM7_A::Bit4)
    }
    ///7-bit address detection
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADDM7_A::Bit7)
    }
}
///Field `CLKEN` reader - Clock enable
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
///Clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    ///0: CK pin disabled
    Disabled = 0,
    ///1: CK pin enabled
    Enabled = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::Disabled,
            true => CLKEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::Enabled
    }
}
///Field `CLKEN` writer - Clock enable
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    ///CK pin disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Disabled)
    }
    ///CK pin enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Enabled)
    }
}
///Field `STOP` reader - STOP bits
pub type STOP_R = crate::FieldReader<u8, STOP_A>;
///STOP bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP_A {
    ///0: 1 stop bit
    Stop1 = 0,
    ///1: 0.5 stop bit
    Stop0p5 = 1,
    ///2: 2 stop bit
    Stop2 = 2,
    ///3: 1.5 stop bit
    Stop1p5 = 3,
}
impl From<STOP_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as _
    }
}
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            0 => STOP_A::Stop1,
            1 => STOP_A::Stop0p5,
            2 => STOP_A::Stop2,
            3 => STOP_A::Stop1p5,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Stop1`
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP_A::Stop1
    }
    ///Checks if the value of the field is `Stop0p5`
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP_A::Stop0p5
    }
    ///Checks if the value of the field is `Stop2`
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP_A::Stop2
    }
    ///Checks if the value of the field is `Stop1p5`
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP_A::Stop1p5
    }
}
///Field `STOP` writer - STOP bits
pub type STOP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, STOP_A, 2, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STOP_A::Stop1)
    }
    ///0.5 stop bit
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STOP_A::Stop0p5)
    }
    ///2 stop bit
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOP_A::Stop2)
    }
    ///1.5 stop bit
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STOP_A::Stop1p5)
    }
}
///Field `SWAP` reader - Swap TX/RX pins
pub type SWAP_R = crate::BitReader<SWAP_A>;
///Swap TX/RX pins
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_A {
    ///0: TX/RX pins are used as defined in standard pinout
    Standard = 0,
    ///1: The TX and RX pins functions are swapped
    Swapped = 1,
}
impl From<SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_A) -> Self {
        variant as u8 != 0
    }
}
impl SWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWAP_A {
        match self.bits {
            false => SWAP_A::Standard,
            true => SWAP_A::Swapped,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SWAP_A::Standard
    }
    ///Checks if the value of the field is `Swapped`
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAP_A::Swapped
    }
}
///Field `SWAP` writer - Swap TX/RX pins
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SWAP_A, O>;
impl<'a, const O: u8> SWAP_W<'a, O> {
    ///TX/RX pins are used as defined in standard pinout
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SWAP_A::Standard)
    }
    ///The TX and RX pins functions are swapped
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(SWAP_A::Swapped)
    }
}
///Field `RXINV` reader - RX pin active level inversion
pub type RXINV_R = crate::BitReader<RXINV_A>;
///RX pin active level inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV_A {
    ///0: RX pin signal works using the standard logic levels
    Standard = 0,
    ///1: RX pin signal values are inverted
    Inverted = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::Standard,
            true => RXINV_A::Inverted,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXINV_A::Standard
    }
    ///Checks if the value of the field is `Inverted`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV_A::Inverted
    }
}
///Field `RXINV` writer - RX pin active level inversion
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RXINV_A, O>;
impl<'a, const O: u8> RXINV_W<'a, O> {
    ///RX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(RXINV_A::Standard)
    }
    ///RX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXINV_A::Inverted)
    }
}
///Field `TXINV` reader - TX pin active level inversion
pub type TXINV_R = crate::BitReader<TXINV_A>;
///TX pin active level inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV_A {
    ///0: TX pin signal works using the standard logic levels
    Standard = 0,
    ///1: TX pin signal values are inverted
    Inverted = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl TXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::Standard,
            true => TXINV_A::Inverted,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXINV_A::Standard
    }
    ///Checks if the value of the field is `Inverted`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV_A::Inverted
    }
}
///Field `TXINV` writer - TX pin active level inversion
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TXINV_A, O>;
impl<'a, const O: u8> TXINV_W<'a, O> {
    ///TX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(TXINV_A::Standard)
    }
    ///TX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXINV_A::Inverted)
    }
}
///Field `DATAINV` reader - Binary data inversion
pub type DATAINV_R = crate::BitReader<DATAINV_A>;
///Binary data inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV_A {
    ///0: Logical data from the data register are send/received in positive/direct logic
    Positive = 0,
    ///1: Logical data from the data register are send/received in negative/inverse logic
    Negative = 1,
}
impl From<DATAINV_A> for bool {
    #[inline(always)]
    fn from(variant: DATAINV_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DATAINV_A {
        match self.bits {
            false => DATAINV_A::Positive,
            true => DATAINV_A::Negative,
        }
    }
    ///Checks if the value of the field is `Positive`
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATAINV_A::Positive
    }
    ///Checks if the value of the field is `Negative`
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATAINV_A::Negative
    }
}
///Field `DATAINV` writer - Binary data inversion
pub type DATAINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, DATAINV_A, O>;
impl<'a, const O: u8> DATAINV_W<'a, O> {
    ///Logical data from the data register are send/received in positive/direct logic
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DATAINV_A::Positive)
    }
    ///Logical data from the data register are send/received in negative/inverse logic
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DATAINV_A::Negative)
    }
}
///Field `MSBFIRST` reader - Most significant bit first
pub type MSBFIRST_R = crate::BitReader<MSBFIRST_A>;
///Most significant bit first
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST_A {
    ///0: data is transmitted/received with data bit 0 first, following the start bit
    Lsb = 0,
    ///1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    Msb = 1,
}
impl From<MSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSBFIRST_A {
        match self.bits {
            false => MSBFIRST_A::Lsb,
            true => MSBFIRST_A::Msb,
        }
    }
    ///Checks if the value of the field is `Lsb`
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRST_A::Lsb
    }
    ///Checks if the value of the field is `Msb`
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRST_A::Msb
    }
}
///Field `MSBFIRST` writer - Most significant bit first
pub type MSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, MSBFIRST_A, O>;
impl<'a, const O: u8> MSBFIRST_W<'a, O> {
    ///data is transmitted/received with data bit 0 first, following the start bit
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSBFIRST_A::Lsb)
    }
    ///data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSBFIRST_A::Msb)
    }
}
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader<u8, u8>;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> ADDM7_W<4> {
        ADDM7_W::new(self)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<11> {
        CLKEN_W::new(self)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<12> {
        STOP_W::new(self)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<15> {
        SWAP_W::new(self)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<16> {
        RXINV_W::new(self)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<17> {
        TXINV_W::new(self)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    #[must_use]
    pub fn datainv(&mut self) -> DATAINV_W<18> {
        DATAINV_W::new(self)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<19> {
        MSBFIRST_W::new(self)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<24> {
        ADD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
