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
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader<u8, u8>;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, u8, 4, O>;
///Field `LBDL` reader - lin break detection length
pub type LBDL_R = crate::BitReader<LBDL_A>;
///lin break detection length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDL_A {
    ///0: 10-bit break detection
    Lbdl10 = 0,
    ///1: 11-bit break detection
    Lbdl11 = 1,
}
impl From<LBDL_A> for bool {
    #[inline(always)]
    fn from(variant: LBDL_A) -> Self {
        variant as u8 != 0
    }
}
impl LBDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBDL_A {
        match self.bits {
            false => LBDL_A::Lbdl10,
            true => LBDL_A::Lbdl11,
        }
    }
    ///Checks if the value of the field is `Lbdl10`
    #[inline(always)]
    pub fn is_lbdl10(&self) -> bool {
        *self == LBDL_A::Lbdl10
    }
    ///Checks if the value of the field is `Lbdl11`
    #[inline(always)]
    pub fn is_lbdl11(&self) -> bool {
        *self == LBDL_A::Lbdl11
    }
}
///Field `LBDL` writer - lin break detection length
pub type LBDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LBDL_A, O>;
impl<'a, const O: u8> LBDL_W<'a, O> {
    ///10-bit break detection
    #[inline(always)]
    pub fn lbdl10(self) -> &'a mut W {
        self.variant(LBDL_A::Lbdl10)
    }
    ///11-bit break detection
    #[inline(always)]
    pub fn lbdl11(self) -> &'a mut W {
        self.variant(LBDL_A::Lbdl11)
    }
}
///Field `LBDIE` reader - LIN break detection interrupt enable
pub type LBDIE_R = crate::BitReader<LBDIE_A>;
///LIN break detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDIE_A {
    ///0: LIN break detection interrupt disabled
    Disabled = 0,
    ///1: LIN break detection interrupt enabled
    Enabled = 1,
}
impl From<LBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LBDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBDIE_A {
        match self.bits {
            false => LBDIE_A::Disabled,
            true => LBDIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE_A::Enabled
    }
}
///Field `LBDIE` writer - LIN break detection interrupt enable
pub type LBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LBDIE_A, O>;
impl<'a, const O: u8> LBDIE_W<'a, O> {
    ///LIN break detection interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBDIE_A::Disabled)
    }
    ///LIN break detection interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBDIE_A::Enabled)
    }
}
///Field `LBCL` reader - Last bit clock pulse
pub type LBCL_R = crate::BitReader<bool>;
///Field `LBCL` writer - Last bit clock pulse
pub type LBCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `CPHA` reader - Clock phase
pub type CPHA_R = crate::BitReader<CPHA_A>;
///Clock phase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    ///0: The first clock transition is the first data capture edge
    First = 0,
    ///1: The second clock transition is the first data capture edge
    Second = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::First,
            true => CPHA_A::Second,
        }
    }
    ///Checks if the value of the field is `First`
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPHA_A::First
    }
    ///Checks if the value of the field is `Second`
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPHA_A::Second
    }
}
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn first(self) -> &'a mut W {
        self.variant(CPHA_A::First)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(CPHA_A::Second)
    }
}
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader<CPOL_A>;
///Clock polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    ///0: Steady low value on CK pin outside transmission window
    Low = 0,
    ///1: Steady high value on CK pin outside transmission window
    High = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::Low,
            true => CPOL_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL_A::High
    }
}
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    ///Steady low value on CK pin outside transmission window
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::Low)
    }
    ///Steady high value on CK pin outside transmission window
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::High)
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
    ///1: 0.5 stop bits
    Stop0p5 = 1,
    ///2: 2 stop bits
    Stop2 = 2,
    ///3: 1.5 stop bits
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
    ///0.5 stop bits
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STOP_A::Stop0p5)
    }
    ///2 stop bits
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOP_A::Stop2)
    }
    ///1.5 stop bits
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STOP_A::Stop1p5)
    }
}
///Field `LINEN` reader - LIN mode enable
pub type LINEN_R = crate::BitReader<LINEN_A>;
///LIN mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEN_A {
    ///0: LIN mode disabled
    Disabled = 0,
    ///1: LIN mode enabled
    Enabled = 1,
}
impl From<LINEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LINEN_A {
        match self.bits {
            false => LINEN_A::Disabled,
            true => LINEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINEN_A::Enabled
    }
}
///Field `LINEN` writer - LIN mode enable
pub type LINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LINEN_A, O>;
impl<'a, const O: u8> LINEN_W<'a, O> {
    ///LIN mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINEN_A::Disabled)
    }
    ///LIN mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINEN_A::Enabled)
    }
}
impl R {
    ///Bits 0:3 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - lin break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
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
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Address of the USART node
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<0> {
        ADD_W::new(self)
    }
    ///Bit 5 - lin break detection length
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LBDL_W<5> {
        LBDL_W::new(self)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<6> {
        LBDIE_W::new(self)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LBCL_W<8> {
        LBCL_W::new(self)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<9> {
        CPHA_W::new(self)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<10> {
        CPOL_W::new(self)
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
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<14> {
        LINEN_W::new(self)
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
