///Register `I2SCFGR` reader
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2SCFGR` writer
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CHLEN` reader - CHLEN
pub type CHLEN_R = crate::BitReader<CHLEN_A>;
///CHLEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN_A {
    ///0: 16-bit wide
    SixteenBit = 0,
    ///1: 32-bit wide
    ThirtyTwoBit = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::SixteenBit,
            true => CHLEN_A::ThirtyTwoBit,
        }
    }
    ///Checks if the value of the field is `SixteenBit`
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CHLEN_A::SixteenBit
    }
    ///Checks if the value of the field is `ThirtyTwoBit`
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == CHLEN_A::ThirtyTwoBit
    }
}
///Field `CHLEN` writer - CHLEN
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, CHLEN_A, O>;
impl<'a, const O: u8> CHLEN_W<'a, O> {
    ///16-bit wide
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::SixteenBit)
    }
    ///32-bit wide
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::ThirtyTwoBit)
    }
}
///Field `DATLEN` reader - DATLEN
pub type DATLEN_R = crate::FieldReader<u8, DATLEN_A>;
///DATLEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN_A {
    ///0: 16-bit data length
    SixteenBit = 0,
    ///1: 24-bit data length
    TwentyFourBit = 1,
    ///2: 32-bit data length
    ThirtyTwoBit = 2,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
impl DATLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DATLEN_A> {
        match self.bits {
            0 => Some(DATLEN_A::SixteenBit),
            1 => Some(DATLEN_A::TwentyFourBit),
            2 => Some(DATLEN_A::ThirtyTwoBit),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SixteenBit`
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DATLEN_A::SixteenBit
    }
    ///Checks if the value of the field is `TwentyFourBit`
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        *self == DATLEN_A::TwentyFourBit
    }
    ///Checks if the value of the field is `ThirtyTwoBit`
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == DATLEN_A::ThirtyTwoBit
    }
}
///Field `DATLEN` writer - DATLEN
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, DATLEN_A, 2, O>;
impl<'a, const O: u8> DATLEN_W<'a, O> {
    ///16-bit data length
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::SixteenBit)
    }
    ///24-bit data length
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::TwentyFourBit)
    }
    ///32-bit data length
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::ThirtyTwoBit)
    }
}
///Field `CKPOL` reader - CKPOL
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
///CKPOL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    ///0: I2S clock inactive state is low level
    IdleLow = 0,
    ///1: I2S clock inactive state is high level
    IdleHigh = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::IdleLow,
            true => CKPOL_A::IdleHigh,
        }
    }
    ///Checks if the value of the field is `IdleLow`
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPOL_A::IdleLow
    }
    ///Checks if the value of the field is `IdleHigh`
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPOL_A::IdleHigh
    }
}
///Field `CKPOL` writer - CKPOL
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, CKPOL_A, O>;
impl<'a, const O: u8> CKPOL_W<'a, O> {
    ///I2S clock inactive state is low level
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPOL_A::IdleLow)
    }
    ///I2S clock inactive state is high level
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPOL_A::IdleHigh)
    }
}
///Field `I2SSTD` reader - I2SSTD
pub type I2SSTD_R = crate::FieldReader<u8, I2SSTD_A>;
///I2SSTD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD_A {
    ///0: I2S Philips standard
    Philips = 0,
    ///1: MSB justified standard
    Msb = 1,
    ///2: LSB justified standard
    Lsb = 2,
    ///3: PCM standard
    Pcm = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
impl I2SSTD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::Philips,
            1 => I2SSTD_A::Msb,
            2 => I2SSTD_A::Lsb,
            3 => I2SSTD_A::Pcm,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Philips`
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD_A::Philips
    }
    ///Checks if the value of the field is `Msb`
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTD_A::Msb
    }
    ///Checks if the value of the field is `Lsb`
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTD_A::Lsb
    }
    ///Checks if the value of the field is `Pcm`
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD_A::Pcm
    }
}
///Field `I2SSTD` writer - I2SSTD
pub type I2SSTD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I2SCFGR_SPEC, u8, I2SSTD_A, 2, O>;
impl<'a, const O: u8> I2SSTD_W<'a, O> {
    ///I2S Philips standard
    #[inline(always)]
    pub fn philips(self) -> &'a mut W {
        self.variant(I2SSTD_A::Philips)
    }
    ///MSB justified standard
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(I2SSTD_A::Msb)
    }
    ///LSB justified standard
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(I2SSTD_A::Lsb)
    }
    ///PCM standard
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTD_A::Pcm)
    }
}
///Field `PCMSYNC` reader - PCMSYNC
pub type PCMSYNC_R = crate::BitReader<PCMSYNC_A>;
///PCMSYNC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC_A {
    ///0: Short frame synchronisation
    Short = 0,
    ///1: Long frame synchronisation
    Long = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl PCMSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::Short,
            true => PCMSYNC_A::Long,
        }
    }
    ///Checks if the value of the field is `Short`
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNC_A::Short
    }
    ///Checks if the value of the field is `Long`
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNC_A::Long
    }
}
///Field `PCMSYNC` writer - PCMSYNC
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, PCMSYNC_A, O>;
impl<'a, const O: u8> PCMSYNC_W<'a, O> {
    ///Short frame synchronisation
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNC_A::Short)
    }
    ///Long frame synchronisation
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSYNC_A::Long)
    }
}
///Field `I2SCFG` reader - I2SCFG
pub type I2SCFG_R = crate::FieldReader<u8, I2SCFG_A>;
///I2SCFG
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG_A {
    ///0: Slave - transmit
    SlaveTx = 0,
    ///1: Slave - receive
    SlaveRx = 1,
    ///2: Master - transmit
    MasterTx = 2,
    ///3: Master - receive
    MasterRx = 3,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
impl I2SCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SCFG_A {
        match self.bits {
            0 => I2SCFG_A::SlaveTx,
            1 => I2SCFG_A::SlaveRx,
            2 => I2SCFG_A::MasterTx,
            3 => I2SCFG_A::MasterRx,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `SlaveTx`
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == I2SCFG_A::SlaveTx
    }
    ///Checks if the value of the field is `SlaveRx`
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == I2SCFG_A::SlaveRx
    }
    ///Checks if the value of the field is `MasterTx`
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == I2SCFG_A::MasterTx
    }
    ///Checks if the value of the field is `MasterRx`
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == I2SCFG_A::MasterRx
    }
}
///Field `I2SCFG` writer - I2SCFG
pub type I2SCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I2SCFGR_SPEC, u8, I2SCFG_A, 2, O>;
impl<'a, const O: u8> I2SCFG_W<'a, O> {
    ///Slave - transmit
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SlaveTx)
    }
    ///Slave - receive
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SlaveRx)
    }
    ///Master - transmit
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MasterTx)
    }
    ///Master - receive
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MasterRx)
    }
}
///Field `I2SE` reader - I2SE
pub type I2SE_R = crate::BitReader<I2SE_A>;
///I2SE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SE_A {
    ///0: I2S peripheral is disabled
    Disabled = 0,
    ///1: I2S peripheral is enabled
    Enabled = 1,
}
impl From<I2SE_A> for bool {
    #[inline(always)]
    fn from(variant: I2SE_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SE_A {
        match self.bits {
            false => I2SE_A::Disabled,
            true => I2SE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2SE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2SE_A::Enabled
    }
}
///Field `I2SE` writer - I2SE
pub type I2SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, I2SE_A, O>;
impl<'a, const O: u8> I2SE_W<'a, O> {
    ///I2S peripheral is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2SE_A::Disabled)
    }
    ///I2S peripheral is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2SE_A::Enabled)
    }
}
///Field `I2SMOD` reader - I2SMOD
pub type I2SMOD_R = crate::BitReader<I2SMOD_A>;
///I2SMOD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD_A {
    ///0: SPI mode is selected
    Spimode = 0,
    ///1: I2S mode is selected
    I2smode = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::Spimode,
            true => I2SMOD_A::I2smode,
        }
    }
    ///Checks if the value of the field is `Spimode`
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        *self == I2SMOD_A::Spimode
    }
    ///Checks if the value of the field is `I2smode`
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        *self == I2SMOD_A::I2smode
    }
}
///Field `I2SMOD` writer - I2SMOD
pub type I2SMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, I2SMOD_A, O>;
impl<'a, const O: u8> I2SMOD_W<'a, O> {
    ///SPI mode is selected
    #[inline(always)]
    pub fn spimode(self) -> &'a mut W {
        self.variant(I2SMOD_A::Spimode)
    }
    ///I2S mode is selected
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut W {
        self.variant(I2SMOD_A::I2smode)
    }
}
///Field `ASTRTEN` reader - ASTRTEN
pub type ASTRTEN_R = crate::BitReader<bool>;
///Field `ASTRTEN` writer - ASTRTEN
pub type ASTRTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ASTRTEN
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CHLEN
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<0> {
        CHLEN_W::new(self)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<1> {
        DATLEN_W::new(self)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<3> {
        CKPOL_W::new(self)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<4> {
        I2SSTD_W::new(self)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<7> {
        PCMSYNC_W::new(self)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<8> {
        I2SCFG_W::new(self)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<10> {
        I2SE_W::new(self)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<11> {
        I2SMOD_W::new(self)
    }
    ///Bit 12 - ASTRTEN
    #[inline(always)]
    #[must_use]
    pub fn astrten(&mut self) -> ASTRTEN_W<12> {
        ASTRTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2scfgr](index.html) module
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2scfgr::R](R) reader structure
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2scfgr::W](W) writer structure
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
