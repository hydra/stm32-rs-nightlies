///Register `PLLSAI1CFGR` reader
pub struct R(crate::R<PLLSAI1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAI1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAI1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAI1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLSAI1CFGR` writer
pub struct W(crate::W<PLLSAI1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAI1CFGR_SPEC>;
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
impl From<crate::W<PLLSAI1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAI1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLSAI1M` reader - Division factor for PLLSAI1 input clock
pub type PLLSAI1M_R = crate::FieldReader<u8, PLLSAI1M_A>;
///Division factor for PLLSAI1 input clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI1M_A {
    ///0: PLLSAI1M = 1
    Div1 = 0,
    ///1: PLLSAI1M = 2
    Div2 = 1,
    ///2: PLLSAI1M = 3
    Div3 = 2,
    ///3: PLLSAI1M = 4
    Div4 = 3,
    ///4: PLLSAI1M = 5
    Div5 = 4,
    ///5: PLLSAI1M = 6
    Div6 = 5,
    ///6: PLLSAI1M = 7
    Div7 = 6,
    ///7: PLLSAI1M = 8
    Div8 = 7,
    ///8: PLLSAI1M = 9
    Div9 = 8,
    ///9: PLLSAI1M = 11
    Div10 = 9,
    ///10: PLLSAI1M = 12
    Div11 = 10,
    ///11: PLLSAI1M = 13
    Div12 = 11,
    ///12: PLLSAI1M = 13
    Div13 = 12,
    ///13: PLLSAI1M = 14
    Div14 = 13,
    ///14: PLLSAI1M = 15
    Div15 = 14,
    ///15: PLLSAI1M = 16
    Div16 = 15,
}
impl From<PLLSAI1M_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI1M_A) -> Self {
        variant as _
    }
}
impl PLLSAI1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1M_A {
        match self.bits {
            0 => PLLSAI1M_A::Div1,
            1 => PLLSAI1M_A::Div2,
            2 => PLLSAI1M_A::Div3,
            3 => PLLSAI1M_A::Div4,
            4 => PLLSAI1M_A::Div5,
            5 => PLLSAI1M_A::Div6,
            6 => PLLSAI1M_A::Div7,
            7 => PLLSAI1M_A::Div8,
            8 => PLLSAI1M_A::Div9,
            9 => PLLSAI1M_A::Div10,
            10 => PLLSAI1M_A::Div11,
            11 => PLLSAI1M_A::Div12,
            12 => PLLSAI1M_A::Div13,
            13 => PLLSAI1M_A::Div14,
            14 => PLLSAI1M_A::Div15,
            15 => PLLSAI1M_A::Div16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAI1M_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI1M_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI1M_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI1M_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI1M_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI1M_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI1M_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI1M_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI1M_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI1M_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI1M_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI1M_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI1M_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI1M_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI1M_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI1M_A::Div16
    }
}
///Field `PLLSAI1M` writer - Division factor for PLLSAI1 input clock
pub type PLLSAI1M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLSAI1CFGR_SPEC, u8, PLLSAI1M_A, 4, O>;
impl<'a, const O: u8> PLLSAI1M_W<'a, O> {
    ///PLLSAI1M = 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div1)
    }
    ///PLLSAI1M = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div2)
    }
    ///PLLSAI1M = 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div3)
    }
    ///PLLSAI1M = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div4)
    }
    ///PLLSAI1M = 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div5)
    }
    ///PLLSAI1M = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div6)
    }
    ///PLLSAI1M = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div7)
    }
    ///PLLSAI1M = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div8)
    }
    ///PLLSAI1M = 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div9)
    }
    ///PLLSAI1M = 11
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div10)
    }
    ///PLLSAI1M = 12
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div11)
    }
    ///PLLSAI1M = 13
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div12)
    }
    ///PLLSAI1M = 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div13)
    }
    ///PLLSAI1M = 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div14)
    }
    ///PLLSAI1M = 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div15)
    }
    ///PLLSAI1M = 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAI1M_A::Div16)
    }
}
///Field `PLLSAI1N` reader - SAI1PLL multiplication factor for VCO
pub type PLLSAI1N_R = crate::FieldReader<u8, u8>;
///Field `PLLSAI1N` writer - SAI1PLL multiplication factor for VCO
pub type PLLSAI1N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 7, O>;
///Field `PLLSAI1PEN` reader - SAI1PLL PLLSAI1CLK output enable
pub type PLLSAI1PEN_R = crate::BitReader<PLLSAI1PEN_A>;
///SAI1PLL PLLSAI1CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1PEN_A {
    ///0: PLLSAI1CLK output disable
    Disabled = 0,
    ///1: PLLSAI1CLK output enabled
    Enabled = 1,
}
impl From<PLLSAI1PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1PEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1PEN_A {
        match self.bits {
            false => PLLSAI1PEN_A::Disabled,
            true => PLLSAI1PEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1PEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1PEN_A::Enabled
    }
}
///Field `PLLSAI1PEN` writer - SAI1PLL PLLSAI1CLK output enable
pub type PLLSAI1PEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, PLLSAI1PEN_A, O>;
impl<'a, const O: u8> PLLSAI1PEN_W<'a, O> {
    ///PLLSAI1CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI1PEN_A::Disabled)
    }
    ///PLLSAI1CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI1PEN_A::Enabled)
    }
}
///Field `PLLSAI1P` reader - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
pub type PLLSAI1P_R = crate::BitReader<PLLSAI1P_A>;
///SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1P_A {
    ///0: PLLSAI1P = 7
    Div7 = 0,
    ///1: PLLSAI1P = 17
    Div17 = 1,
}
impl From<PLLSAI1P_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1P_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1P_A {
        match self.bits {
            false => PLLSAI1P_A::Div7,
            true => PLLSAI1P_A::Div17,
        }
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI1P_A::Div7
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI1P_A::Div17
    }
}
///Field `PLLSAI1P` writer - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
pub type PLLSAI1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, PLLSAI1P_A, O>;
impl<'a, const O: u8> PLLSAI1P_W<'a, O> {
    ///PLLSAI1P = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAI1P_A::Div7)
    }
    ///PLLSAI1P = 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAI1P_A::Div17)
    }
}
///Field `PLLSAI1QEN` reader - SAI1PLL PLLUSB2CLK output enable
pub type PLLSAI1QEN_R = crate::BitReader<PLLSAI1QEN_A>;
///SAI1PLL PLLUSB2CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1QEN_A {
    ///0: PLL48M2CLK output disable
    Disabled = 0,
    ///1: PLL48M2CLK output enabled
    Enabled = 1,
}
impl From<PLLSAI1QEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1QEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1QEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1QEN_A {
        match self.bits {
            false => PLLSAI1QEN_A::Disabled,
            true => PLLSAI1QEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1QEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1QEN_A::Enabled
    }
}
///Field `PLLSAI1QEN` writer - SAI1PLL PLLUSB2CLK output enable
pub type PLLSAI1QEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, PLLSAI1QEN_A, O>;
impl<'a, const O: u8> PLLSAI1QEN_W<'a, O> {
    ///PLL48M2CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI1QEN_A::Disabled)
    }
    ///PLL48M2CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI1QEN_A::Enabled)
    }
}
///Field `PLLSAI1Q` reader - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
pub type PLLSAI1Q_R = crate::FieldReader<u8, PLLSAI1Q_A>;
///SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI1Q_A {
    ///0: PLLSAI1x = 2
    Div2 = 0,
    ///1: PLLSAI1x = 4
    Div4 = 1,
    ///2: PLLSAI1x = 6
    Div6 = 2,
    ///3: PLLSAI1x = 8
    Div8 = 3,
}
impl From<PLLSAI1Q_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI1Q_A) -> Self {
        variant as _
    }
}
impl PLLSAI1Q_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1Q_A {
        match self.bits {
            0 => PLLSAI1Q_A::Div2,
            1 => PLLSAI1Q_A::Div4,
            2 => PLLSAI1Q_A::Div6,
            3 => PLLSAI1Q_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI1Q_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI1Q_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI1Q_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI1Q_A::Div8
    }
}
///Field `PLLSAI1Q` writer - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
pub type PLLSAI1Q_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLSAI1CFGR_SPEC, u8, PLLSAI1Q_A, 2, O>;
impl<'a, const O: u8> PLLSAI1Q_W<'a, O> {
    ///PLLSAI1x = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI1Q_A::Div2)
    }
    ///PLLSAI1x = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI1Q_A::Div4)
    }
    ///PLLSAI1x = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI1Q_A::Div6)
    }
    ///PLLSAI1x = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI1Q_A::Div8)
    }
}
///Field `PLLSAI1REN` reader - PLLSAI1 PLLADC1CLK output enable
pub type PLLSAI1REN_R = crate::BitReader<PLLSAI1REN_A>;
///PLLSAI1 PLLADC1CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1REN_A {
    ///0: PLLADC1CLK output disable
    Disabled = 0,
    ///1: PLLADC1CLK output enabled
    Enabled = 1,
}
impl From<PLLSAI1REN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1REN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1REN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1REN_A {
        match self.bits {
            false => PLLSAI1REN_A::Disabled,
            true => PLLSAI1REN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1REN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1REN_A::Enabled
    }
}
///Field `PLLSAI1REN` writer - PLLSAI1 PLLADC1CLK output enable
pub type PLLSAI1REN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, PLLSAI1REN_A, O>;
impl<'a, const O: u8> PLLSAI1REN_W<'a, O> {
    ///PLLADC1CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI1REN_A::Disabled)
    }
    ///PLLADC1CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI1REN_A::Enabled)
    }
}
///Field `PLLSAI1R` reader - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
pub use PLLSAI1Q_R as PLLSAI1R_R;
///Field `PLLSAI1R` writer - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
pub use PLLSAI1Q_W as PLLSAI1R_W;
///Field `PLLSAI1PDIV` reader - PLLSAI1 division factor for PLLSAI1CLK
pub type PLLSAI1PDIV_R = crate::FieldReader<u8, PLLSAI1PDIV_A>;
///PLLSAI1 division factor for PLLSAI1CLK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI1PDIV_A {
    ///0: PLLSAI1CLK is controlled by the bit PLLSAI1P
    Pllsai1p = 0,
    ///2: PLLSAI1CLK = VCOSAI / 2
    Div2 = 2,
    ///3: PLLSAI1CLK = VCOSAI / 3
    Div3 = 3,
    ///4: PLLSAI1CLK = VCOSAI / 4
    Div4 = 4,
    ///5: PLLSAI1CLK = VCOSAI / 5
    Div5 = 5,
    ///6: PLLSAI1CLK = VCOSAI / 6
    Div6 = 6,
    ///7: PLLSAI1CLK = VCOSAI / 7
    Div7 = 7,
    ///8: PLLSAI1CLK = VCOSAI / 8
    Div8 = 8,
    ///9: PLLSAI1CLK = VCOSAI / 9
    Div9 = 9,
    ///10: PLLSAI1CLK = VCOSAI / 10
    Div10 = 10,
    ///11: PLLSAI1CLK = VCOSAI / 11
    Div11 = 11,
    ///12: PLLSAI1CLK = VCOSAI / 12
    Div12 = 12,
    ///13: PLLSAI1CLK = VCOSAI / 13
    Div13 = 13,
    ///14: PLLSAI1CLK = VCOSAI / 14
    Div14 = 14,
    ///15: PLLSAI1CLK = VCOSAI / 15
    Div15 = 15,
    ///16: PLLSAI1CLK = VCOSAI / 16
    Div16 = 16,
    ///17: PLLSAI1CLK = VCOSAI / 17
    Div17 = 17,
    ///18: PLLSAI1CLK = VCOSAI / 18
    Div18 = 18,
    ///19: PLLSAI1CLK = VCOSAI / 19
    Div19 = 19,
    ///20: PLLSAI1CLK = VCOSAI / 20
    Div20 = 20,
    ///21: PLLSAI1CLK = VCOSAI / 21
    Div21 = 21,
    ///22: PLLSAI1CLK = VCOSAI / 22
    Div22 = 22,
    ///23: PLLSAI1CLK = VCOSAI / 23
    Div23 = 23,
    ///24: PLLSAI1CLK = VCOSAI / 24
    Div24 = 24,
    ///25: PLLSAI1CLK = VCOSAI / 25
    Div25 = 25,
    ///26: PLLSAI1CLK = VCOSAI / 26
    Div26 = 26,
    ///27: PLLSAI1CLK = VCOSAI / 27
    Div27 = 27,
    ///28: PLLSAI1CLK = VCOSAI / 28
    Div28 = 28,
    ///29: PLLSAI1CLK = VCOSAI / 29
    Div29 = 29,
    ///30: PLLSAI1CLK = VCOSAI / 30
    Div30 = 30,
    ///31: PLLSAI1CLK = VCOSAI / 31
    Div31 = 31,
}
impl From<PLLSAI1PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI1PDIV_A) -> Self {
        variant as _
    }
}
impl PLLSAI1PDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLSAI1PDIV_A> {
        match self.bits {
            0 => Some(PLLSAI1PDIV_A::Pllsai1p),
            2 => Some(PLLSAI1PDIV_A::Div2),
            3 => Some(PLLSAI1PDIV_A::Div3),
            4 => Some(PLLSAI1PDIV_A::Div4),
            5 => Some(PLLSAI1PDIV_A::Div5),
            6 => Some(PLLSAI1PDIV_A::Div6),
            7 => Some(PLLSAI1PDIV_A::Div7),
            8 => Some(PLLSAI1PDIV_A::Div8),
            9 => Some(PLLSAI1PDIV_A::Div9),
            10 => Some(PLLSAI1PDIV_A::Div10),
            11 => Some(PLLSAI1PDIV_A::Div11),
            12 => Some(PLLSAI1PDIV_A::Div12),
            13 => Some(PLLSAI1PDIV_A::Div13),
            14 => Some(PLLSAI1PDIV_A::Div14),
            15 => Some(PLLSAI1PDIV_A::Div15),
            16 => Some(PLLSAI1PDIV_A::Div16),
            17 => Some(PLLSAI1PDIV_A::Div17),
            18 => Some(PLLSAI1PDIV_A::Div18),
            19 => Some(PLLSAI1PDIV_A::Div19),
            20 => Some(PLLSAI1PDIV_A::Div20),
            21 => Some(PLLSAI1PDIV_A::Div21),
            22 => Some(PLLSAI1PDIV_A::Div22),
            23 => Some(PLLSAI1PDIV_A::Div23),
            24 => Some(PLLSAI1PDIV_A::Div24),
            25 => Some(PLLSAI1PDIV_A::Div25),
            26 => Some(PLLSAI1PDIV_A::Div26),
            27 => Some(PLLSAI1PDIV_A::Div27),
            28 => Some(PLLSAI1PDIV_A::Div28),
            29 => Some(PLLSAI1PDIV_A::Div29),
            30 => Some(PLLSAI1PDIV_A::Div30),
            31 => Some(PLLSAI1PDIV_A::Div31),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pllsai1p`
    #[inline(always)]
    pub fn is_pllsai1p(&self) -> bool {
        *self == PLLSAI1PDIV_A::Pllsai1p
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div16
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div17
    }
    ///Checks if the value of the field is `Div18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div18
    }
    ///Checks if the value of the field is `Div19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div19
    }
    ///Checks if the value of the field is `Div20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div20
    }
    ///Checks if the value of the field is `Div21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div21
    }
    ///Checks if the value of the field is `Div22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div22
    }
    ///Checks if the value of the field is `Div23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div23
    }
    ///Checks if the value of the field is `Div24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div24
    }
    ///Checks if the value of the field is `Div25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div25
    }
    ///Checks if the value of the field is `Div26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div26
    }
    ///Checks if the value of the field is `Div27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div27
    }
    ///Checks if the value of the field is `Div28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div28
    }
    ///Checks if the value of the field is `Div29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div29
    }
    ///Checks if the value of the field is `Div30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div30
    }
    ///Checks if the value of the field is `Div31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAI1PDIV_A::Div31
    }
}
///Field `PLLSAI1PDIV` writer - PLLSAI1 division factor for PLLSAI1CLK
pub type PLLSAI1PDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, PLLSAI1PDIV_A, 5, O>;
impl<'a, const O: u8> PLLSAI1PDIV_W<'a, O> {
    ///PLLSAI1CLK is controlled by the bit PLLSAI1P
    #[inline(always)]
    pub fn pllsai1p(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Pllsai1p)
    }
    ///PLLSAI1CLK = VCOSAI / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div2)
    }
    ///PLLSAI1CLK = VCOSAI / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div3)
    }
    ///PLLSAI1CLK = VCOSAI / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div4)
    }
    ///PLLSAI1CLK = VCOSAI / 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div5)
    }
    ///PLLSAI1CLK = VCOSAI / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div6)
    }
    ///PLLSAI1CLK = VCOSAI / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div7)
    }
    ///PLLSAI1CLK = VCOSAI / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div8)
    }
    ///PLLSAI1CLK = VCOSAI / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div9)
    }
    ///PLLSAI1CLK = VCOSAI / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div10)
    }
    ///PLLSAI1CLK = VCOSAI / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div11)
    }
    ///PLLSAI1CLK = VCOSAI / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div12)
    }
    ///PLLSAI1CLK = VCOSAI / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div13)
    }
    ///PLLSAI1CLK = VCOSAI / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div14)
    }
    ///PLLSAI1CLK = VCOSAI / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div15)
    }
    ///PLLSAI1CLK = VCOSAI / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div16)
    }
    ///PLLSAI1CLK = VCOSAI / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div17)
    }
    ///PLLSAI1CLK = VCOSAI / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div18)
    }
    ///PLLSAI1CLK = VCOSAI / 19
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div19)
    }
    ///PLLSAI1CLK = VCOSAI / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div20)
    }
    ///PLLSAI1CLK = VCOSAI / 21
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div21)
    }
    ///PLLSAI1CLK = VCOSAI / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div22)
    }
    ///PLLSAI1CLK = VCOSAI / 23
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div23)
    }
    ///PLLSAI1CLK = VCOSAI / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div24)
    }
    ///PLLSAI1CLK = VCOSAI / 25
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div25)
    }
    ///PLLSAI1CLK = VCOSAI / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div26)
    }
    ///PLLSAI1CLK = VCOSAI / 27
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div27)
    }
    ///PLLSAI1CLK = VCOSAI / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div28)
    }
    ///PLLSAI1CLK = VCOSAI / 29
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div29)
    }
    ///PLLSAI1CLK = VCOSAI / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div30)
    }
    ///PLLSAI1CLK = VCOSAI / 31
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLSAI1PDIV_A::Div31)
    }
}
impl R {
    ///Bits 4:7 - Division factor for PLLSAI1 input clock
    #[inline(always)]
    pub fn pllsai1m(&self) -> PLLSAI1M_R {
        PLLSAI1M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - SAI1PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai1n(&self) -> PLLSAI1N_R {
        PLLSAI1N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - SAI1PLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllsai1pen(&self) -> PLLSAI1PEN_R {
        PLLSAI1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai1p(&self) -> PLLSAI1P_R {
        PLLSAI1P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - SAI1PLL PLLUSB2CLK output enable
    #[inline(always)]
    pub fn pllsai1qen(&self) -> PLLSAI1QEN_R {
        PLLSAI1QEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
    #[inline(always)]
    pub fn pllsai1q(&self) -> PLLSAI1Q_R {
        PLLSAI1Q_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - PLLSAI1 PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllsai1ren(&self) -> PLLSAI1REN_R {
        PLLSAI1REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai1r(&self) -> PLLSAI1R_R {
        PLLSAI1R_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK
    #[inline(always)]
    pub fn pllsai1pdiv(&self) -> PLLSAI1PDIV_R {
        PLLSAI1PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 4:7 - Division factor for PLLSAI1 input clock
    #[inline(always)]
    #[must_use]
    pub fn pllsai1m(&mut self) -> PLLSAI1M_W<4> {
        PLLSAI1M_W::new(self)
    }
    ///Bits 8:14 - SAI1PLL multiplication factor for VCO
    #[inline(always)]
    #[must_use]
    pub fn pllsai1n(&mut self) -> PLLSAI1N_W<8> {
        PLLSAI1N_W::new(self)
    }
    ///Bit 16 - SAI1PLL PLLSAI1CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai1pen(&mut self) -> PLLSAI1PEN_W<16> {
        PLLSAI1PEN_W::new(self)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    #[must_use]
    pub fn pllsai1p(&mut self) -> PLLSAI1P_W<17> {
        PLLSAI1P_W::new(self)
    }
    ///Bit 20 - SAI1PLL PLLUSB2CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai1qen(&mut self) -> PLLSAI1QEN_W<20> {
        PLLSAI1QEN_W::new(self)
    }
    ///Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
    #[inline(always)]
    #[must_use]
    pub fn pllsai1q(&mut self) -> PLLSAI1Q_W<21> {
        PLLSAI1Q_W::new(self)
    }
    ///Bit 24 - PLLSAI1 PLLADC1CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai1ren(&mut self) -> PLLSAI1REN_W<24> {
        PLLSAI1REN_W::new(self)
    }
    ///Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
    #[inline(always)]
    #[must_use]
    pub fn pllsai1r(&mut self) -> PLLSAI1R_W<25> {
        PLLSAI1R_W::new(self)
    }
    ///Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK
    #[inline(always)]
    #[must_use]
    pub fn pllsai1pdiv(&mut self) -> PLLSAI1PDIV_W<27> {
        PLLSAI1PDIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PLLSAI1 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllsai1cfgr](index.html) module
pub struct PLLSAI1CFGR_SPEC;
impl crate::RegisterSpec for PLLSAI1CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllsai1cfgr::R](R) reader structure
impl crate::Readable for PLLSAI1CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllsai1cfgr::W](W) writer structure
impl crate::Writable for PLLSAI1CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLSAI1CFGR to value 0x1000
impl crate::Resettable for PLLSAI1CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
