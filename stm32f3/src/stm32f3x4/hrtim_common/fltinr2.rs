///Register `FLTINR2` reader
pub struct R(crate::R<FLTINR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLTINR2` writer
pub struct W(crate::W<FLTINR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR2_SPEC>;
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
impl From<crate::W<FLTINR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FLT5E` reader - FLT5E
pub type FLT5E_R = crate::BitReader<FLT5E_A>;
///FLT5E
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5E_A {
    ///0: Fault input disabled
    Disabled = 0,
    ///1: Fault input enabled
    Enabled = 1,
}
impl From<FLT5E_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5E_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT5E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLT5E_A {
        match self.bits {
            false => FLT5E_A::Disabled,
            true => FLT5E_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5E_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT5E_A::Enabled
    }
}
///Field `FLT5E` writer - FLT5E
pub type FLT5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, FLT5E_A, O>;
impl<'a, const O: u8> FLT5E_W<'a, O> {
    ///Fault input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT5E_A::Disabled)
    }
    ///Fault input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT5E_A::Enabled)
    }
}
///Field `FLT5P` reader - FLT5P
pub type FLT5P_R = crate::BitReader<FLT5P_A>;
///FLT5P
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5P_A {
    ///0: Fault input is active low
    ActiveLow = 0,
    ///1: Fault input is active high
    ActiveHigh = 1,
}
impl From<FLT5P_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5P_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT5P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLT5P_A {
        match self.bits {
            false => FLT5P_A::ActiveLow,
            true => FLT5P_A::ActiveHigh,
        }
    }
    ///Checks if the value of the field is `ActiveLow`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FLT5P_A::ActiveLow
    }
    ///Checks if the value of the field is `ActiveHigh`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FLT5P_A::ActiveHigh
    }
}
///Field `FLT5P` writer - FLT5P
pub type FLT5P_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, FLT5P_A, O>;
impl<'a, const O: u8> FLT5P_W<'a, O> {
    ///Fault input is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FLT5P_A::ActiveLow)
    }
    ///Fault input is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FLT5P_A::ActiveHigh)
    }
}
///Field `FLT5SRC` reader - FLT5SRC
pub type FLT5SRC_R = crate::BitReader<FLT5SRC_A>;
///FLT5SRC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5SRC_A {
    ///0: Fault input is FLTx input pin
    Input = 0,
    ///1: Fault input is FLTn_Int signal
    Internal = 1,
}
impl From<FLT5SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT5SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLT5SRC_A {
        match self.bits {
            false => FLT5SRC_A::Input,
            true => FLT5SRC_A::Internal,
        }
    }
    ///Checks if the value of the field is `Input`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FLT5SRC_A::Input
    }
    ///Checks if the value of the field is `Internal`
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == FLT5SRC_A::Internal
    }
}
///Field `FLT5SRC` writer - FLT5SRC
pub type FLT5SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, FLT5SRC_A, O>;
impl<'a, const O: u8> FLT5SRC_W<'a, O> {
    ///Fault input is FLTx input pin
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FLT5SRC_A::Input)
    }
    ///Fault input is FLTn_Int signal
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FLT5SRC_A::Internal)
    }
}
///Field `FLT5F` reader - FLT5F
pub type FLT5F_R = crate::FieldReader<u8, FLT5F_A>;
///FLT5F
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT5F_A {
    ///0: No filter, FLTx acts asynchronously
    Disabled = 0,
    ///1: f_SAMPLING=f_HRTIM, N=2
    Div1N2 = 1,
    ///2: f_SAMPLING=f_HRTIM, N=4
    Div1N4 = 2,
    ///3: f_SAMPLING=f_HRTIM, N=8
    Div1N8 = 3,
    ///4: f_SAMPLING=f_HRTIM/2, N=6
    Div2N6 = 4,
    ///5: f_SAMPLING=f_HRTIM/2, N=8
    Div2N8 = 5,
    ///6: f_SAMPLING=f_HRTIM/4, N=6
    Div4N6 = 6,
    ///7: f_SAMPLING=f_HRTIM/4, N=8
    Div4N8 = 7,
    ///8: f_SAMPLING=f_HRTIM/8, N=6
    Div8N6 = 8,
    ///9: f_SAMPLING=f_HRTIM/8, N=8
    Div8N8 = 9,
    ///10: f_SAMPLING=f_HRTIM/16, N=5
    Div16N5 = 10,
    ///11: f_SAMPLING=f_HRTIM/16, N=6
    Div16N6 = 11,
    ///12: f_SAMPLING=f_HRTIM/16, N=8
    Div16N8 = 12,
    ///13: f_SAMPLING=f_HRTIM/32, N=5
    Div32N5 = 13,
    ///14: f_SAMPLING=f_HRTIM/32, N=6
    Div32N6 = 14,
    ///15: f_SAMPLING=f_HRTIM/32, N=8
    Div32N8 = 15,
}
impl From<FLT5F_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT5F_A) -> Self {
        variant as _
    }
}
impl FLT5F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLT5F_A {
        match self.bits {
            0 => FLT5F_A::Disabled,
            1 => FLT5F_A::Div1N2,
            2 => FLT5F_A::Div1N4,
            3 => FLT5F_A::Div1N8,
            4 => FLT5F_A::Div2N6,
            5 => FLT5F_A::Div2N8,
            6 => FLT5F_A::Div4N6,
            7 => FLT5F_A::Div4N8,
            8 => FLT5F_A::Div8N6,
            9 => FLT5F_A::Div8N8,
            10 => FLT5F_A::Div16N5,
            11 => FLT5F_A::Div16N6,
            12 => FLT5F_A::Div16N8,
            13 => FLT5F_A::Div32N5,
            14 => FLT5F_A::Div32N6,
            15 => FLT5F_A::Div32N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5F_A::Disabled
    }
    ///Checks if the value of the field is `Div1N2`
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == FLT5F_A::Div1N2
    }
    ///Checks if the value of the field is `Div1N4`
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == FLT5F_A::Div1N4
    }
    ///Checks if the value of the field is `Div1N8`
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == FLT5F_A::Div1N8
    }
    ///Checks if the value of the field is `Div2N6`
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == FLT5F_A::Div2N6
    }
    ///Checks if the value of the field is `Div2N8`
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == FLT5F_A::Div2N8
    }
    ///Checks if the value of the field is `Div4N6`
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == FLT5F_A::Div4N6
    }
    ///Checks if the value of the field is `Div4N8`
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == FLT5F_A::Div4N8
    }
    ///Checks if the value of the field is `Div8N6`
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == FLT5F_A::Div8N6
    }
    ///Checks if the value of the field is `Div8N8`
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == FLT5F_A::Div8N8
    }
    ///Checks if the value of the field is `Div16N5`
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == FLT5F_A::Div16N5
    }
    ///Checks if the value of the field is `Div16N6`
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == FLT5F_A::Div16N6
    }
    ///Checks if the value of the field is `Div16N8`
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == FLT5F_A::Div16N8
    }
    ///Checks if the value of the field is `Div32N5`
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == FLT5F_A::Div32N5
    }
    ///Checks if the value of the field is `Div32N6`
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == FLT5F_A::Div32N6
    }
    ///Checks if the value of the field is `Div32N8`
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == FLT5F_A::Div32N8
    }
}
///Field `FLT5F` writer - FLT5F
pub type FLT5F_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLTINR2_SPEC, u8, FLT5F_A, 4, O>;
impl<'a, const O: u8> FLT5F_W<'a, O> {
    ///No filter, FLTx acts asynchronously
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT5F_A::Disabled)
    }
    ///f_SAMPLING=f_HRTIM, N=2
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(FLT5F_A::Div1N2)
    }
    ///f_SAMPLING=f_HRTIM, N=4
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(FLT5F_A::Div1N4)
    }
    ///f_SAMPLING=f_HRTIM, N=8
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::Div1N8)
    }
    ///f_SAMPLING=f_HRTIM/2, N=6
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::Div2N6)
    }
    ///f_SAMPLING=f_HRTIM/2, N=8
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::Div2N8)
    }
    ///f_SAMPLING=f_HRTIM/4, N=6
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::Div4N6)
    }
    ///f_SAMPLING=f_HRTIM/4, N=8
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::Div4N8)
    }
    ///f_SAMPLING=f_HRTIM/8, N=6
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::Div8N6)
    }
    ///f_SAMPLING=f_HRTIM/8, N=8
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::Div8N8)
    }
    ///f_SAMPLING=f_HRTIM/16, N=5
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(FLT5F_A::Div16N5)
    }
    ///f_SAMPLING=f_HRTIM/16, N=6
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::Div16N6)
    }
    ///f_SAMPLING=f_HRTIM/16, N=8
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::Div16N8)
    }
    ///f_SAMPLING=f_HRTIM/32, N=5
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(FLT5F_A::Div32N5)
    }
    ///f_SAMPLING=f_HRTIM/32, N=6
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::Div32N6)
    }
    ///f_SAMPLING=f_HRTIM/32, N=8
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::Div32N8)
    }
}
///Field `FLT5LCK` reader - FLT5LCK
pub type FLT5LCK_R = crate::BitReader<FLT5LCKR_A>;
///FLT5LCK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5LCKR_A {
    ///0: Fault bits are read/write
    Unlocked = 0,
    ///1: Fault bits are read-only
    Locked = 1,
}
impl From<FLT5LCKR_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCKR_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT5LCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLT5LCKR_A {
        match self.bits {
            false => FLT5LCKR_A::Unlocked,
            true => FLT5LCKR_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLT5LCKR_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLT5LCKR_A::Locked
    }
}
///FLT5LCK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5LCKW_AW {
    ///1: Lock corresponding fault bits
    Lock = 1,
}
impl From<FLT5LCKW_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCKW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT5LCK` writer - FLT5LCK
pub type FLT5LCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, FLT5LCKW_AW, O>;
impl<'a, const O: u8> FLT5LCK_W<'a, O> {
    ///Lock corresponding fault bits
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(FLT5LCKW_AW::Lock)
    }
}
///Field `FLTSD` reader - FLTSD
pub type FLTSD_R = crate::FieldReader<u8, FLTSD_A>;
///FLTSD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLTSD_A {
    ///0: f_FLTS=f_HRTIM
    Div1 = 0,
    ///1: f_FLTS=f_HRTIM/2
    Div2 = 1,
    ///2: f_FLTS=f_HRTIM/4
    Div4 = 2,
    ///3: f_FLTS=f_HRTIM/8
    Div8 = 3,
}
impl From<FLTSD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTSD_A) -> Self {
        variant as _
    }
}
impl FLTSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLTSD_A {
        match self.bits {
            0 => FLTSD_A::Div1,
            1 => FLTSD_A::Div2,
            2 => FLTSD_A::Div4,
            3 => FLTSD_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FLTSD_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FLTSD_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FLTSD_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FLTSD_A::Div8
    }
}
///Field `FLTSD` writer - FLTSD
pub type FLTSD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLTINR2_SPEC, u8, FLTSD_A, 2, O>;
impl<'a, const O: u8> FLTSD_W<'a, O> {
    ///f_FLTS=f_HRTIM
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(FLTSD_A::Div1)
    }
    ///f_FLTS=f_HRTIM/2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(FLTSD_A::Div2)
    }
    ///f_FLTS=f_HRTIM/4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(FLTSD_A::Div4)
    }
    ///f_FLTS=f_HRTIM/8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(FLTSD_A::Div8)
    }
}
impl R {
    ///Bit 0 - FLT5E
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FLT5P
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FLT5SRC
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - FLT5F
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bit 7 - FLT5LCK
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 24:25 - FLTSD
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - FLT5E
    #[inline(always)]
    #[must_use]
    pub fn flt5e(&mut self) -> FLT5E_W<0> {
        FLT5E_W::new(self)
    }
    ///Bit 1 - FLT5P
    #[inline(always)]
    #[must_use]
    pub fn flt5p(&mut self) -> FLT5P_W<1> {
        FLT5P_W::new(self)
    }
    ///Bit 2 - FLT5SRC
    #[inline(always)]
    #[must_use]
    pub fn flt5src(&mut self) -> FLT5SRC_W<2> {
        FLT5SRC_W::new(self)
    }
    ///Bits 3:6 - FLT5F
    #[inline(always)]
    #[must_use]
    pub fn flt5f(&mut self) -> FLT5F_W<3> {
        FLT5F_W::new(self)
    }
    ///Bit 7 - FLT5LCK
    #[inline(always)]
    #[must_use]
    pub fn flt5lck(&mut self) -> FLT5LCK_W<7> {
        FLT5LCK_W::new(self)
    }
    ///Bits 24:25 - FLTSD
    #[inline(always)]
    #[must_use]
    pub fn fltsd(&mut self) -> FLTSD_W<24> {
        FLTSD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HRTIM Fault Input Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltinr2](index.html) module
pub struct FLTINR2_SPEC;
impl crate::RegisterSpec for FLTINR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [fltinr2::R](R) reader structure
impl crate::Readable for FLTINR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fltinr2::W](W) writer structure
impl crate::Writable for FLTINR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLTINR2 to value 0
impl crate::Resettable for FLTINR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
