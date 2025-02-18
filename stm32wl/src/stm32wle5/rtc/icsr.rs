///Register `ICSR` reader
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICSR` writer
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUTWF` reader - Wakeup timer write flag
pub type WUTWF_R = crate::BitReader<WUTWFR_A>;
///Wakeup timer write flag
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTWFR_A {
    ///0: Wakeup timer configuration update not allowed
    UpdateNotAllowed = 0,
    ///1: Wakeup timer configuration update allowed
    UpdateAllowed = 1,
}
impl From<WUTWFR_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWFR_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTWFR_A {
        match self.bits {
            false => WUTWFR_A::UpdateNotAllowed,
            true => WUTWFR_A::UpdateAllowed,
        }
    }
    ///Checks if the value of the field is `UpdateNotAllowed`
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWFR_A::UpdateNotAllowed
    }
    ///Checks if the value of the field is `UpdateAllowed`
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWFR_A::UpdateAllowed
    }
}
///Field `SHPF` reader - Shift operation pending
pub type SHPF_R = crate::BitReader<SHPFR_A>;
///Shift operation pending
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHPFR_A {
    ///0: No shift operation is pending
    NoShiftPending = 0,
    ///1: A shift operation is pending
    ShiftPending = 1,
}
impl From<SHPFR_A> for bool {
    #[inline(always)]
    fn from(variant: SHPFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SHPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SHPFR_A {
        match self.bits {
            false => SHPFR_A::NoShiftPending,
            true => SHPFR_A::ShiftPending,
        }
    }
    ///Checks if the value of the field is `NoShiftPending`
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPFR_A::NoShiftPending
    }
    ///Checks if the value of the field is `ShiftPending`
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPFR_A::ShiftPending
    }
}
///Field `INITS` reader - Initialization status flag
pub type INITS_R = crate::BitReader<INITSR_A>;
///Initialization status flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITSR_A {
    ///0: Calendar has not been initialized
    NotInitalized = 0,
    ///1: Calendar has been initialized
    Initalized = 1,
}
impl From<INITSR_A> for bool {
    #[inline(always)]
    fn from(variant: INITSR_A) -> Self {
        variant as u8 != 0
    }
}
impl INITS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INITSR_A {
        match self.bits {
            false => INITSR_A::NotInitalized,
            true => INITSR_A::Initalized,
        }
    }
    ///Checks if the value of the field is `NotInitalized`
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITSR_A::NotInitalized
    }
    ///Checks if the value of the field is `Initalized`
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITSR_A::Initalized
    }
}
///Field `RSF` reader - Registers synchronization flag
pub type RSF_R = crate::BitReader<RSFR_A>;
///Registers synchronization flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR_A {
    ///0: Calendar shadow registers not yet synchronized
    NotSynced = 0,
    ///1: Calendar shadow registers synchronized
    Synced = 1,
}
impl From<RSFR_A> for bool {
    #[inline(always)]
    fn from(variant: RSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSFR_A {
        match self.bits {
            false => RSFR_A::NotSynced,
            true => RSFR_A::Synced,
        }
    }
    ///Checks if the value of the field is `NotSynced`
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSFR_A::NotSynced
    }
    ///Checks if the value of the field is `Synced`
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSFR_A::Synced
    }
}
///Registers synchronization flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFW_AW {
    ///0: This flag is cleared by software by writing 0
    Clear = 0,
}
impl From<RSFW_AW> for bool {
    #[inline(always)]
    fn from(variant: RSFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` writer - Registers synchronization flag
pub type RSF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ICSR_SPEC, RSFW_AW, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSFW_AW::Clear)
    }
}
///Field `INITF` reader - Initialization flag
pub type INITF_R = crate::BitReader<INITFR_A>;
///Initialization flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITFR_A {
    ///0: Calendar registers update is not allowed
    NotAllowed = 0,
    ///1: Calendar registers update is allowed
    Allowed = 1,
}
impl From<INITFR_A> for bool {
    #[inline(always)]
    fn from(variant: INITFR_A) -> Self {
        variant as u8 != 0
    }
}
impl INITF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INITFR_A {
        match self.bits {
            false => INITFR_A::NotAllowed,
            true => INITFR_A::Allowed,
        }
    }
    ///Checks if the value of the field is `NotAllowed`
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITFR_A::NotAllowed
    }
    ///Checks if the value of the field is `Allowed`
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITFR_A::Allowed
    }
}
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader<INIT_A>;
///Initialization mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    ///0: Free running mode
    FreeRunningMode = 0,
    ///1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    InitMode = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
impl INIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::FreeRunningMode,
            true => INIT_A::InitMode,
        }
    }
    ///Checks if the value of the field is `FreeRunningMode`
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT_A::FreeRunningMode
    }
    ///Checks if the value of the field is `InitMode`
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT_A::InitMode
    }
}
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, INIT_A, O>;
impl<'a, const O: u8> INIT_W<'a, O> {
    ///Free running mode
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut W {
        self.variant(INIT_A::FreeRunningMode)
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut W {
        self.variant(INIT_A::InitMode)
    }
}
///Field `BIN` reader - Binary mode
pub type BIN_R = crate::FieldReader<u8, BIN_A>;
///Binary mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIN_A {
    ///0: Free running BCD calendar mode (Binary mode disabled)
    Bcd = 0,
    ///1: Free running Binary mode (BCD mode disabled)
    Binary = 1,
    ///2: Free running BCD calendar and Binary modes
    BinBcd = 2,
    ///3: Free running BCD calendar and Binary modes
    BinBcd2 = 3,
}
impl From<BIN_A> for u8 {
    #[inline(always)]
    fn from(variant: BIN_A) -> Self {
        variant as _
    }
}
impl BIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIN_A {
        match self.bits {
            0 => BIN_A::Bcd,
            1 => BIN_A::Binary,
            2 => BIN_A::BinBcd,
            3 => BIN_A::BinBcd2,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Bcd`
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == BIN_A::Bcd
    }
    ///Checks if the value of the field is `Binary`
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        *self == BIN_A::Binary
    }
    ///Checks if the value of the field is `BinBcd`
    #[inline(always)]
    pub fn is_bin_bcd(&self) -> bool {
        *self == BIN_A::BinBcd
    }
    ///Checks if the value of the field is `BinBcd2`
    #[inline(always)]
    pub fn is_bin_bcd2(&self) -> bool {
        *self == BIN_A::BinBcd2
    }
}
///Field `BIN` writer - Binary mode
pub type BIN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSR_SPEC, u8, BIN_A, 2, O>;
impl<'a, const O: u8> BIN_W<'a, O> {
    ///Free running BCD calendar mode (Binary mode disabled)
    #[inline(always)]
    pub fn bcd(self) -> &'a mut W {
        self.variant(BIN_A::Bcd)
    }
    ///Free running Binary mode (BCD mode disabled)
    #[inline(always)]
    pub fn binary(self) -> &'a mut W {
        self.variant(BIN_A::Binary)
    }
    ///Free running BCD calendar and Binary modes
    #[inline(always)]
    pub fn bin_bcd(self) -> &'a mut W {
        self.variant(BIN_A::BinBcd)
    }
    ///Free running BCD calendar and Binary modes
    #[inline(always)]
    pub fn bin_bcd2(self) -> &'a mut W {
        self.variant(BIN_A::BinBcd2)
    }
}
///Field `BCDU` reader - BCD update
pub type BCDU_R = crate::FieldReader<u8, BCDU_A>;
///BCD update
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCDU_A {
    ///0: 1s increment each time SS\[7:0\]=0
    Bit7 = 0,
    ///1: 1s increment each time SS\[8:0\]=0
    Bit8 = 1,
    ///2: 1s increment each time SS\[9:0\]=0
    Bit9 = 2,
    ///3: 1s increment each time SS\[10:0\]=0
    Bit10 = 3,
    ///4: 1s increment each time SS\[11:0\]=0
    Bit11 = 4,
    ///5: 1s increment each time SS\[12:0\]=0
    Bit12 = 5,
    ///6: 1s increment each time SS\[13:0\]=0
    Bit13 = 6,
    ///7: 1s increment each time SS\[14:0\]=0
    Bit14 = 7,
}
impl From<BCDU_A> for u8 {
    #[inline(always)]
    fn from(variant: BCDU_A) -> Self {
        variant as _
    }
}
impl BCDU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BCDU_A {
        match self.bits {
            0 => BCDU_A::Bit7,
            1 => BCDU_A::Bit8,
            2 => BCDU_A::Bit9,
            3 => BCDU_A::Bit10,
            4 => BCDU_A::Bit11,
            5 => BCDU_A::Bit12,
            6 => BCDU_A::Bit13,
            7 => BCDU_A::Bit14,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Bit7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == BCDU_A::Bit7
    }
    ///Checks if the value of the field is `Bit8`
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == BCDU_A::Bit8
    }
    ///Checks if the value of the field is `Bit9`
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == BCDU_A::Bit9
    }
    ///Checks if the value of the field is `Bit10`
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == BCDU_A::Bit10
    }
    ///Checks if the value of the field is `Bit11`
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == BCDU_A::Bit11
    }
    ///Checks if the value of the field is `Bit12`
    #[inline(always)]
    pub fn is_bit12(&self) -> bool {
        *self == BCDU_A::Bit12
    }
    ///Checks if the value of the field is `Bit13`
    #[inline(always)]
    pub fn is_bit13(&self) -> bool {
        *self == BCDU_A::Bit13
    }
    ///Checks if the value of the field is `Bit14`
    #[inline(always)]
    pub fn is_bit14(&self) -> bool {
        *self == BCDU_A::Bit14
    }
}
///Field `BCDU` writer - BCD update
pub type BCDU_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSR_SPEC, u8, BCDU_A, 3, O>;
impl<'a, const O: u8> BCDU_W<'a, O> {
    ///1s increment each time SS\[7:0\]=0
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(BCDU_A::Bit7)
    }
    ///1s increment each time SS\[8:0\]=0
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(BCDU_A::Bit8)
    }
    ///1s increment each time SS\[9:0\]=0
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(BCDU_A::Bit9)
    }
    ///1s increment each time SS\[10:0\]=0
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(BCDU_A::Bit10)
    }
    ///1s increment each time SS\[11:0\]=0
    #[inline(always)]
    pub fn bit11(self) -> &'a mut W {
        self.variant(BCDU_A::Bit11)
    }
    ///1s increment each time SS\[12:0\]=0
    #[inline(always)]
    pub fn bit12(self) -> &'a mut W {
        self.variant(BCDU_A::Bit12)
    }
    ///1s increment each time SS\[13:0\]=0
    #[inline(always)]
    pub fn bit13(self) -> &'a mut W {
        self.variant(BCDU_A::Bit13)
    }
    ///1s increment each time SS\[14:0\]=0
    #[inline(always)]
    pub fn bit14(self) -> &'a mut W {
        self.variant(BCDU_A::Bit14)
    }
}
///Field `RECALPF` reader - Recalibration pending Flag
pub type RECALPF_R = crate::BitReader<RECALPFR_A>;
///Recalibration pending Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECALPFR_A {
    ///1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
    Pending = 1,
}
impl From<RECALPFR_A> for bool {
    #[inline(always)]
    fn from(variant: RECALPFR_A) -> Self {
        variant as u8 != 0
    }
}
impl RECALPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RECALPFR_A> {
        match self.bits {
            true => Some(RECALPFR_A::Pending),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPFR_A::Pending
    }
}
impl R {
    ///Bit 2 - Wakeup timer write flag
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Binary mode
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:12 - BCD update
    #[inline(always)]
    pub fn bcdu(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 16 - Recalibration pending Flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<7> {
        INIT_W::new(self)
    }
    ///Bits 8:9 - Binary mode
    #[inline(always)]
    #[must_use]
    pub fn bin(&mut self) -> BIN_W<8> {
        BIN_W::new(self)
    }
    ///Bits 10:12 - BCD update
    #[inline(always)]
    #[must_use]
    pub fn bcdu(&mut self) -> BCDU_W<10> {
        BCDU_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Initialization control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icsr](index.html) module
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icsr::R](R) reader structure
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icsr::W](W) writer structure
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x20;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICSR to value 0x07
impl crate::Resettable for ICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
