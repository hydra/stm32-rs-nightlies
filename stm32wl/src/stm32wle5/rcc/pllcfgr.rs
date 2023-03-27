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
///Field `PLLSRC` reader - Main PLL entry clock source
pub type PLLSRC_R = crate::FieldReader<u8, PLLSRC_A>;
///Main PLL entry clock source
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
    ///3: HSE32 clock selected as PLL clock entry
    Hse32 = 3,
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
            3 => PLLSRC_A::Hse32,
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
    ///Checks if the value of the field is `Hse32`
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == PLLSRC_A::Hse32
    }
}
///Field `PLLSRC` writer - Main PLL entry clock source
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
    ///HSE32 clock selected as PLL clock entry
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hse32)
    }
}
///Field `PLLM` reader - Division factor for the main PLL input clock
pub type PLLM_R = crate::FieldReader<u8, PLLM_A>;
///Division factor for the main PLL input clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM_A {
    ///0: VCO input = PLL input / PLLM
    Div1 = 0,
    ///1: VCO input = PLL input / PLLM
    Div2 = 1,
    ///2: VCO input = PLL input / PLLM
    Div3 = 2,
    ///3: VCO input = PLL input / PLLM
    Div4 = 3,
    ///4: VCO input = PLL input / PLLM
    Div5 = 4,
    ///5: VCO input = PLL input / PLLM
    Div6 = 5,
    ///6: VCO input = PLL input / PLLM
    Div7 = 6,
    ///7: VCO input = PLL input / PLLM
    Div8 = 7,
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
}
///Field `PLLM` writer - Division factor for the main PLL input clock
pub type PLLM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLLM_A, 3, O>;
impl<'a, const O: u8> PLLM_W<'a, O> {
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLM_A::Div1)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLM_A::Div2)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLM_A::Div3)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLM_A::Div4)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLM_A::Div5)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLM_A::Div6)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLM_A::Div7)
    }
    ///VCO input = PLL input / PLLM
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLM_A::Div8)
    }
}
///Field `PLLN` reader - Main PLL multiplication factor for VCO
pub type PLLN_R = crate::FieldReader<u8, u8>;
///Field `PLLN` writer - Main PLL multiplication factor for VCO
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 7, O>;
///Field `PLLPEN` reader - Main PLL PLLPCLK output enable
pub type PLLPEN_R = crate::BitReader<PLLPEN_A>;
///Main PLL PLLPCLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPEN_A {
    ///0: PLLCLK output disabled
    Disabled = 0,
    ///1: PLLCLK output enabled
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
///Field `PLLPEN` writer - Main PLL PLLPCLK output enable
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLLPEN_A, O>;
impl<'a, const O: u8> PLLPEN_W<'a, O> {
    ///PLLCLK output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::Disabled)
    }
    ///PLLCLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::Enabled)
    }
}
///Field `PLLP` reader - Main PLL division factor for PLLPCLK.
pub type PLLP_R = crate::FieldReader<u8, PLLP_A>;
///Main PLL division factor for PLLPCLK.
///
///Value on reset: 2
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP_A {
    ///1: PLL = VCO/(N+1)
    Div2 = 1,
    ///2: PLL = VCO/(N+1)
    Div3 = 2,
    ///3: PLL = VCO/(N+1)
    Div4 = 3,
    ///4: PLL = VCO/(N+1)
    Div5 = 4,
    ///5: PLL = VCO/(N+1)
    Div6 = 5,
    ///6: PLL = VCO/(N+1)
    Div7 = 6,
    ///7: PLL = VCO/(N+1)
    Div8 = 7,
    ///8: PLL = VCO/(N+1)
    Div9 = 8,
    ///9: PLL = VCO/(N+1)
    Div10 = 9,
    ///10: PLL = VCO/(N+1)
    Div11 = 10,
    ///11: PLL = VCO/(N+1)
    Div12 = 11,
    ///12: PLL = VCO/(N+1)
    Div13 = 12,
    ///13: PLL = VCO/(N+1)
    Div14 = 13,
    ///14: PLL = VCO/(N+1)
    Div15 = 14,
    ///15: PLL = VCO/(N+1)
    Div16 = 15,
    ///16: PLL = VCO/(N+1)
    Div17 = 16,
    ///17: PLL = VCO/(N+1)
    Div18 = 17,
    ///18: PLL = VCO/(N+1)
    Div19 = 18,
    ///19: PLL = VCO/(N+1)
    Div20 = 19,
    ///20: PLL = VCO/(N+1)
    Div21 = 20,
    ///21: PLL = VCO/(N+1)
    Div22 = 21,
    ///22: PLL = VCO/(N+1)
    Div23 = 22,
    ///23: PLL = VCO/(N+1)
    Div24 = 23,
    ///24: PLL = VCO/(N+1)
    Div25 = 24,
    ///25: PLL = VCO/(N+1)
    Div26 = 25,
    ///26: PLL = VCO/(N+1)
    Div27 = 26,
    ///27: PLL = VCO/(N+1)
    Div28 = 27,
    ///28: PLL = VCO/(N+1)
    Div29 = 28,
    ///29: PLL = VCO/(N+1)
    Div30 = 29,
    ///30: PLL = VCO/(N+1)
    Div31 = 30,
    ///31: PLL = VCO/(N+1)
    Div32 = 31,
}
impl From<PLLP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as _
    }
}
impl PLLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLP_A> {
        match self.bits {
            1 => Some(PLLP_A::Div2),
            2 => Some(PLLP_A::Div3),
            3 => Some(PLLP_A::Div4),
            4 => Some(PLLP_A::Div5),
            5 => Some(PLLP_A::Div6),
            6 => Some(PLLP_A::Div7),
            7 => Some(PLLP_A::Div8),
            8 => Some(PLLP_A::Div9),
            9 => Some(PLLP_A::Div10),
            10 => Some(PLLP_A::Div11),
            11 => Some(PLLP_A::Div12),
            12 => Some(PLLP_A::Div13),
            13 => Some(PLLP_A::Div14),
            14 => Some(PLLP_A::Div15),
            15 => Some(PLLP_A::Div16),
            16 => Some(PLLP_A::Div17),
            17 => Some(PLLP_A::Div18),
            18 => Some(PLLP_A::Div19),
            19 => Some(PLLP_A::Div20),
            20 => Some(PLLP_A::Div21),
            21 => Some(PLLP_A::Div22),
            22 => Some(PLLP_A::Div23),
            23 => Some(PLLP_A::Div24),
            24 => Some(PLLP_A::Div25),
            25 => Some(PLLP_A::Div26),
            26 => Some(PLLP_A::Div27),
            27 => Some(PLLP_A::Div28),
            28 => Some(PLLP_A::Div29),
            29 => Some(PLLP_A::Div30),
            30 => Some(PLLP_A::Div31),
            31 => Some(PLLP_A::Div32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLP_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLP_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLP_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLP_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLP_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLP_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLP_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLP_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLP_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLP_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLP_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLP_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLP_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLP_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLP_A::Div16
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP_A::Div17
    }
    ///Checks if the value of the field is `Div18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLP_A::Div18
    }
    ///Checks if the value of the field is `Div19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLP_A::Div19
    }
    ///Checks if the value of the field is `Div20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLP_A::Div20
    }
    ///Checks if the value of the field is `Div21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLP_A::Div21
    }
    ///Checks if the value of the field is `Div22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLP_A::Div22
    }
    ///Checks if the value of the field is `Div23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLP_A::Div23
    }
    ///Checks if the value of the field is `Div24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLP_A::Div24
    }
    ///Checks if the value of the field is `Div25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLP_A::Div25
    }
    ///Checks if the value of the field is `Div26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLP_A::Div26
    }
    ///Checks if the value of the field is `Div27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLP_A::Div27
    }
    ///Checks if the value of the field is `Div28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLP_A::Div28
    }
    ///Checks if the value of the field is `Div29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLP_A::Div29
    }
    ///Checks if the value of the field is `Div30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLP_A::Div30
    }
    ///Checks if the value of the field is `Div31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLP_A::Div31
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLP_A::Div32
    }
}
///Field `PLLP` writer - Main PLL division factor for PLLPCLK.
pub type PLLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, PLLP_A, 5, O>;
impl<'a, const O: u8> PLLP_W<'a, O> {
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLP_A::Div2)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLP_A::Div3)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLP_A::Div4)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLP_A::Div5)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLP_A::Div6)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLP_A::Div7)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLP_A::Div8)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLP_A::Div9)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLP_A::Div10)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLP_A::Div11)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLP_A::Div12)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLP_A::Div13)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLP_A::Div14)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLP_A::Div15)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLP_A::Div16)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLP_A::Div17)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLP_A::Div18)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLP_A::Div19)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLP_A::Div20)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLP_A::Div21)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLP_A::Div22)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLP_A::Div23)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLP_A::Div24)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLP_A::Div25)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLP_A::Div26)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLP_A::Div27)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLP_A::Div28)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLP_A::Div29)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLP_A::Div30)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLP_A::Div31)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLP_A::Div32)
    }
}
///Field `PLLQEN` reader - Main PLL PLLQCLK output enable
pub use PLLPEN_R as PLLQEN_R;
///Field `PLLQEN` writer - Main PLL PLLQCLK output enable
pub use PLLPEN_W as PLLQEN_W;
///Field `PLLQ` reader - Main PLL division factor for PLLQCLK
pub type PLLQ_R = crate::FieldReader<u8, PLLQ_A>;
///Main PLL division factor for PLLQCLK
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ_A {
    ///1: PLL = VCO/(N+1)
    Div2 = 1,
    ///2: PLL = VCO/(N+1)
    Div3 = 2,
    ///3: PLL = VCO/(N+1)
    Div4 = 3,
    ///4: PLL = VCO/(N+1)
    Div5 = 4,
    ///5: PLL = VCO/(N+1)
    Div6 = 5,
    ///6: PLL = VCO/(N+1)
    Div7 = 6,
    ///7: PLL = VCO/(N+1)
    Div8 = 7,
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
    pub fn variant(&self) -> Option<PLLQ_A> {
        match self.bits {
            1 => Some(PLLQ_A::Div2),
            2 => Some(PLLQ_A::Div3),
            3 => Some(PLLQ_A::Div4),
            4 => Some(PLLQ_A::Div5),
            5 => Some(PLLQ_A::Div6),
            6 => Some(PLLQ_A::Div7),
            7 => Some(PLLQ_A::Div8),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLQ_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLQ_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLQ_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLQ_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ_A::Div8
    }
}
///Field `PLLQ` writer - Main PLL division factor for PLLQCLK
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, PLLQ_A, 3, O>;
impl<'a, const O: u8> PLLQ_W<'a, O> {
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLQ_A::Div2)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLQ_A::Div3)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLQ_A::Div4)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLQ_A::Div5)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLQ_A::Div6)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLQ_A::Div7)
    }
    ///PLL = VCO/(N+1)
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLQ_A::Div8)
    }
}
///Field `PLLREN` reader - Main PLL PLLRCLK output enable
pub use PLLPEN_R as PLLREN_R;
///Field `PLLREN` writer - Main PLL PLLRCLK output enable
pub use PLLPEN_W as PLLREN_W;
///Field `PLLR` reader - Main PLL division factor for PLLRCLK
pub use PLLQ_R as PLLR_R;
///Field `PLLR` writer - Main PLL division factor for PLLRCLK
pub use PLLQ_W as PLLR_W;
impl R {
    ///Bits 0:1 - Main PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Division factor for the main PLL input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Main PLL PLLPCLK output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - Main PLL division factor for PLLPCLK.
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - Main PLL PLLQCLK output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - Main PLL division factor for PLLQCLK
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - Main PLL PLLRCLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - Main PLL division factor for PLLRCLK
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:1 - Main PLL entry clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    ///Bits 4:6 - Division factor for the main PLL input clock
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
    ///Bit 16 - Main PLL PLLPCLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    ///Bits 17:21 - Main PLL division factor for PLLPCLK.
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    ///Bit 24 - Main PLL PLLQCLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<24> {
        PLLQEN_W::new(self)
    }
    ///Bits 25:27 - Main PLL division factor for PLLQCLK
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<25> {
        PLLQ_W::new(self)
    }
    ///Bit 28 - Main PLL PLLRCLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<28> {
        PLLREN_W::new(self)
    }
    ///Bits 29:31 - Main PLL division factor for PLLRCLK
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<29> {
        PLLR_W::new(self)
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
///`reset()` method sets PLLCFGR to value 0x2204_0100
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2204_0100;
}
