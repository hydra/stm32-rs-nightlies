///Register `DMABMR` reader
pub struct R(crate::R<DMABMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMABMR` writer
pub struct W(crate::W<DMABMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABMR_SPEC>;
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
impl From<crate::W<DMABMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SR` reader - Software reset
pub type SR_R = crate::BitReader<SR_A>;
///Software reset
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR_A {
    ///1: Reset all MAC subsystem internal registers and logic. Cleared automatically
    Reset = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
impl SR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SR_A> {
        match self.bits {
            true => Some(SR_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SR_A::Reset
    }
}
///Field `SR` writer - Software reset
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, SR_A, O>;
impl<'a, const O: u8> SR_W<'a, O> {
    ///Reset all MAC subsystem internal registers and logic. Cleared automatically
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SR_A::Reset)
    }
}
///Field `DA` reader - DMA arbitration
pub type DA_R = crate::BitReader<DA_A>;
///DMA arbitration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DA_A {
    ///0: Round-robin with Rx:Tx priority given by PM
    RoundRobin = 0,
    ///1: Rx has priority over Tx
    RxPriority = 1,
}
impl From<DA_A> for bool {
    #[inline(always)]
    fn from(variant: DA_A) -> Self {
        variant as u8 != 0
    }
}
impl DA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DA_A {
        match self.bits {
            false => DA_A::RoundRobin,
            true => DA_A::RxPriority,
        }
    }
    ///Checks if the value of the field is `RoundRobin`
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == DA_A::RoundRobin
    }
    ///Checks if the value of the field is `RxPriority`
    #[inline(always)]
    pub fn is_rx_priority(&self) -> bool {
        *self == DA_A::RxPriority
    }
}
///Field `DA` writer - DMA arbitration
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, DA_A, O>;
impl<'a, const O: u8> DA_W<'a, O> {
    ///Round-robin with Rx:Tx priority given by PM
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(DA_A::RoundRobin)
    }
    ///Rx has priority over Tx
    #[inline(always)]
    pub fn rx_priority(self) -> &'a mut W {
        self.variant(DA_A::RxPriority)
    }
}
///Field `DSL` reader - Descriptor skip length
pub type DSL_R = crate::FieldReader<u8, u8>;
///Field `DSL` writer - Descriptor skip length
pub type DSL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DMABMR_SPEC, u8, u8, 5, O>;
///Field `EDFE` reader - Enhanced descriptor format enable
pub type EDFE_R = crate::BitReader<EDFE_A>;
///Enhanced descriptor format enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDFE_A {
    ///0: Normal descriptor format
    Disabled = 0,
    ///1: Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload
    Enabled = 1,
}
impl From<EDFE_A> for bool {
    #[inline(always)]
    fn from(variant: EDFE_A) -> Self {
        variant as u8 != 0
    }
}
impl EDFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EDFE_A {
        match self.bits {
            false => EDFE_A::Disabled,
            true => EDFE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDFE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDFE_A::Enabled
    }
}
///Field `EDFE` writer - Enhanced descriptor format enable
pub type EDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, EDFE_A, O>;
impl<'a, const O: u8> EDFE_W<'a, O> {
    ///Normal descriptor format
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDFE_A::Disabled)
    }
    ///Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDFE_A::Enabled)
    }
}
///Field `PBL` reader - Programmable burst length
pub type PBL_R = crate::FieldReader<u8, PBL_A>;
///Programmable burst length
///
///Value on reset: 33
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBL_A {
    ///1: Maximum of 1 beat per DMA transaction
    Pbl1 = 1,
    ///2: Maximum of 2 beats per DMA transaction
    Pbl2 = 2,
    ///4: Maximum of 4 beats per DMA transaction
    Pbl4 = 4,
    ///8: Maximum of 8 beats per DMA transaction
    Pbl8 = 8,
    ///16: Maximum of 16 beats per DMA transaction
    Pbl16 = 16,
    ///32: Maximum of 32 beats per DMA transaction
    Pbl32 = 32,
}
impl From<PBL_A> for u8 {
    #[inline(always)]
    fn from(variant: PBL_A) -> Self {
        variant as _
    }
}
impl PBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PBL_A> {
        match self.bits {
            1 => Some(PBL_A::Pbl1),
            2 => Some(PBL_A::Pbl2),
            4 => Some(PBL_A::Pbl4),
            8 => Some(PBL_A::Pbl8),
            16 => Some(PBL_A::Pbl16),
            32 => Some(PBL_A::Pbl32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pbl1`
    #[inline(always)]
    pub fn is_pbl1(&self) -> bool {
        *self == PBL_A::Pbl1
    }
    ///Checks if the value of the field is `Pbl2`
    #[inline(always)]
    pub fn is_pbl2(&self) -> bool {
        *self == PBL_A::Pbl2
    }
    ///Checks if the value of the field is `Pbl4`
    #[inline(always)]
    pub fn is_pbl4(&self) -> bool {
        *self == PBL_A::Pbl4
    }
    ///Checks if the value of the field is `Pbl8`
    #[inline(always)]
    pub fn is_pbl8(&self) -> bool {
        *self == PBL_A::Pbl8
    }
    ///Checks if the value of the field is `Pbl16`
    #[inline(always)]
    pub fn is_pbl16(&self) -> bool {
        *self == PBL_A::Pbl16
    }
    ///Checks if the value of the field is `Pbl32`
    #[inline(always)]
    pub fn is_pbl32(&self) -> bool {
        *self == PBL_A::Pbl32
    }
}
///Field `PBL` writer - Programmable burst length
pub type PBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, PBL_A, 6, O>;
impl<'a, const O: u8> PBL_W<'a, O> {
    ///Maximum of 1 beat per DMA transaction
    #[inline(always)]
    pub fn pbl1(self) -> &'a mut W {
        self.variant(PBL_A::Pbl1)
    }
    ///Maximum of 2 beats per DMA transaction
    #[inline(always)]
    pub fn pbl2(self) -> &'a mut W {
        self.variant(PBL_A::Pbl2)
    }
    ///Maximum of 4 beats per DMA transaction
    #[inline(always)]
    pub fn pbl4(self) -> &'a mut W {
        self.variant(PBL_A::Pbl4)
    }
    ///Maximum of 8 beats per DMA transaction
    #[inline(always)]
    pub fn pbl8(self) -> &'a mut W {
        self.variant(PBL_A::Pbl8)
    }
    ///Maximum of 16 beats per DMA transaction
    #[inline(always)]
    pub fn pbl16(self) -> &'a mut W {
        self.variant(PBL_A::Pbl16)
    }
    ///Maximum of 32 beats per DMA transaction
    #[inline(always)]
    pub fn pbl32(self) -> &'a mut W {
        self.variant(PBL_A::Pbl32)
    }
}
///Field `PM` reader - Rx-Tx priority ratio
pub type PM_R = crate::FieldReader<u8, PM_A>;
///Rx-Tx priority ratio
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PM_A {
    ///0: RxDMA priority over TxDMA is 1:1
    OneToOne = 0,
    ///1: RxDMA priority over TxDMA is 2:1
    TwoToOne = 1,
    ///2: RxDMA priority over TxDMA is 3:1
    ThreeToOne = 2,
    ///3: RxDMA priority over TxDMA is 4:1
    FourToOne = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            0 => PM_A::OneToOne,
            1 => PM_A::TwoToOne,
            2 => PM_A::ThreeToOne,
            3 => PM_A::FourToOne,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `OneToOne`
    #[inline(always)]
    pub fn is_one_to_one(&self) -> bool {
        *self == PM_A::OneToOne
    }
    ///Checks if the value of the field is `TwoToOne`
    #[inline(always)]
    pub fn is_two_to_one(&self) -> bool {
        *self == PM_A::TwoToOne
    }
    ///Checks if the value of the field is `ThreeToOne`
    #[inline(always)]
    pub fn is_three_to_one(&self) -> bool {
        *self == PM_A::ThreeToOne
    }
    ///Checks if the value of the field is `FourToOne`
    #[inline(always)]
    pub fn is_four_to_one(&self) -> bool {
        *self == PM_A::FourToOne
    }
}
///Field `PM` writer - Rx-Tx priority ratio
pub type PM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DMABMR_SPEC, u8, PM_A, 2, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    ///RxDMA priority over TxDMA is 1:1
    #[inline(always)]
    pub fn one_to_one(self) -> &'a mut W {
        self.variant(PM_A::OneToOne)
    }
    ///RxDMA priority over TxDMA is 2:1
    #[inline(always)]
    pub fn two_to_one(self) -> &'a mut W {
        self.variant(PM_A::TwoToOne)
    }
    ///RxDMA priority over TxDMA is 3:1
    #[inline(always)]
    pub fn three_to_one(self) -> &'a mut W {
        self.variant(PM_A::ThreeToOne)
    }
    ///RxDMA priority over TxDMA is 4:1
    #[inline(always)]
    pub fn four_to_one(self) -> &'a mut W {
        self.variant(PM_A::FourToOne)
    }
}
///Field `FB` reader - Fixed burst
pub type FB_R = crate::BitReader<FB_A>;
///Fixed burst
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FB_A {
    ///0: AHB uses SINGLE and INCR burst transfers
    Variable = 0,
    ///1: AHB uses only fixed burst transfers
    Fixed = 1,
}
impl From<FB_A> for bool {
    #[inline(always)]
    fn from(variant: FB_A) -> Self {
        variant as u8 != 0
    }
}
impl FB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FB_A {
        match self.bits {
            false => FB_A::Variable,
            true => FB_A::Fixed,
        }
    }
    ///Checks if the value of the field is `Variable`
    #[inline(always)]
    pub fn is_variable(&self) -> bool {
        *self == FB_A::Variable
    }
    ///Checks if the value of the field is `Fixed`
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == FB_A::Fixed
    }
}
///Field `FB` writer - Fixed burst
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, FB_A, O>;
impl<'a, const O: u8> FB_W<'a, O> {
    ///AHB uses SINGLE and INCR burst transfers
    #[inline(always)]
    pub fn variable(self) -> &'a mut W {
        self.variant(FB_A::Variable)
    }
    ///AHB uses only fixed burst transfers
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(FB_A::Fixed)
    }
}
///Field `RDP` reader - Rx DMA PBL
pub type RDP_R = crate::FieldReader<u8, RDP_A>;
///Rx DMA PBL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDP_A {
    ///1: 1 beat per RxDMA transaction
    Rdp1 = 1,
    ///2: 2 beats per RxDMA transaction
    Rdp2 = 2,
    ///4: 4 beats per RxDMA transaction
    Rdp4 = 4,
    ///8: 8 beats per RxDMA transaction
    Rdp8 = 8,
    ///16: 16 beats per RxDMA transaction
    Rdp16 = 16,
    ///32: 32 beats per RxDMA transaction
    Rdp32 = 32,
}
impl From<RDP_A> for u8 {
    #[inline(always)]
    fn from(variant: RDP_A) -> Self {
        variant as _
    }
}
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RDP_A> {
        match self.bits {
            1 => Some(RDP_A::Rdp1),
            2 => Some(RDP_A::Rdp2),
            4 => Some(RDP_A::Rdp4),
            8 => Some(RDP_A::Rdp8),
            16 => Some(RDP_A::Rdp16),
            32 => Some(RDP_A::Rdp32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Rdp1`
    #[inline(always)]
    pub fn is_rdp1(&self) -> bool {
        *self == RDP_A::Rdp1
    }
    ///Checks if the value of the field is `Rdp2`
    #[inline(always)]
    pub fn is_rdp2(&self) -> bool {
        *self == RDP_A::Rdp2
    }
    ///Checks if the value of the field is `Rdp4`
    #[inline(always)]
    pub fn is_rdp4(&self) -> bool {
        *self == RDP_A::Rdp4
    }
    ///Checks if the value of the field is `Rdp8`
    #[inline(always)]
    pub fn is_rdp8(&self) -> bool {
        *self == RDP_A::Rdp8
    }
    ///Checks if the value of the field is `Rdp16`
    #[inline(always)]
    pub fn is_rdp16(&self) -> bool {
        *self == RDP_A::Rdp16
    }
    ///Checks if the value of the field is `Rdp32`
    #[inline(always)]
    pub fn is_rdp32(&self) -> bool {
        *self == RDP_A::Rdp32
    }
}
///Field `RDP` writer - Rx DMA PBL
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, RDP_A, 6, O>;
impl<'a, const O: u8> RDP_W<'a, O> {
    ///1 beat per RxDMA transaction
    #[inline(always)]
    pub fn rdp1(self) -> &'a mut W {
        self.variant(RDP_A::Rdp1)
    }
    ///2 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp2(self) -> &'a mut W {
        self.variant(RDP_A::Rdp2)
    }
    ///4 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp4(self) -> &'a mut W {
        self.variant(RDP_A::Rdp4)
    }
    ///8 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp8(self) -> &'a mut W {
        self.variant(RDP_A::Rdp8)
    }
    ///16 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp16(self) -> &'a mut W {
        self.variant(RDP_A::Rdp16)
    }
    ///32 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp32(self) -> &'a mut W {
        self.variant(RDP_A::Rdp32)
    }
}
///Field `USP` reader - Use separate PBL
pub type USP_R = crate::BitReader<USP_A>;
///Use separate PBL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USP_A {
    ///0: PBL value used for both Rx and Tx DMA
    Combined = 0,
    ///1: RxDMA uses RDP value, TxDMA uses PBL value
    Separate = 1,
}
impl From<USP_A> for bool {
    #[inline(always)]
    fn from(variant: USP_A) -> Self {
        variant as u8 != 0
    }
}
impl USP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USP_A {
        match self.bits {
            false => USP_A::Combined,
            true => USP_A::Separate,
        }
    }
    ///Checks if the value of the field is `Combined`
    #[inline(always)]
    pub fn is_combined(&self) -> bool {
        *self == USP_A::Combined
    }
    ///Checks if the value of the field is `Separate`
    #[inline(always)]
    pub fn is_separate(&self) -> bool {
        *self == USP_A::Separate
    }
}
///Field `USP` writer - Use separate PBL
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, USP_A, O>;
impl<'a, const O: u8> USP_W<'a, O> {
    ///PBL value used for both Rx and Tx DMA
    #[inline(always)]
    pub fn combined(self) -> &'a mut W {
        self.variant(USP_A::Combined)
    }
    ///RxDMA uses RDP value, TxDMA uses PBL value
    #[inline(always)]
    pub fn separate(self) -> &'a mut W {
        self.variant(USP_A::Separate)
    }
}
///Field `FPM` reader - 4xPBL mode
pub type FPM_R = crate::BitReader<FPM_A>;
///4xPBL mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPM_A {
    ///0: PBL values used as-is
    X1 = 0,
    ///1: PBL values multiplied by 4
    X4 = 1,
}
impl From<FPM_A> for bool {
    #[inline(always)]
    fn from(variant: FPM_A) -> Self {
        variant as u8 != 0
    }
}
impl FPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPM_A {
        match self.bits {
            false => FPM_A::X1,
            true => FPM_A::X4,
        }
    }
    ///Checks if the value of the field is `X1`
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == FPM_A::X1
    }
    ///Checks if the value of the field is `X4`
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == FPM_A::X4
    }
}
///Field `FPM` writer - 4xPBL mode
pub type FPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, FPM_A, O>;
impl<'a, const O: u8> FPM_W<'a, O> {
    ///PBL values used as-is
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(FPM_A::X1)
    }
    ///PBL values multiplied by 4
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(FPM_A::X4)
    }
}
///Field `AAB` reader - Address-aligned beats
pub type AAB_R = crate::BitReader<AAB_A>;
///Address-aligned beats
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAB_A {
    ///0: Bursts are not aligned
    Unaligned = 0,
    ///1: Align bursts to start address LS bits. First burst alignment depends on FB bit
    Aligned = 1,
}
impl From<AAB_A> for bool {
    #[inline(always)]
    fn from(variant: AAB_A) -> Self {
        variant as u8 != 0
    }
}
impl AAB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AAB_A {
        match self.bits {
            false => AAB_A::Unaligned,
            true => AAB_A::Aligned,
        }
    }
    ///Checks if the value of the field is `Unaligned`
    #[inline(always)]
    pub fn is_unaligned(&self) -> bool {
        *self == AAB_A::Unaligned
    }
    ///Checks if the value of the field is `Aligned`
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        *self == AAB_A::Aligned
    }
}
///Field `AAB` writer - Address-aligned beats
pub type AAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, AAB_A, O>;
impl<'a, const O: u8> AAB_W<'a, O> {
    ///Bursts are not aligned
    #[inline(always)]
    pub fn unaligned(self) -> &'a mut W {
        self.variant(AAB_A::Unaligned)
    }
    ///Align bursts to start address LS bits. First burst alignment depends on FB bit
    #[inline(always)]
    pub fn aligned(self) -> &'a mut W {
        self.variant(AAB_A::Aligned)
    }
}
///Field `MB` reader - Mixed burst
pub type MB_R = crate::BitReader<MB_A>;
///Mixed burst
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB_A {
    ///0: Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below
    Normal = 0,
    ///1: If FB is low, start all bursts greater than 16 with INCR (undefined burst)
    Mixed = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
impl MB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MB_A {
        match self.bits {
            false => MB_A::Normal,
            true => MB_A::Mixed,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MB_A::Normal
    }
    ///Checks if the value of the field is `Mixed`
    #[inline(always)]
    pub fn is_mixed(&self) -> bool {
        *self == MB_A::Mixed
    }
}
///Field `MB` writer - Mixed burst
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, MB_A, O>;
impl<'a, const O: u8> MB_W<'a, O> {
    ///Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MB_A::Normal)
    }
    ///If FB is low, start all bursts greater than 16 with INCR (undefined burst)
    #[inline(always)]
    pub fn mixed(self) -> &'a mut W {
        self.variant(MB_A::Mixed)
    }
}
impl R {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA arbitration
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - Rx-Tx priority ratio
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Mixed burst
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software reset
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    ///Bit 1 - DMA arbitration
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<2> {
        DSL_W::new(self)
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    #[must_use]
    pub fn edfe(&mut self) -> EDFE_W<7> {
        EDFE_W::new(self)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<8> {
        PBL_W::new(self)
    }
    ///Bits 14:15 - Rx-Tx priority ratio
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<14> {
        PM_W::new(self)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<16> {
        FB_W::new(self)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<17> {
        RDP_W::new(self)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<23> {
        USP_W::new(self)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    #[must_use]
    pub fn fpm(&mut self) -> FPM_W<24> {
        FPM_W::new(self)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    #[must_use]
    pub fn aab(&mut self) -> AAB_W<25> {
        AAB_W::new(self)
    }
    ///Bit 26 - Mixed burst
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<26> {
        MB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA bus mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmabmr](index.html) module
pub struct DMABMR_SPEC;
impl crate::RegisterSpec for DMABMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmabmr::R](R) reader structure
impl crate::Readable for DMABMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmabmr::W](W) writer structure
impl crate::Writable for DMABMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMABMR to value 0x2101
impl crate::Resettable for DMABMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
