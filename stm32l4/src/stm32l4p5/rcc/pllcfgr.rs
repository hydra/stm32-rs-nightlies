///Register `PLLCFGR` reader
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLCFGR` writer
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_R = crate::FieldReader<u8, PLLSRC_A>;
///Main PLL, PLLSAI1 and PLLSAI2 entry clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC_A {
    ///0: No clock sent to PLL
    NoClock = 0,
    ///1: MSI clock selected as PLL clock entry
    Msi = 1,
    ///2: HSI16 clock selected as PLL clock entry
    Hsi16 = 2,
    ///3: HSE clock selected as PLL clock entry
    Hse = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            0 => PLLSRC_A::NoClock,
            1 => PLLSRC_A::Msi,
            2 => PLLSRC_A::Hsi16,
            3 => PLLSRC_A::Hse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoClock`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLLSRC_A::NoClock
    }
    ///Checks if the value of the field is `Msi`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == PLLSRC_A::Msi
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC_A::Hsi16
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::Hse
    }
}
///Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLSRC_A, 2, O>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    ///No clock sent to PLL
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(PLLSRC_A::NoClock)
    }
    ///MSI clock selected as PLL clock entry
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(PLLSRC_A::Msi)
    }
    ///HSI16 clock selected as PLL clock entry
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hsi16)
    }
    ///HSE clock selected as PLL clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hse)
    }
}
///Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_R = crate::FieldReader<u8, PLLM_A>;
///Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM_A {
    ///0: PLLM = 1
    Div1 = 0,
    ///1: PLLM = 2
    Div2 = 1,
    ///2: PLLM = 3
    Div3 = 2,
    ///3: PLLM = 4
    Div4 = 3,
    ///4: PLLM = 5
    Div5 = 4,
    ///5: PLLM = 6
    Div6 = 5,
    ///6: PLLM = 7
    Div7 = 6,
    ///7: PLLM = 8
    Div8 = 7,
    ///8: PLLM = 9
    Div9 = 8,
    ///9: PLLM = 11
    Div10 = 9,
    ///10: PLLM = 12
    Div11 = 10,
    ///11: PLLM = 13
    Div12 = 11,
    ///12: PLLM = 13
    Div13 = 12,
    ///13: PLLM = 14
    Div14 = 13,
    ///14: PLLM = 15
    Div15 = 14,
    ///15: PLLM = 16
    Div16 = 15,
}
impl From<PLLM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLM_A) -> Self {
        variant as _
    }
}
impl PLLM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLM_A {
        match self.bits {
            0 => PLLM_A::Div1,
            1 => PLLM_A::Div2,
            2 => PLLM_A::Div3,
            3 => PLLM_A::Div4,
            4 => PLLM_A::Div5,
            5 => PLLM_A::Div6,
            6 => PLLM_A::Div7,
            7 => PLLM_A::Div8,
            8 => PLLM_A::Div9,
            9 => PLLM_A::Div10,
            10 => PLLM_A::Div11,
            11 => PLLM_A::Div12,
            12 => PLLM_A::Div13,
            13 => PLLM_A::Div14,
            14 => PLLM_A::Div15,
            15 => PLLM_A::Div16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLM_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLM_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLM_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLM_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLM_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLM_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLM_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLM_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLM_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLM_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLM_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLM_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLM_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLM_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLM_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLM_A::Div16
    }
}
///Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLM_A, 4, O>;
impl<'a, const O: u8> PLLM_W<'a, O> {
    ///PLLM = 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLM_A::Div1)
    }
    ///PLLM = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLM_A::Div2)
    }
    ///PLLM = 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLM_A::Div3)
    }
    ///PLLM = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLM_A::Div4)
    }
    ///PLLM = 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLM_A::Div5)
    }
    ///PLLM = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLM_A::Div6)
    }
    ///PLLM = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLM_A::Div7)
    }
    ///PLLM = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLM_A::Div8)
    }
    ///PLLM = 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLM_A::Div9)
    }
    ///PLLM = 11
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLM_A::Div10)
    }
    ///PLLM = 12
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLM_A::Div11)
    }
    ///PLLM = 13
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLM_A::Div12)
    }
    ///PLLM = 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLM_A::Div13)
    }
    ///PLLM = 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLM_A::Div14)
    }
    ///PLLM = 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLM_A::Div15)
    }
    ///PLLM = 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLM_A::Div16)
    }
}
///Field `PLLN` reader - Main PLL multiplication factor for VCO
pub type PLLN_R = crate::FieldReader<u8, u8>;
///Field `PLLN` writer - Main PLL multiplication factor for VCO
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 7, O>;
///Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable
pub type PLLPEN_R = crate::BitReader<PLLPEN_A>;
///Main PLL PLLSAI3CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPEN_A {
    ///0: PLLSAI3CLK output disable
    Disabled = 0,
    ///1: PLLSAI3CLK output enabled
    Enabled = 1,
}
impl From<PLLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLPEN_A {
        match self.bits {
            false => PLLPEN_A::Disabled,
            true => PLLPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLPEN_A::Enabled
    }
}
///Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLLPEN_A, O>;
impl<'a, const O: u8> PLLPEN_W<'a, O> {
    ///PLLSAI3CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::Disabled)
    }
    ///PLLSAI3CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::Enabled)
    }
}
///Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_R = crate::BitReader<PLLP_A>;
///Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLP_A {
    ///0: PLLP = 7
    Div7 = 0,
    ///1: PLLP = 17
    Div17 = 1,
}
impl From<PLLP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            false => PLLP_A::Div7,
            true => PLLP_A::Div17,
        }
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLP_A::Div7
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP_A::Div17
    }
}
///Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLLP_A, O>;
impl<'a, const O: u8> PLLP_W<'a, O> {
    ///PLLP = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLP_A::Div7)
    }
    ///PLLP = 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLP_A::Div17)
    }
}
///Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_R = crate::BitReader<PLLQEN_A>;
///Main PLL PLLUSB1CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLQEN_A {
    ///0: PLL48M1CLK output disable
    Disabled = 0,
    ///1: PLL48M1CLK output enabled
    Enabled = 1,
}
impl From<PLLQEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLQEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLQEN_A {
        match self.bits {
            false => PLLQEN_A::Disabled,
            true => PLLQEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLQEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLQEN_A::Enabled
    }
}
///Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLLQEN_A, O>;
impl<'a, const O: u8> PLLQEN_W<'a, O> {
    ///PLL48M1CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLQEN_A::Disabled)
    }
    ///PLL48M1CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLQEN_A::Enabled)
    }
}
///Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_R = crate::FieldReader<u8, PLLQ_A>;
///Main PLL division factor for PLLUSB1CLK(48 MHz clock)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ_A {
    ///0: PLLx = 2
    Div2 = 0,
    ///1: PLLx = 4
    Div4 = 1,
    ///2: PLLx = 6
    Div6 = 2,
    ///3: PLLx = 8
    Div8 = 3,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        variant as _
    }
}
impl PLLQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLQ_A {
        match self.bits {
            0 => PLLQ_A::Div2,
            1 => PLLQ_A::Div4,
            2 => PLLQ_A::Div6,
            3 => PLLQ_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLQ_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ_A::Div8
    }
}
///Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLQ_A, 2, O>;
impl<'a, const O: u8> PLLQ_W<'a, O> {
    ///PLLx = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLQ_A::Div2)
    }
    ///PLLx = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLQ_A::Div4)
    }
    ///PLLx = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLQ_A::Div6)
    }
    ///PLLx = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLQ_A::Div8)
    }
}
///Field `PLLREN` reader - Main PLL PLLCLK output enable
pub type PLLREN_R = crate::BitReader<PLLREN_A>;
///Main PLL PLLCLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLREN_A {
    ///0: PLLCLK output disable
    Disabled = 0,
    ///1: PLLCLK output enabled
    Enabled = 1,
}
impl From<PLLREN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLREN_A {
        match self.bits {
            false => PLLREN_A::Disabled,
            true => PLLREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLREN_A::Enabled
    }
}
///Field `PLLREN` writer - Main PLL PLLCLK output enable
pub type PLLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLLREN_A, O>;
impl<'a, const O: u8> PLLREN_W<'a, O> {
    ///PLLCLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLREN_A::Disabled)
    }
    ///PLLCLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLREN_A::Enabled)
    }
}
///Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)
pub use PLLQ_R as PLLR_R;
///Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)
pub use PLLQ_W as PLLR_W;
///Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_R = crate::FieldReader<u8, PLLPDIV_A>;
///Main PLL division factor for PLLSAI2CLK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLPDIV_A {
    ///0: PLLSAI3CLK is controlled by the bit PLLP
    Pllp = 0,
    ///2: PLLSAI3CLK = VCO / 2
    Div2 = 2,
    ///3: PLLSAI3CLK = VCO / 3
    Div3 = 3,
    ///4: PLLSAI3CLK = VCO / 4
    Div4 = 4,
    ///5: PLLSAI3CLK = VCO / 5
    Div5 = 5,
    ///6: PLLSAI3CLK = VCO / 6
    Div6 = 6,
    ///7: PLLSAI3CLK = VCO / 7
    Div7 = 7,
    ///8: PLLSAI3CLK = VCO / 8
    Div8 = 8,
    ///9: PLLSAI3CLK = VCO / 9
    Div9 = 9,
    ///10: PLLSAI3CLK = VCO / 10
    Div10 = 10,
    ///11: PLLSAI3CLK = VCO / 11
    Div11 = 11,
    ///12: PLLSAI3CLK = VCO / 12
    Div12 = 12,
    ///13: PLLSAI3CLK = VCO / 13
    Div13 = 13,
    ///14: PLLSAI3CLK = VCO / 14
    Div14 = 14,
    ///15: PLLSAI3CLK = VCO / 15
    Div15 = 15,
    ///16: PLLSAI3CLK = VCO / 16
    Div16 = 16,
    ///17: PLLSAI3CLK = VCO / 17
    Div17 = 17,
    ///18: PLLSAI3CLK = VCO / 18
    Div18 = 18,
    ///19: PLLSAI3CLK = VCO / 19
    Div19 = 19,
    ///20: PLLSAI3CLK = VCO / 20
    Div20 = 20,
    ///21: PLLSAI3CLK = VCO / 21
    Div21 = 21,
    ///22: PLLSAI3CLK = VCO / 22
    Div22 = 22,
    ///23: PLLSAI3CLK = VCO / 23
    Div23 = 23,
    ///24: PLLSAI3CLK = VCO / 24
    Div24 = 24,
    ///25: PLLSAI3CLK = VCO / 25
    Div25 = 25,
    ///26: PLLSAI3CLK = VCO / 26
    Div26 = 26,
    ///27: PLLSAI3CLK = VCO / 27
    Div27 = 27,
    ///28: PLLSAI3CLK = VCO / 28
    Div28 = 28,
    ///29: PLLSAI3CLK = VCO / 29
    Div29 = 29,
    ///30: PLLSAI3CLK = VCO / 30
    Div30 = 30,
    ///31: PLLSAI3CLK = VCO / 31
    Div31 = 31,
}
impl From<PLLPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLPDIV_A) -> Self {
        variant as _
    }
}
impl PLLPDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLPDIV_A> {
        match self.bits {
            0 => Some(PLLPDIV_A::Pllp),
            2 => Some(PLLPDIV_A::Div2),
            3 => Some(PLLPDIV_A::Div3),
            4 => Some(PLLPDIV_A::Div4),
            5 => Some(PLLPDIV_A::Div5),
            6 => Some(PLLPDIV_A::Div6),
            7 => Some(PLLPDIV_A::Div7),
            8 => Some(PLLPDIV_A::Div8),
            9 => Some(PLLPDIV_A::Div9),
            10 => Some(PLLPDIV_A::Div10),
            11 => Some(PLLPDIV_A::Div11),
            12 => Some(PLLPDIV_A::Div12),
            13 => Some(PLLPDIV_A::Div13),
            14 => Some(PLLPDIV_A::Div14),
            15 => Some(PLLPDIV_A::Div15),
            16 => Some(PLLPDIV_A::Div16),
            17 => Some(PLLPDIV_A::Div17),
            18 => Some(PLLPDIV_A::Div18),
            19 => Some(PLLPDIV_A::Div19),
            20 => Some(PLLPDIV_A::Div20),
            21 => Some(PLLPDIV_A::Div21),
            22 => Some(PLLPDIV_A::Div22),
            23 => Some(PLLPDIV_A::Div23),
            24 => Some(PLLPDIV_A::Div24),
            25 => Some(PLLPDIV_A::Div25),
            26 => Some(PLLPDIV_A::Div26),
            27 => Some(PLLPDIV_A::Div27),
            28 => Some(PLLPDIV_A::Div28),
            29 => Some(PLLPDIV_A::Div29),
            30 => Some(PLLPDIV_A::Div30),
            31 => Some(PLLPDIV_A::Div31),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pllp`
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == PLLPDIV_A::Pllp
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLPDIV_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLPDIV_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLPDIV_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLPDIV_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLPDIV_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLPDIV_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLPDIV_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLPDIV_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLPDIV_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLPDIV_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLPDIV_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLPDIV_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLPDIV_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLPDIV_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLPDIV_A::Div16
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLPDIV_A::Div17
    }
    ///Checks if the value of the field is `Div18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLPDIV_A::Div18
    }
    ///Checks if the value of the field is `Div19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLPDIV_A::Div19
    }
    ///Checks if the value of the field is `Div20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLPDIV_A::Div20
    }
    ///Checks if the value of the field is `Div21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLPDIV_A::Div21
    }
    ///Checks if the value of the field is `Div22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLPDIV_A::Div22
    }
    ///Checks if the value of the field is `Div23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLPDIV_A::Div23
    }
    ///Checks if the value of the field is `Div24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLPDIV_A::Div24
    }
    ///Checks if the value of the field is `Div25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLPDIV_A::Div25
    }
    ///Checks if the value of the field is `Div26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLPDIV_A::Div26
    }
    ///Checks if the value of the field is `Div27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLPDIV_A::Div27
    }
    ///Checks if the value of the field is `Div28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLPDIV_A::Div28
    }
    ///Checks if the value of the field is `Div29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLPDIV_A::Div29
    }
    ///Checks if the value of the field is `Div30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLPDIV_A::Div30
    }
    ///Checks if the value of the field is `Div31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLPDIV_A::Div31
    }
}
///Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, PLLPDIV_A, 5, O>;
impl<'a, const O: u8> PLLPDIV_W<'a, O> {
    ///PLLSAI3CLK is controlled by the bit PLLP
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Pllp)
    }
    ///PLLSAI3CLK = VCO / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div2)
    }
    ///PLLSAI3CLK = VCO / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div3)
    }
    ///PLLSAI3CLK = VCO / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div4)
    }
    ///PLLSAI3CLK = VCO / 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div5)
    }
    ///PLLSAI3CLK = VCO / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div6)
    }
    ///PLLSAI3CLK = VCO / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div7)
    }
    ///PLLSAI3CLK = VCO / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div8)
    }
    ///PLLSAI3CLK = VCO / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div9)
    }
    ///PLLSAI3CLK = VCO / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div10)
    }
    ///PLLSAI3CLK = VCO / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div11)
    }
    ///PLLSAI3CLK = VCO / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div12)
    }
    ///PLLSAI3CLK = VCO / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div13)
    }
    ///PLLSAI3CLK = VCO / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div14)
    }
    ///PLLSAI3CLK = VCO / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div15)
    }
    ///PLLSAI3CLK = VCO / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div16)
    }
    ///PLLSAI3CLK = VCO / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div17)
    }
    ///PLLSAI3CLK = VCO / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div18)
    }
    ///PLLSAI3CLK = VCO / 19
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div19)
    }
    ///PLLSAI3CLK = VCO / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div20)
    }
    ///PLLSAI3CLK = VCO / 21
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div21)
    }
    ///PLLSAI3CLK = VCO / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div22)
    }
    ///PLLSAI3CLK = VCO / 23
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div23)
    }
    ///PLLSAI3CLK = VCO / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div24)
    }
    ///PLLSAI3CLK = VCO / 25
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div25)
    }
    ///PLLSAI3CLK = VCO / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div26)
    }
    ///PLLSAI3CLK = VCO / 27
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div27)
    }
    ///PLLSAI3CLK = VCO / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div28)
    }
    ///PLLSAI3CLK = VCO / 29
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div29)
    }
    ///PLLSAI3CLK = VCO / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div30)
    }
    ///PLLSAI3CLK = VCO / 31
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLPDIV_A::Div31)
    }
}
impl R {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<4> {
        PLLM_W::new(self)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<8> {
        PLLN_W::new(self)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<20> {
        PLLQEN_W::new(self)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<21> {
        PLLQ_W::new(self)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<24> {
        PLLREN_W::new(self)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<25> {
        PLLR_W::new(self)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    #[must_use]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W<27> {
        PLLPDIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PLL configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllcfgr](index.html) module
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllcfgr::R](R) reader structure
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllcfgr::W](W) writer structure
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLCFGR to value 0x1000
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
