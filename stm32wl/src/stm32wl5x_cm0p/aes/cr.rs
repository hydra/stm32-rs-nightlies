///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - AES enable
pub type EN_R = crate::BitReader<EN_A>;
///AES enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///0: Disable AES
    Disabled = 0,
    ///1: Enable AES
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
///Field `EN` writer - AES enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    ///Disable AES
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    ///Enable AES
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
///Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)
pub type DATATYPE_R = crate::FieldReader<u8, DATATYPE_A>;
///Data type selection (for data in and data out to/from the cryptographic block)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATATYPE_A {
    ///0: Word
    None = 0,
    ///1: Half-word (16-bit)
    HalfWord = 1,
    ///2: Byte (8-bit)
    Byte = 2,
    ///3: Bit
    Bit = 3,
}
impl From<DATATYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATATYPE_A) -> Self {
        variant as _
    }
}
impl DATATYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DATATYPE_A {
        match self.bits {
            0 => DATATYPE_A::None,
            1 => DATATYPE_A::HalfWord,
            2 => DATATYPE_A::Byte,
            3 => DATATYPE_A::Bit,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `None`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DATATYPE_A::None
    }
    ///Checks if the value of the field is `HalfWord`
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DATATYPE_A::HalfWord
    }
    ///Checks if the value of the field is `Byte`
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DATATYPE_A::Byte
    }
    ///Checks if the value of the field is `Bit`
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        *self == DATATYPE_A::Bit
    }
}
///Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)
pub type DATATYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, DATATYPE_A, 2, O>;
impl<'a, const O: u8> DATATYPE_W<'a, O> {
    ///Word
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DATATYPE_A::None)
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DATATYPE_A::HalfWord)
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DATATYPE_A::Byte)
    }
    ///Bit
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(DATATYPE_A::Bit)
    }
}
///Field `MODE` reader - AES operating mode
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
///AES operating mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    ///0: Mode 1: encryption
    Mode1 = 0,
    ///1: Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    Mode2 = 1,
    ///2: Mode 3: decryption
    Mode3 = 2,
    ///3: Mode 4: key derivation &amp; decrypt (UNDOCUMENTED in ref. manual, exists in CubeMX code)
    Mode4 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::Mode1,
            1 => MODE_A::Mode2,
            2 => MODE_A::Mode3,
            3 => MODE_A::Mode4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Mode1`
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == MODE_A::Mode1
    }
    ///Checks if the value of the field is `Mode2`
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == MODE_A::Mode2
    }
    ///Checks if the value of the field is `Mode3`
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == MODE_A::Mode3
    }
    ///Checks if the value of the field is `Mode4`
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == MODE_A::Mode4
    }
}
///Field `MODE` writer - AES operating mode
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    ///Mode 1: encryption
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(MODE_A::Mode1)
    }
    ///Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(MODE_A::Mode2)
    }
    ///Mode 3: decryption
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(MODE_A::Mode3)
    }
    ///Mode 4: key derivation &amp; decrypt (UNDOCUMENTED in ref. manual, exists in CubeMX code)
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(MODE_A::Mode4)
    }
}
///Field `CHMOD` reader - AES chaining mode Bit1 Bit0
pub type CHMOD_R = crate::FieldReader<u8, CHMOD_A>;
///AES chaining mode Bit1 Bit0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMOD_A {
    ///0: Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    Ecb = 0,
    ///1: Cipher-block chaining (CBC)
    Cbc = 1,
    ///2: Counter mode (CTR)
    Ctr = 2,
    ///3: Galois counter mode (GCM) and Galois message authentication code (GMAC)
    Gcm = 3,
}
impl From<CHMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMOD_A) -> Self {
        variant as _
    }
}
impl CHMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHMOD_A {
        match self.bits {
            0 => CHMOD_A::Ecb,
            1 => CHMOD_A::Cbc,
            2 => CHMOD_A::Ctr,
            3 => CHMOD_A::Gcm,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Ecb`
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CHMOD_A::Ecb
    }
    ///Checks if the value of the field is `Cbc`
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CHMOD_A::Cbc
    }
    ///Checks if the value of the field is `Ctr`
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == CHMOD_A::Ctr
    }
    ///Checks if the value of the field is `Gcm`
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == CHMOD_A::Gcm
    }
}
///Field `CHMOD` writer - AES chaining mode Bit1 Bit0
pub type CHMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, CHMOD_A, 2, O>;
impl<'a, const O: u8> CHMOD_W<'a, O> {
    ///Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(CHMOD_A::Ecb)
    }
    ///Cipher-block chaining (CBC)
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(CHMOD_A::Cbc)
    }
    ///Counter mode (CTR)
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(CHMOD_A::Ctr)
    }
    ///Galois counter mode (GCM) and Galois message authentication code (GMAC)
    #[inline(always)]
    pub fn gcm(self) -> &'a mut W {
        self.variant(CHMOD_A::Gcm)
    }
}
///Field `CCFC` reader - Computation Complete Flag Clear
pub type CCFC_R = crate::BitReader<CCFCW_A>;
///Computation Complete Flag Clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFCW_A {
    ///1: Clear computation complete flag
    Clear = 1,
}
impl From<CCFCW_A> for bool {
    #[inline(always)]
    fn from(variant: CCFCW_A) -> Self {
        variant as u8 != 0
    }
}
impl CCFC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CCFCW_A> {
        match self.bits {
            true => Some(CCFCW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCFCW_A::Clear
    }
}
///Field `CCFC` writer - Computation Complete Flag Clear
pub type CCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CCFCW_A, O>;
impl<'a, const O: u8> CCFC_W<'a, O> {
    ///Clear computation complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCFCW_A::Clear)
    }
}
///Field `ERRC` reader - Error clear
pub type ERRC_R = crate::BitReader<ERRCW_A>;
///Error clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRCW_A {
    ///1: Clear RDERR and WRERR flags
    Clear = 1,
}
impl From<ERRCW_A> for bool {
    #[inline(always)]
    fn from(variant: ERRCW_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ERRCW_A> {
        match self.bits {
            true => Some(ERRCW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ERRCW_A::Clear
    }
}
///Field `ERRC` writer - Error clear
pub type ERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ERRCW_A, O>;
impl<'a, const O: u8> ERRC_W<'a, O> {
    ///Clear RDERR and WRERR flags
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRCW_A::Clear)
    }
}
///Field `CCFIE` reader - CCF flag interrupt enable
pub type CCFIE_R = crate::BitReader<CCFIE_A>;
///CCF flag interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFIE_A {
    ///0: Disable (mask) CCF interrupt
    Disabled = 0,
    ///1: Enable CCF interrupt
    Enabled = 1,
}
impl From<CCFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCFIE_A {
        match self.bits {
            false => CCFIE_A::Disabled,
            true => CCFIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCFIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCFIE_A::Enabled
    }
}
///Field `CCFIE` writer - CCF flag interrupt enable
pub type CCFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CCFIE_A, O>;
impl<'a, const O: u8> CCFIE_W<'a, O> {
    ///Disable (mask) CCF interrupt
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCFIE_A::Disabled)
    }
    ///Enable CCF interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCFIE_A::Enabled)
    }
}
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    ///0: Disable (mask) error interrupt
    Disabled = 0,
    ///1: Enable error interrupt
    Enabled = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::Disabled,
            true => ERRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::Enabled
    }
}
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    ///Disable (mask) error interrupt
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Disabled)
    }
    ///Enable error interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Enabled)
    }
}
///Field `DMAINEN` reader - Enable DMA management of data input phase
pub type DMAINEN_R = crate::BitReader<DMAINEN_A>;
///Enable DMA management of data input phase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAINEN_A {
    ///0: Disable DMA Input
    Disabled = 0,
    ///1: Enable DMA Input
    Enabled = 1,
}
impl From<DMAINEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAINEN_A {
        match self.bits {
            false => DMAINEN_A::Disabled,
            true => DMAINEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAINEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAINEN_A::Enabled
    }
}
///Field `DMAINEN` writer - Enable DMA management of data input phase
pub type DMAINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAINEN_A, O>;
impl<'a, const O: u8> DMAINEN_W<'a, O> {
    ///Disable DMA Input
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAINEN_A::Disabled)
    }
    ///Enable DMA Input
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAINEN_A::Enabled)
    }
}
///Field `DMAOUTEN` reader - Enable DMA management of data output phase
pub type DMAOUTEN_R = crate::BitReader<DMAOUTEN_A>;
///Enable DMA management of data output phase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAOUTEN_A {
    ///0: Disable DMA Output
    Disabled = 0,
    ///1: Enabled DMA Output
    Enabled = 1,
}
impl From<DMAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAOUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAOUTEN_A {
        match self.bits {
            false => DMAOUTEN_A::Disabled,
            true => DMAOUTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAOUTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAOUTEN_A::Enabled
    }
}
///Field `DMAOUTEN` writer - Enable DMA management of data output phase
pub type DMAOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAOUTEN_A, O>;
impl<'a, const O: u8> DMAOUTEN_W<'a, O> {
    ///Disable DMA Output
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAOUTEN_A::Disabled)
    }
    ///Enabled DMA Output
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAOUTEN_A::Enabled)
    }
}
///Field `GCMPH` reader - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected
pub type GCMPH_R = crate::FieldReader<u8, GCMPH_A>;
///Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GCMPH_A {
    ///0: Init phase
    Init = 0,
    ///1: Header phase
    Header = 1,
    ///2: Payload phase
    Payload = 2,
    ///3: Final phase
    Final = 3,
}
impl From<GCMPH_A> for u8 {
    #[inline(always)]
    fn from(variant: GCMPH_A) -> Self {
        variant as _
    }
}
impl GCMPH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GCMPH_A {
        match self.bits {
            0 => GCMPH_A::Init,
            1 => GCMPH_A::Header,
            2 => GCMPH_A::Payload,
            3 => GCMPH_A::Final,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Init`
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        *self == GCMPH_A::Init
    }
    ///Checks if the value of the field is `Header`
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == GCMPH_A::Header
    }
    ///Checks if the value of the field is `Payload`
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        *self == GCMPH_A::Payload
    }
    ///Checks if the value of the field is `Final`
    #[inline(always)]
    pub fn is_final(&self) -> bool {
        *self == GCMPH_A::Final
    }
}
///Field `GCMPH` writer - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected
pub type GCMPH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, GCMPH_A, 2, O>;
impl<'a, const O: u8> GCMPH_W<'a, O> {
    ///Init phase
    #[inline(always)]
    pub fn init(self) -> &'a mut W {
        self.variant(GCMPH_A::Init)
    }
    ///Header phase
    #[inline(always)]
    pub fn header(self) -> &'a mut W {
        self.variant(GCMPH_A::Header)
    }
    ///Payload phase
    #[inline(always)]
    pub fn payload(self) -> &'a mut W {
        self.variant(GCMPH_A::Payload)
    }
    ///Final phase
    #[inline(always)]
    pub fn final_(self) -> &'a mut W {
        self.variant(GCMPH_A::Final)
    }
}
///Field `CHMOD2` reader - AES chaining mode Bit2
pub type CHMOD2_R = crate::BitReader<CHMOD2_A>;
///AES chaining mode Bit2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHMOD2_A {
    ///0: Mode as per CHMOD (ECB, CBC, CTR, GCM)
    Chmod = 0,
    ///1: Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    Ccm = 1,
}
impl From<CHMOD2_A> for bool {
    #[inline(always)]
    fn from(variant: CHMOD2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHMOD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHMOD2_A {
        match self.bits {
            false => CHMOD2_A::Chmod,
            true => CHMOD2_A::Ccm,
        }
    }
    ///Checks if the value of the field is `Chmod`
    #[inline(always)]
    pub fn is_chmod(&self) -> bool {
        *self == CHMOD2_A::Chmod
    }
    ///Checks if the value of the field is `Ccm`
    #[inline(always)]
    pub fn is_ccm(&self) -> bool {
        *self == CHMOD2_A::Ccm
    }
}
///Field `CHMOD2` writer - AES chaining mode Bit2
pub type CHMOD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CHMOD2_A, O>;
impl<'a, const O: u8> CHMOD2_W<'a, O> {
    ///Mode as per CHMOD (ECB, CBC, CTR, GCM)
    #[inline(always)]
    pub fn chmod(self) -> &'a mut W {
        self.variant(CHMOD2_A::Chmod)
    }
    ///Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    #[inline(always)]
    pub fn ccm(self) -> &'a mut W {
        self.variant(CHMOD2_A::Ccm)
    }
}
///Field `KEYSIZE` reader - Key size selection
pub type KEYSIZE_R = crate::BitReader<KEYSIZE_A>;
///Key size selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYSIZE_A {
    ///0: 128 bits
    Bits128 = 0,
    ///1: 256 bits
    Bits256 = 1,
}
impl From<KEYSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl KEYSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> KEYSIZE_A {
        match self.bits {
            false => KEYSIZE_A::Bits128,
            true => KEYSIZE_A::Bits256,
        }
    }
    ///Checks if the value of the field is `Bits128`
    #[inline(always)]
    pub fn is_bits128(&self) -> bool {
        *self == KEYSIZE_A::Bits128
    }
    ///Checks if the value of the field is `Bits256`
    #[inline(always)]
    pub fn is_bits256(&self) -> bool {
        *self == KEYSIZE_A::Bits256
    }
}
///Field `KEYSIZE` writer - Key size selection
pub type KEYSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, KEYSIZE_A, O>;
impl<'a, const O: u8> KEYSIZE_W<'a, O> {
    ///128 bits
    #[inline(always)]
    pub fn bits128(self) -> &'a mut W {
        self.variant(KEYSIZE_A::Bits128)
    }
    ///256 bits
    #[inline(always)]
    pub fn bits256(self) -> &'a mut W {
        self.variant(KEYSIZE_A::Bits256)
    }
}
///Field `NPBLB` reader - Number of padding bytes in last block of payload
pub type NPBLB_R = crate::FieldReader<u8, u8>;
///Field `NPBLB` writer - Number of padding bytes in last block of payload
pub type NPBLB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - AES enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - AES chaining mode Bit1 Bit0
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Computation Complete Flag Clear
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Error clear
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CCF flag interrupt enable
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - AES chaining mode Bit2
    #[inline(always)]
    pub fn chmod2(&self) -> CHMOD2_R {
        CHMOD2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Key size selection
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block of payload
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - AES enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<1> {
        DATATYPE_W::new(self)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    ///Bits 5:6 - AES chaining mode Bit1 Bit0
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> CHMOD_W<5> {
        CHMOD_W::new(self)
    }
    ///Bit 7 - Computation Complete Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn ccfc(&mut self) -> CCFC_W<7> {
        CCFC_W::new(self)
    }
    ///Bit 8 - Error clear
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<8> {
        ERRC_W::new(self)
    }
    ///Bit 9 - CCF flag interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CCFIE_W<9> {
        CCFIE_W::new(self)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<10> {
        ERRIE_W::new(self)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DMAINEN_W<11> {
        DMAINEN_W::new(self)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<12> {
        DMAOUTEN_W::new(self)
    }
    ///Bits 13:14 - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected
    #[inline(always)]
    #[must_use]
    pub fn gcmph(&mut self) -> GCMPH_W<13> {
        GCMPH_W::new(self)
    }
    ///Bit 16 - AES chaining mode Bit2
    #[inline(always)]
    #[must_use]
    pub fn chmod2(&mut self) -> CHMOD2_W<16> {
        CHMOD2_W::new(self)
    }
    ///Bit 18 - Key size selection
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<18> {
        KEYSIZE_W::new(self)
    }
    ///Bits 20:23 - Number of padding bytes in last block of payload
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NPBLB_W<20> {
        NPBLB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
