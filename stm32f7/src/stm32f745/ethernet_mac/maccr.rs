///Register `MACCR` reader
pub struct R(crate::R<MACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACCR` writer
pub struct W(crate::W<MACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACCR_SPEC>;
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
impl From<crate::W<MACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader<RE_A>;
///Receiver enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    ///0: MAC receive state machine is disabled after the completion of the reception of the current frame
    Disabled = 0,
    ///1: MAC receive state machine is enabled
    Enabled = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::Disabled,
            true => RE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::Enabled
    }
}
///Field `RE` writer - Receiver enable
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    ///MAC receive state machine is disabled after the completion of the reception of the current frame
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::Disabled)
    }
    ///MAC receive state machine is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::Enabled)
    }
}
///Field `TE` reader - Transmitter enable
pub type TE_R = crate::BitReader<TE_A>;
///Transmitter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    ///0: MAC transmit state machine is disabled after completion of the transmission of the current frame
    Disabled = 0,
    ///1: MAC transmit state machine is enabled
    Enabled = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::Disabled,
            true => TE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::Enabled
    }
}
///Field `TE` writer - Transmitter enable
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    ///MAC transmit state machine is disabled after completion of the transmission of the current frame
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::Disabled)
    }
    ///MAC transmit state machine is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::Enabled)
    }
}
///Field `DC` reader - Deferral check
pub type DC_R = crate::BitReader<DC_A>;
///Deferral check
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC_A {
    ///0: MAC defers until CRS signal goes inactive
    Disabled = 0,
    ///1: Deferral check function enabled
    Enabled = 1,
}
impl From<DC_A> for bool {
    #[inline(always)]
    fn from(variant: DC_A) -> Self {
        variant as u8 != 0
    }
}
impl DC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DC_A {
        match self.bits {
            false => DC_A::Disabled,
            true => DC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DC_A::Enabled
    }
}
///Field `DC` writer - Deferral check
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, DC_A, O>;
impl<'a, const O: u8> DC_W<'a, O> {
    ///MAC defers until CRS signal goes inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DC_A::Disabled)
    }
    ///Deferral check function enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DC_A::Enabled)
    }
}
///Field `BL` reader - Back-off limit
pub type BL_R = crate::FieldReader<u8, BL_A>;
///Back-off limit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BL_A {
    ///0: For retransmission n, wait up to 2^min(n, 10) time slots
    Bl10 = 0,
    ///1: For retransmission n, wait up to 2^min(n, 8) time slots
    Bl8 = 1,
    ///2: For retransmission n, wait up to 2^min(n, 4) time slots
    Bl4 = 2,
    ///3: For retransmission n, wait up to 2^min(n, 1) time slots
    Bl1 = 3,
}
impl From<BL_A> for u8 {
    #[inline(always)]
    fn from(variant: BL_A) -> Self {
        variant as _
    }
}
impl BL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BL_A {
        match self.bits {
            0 => BL_A::Bl10,
            1 => BL_A::Bl8,
            2 => BL_A::Bl4,
            3 => BL_A::Bl1,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Bl10`
    #[inline(always)]
    pub fn is_bl10(&self) -> bool {
        *self == BL_A::Bl10
    }
    ///Checks if the value of the field is `Bl8`
    #[inline(always)]
    pub fn is_bl8(&self) -> bool {
        *self == BL_A::Bl8
    }
    ///Checks if the value of the field is `Bl4`
    #[inline(always)]
    pub fn is_bl4(&self) -> bool {
        *self == BL_A::Bl4
    }
    ///Checks if the value of the field is `Bl1`
    #[inline(always)]
    pub fn is_bl1(&self) -> bool {
        *self == BL_A::Bl1
    }
}
///Field `BL` writer - Back-off limit
pub type BL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACCR_SPEC, u8, BL_A, 2, O>;
impl<'a, const O: u8> BL_W<'a, O> {
    ///For retransmission n, wait up to 2^min(n, 10) time slots
    #[inline(always)]
    pub fn bl10(self) -> &'a mut W {
        self.variant(BL_A::Bl10)
    }
    ///For retransmission n, wait up to 2^min(n, 8) time slots
    #[inline(always)]
    pub fn bl8(self) -> &'a mut W {
        self.variant(BL_A::Bl8)
    }
    ///For retransmission n, wait up to 2^min(n, 4) time slots
    #[inline(always)]
    pub fn bl4(self) -> &'a mut W {
        self.variant(BL_A::Bl4)
    }
    ///For retransmission n, wait up to 2^min(n, 1) time slots
    #[inline(always)]
    pub fn bl1(self) -> &'a mut W {
        self.variant(BL_A::Bl1)
    }
}
///Field `APCS` reader - Automatic pad/CRC stripping
pub type APCS_R = crate::BitReader<APCS_A>;
///Automatic pad/CRC stripping
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APCS_A {
    ///0: MAC passes all incoming frames unmodified
    Disabled = 0,
    ///1: MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes
    Strip = 1,
}
impl From<APCS_A> for bool {
    #[inline(always)]
    fn from(variant: APCS_A) -> Self {
        variant as u8 != 0
    }
}
impl APCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> APCS_A {
        match self.bits {
            false => APCS_A::Disabled,
            true => APCS_A::Strip,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APCS_A::Disabled
    }
    ///Checks if the value of the field is `Strip`
    #[inline(always)]
    pub fn is_strip(&self) -> bool {
        *self == APCS_A::Strip
    }
}
///Field `APCS` writer - Automatic pad/CRC stripping
pub type APCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, APCS_A, O>;
impl<'a, const O: u8> APCS_W<'a, O> {
    ///MAC passes all incoming frames unmodified
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APCS_A::Disabled)
    }
    ///MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes
    #[inline(always)]
    pub fn strip(self) -> &'a mut W {
        self.variant(APCS_A::Strip)
    }
}
///Field `RD` reader - Retry disable
pub type RD_R = crate::BitReader<RD_A>;
///Retry disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_A {
    ///0: MAC attempts retries based on the settings of BL
    Enabled = 0,
    ///1: MAC attempts only 1 transmission
    Disabled = 1,
}
impl From<RD_A> for bool {
    #[inline(always)]
    fn from(variant: RD_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RD_A {
        match self.bits {
            false => RD_A::Enabled,
            true => RD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RD_A::Disabled
    }
}
///Field `RD` writer - Retry disable
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, RD_A, O>;
impl<'a, const O: u8> RD_W<'a, O> {
    ///MAC attempts retries based on the settings of BL
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RD_A::Enabled)
    }
    ///MAC attempts only 1 transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RD_A::Disabled)
    }
}
///Field `IPCO` reader - IPv4 checksum offload
pub type IPCO_R = crate::BitReader<IPCO_A>;
///IPv4 checksum offload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCO_A {
    ///0: IPv4 checksum offload disabled
    Disabled = 0,
    ///1: IPv4 checksums are checked in received frames
    Offload = 1,
}
impl From<IPCO_A> for bool {
    #[inline(always)]
    fn from(variant: IPCO_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IPCO_A {
        match self.bits {
            false => IPCO_A::Disabled,
            true => IPCO_A::Offload,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IPCO_A::Disabled
    }
    ///Checks if the value of the field is `Offload`
    #[inline(always)]
    pub fn is_offload(&self) -> bool {
        *self == IPCO_A::Offload
    }
}
///Field `IPCO` writer - IPv4 checksum offload
pub type IPCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, IPCO_A, O>;
impl<'a, const O: u8> IPCO_W<'a, O> {
    ///IPv4 checksum offload disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IPCO_A::Disabled)
    }
    ///IPv4 checksums are checked in received frames
    #[inline(always)]
    pub fn offload(self) -> &'a mut W {
        self.variant(IPCO_A::Offload)
    }
}
///Field `DM` reader - Duplex mode
pub type DM_R = crate::BitReader<DM_A>;
///Duplex mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM_A {
    ///0: MAC operates in half-duplex mode
    HalfDuplex = 0,
    ///1: MAC operates in full-duplex mode
    FullDuplex = 1,
}
impl From<DM_A> for bool {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as u8 != 0
    }
}
impl DM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DM_A {
        match self.bits {
            false => DM_A::HalfDuplex,
            true => DM_A::FullDuplex,
        }
    }
    ///Checks if the value of the field is `HalfDuplex`
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == DM_A::HalfDuplex
    }
    ///Checks if the value of the field is `FullDuplex`
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == DM_A::FullDuplex
    }
}
///Field `DM` writer - Duplex mode
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, DM_A, O>;
impl<'a, const O: u8> DM_W<'a, O> {
    ///MAC operates in half-duplex mode
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(DM_A::HalfDuplex)
    }
    ///MAC operates in full-duplex mode
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(DM_A::FullDuplex)
    }
}
///Field `LM` reader - Loopback mode
pub type LM_R = crate::BitReader<LM_A>;
///Loopback mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LM_A {
    ///0: Normal mode
    Normal = 0,
    ///1: MAC operates in loopback mode at the MII
    Loopback = 1,
}
impl From<LM_A> for bool {
    #[inline(always)]
    fn from(variant: LM_A) -> Self {
        variant as u8 != 0
    }
}
impl LM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LM_A {
        match self.bits {
            false => LM_A::Normal,
            true => LM_A::Loopback,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LM_A::Normal
    }
    ///Checks if the value of the field is `Loopback`
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == LM_A::Loopback
    }
}
///Field `LM` writer - Loopback mode
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, LM_A, O>;
impl<'a, const O: u8> LM_W<'a, O> {
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LM_A::Normal)
    }
    ///MAC operates in loopback mode at the MII
    #[inline(always)]
    pub fn loopback(self) -> &'a mut W {
        self.variant(LM_A::Loopback)
    }
}
///Field `ROD` reader - Receive own disable
pub type ROD_R = crate::BitReader<ROD_A>;
///Receive own disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROD_A {
    ///0: MAC receives all packets from PHY while transmitting
    Enabled = 0,
    ///1: MAC disables reception of frames in half-duplex mode
    Disabled = 1,
}
impl From<ROD_A> for bool {
    #[inline(always)]
    fn from(variant: ROD_A) -> Self {
        variant as u8 != 0
    }
}
impl ROD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ROD_A {
        match self.bits {
            false => ROD_A::Enabled,
            true => ROD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROD_A::Disabled
    }
}
///Field `ROD` writer - Receive own disable
pub type ROD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, ROD_A, O>;
impl<'a, const O: u8> ROD_W<'a, O> {
    ///MAC receives all packets from PHY while transmitting
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROD_A::Enabled)
    }
    ///MAC disables reception of frames in half-duplex mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROD_A::Disabled)
    }
}
///Field `FES` reader - Fast Ethernet speed
pub type FES_R = crate::BitReader<FES_A>;
///Fast Ethernet speed
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FES_A {
    ///0: 10 Mbit/s
    Fes10 = 0,
    ///1: 100 Mbit/s
    Fes100 = 1,
}
impl From<FES_A> for bool {
    #[inline(always)]
    fn from(variant: FES_A) -> Self {
        variant as u8 != 0
    }
}
impl FES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FES_A {
        match self.bits {
            false => FES_A::Fes10,
            true => FES_A::Fes100,
        }
    }
    ///Checks if the value of the field is `Fes10`
    #[inline(always)]
    pub fn is_fes10(&self) -> bool {
        *self == FES_A::Fes10
    }
    ///Checks if the value of the field is `Fes100`
    #[inline(always)]
    pub fn is_fes100(&self) -> bool {
        *self == FES_A::Fes100
    }
}
///Field `FES` writer - Fast Ethernet speed
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, FES_A, O>;
impl<'a, const O: u8> FES_W<'a, O> {
    ///10 Mbit/s
    #[inline(always)]
    pub fn fes10(self) -> &'a mut W {
        self.variant(FES_A::Fes10)
    }
    ///100 Mbit/s
    #[inline(always)]
    pub fn fes100(self) -> &'a mut W {
        self.variant(FES_A::Fes100)
    }
}
///Field `CSD` reader - Carrier sense disable
pub type CSD_R = crate::BitReader<CSD_A>;
///Carrier sense disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSD_A {
    ///0: Errors generated due to loss of carrier
    Enabled = 0,
    ///1: No error generated due to loss of carrier
    Disabled = 1,
}
impl From<CSD_A> for bool {
    #[inline(always)]
    fn from(variant: CSD_A) -> Self {
        variant as u8 != 0
    }
}
impl CSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSD_A {
        match self.bits {
            false => CSD_A::Enabled,
            true => CSD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSD_A::Disabled
    }
}
///Field `CSD` writer - Carrier sense disable
pub type CSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, CSD_A, O>;
impl<'a, const O: u8> CSD_W<'a, O> {
    ///Errors generated due to loss of carrier
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSD_A::Enabled)
    }
    ///No error generated due to loss of carrier
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSD_A::Disabled)
    }
}
///Field `IFG` reader - Interframe gap
pub type IFG_R = crate::FieldReader<u8, IFG_A>;
///Interframe gap
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFG_A {
    ///0: 96 bit times
    Ifg96 = 0,
    ///1: 88 bit times
    Ifg88 = 1,
    ///2: 80 bit times
    Ifg80 = 2,
    ///3: 72 bit times
    Ifg72 = 3,
    ///4: 64 bit times
    Ifg64 = 4,
    ///5: 56 bit times
    Ifg56 = 5,
    ///6: 48 bit times
    Ifg48 = 6,
    ///7: 40 bit times
    Ifg40 = 7,
}
impl From<IFG_A> for u8 {
    #[inline(always)]
    fn from(variant: IFG_A) -> Self {
        variant as _
    }
}
impl IFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IFG_A {
        match self.bits {
            0 => IFG_A::Ifg96,
            1 => IFG_A::Ifg88,
            2 => IFG_A::Ifg80,
            3 => IFG_A::Ifg72,
            4 => IFG_A::Ifg64,
            5 => IFG_A::Ifg56,
            6 => IFG_A::Ifg48,
            7 => IFG_A::Ifg40,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Ifg96`
    #[inline(always)]
    pub fn is_ifg96(&self) -> bool {
        *self == IFG_A::Ifg96
    }
    ///Checks if the value of the field is `Ifg88`
    #[inline(always)]
    pub fn is_ifg88(&self) -> bool {
        *self == IFG_A::Ifg88
    }
    ///Checks if the value of the field is `Ifg80`
    #[inline(always)]
    pub fn is_ifg80(&self) -> bool {
        *self == IFG_A::Ifg80
    }
    ///Checks if the value of the field is `Ifg72`
    #[inline(always)]
    pub fn is_ifg72(&self) -> bool {
        *self == IFG_A::Ifg72
    }
    ///Checks if the value of the field is `Ifg64`
    #[inline(always)]
    pub fn is_ifg64(&self) -> bool {
        *self == IFG_A::Ifg64
    }
    ///Checks if the value of the field is `Ifg56`
    #[inline(always)]
    pub fn is_ifg56(&self) -> bool {
        *self == IFG_A::Ifg56
    }
    ///Checks if the value of the field is `Ifg48`
    #[inline(always)]
    pub fn is_ifg48(&self) -> bool {
        *self == IFG_A::Ifg48
    }
    ///Checks if the value of the field is `Ifg40`
    #[inline(always)]
    pub fn is_ifg40(&self) -> bool {
        *self == IFG_A::Ifg40
    }
}
///Field `IFG` writer - Interframe gap
pub type IFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACCR_SPEC, u8, IFG_A, 3, O>;
impl<'a, const O: u8> IFG_W<'a, O> {
    ///96 bit times
    #[inline(always)]
    pub fn ifg96(self) -> &'a mut W {
        self.variant(IFG_A::Ifg96)
    }
    ///88 bit times
    #[inline(always)]
    pub fn ifg88(self) -> &'a mut W {
        self.variant(IFG_A::Ifg88)
    }
    ///80 bit times
    #[inline(always)]
    pub fn ifg80(self) -> &'a mut W {
        self.variant(IFG_A::Ifg80)
    }
    ///72 bit times
    #[inline(always)]
    pub fn ifg72(self) -> &'a mut W {
        self.variant(IFG_A::Ifg72)
    }
    ///64 bit times
    #[inline(always)]
    pub fn ifg64(self) -> &'a mut W {
        self.variant(IFG_A::Ifg64)
    }
    ///56 bit times
    #[inline(always)]
    pub fn ifg56(self) -> &'a mut W {
        self.variant(IFG_A::Ifg56)
    }
    ///48 bit times
    #[inline(always)]
    pub fn ifg48(self) -> &'a mut W {
        self.variant(IFG_A::Ifg48)
    }
    ///40 bit times
    #[inline(always)]
    pub fn ifg40(self) -> &'a mut W {
        self.variant(IFG_A::Ifg40)
    }
}
///Field `JD` reader - Jabber disable
pub type JD_R = crate::BitReader<JD_A>;
///Jabber disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JD_A {
    ///0: Jabber enabled, transmit frames up to 2048 bytes
    Enabled = 0,
    ///1: Jabber disabled, transmit frames up to 16384 bytes
    Disabled = 1,
}
impl From<JD_A> for bool {
    #[inline(always)]
    fn from(variant: JD_A) -> Self {
        variant as u8 != 0
    }
}
impl JD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JD_A {
        match self.bits {
            false => JD_A::Enabled,
            true => JD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JD_A::Disabled
    }
}
///Field `JD` writer - Jabber disable
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, JD_A, O>;
impl<'a, const O: u8> JD_W<'a, O> {
    ///Jabber enabled, transmit frames up to 2048 bytes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JD_A::Enabled)
    }
    ///Jabber disabled, transmit frames up to 16384 bytes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JD_A::Disabled)
    }
}
///Field `WD` reader - Watchdog disable
pub type WD_R = crate::BitReader<WD_A>;
///Watchdog disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WD_A {
    ///0: Watchdog enabled, receive frames limited to 2048 bytes
    Enabled = 0,
    ///1: Watchdog disabled, receive frames may be up to to 16384 bytes
    Disabled = 1,
}
impl From<WD_A> for bool {
    #[inline(always)]
    fn from(variant: WD_A) -> Self {
        variant as u8 != 0
    }
}
impl WD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WD_A {
        match self.bits {
            false => WD_A::Enabled,
            true => WD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WD_A::Disabled
    }
}
///Field `WD` writer - Watchdog disable
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, WD_A, O>;
impl<'a, const O: u8> WD_W<'a, O> {
    ///Watchdog enabled, receive frames limited to 2048 bytes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WD_A::Enabled)
    }
    ///Watchdog disabled, receive frames may be up to to 16384 bytes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WD_A::Disabled)
    }
}
///Field `CSTF` reader - CRC stripping for type frames
pub type CSTF_R = crate::BitReader<CSTF_A>;
///CRC stripping for type frames
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTF_A {
    ///0: CRC not stripped
    Disabled = 0,
    ///1: CRC stripped
    Enabled = 1,
}
impl From<CSTF_A> for bool {
    #[inline(always)]
    fn from(variant: CSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSTF_A {
        match self.bits {
            false => CSTF_A::Disabled,
            true => CSTF_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSTF_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSTF_A::Enabled
    }
}
///Field `CSTF` writer - CRC stripping for type frames
pub type CSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, CSTF_A, O>;
impl<'a, const O: u8> CSTF_W<'a, O> {
    ///CRC not stripped
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSTF_A::Disabled)
    }
    ///CRC stripped
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSTF_A::Enabled)
    }
}
impl R {
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Deferral check
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Back-off limit
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Automatic pad/CRC stripping
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Retry disable
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IPv4 checksum offload
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Duplex mode
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Loopback mode
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Receive own disable
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Fast Ethernet speed
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Carrier sense disable
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Interframe gap
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 22 - Jabber disable
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Watchdog disable
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CRC stripping for type frames
    #[inline(always)]
    pub fn cstf(&self) -> CSTF_R {
        CSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Receiver enable
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    ///Bit 4 - Deferral check
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    ///Bits 5:6 - Back-off limit
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    ///Bit 7 - Automatic pad/CRC stripping
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<7> {
        APCS_W::new(self)
    }
    ///Bit 9 - Retry disable
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<9> {
        RD_W::new(self)
    }
    ///Bit 10 - IPv4 checksum offload
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<10> {
        IPCO_W::new(self)
    }
    ///Bit 11 - Duplex mode
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<11> {
        DM_W::new(self)
    }
    ///Bit 12 - Loopback mode
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    ///Bit 13 - Receive own disable
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<13> {
        ROD_W::new(self)
    }
    ///Bit 14 - Fast Ethernet speed
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    ///Bit 16 - Carrier sense disable
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<16> {
        CSD_W::new(self)
    }
    ///Bits 17:19 - Interframe gap
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<17> {
        IFG_W::new(self)
    }
    ///Bit 22 - Jabber disable
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<22> {
        JD_W::new(self)
    }
    ///Bit 23 - Watchdog disable
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<23> {
        WD_W::new(self)
    }
    ///Bit 25 - CRC stripping for type frames
    #[inline(always)]
    #[must_use]
    pub fn cstf(&mut self) -> CSTF_W<25> {
        CSTF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maccr](index.html) module
pub struct MACCR_SPEC;
impl crate::RegisterSpec for MACCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maccr::R](R) reader structure
impl crate::Readable for MACCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maccr::W](W) writer structure
impl crate::Writable for MACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACCR to value 0x8000
impl crate::Resettable for MACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
