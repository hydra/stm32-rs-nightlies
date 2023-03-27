///Register `MACFFR` reader
pub struct R(crate::R<MACFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACFFR` writer
pub struct W(crate::W<MACFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFFR_SPEC>;
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
impl From<crate::W<MACFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PM` reader - Promiscuous mode
pub type PM_R = crate::BitReader<PM_A>;
///Promiscuous mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    ///0: Normal address filtering
    Disabled = 0,
    ///1: Address filters pass all incoming frames regardless of their destination or source address
    Enabled = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::Disabled,
            true => PM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PM_A::Enabled
    }
}
///Field `PM` writer - Promiscuous mode
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, PM_A, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    ///Normal address filtering
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PM_A::Disabled)
    }
    ///Address filters pass all incoming frames regardless of their destination or source address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PM_A::Enabled)
    }
}
///Field `HU` reader - Hash unicast
pub type HU_R = crate::BitReader<HU_A>;
///Hash unicast
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HU_A {
    ///0: MAC performs a perfect destination address filtering for unicast frames
    Perfect = 0,
    ///1: MAC performs destination address filtering of received unicast frames according to the hash table
    Hash = 1,
}
impl From<HU_A> for bool {
    #[inline(always)]
    fn from(variant: HU_A) -> Self {
        variant as u8 != 0
    }
}
impl HU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HU_A {
        match self.bits {
            false => HU_A::Perfect,
            true => HU_A::Hash,
        }
    }
    ///Checks if the value of the field is `Perfect`
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HU_A::Perfect
    }
    ///Checks if the value of the field is `Hash`
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HU_A::Hash
    }
}
///Field `HU` writer - Hash unicast
pub type HU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, HU_A, O>;
impl<'a, const O: u8> HU_W<'a, O> {
    ///MAC performs a perfect destination address filtering for unicast frames
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HU_A::Perfect)
    }
    ///MAC performs destination address filtering of received unicast frames according to the hash table
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HU_A::Hash)
    }
}
///Field `HM` reader - Hash multicast
pub type HM_R = crate::BitReader<HM_A>;
///Hash multicast
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HM_A {
    ///0: MAC performs a perfect destination address filtering for multicast frames
    Perfect = 0,
    ///1: MAC performs destination address filtering of received multicast frames according to the hash table
    Hash = 1,
}
impl From<HM_A> for bool {
    #[inline(always)]
    fn from(variant: HM_A) -> Self {
        variant as u8 != 0
    }
}
impl HM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HM_A {
        match self.bits {
            false => HM_A::Perfect,
            true => HM_A::Hash,
        }
    }
    ///Checks if the value of the field is `Perfect`
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HM_A::Perfect
    }
    ///Checks if the value of the field is `Hash`
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HM_A::Hash
    }
}
///Field `HM` writer - Hash multicast
pub type HM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, HM_A, O>;
impl<'a, const O: u8> HM_W<'a, O> {
    ///MAC performs a perfect destination address filtering for multicast frames
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HM_A::Perfect)
    }
    ///MAC performs destination address filtering of received multicast frames according to the hash table
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HM_A::Hash)
    }
}
///Field `DAIF` reader - Destination address unique filtering
pub type DAIF_R = crate::BitReader<DAIF_A>;
///Destination address unique filtering
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAIF_A {
    ///0: Normal filtering of frames
    Normal = 0,
    ///1: Address check block operates in inverse filtering mode for the DA address comparison
    Invert = 1,
}
impl From<DAIF_A> for bool {
    #[inline(always)]
    fn from(variant: DAIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DAIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DAIF_A {
        match self.bits {
            false => DAIF_A::Normal,
            true => DAIF_A::Invert,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DAIF_A::Normal
    }
    ///Checks if the value of the field is `Invert`
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == DAIF_A::Invert
    }
}
///Field `DAIF` writer - Destination address unique filtering
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, DAIF_A, O>;
impl<'a, const O: u8> DAIF_W<'a, O> {
    ///Normal filtering of frames
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DAIF_A::Normal)
    }
    ///Address check block operates in inverse filtering mode for the DA address comparison
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(DAIF_A::Invert)
    }
}
///Field `PAM` reader - Pass all multicast
pub type PAM_R = crate::BitReader<PAM_A>;
///Pass all multicast
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAM_A {
    ///0: Filtering of multicast frames depends on HM
    Disabled = 0,
    ///1: All received frames with a multicast destination address are passed
    Enabled = 1,
}
impl From<PAM_A> for bool {
    #[inline(always)]
    fn from(variant: PAM_A) -> Self {
        variant as u8 != 0
    }
}
impl PAM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PAM_A {
        match self.bits {
            false => PAM_A::Disabled,
            true => PAM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAM_A::Enabled
    }
}
///Field `PAM` writer - Pass all multicast
pub type PAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, PAM_A, O>;
impl<'a, const O: u8> PAM_W<'a, O> {
    ///Filtering of multicast frames depends on HM
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAM_A::Disabled)
    }
    ///All received frames with a multicast destination address are passed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAM_A::Enabled)
    }
}
///Field `BFD` reader - Broadcast frames disable
pub type BFD_R = crate::BitReader<BFD_A>;
///Broadcast frames disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFD_A {
    ///0: Address filters pass all received broadcast frames
    Enabled = 0,
    ///1: Address filters filter all incoming broadcast frames
    Disabled = 1,
}
impl From<BFD_A> for bool {
    #[inline(always)]
    fn from(variant: BFD_A) -> Self {
        variant as u8 != 0
    }
}
impl BFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BFD_A {
        match self.bits {
            false => BFD_A::Enabled,
            true => BFD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFD_A::Disabled
    }
}
///Field `BFD` writer - Broadcast frames disable
pub type BFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, BFD_A, O>;
impl<'a, const O: u8> BFD_W<'a, O> {
    ///Address filters pass all received broadcast frames
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BFD_A::Enabled)
    }
    ///Address filters filter all incoming broadcast frames
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BFD_A::Disabled)
    }
}
///Field `PCF` reader - Pass control frames
pub type PCF_R = crate::FieldReader<u8, PCF_A>;
///Pass control frames
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCF_A {
    ///0: MAC prevents all control frames from reaching the application
    PreventAll = 0,
    ///1: MAC forwards all control frames to application except Pause
    ForwardAllExceptPause = 1,
    ///2: MAC forwards all control frames to application even if they fail the address filter
    ForwardAll = 2,
    ///3: MAC forwards control frames that pass the address filter
    ForwardAllFiltered = 3,
}
impl From<PCF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCF_A) -> Self {
        variant as _
    }
}
impl PCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCF_A {
        match self.bits {
            0 => PCF_A::PreventAll,
            1 => PCF_A::ForwardAllExceptPause,
            2 => PCF_A::ForwardAll,
            3 => PCF_A::ForwardAllFiltered,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PreventAll`
    #[inline(always)]
    pub fn is_prevent_all(&self) -> bool {
        *self == PCF_A::PreventAll
    }
    ///Checks if the value of the field is `ForwardAllExceptPause`
    #[inline(always)]
    pub fn is_forward_all_except_pause(&self) -> bool {
        *self == PCF_A::ForwardAllExceptPause
    }
    ///Checks if the value of the field is `ForwardAll`
    #[inline(always)]
    pub fn is_forward_all(&self) -> bool {
        *self == PCF_A::ForwardAll
    }
    ///Checks if the value of the field is `ForwardAllFiltered`
    #[inline(always)]
    pub fn is_forward_all_filtered(&self) -> bool {
        *self == PCF_A::ForwardAllFiltered
    }
}
///Field `PCF` writer - Pass control frames
pub type PCF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACFFR_SPEC, u8, PCF_A, 2, O>;
impl<'a, const O: u8> PCF_W<'a, O> {
    ///MAC prevents all control frames from reaching the application
    #[inline(always)]
    pub fn prevent_all(self) -> &'a mut W {
        self.variant(PCF_A::PreventAll)
    }
    ///MAC forwards all control frames to application except Pause
    #[inline(always)]
    pub fn forward_all_except_pause(self) -> &'a mut W {
        self.variant(PCF_A::ForwardAllExceptPause)
    }
    ///MAC forwards all control frames to application even if they fail the address filter
    #[inline(always)]
    pub fn forward_all(self) -> &'a mut W {
        self.variant(PCF_A::ForwardAll)
    }
    ///MAC forwards control frames that pass the address filter
    #[inline(always)]
    pub fn forward_all_filtered(self) -> &'a mut W {
        self.variant(PCF_A::ForwardAllFiltered)
    }
}
///Field `SAIF` reader - Source address inverse filtering
pub type SAIF_R = crate::BitReader<SAIF_A>;
///Source address inverse filtering
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIF_A {
    ///0: Source address filter operates normally
    Normal = 0,
    ///1: Source address filter operation inverted
    Invert = 1,
}
impl From<SAIF_A> for bool {
    #[inline(always)]
    fn from(variant: SAIF_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAIF_A {
        match self.bits {
            false => SAIF_A::Normal,
            true => SAIF_A::Invert,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SAIF_A::Normal
    }
    ///Checks if the value of the field is `Invert`
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SAIF_A::Invert
    }
}
///Field `SAIF` writer - Source address inverse filtering
pub type SAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, SAIF_A, O>;
impl<'a, const O: u8> SAIF_W<'a, O> {
    ///Source address filter operates normally
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SAIF_A::Normal)
    }
    ///Source address filter operation inverted
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(SAIF_A::Invert)
    }
}
///Field `SAF` reader - Source address filter
pub type SAF_R = crate::BitReader<SAF_A>;
///Source address filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAF_A {
    ///0: Source address ignored
    Disabled = 0,
    ///1: MAC drops frames that fail the source address filter
    Enabled = 1,
}
impl From<SAF_A> for bool {
    #[inline(always)]
    fn from(variant: SAF_A) -> Self {
        variant as u8 != 0
    }
}
impl SAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAF_A {
        match self.bits {
            false => SAF_A::Disabled,
            true => SAF_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAF_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAF_A::Enabled
    }
}
///Field `SAF` writer - Source address filter
pub type SAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, SAF_A, O>;
impl<'a, const O: u8> SAF_W<'a, O> {
    ///Source address ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAF_A::Disabled)
    }
    ///MAC drops frames that fail the source address filter
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAF_A::Enabled)
    }
}
///Field `HPF` reader - Hash or perfect filter
pub type HPF_R = crate::BitReader<HPF_A>;
///Hash or perfect filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPF_A {
    ///0: If HM or HU is set, only frames that match the Hash filter are passed
    HashOnly = 0,
    ///1: If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
    HashOrPerfect = 1,
}
impl From<HPF_A> for bool {
    #[inline(always)]
    fn from(variant: HPF_A) -> Self {
        variant as u8 != 0
    }
}
impl HPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HPF_A {
        match self.bits {
            false => HPF_A::HashOnly,
            true => HPF_A::HashOrPerfect,
        }
    }
    ///Checks if the value of the field is `HashOnly`
    #[inline(always)]
    pub fn is_hash_only(&self) -> bool {
        *self == HPF_A::HashOnly
    }
    ///Checks if the value of the field is `HashOrPerfect`
    #[inline(always)]
    pub fn is_hash_or_perfect(&self) -> bool {
        *self == HPF_A::HashOrPerfect
    }
}
///Field `HPF` writer - Hash or perfect filter
pub type HPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, HPF_A, O>;
impl<'a, const O: u8> HPF_W<'a, O> {
    ///If HM or HU is set, only frames that match the Hash filter are passed
    #[inline(always)]
    pub fn hash_only(self) -> &'a mut W {
        self.variant(HPF_A::HashOnly)
    }
    ///If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
    #[inline(always)]
    pub fn hash_or_perfect(self) -> &'a mut W {
        self.variant(HPF_A::HashOrPerfect)
    }
}
///Field `RA` reader - Receive all
pub type RA_R = crate::BitReader<RA_A>;
///Receive all
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RA_A {
    ///0: MAC receiver passes on to the application only those frames that have passed the SA/DA address file
    Disabled = 0,
    ///1: MAC receiver passes oll received frames on to the application
    Enabled = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
impl RA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::Disabled,
            true => RA_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RA_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RA_A::Enabled
    }
}
///Field `RA` writer - Receive all
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, RA_A, O>;
impl<'a, const O: u8> RA_W<'a, O> {
    ///MAC receiver passes on to the application only those frames that have passed the SA/DA address file
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RA_A::Disabled)
    }
    ///MAC receiver passes oll received frames on to the application
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RA_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Destination address unique filtering
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 7 - Source address inverse filtering
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Source address filter
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Hash or perfect filter
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<0> {
        PM_W::new(self)
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<1> {
        HU_W::new(self)
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HM_W<2> {
        HM_W::new(self)
    }
    ///Bit 3 - Destination address unique filtering
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<4> {
        PAM_W::new(self)
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    #[must_use]
    pub fn bfd(&mut self) -> BFD_W<5> {
        BFD_W::new(self)
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    ///Bit 7 - Source address inverse filtering
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<7> {
        SAIF_W::new(self)
    }
    ///Bit 8 - Source address filter
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<8> {
        SAF_W::new(self)
    }
    ///Bit 9 - Hash or perfect filter
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<9> {
        HPF_W::new(self)
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<31> {
        RA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC frame filter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macffr](index.html) module
pub struct MACFFR_SPEC;
impl crate::RegisterSpec for MACFFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macffr::R](R) reader structure
impl crate::Readable for MACFFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macffr::W](W) writer structure
impl crate::Writable for MACFFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACFFR to value 0
impl crate::Resettable for MACFFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
