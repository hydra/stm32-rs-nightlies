///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXPIE` reader - RXP Interrupt Enable
pub type RXPIE_R = crate::BitReader<RXPIE_A>;
///RXP Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPIE_A {
    ///0: RX data available interrupt masked
    Masked = 0,
    ///1: RX data available interrupt not masked
    NotMasked = 1,
}
impl From<RXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXPIE_A {
        match self.bits {
            false => RXPIE_A::Masked,
            true => RXPIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXPIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXPIE_A::NotMasked
    }
}
///Field `RXPIE` writer - RXP Interrupt Enable
pub type RXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RXPIE_A, O>;
impl<'a, const O: u8> RXPIE_W<'a, O> {
    ///RX data available interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXPIE_A::Masked)
    }
    ///RX data available interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXPIE_A::NotMasked)
    }
}
///Field `TXPIE` reader - TXP interrupt enable
pub type TXPIE_R = crate::BitReader<TXPIE_A>;
///TXP interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPIE_A {
    ///0: TX space available interrupt masked
    Masked = 0,
    ///1: TX space available interrupt not masked
    NotMasked = 1,
}
impl From<TXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXPIE_A {
        match self.bits {
            false => TXPIE_A::Masked,
            true => TXPIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXPIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXPIE_A::NotMasked
    }
}
///Field `TXPIE` writer - TXP interrupt enable
pub type TXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TXPIE_A, O>;
impl<'a, const O: u8> TXPIE_W<'a, O> {
    ///TX space available interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXPIE_A::Masked)
    }
    ///TX space available interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXPIE_A::NotMasked)
    }
}
///Field `DXPIE` reader - DXP interrupt enabled
pub type DXPIE_R = crate::BitReader<DXPIE_A>;
///DXP interrupt enabled
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXPIE_A {
    ///0: Duplex transfer complete interrupt masked
    Masked = 0,
    ///1: Duplex transfer complete interrupt not masked
    NotMasked = 1,
}
impl From<DXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: DXPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DXPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DXPIE_A {
        match self.bits {
            false => DXPIE_A::Masked,
            true => DXPIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DXPIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == DXPIE_A::NotMasked
    }
}
///Field `DXPIE` writer - DXP interrupt enabled
pub type DXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, DXPIE_A, O>;
impl<'a, const O: u8> DXPIE_W<'a, O> {
    ///Duplex transfer complete interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(DXPIE_A::Masked)
    }
    ///Duplex transfer complete interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(DXPIE_A::NotMasked)
    }
}
///Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable
pub type EOTIE_R = crate::BitReader<EOTIE_A>;
///EOT, SUSP and TXC interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTIE_A {
    ///0: End-of-transfer interrupt masked
    Masked = 0,
    ///1: End-of-transfer interrupt not masked
    NotMasked = 1,
}
impl From<EOTIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOTIE_A {
        match self.bits {
            false => EOTIE_A::Masked,
            true => EOTIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EOTIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == EOTIE_A::NotMasked
    }
}
///Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable
pub type EOTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOTIE_A, O>;
impl<'a, const O: u8> EOTIE_W<'a, O> {
    ///End-of-transfer interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EOTIE_A::Masked)
    }
    ///End-of-transfer interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(EOTIE_A::NotMasked)
    }
}
///Field `TXTFIE` reader - TXTFIE interrupt enable
pub type TXTFIE_R = crate::BitReader<TXTFIE_A>;
///TXTFIE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTFIE_A {
    ///0: Transmission transfer filled interrupt masked
    Masked = 0,
    ///1: Transmission transfer filled interrupt not masked
    NotMasked = 1,
}
impl From<TXTFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXTFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXTFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXTFIE_A {
        match self.bits {
            false => TXTFIE_A::Masked,
            true => TXTFIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXTFIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXTFIE_A::NotMasked
    }
}
///Field `TXTFIE` writer - TXTFIE interrupt enable
pub type TXTFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TXTFIE_A, O>;
impl<'a, const O: u8> TXTFIE_W<'a, O> {
    ///Transmission transfer filled interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXTFIE_A::Masked)
    }
    ///Transmission transfer filled interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXTFIE_A::NotMasked)
    }
}
///Field `UDRIE` reader - UDR interrupt enable
pub type UDRIE_R = crate::BitReader<UDRIE_A>;
///UDR interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRIE_A {
    ///0: Underrun interrupt masked
    Masked = 0,
    ///1: Underrun interrupt not masked
    NotMasked = 1,
}
impl From<UDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: UDRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDRIE_A {
        match self.bits {
            false => UDRIE_A::Masked,
            true => UDRIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == UDRIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == UDRIE_A::NotMasked
    }
}
///Field `UDRIE` writer - UDR interrupt enable
pub type UDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, UDRIE_A, O>;
impl<'a, const O: u8> UDRIE_W<'a, O> {
    ///Underrun interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(UDRIE_A::Masked)
    }
    ///Underrun interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(UDRIE_A::NotMasked)
    }
}
///Field `OVRIE` reader - OVR interrupt enable
pub type OVRIE_R = crate::BitReader<OVRIE_A>;
///OVR interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE_A {
    ///0: Overrun interrupt masked
    Masked = 0,
    ///1: Overrun interrupt not masked
    NotMasked = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::Masked,
            true => OVRIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == OVRIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == OVRIE_A::NotMasked
    }
}
///Field `OVRIE` writer - OVR interrupt enable
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVRIE_A, O>;
impl<'a, const O: u8> OVRIE_W<'a, O> {
    ///Overrun interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(OVRIE_A::Masked)
    }
    ///Overrun interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(OVRIE_A::NotMasked)
    }
}
///Field `CRCEIE` reader - CRC Interrupt enable
pub type CRCEIE_R = crate::BitReader<CRCEIE_A>;
///CRC Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEIE_A {
    ///0: CRC error interrupt masked
    Masked = 0,
    ///1: CRC error interrupt not masked
    NotMasked = 1,
}
impl From<CRCEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCEIE_A {
        match self.bits {
            false => CRCEIE_A::Masked,
            true => CRCEIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CRCEIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == CRCEIE_A::NotMasked
    }
}
///Field `CRCEIE` writer - CRC Interrupt enable
pub type CRCEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CRCEIE_A, O>;
impl<'a, const O: u8> CRCEIE_W<'a, O> {
    ///CRC error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CRCEIE_A::Masked)
    }
    ///CRC error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(CRCEIE_A::NotMasked)
    }
}
///Field `TIFREIE` reader - TIFRE interrupt enable
pub type TIFREIE_R = crate::BitReader<TIFREIE_A>;
///TIFRE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFREIE_A {
    ///0: TI frame format error interrupt masked
    Masked = 0,
    ///1: TI frame format error interrupt not masked
    NotMasked = 1,
}
impl From<TIFREIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIFREIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIFREIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIFREIE_A {
        match self.bits {
            false => TIFREIE_A::Masked,
            true => TIFREIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TIFREIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TIFREIE_A::NotMasked
    }
}
///Field `TIFREIE` writer - TIFRE interrupt enable
pub type TIFREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TIFREIE_A, O>;
impl<'a, const O: u8> TIFREIE_W<'a, O> {
    ///TI frame format error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TIFREIE_A::Masked)
    }
    ///TI frame format error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TIFREIE_A::NotMasked)
    }
}
///Field `MODFIE` reader - Mode Fault interrupt enable
pub type MODFIE_R = crate::BitReader<MODFIE_A>;
///Mode Fault interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFIE_A {
    ///0: Mode fault interrupt masked
    Masked = 0,
    ///1: Mode fault interrupt not masked
    NotMasked = 1,
}
impl From<MODFIE_A> for bool {
    #[inline(always)]
    fn from(variant: MODFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODFIE_A {
        match self.bits {
            false => MODFIE_A::Masked,
            true => MODFIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MODFIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == MODFIE_A::NotMasked
    }
}
///Field `MODFIE` writer - Mode Fault interrupt enable
pub type MODFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, MODFIE_A, O>;
impl<'a, const O: u8> MODFIE_W<'a, O> {
    ///Mode fault interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MODFIE_A::Masked)
    }
    ///Mode fault interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(MODFIE_A::NotMasked)
    }
}
///Field `TSERFIE` reader - Additional number of transactions reload interrupt enable
pub type TSERFIE_R = crate::BitReader<TSERFIE_A>;
///Additional number of transactions reload interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSERFIE_A {
    ///0: TSER loaded interrupt masked
    Masked = 0,
    ///1: TSER loaded interrupt not masked
    NotMasked = 1,
}
impl From<TSERFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSERFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSERFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSERFIE_A {
        match self.bits {
            false => TSERFIE_A::Masked,
            true => TSERFIE_A::NotMasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TSERFIE_A::Masked
    }
    ///Checks if the value of the field is `NotMasked`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TSERFIE_A::NotMasked
    }
}
///Field `TSERFIE` writer - Additional number of transactions reload interrupt enable
pub type TSERFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TSERFIE_A, O>;
impl<'a, const O: u8> TSERFIE_W<'a, O> {
    ///TSER loaded interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TSERFIE_A::Masked)
    }
    ///TSER loaded interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TSERFIE_A::NotMasked)
    }
}
impl R {
    ///Bit 0 - RXP Interrupt Enable
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXP interrupt enable
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DXP interrupt enabled
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC Interrupt enable
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Mode Fault interrupt enable
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Additional number of transactions reload interrupt enable
    #[inline(always)]
    pub fn tserfie(&self) -> TSERFIE_R {
        TSERFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RXP Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxpie(&mut self) -> RXPIE_W<0> {
        RXPIE_W::new(self)
    }
    ///Bit 1 - TXP interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txpie(&mut self) -> TXPIE_W<1> {
        TXPIE_W::new(self)
    }
    ///Bit 2 - DXP interrupt enabled
    #[inline(always)]
    #[must_use]
    pub fn dxpie(&mut self) -> DXPIE_W<2> {
        DXPIE_W::new(self)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eotie(&mut self) -> EOTIE_W<3> {
        EOTIE_W::new(self)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txtfie(&mut self) -> TXTFIE_W<4> {
        TXTFIE_W::new(self)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<5> {
        UDRIE_W::new(self)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<6> {
        OVRIE_W::new(self)
    }
    ///Bit 7 - CRC Interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<7> {
        CRCEIE_W::new(self)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tifreie(&mut self) -> TIFREIE_W<8> {
        TIFREIE_W::new(self)
    }
    ///Bit 9 - Mode Fault interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn modfie(&mut self) -> MODFIE_W<9> {
        MODFIE_W::new(self)
    }
    ///Bit 10 - Additional number of transactions reload interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tserfie(&mut self) -> TSERFIE_W<10> {
        TSERFIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
