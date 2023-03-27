///Register `PLLSAI2CFGR` reader
pub struct R(crate::R<PLLSAI2CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAI2CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAI2CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAI2CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLLSAI2CFGR` writer
pub struct W(crate::W<PLLSAI2CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAI2CFGR_SPEC>;
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
impl From<crate::W<PLLSAI2CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAI2CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLSAI2M` reader - Division factor for PLLSAI2 input clock
pub type PLLSAI2M_R = crate::FieldReader<u8, PLLSAI2M_A>;
///Division factor for PLLSAI2 input clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2M_A {
    ///0: PLLSAI2M = 1
    Div1 = 0,
    ///1: PLLSAI2M = 2
    Div2 = 1,
    ///2: PLLSAI2M = 3
    Div3 = 2,
    ///3: PLLSAI2M = 4
    Div4 = 3,
    ///4: PLLSAI2M = 5
    Div5 = 4,
    ///5: PLLSAI2M = 6
    Div6 = 5,
    ///6: PLLSAI2M = 7
    Div7 = 6,
    ///7: PLLSAI2M = 8
    Div8 = 7,
    ///8: PLLSAI2M = 9
    Div9 = 8,
    ///9: PLLSAI2M = 11
    Div10 = 9,
    ///10: PLLSAI2M = 12
    Div11 = 10,
    ///11: PLLSAI2M = 13
    Div12 = 11,
    ///12: PLLSAI2M = 13
    Div13 = 12,
    ///13: PLLSAI2M = 14
    Div14 = 13,
    ///14: PLLSAI2M = 15
    Div15 = 14,
    ///15: PLLSAI2M = 16
    Div16 = 15,
}
impl From<PLLSAI2M_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2M_A) -> Self {
        variant as _
    }
}
impl PLLSAI2M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2M_A {
        match self.bits {
            0 => PLLSAI2M_A::Div1,
            1 => PLLSAI2M_A::Div2,
            2 => PLLSAI2M_A::Div3,
            3 => PLLSAI2M_A::Div4,
            4 => PLLSAI2M_A::Div5,
            5 => PLLSAI2M_A::Div6,
            6 => PLLSAI2M_A::Div7,
            7 => PLLSAI2M_A::Div8,
            8 => PLLSAI2M_A::Div9,
            9 => PLLSAI2M_A::Div10,
            10 => PLLSAI2M_A::Div11,
            11 => PLLSAI2M_A::Div12,
            12 => PLLSAI2M_A::Div13,
            13 => PLLSAI2M_A::Div14,
            14 => PLLSAI2M_A::Div15,
            15 => PLLSAI2M_A::Div16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAI2M_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2M_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI2M_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2M_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI2M_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2M_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI2M_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2M_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI2M_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI2M_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI2M_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI2M_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI2M_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI2M_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI2M_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI2M_A::Div16
    }
}
///Field `PLLSAI2M` writer - Division factor for PLLSAI2 input clock
pub type PLLSAI2M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLSAI2CFGR_SPEC, u8, PLLSAI2M_A, 4, O>;
impl<'a, const O: u8> PLLSAI2M_W<'a, O> {
    ///PLLSAI2M = 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div1)
    }
    ///PLLSAI2M = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div2)
    }
    ///PLLSAI2M = 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div3)
    }
    ///PLLSAI2M = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div4)
    }
    ///PLLSAI2M = 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div5)
    }
    ///PLLSAI2M = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div6)
    }
    ///PLLSAI2M = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div7)
    }
    ///PLLSAI2M = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div8)
    }
    ///PLLSAI2M = 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div9)
    }
    ///PLLSAI2M = 11
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div10)
    }
    ///PLLSAI2M = 12
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div11)
    }
    ///PLLSAI2M = 13
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div12)
    }
    ///PLLSAI2M = 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div13)
    }
    ///PLLSAI2M = 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div14)
    }
    ///PLLSAI2M = 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div15)
    }
    ///PLLSAI2M = 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAI2M_A::Div16)
    }
}
///Field `PLLSAI2N` reader - SAI2PLL multiplication factor for VCO
pub type PLLSAI2N_R = crate::FieldReader<u8, u8>;
///Field `PLLSAI2N` writer - SAI2PLL multiplication factor for VCO
pub type PLLSAI2N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI2CFGR_SPEC, u8, u8, 7, O>;
///Field `PLLSAI2PEN` reader - SAI2PLL PLLSAI2CLK output enable
pub type PLLSAI2PEN_R = crate::BitReader<PLLSAI2PEN_A>;
///SAI2PLL PLLSAI2CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2PEN_A {
    ///0: PLLSAI2CLK output disable
    Disabled = 0,
    ///1: PLLSAI2CLK output enabled
    Enabled = 1,
}
impl From<PLLSAI2PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2PEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2PEN_A {
        match self.bits {
            false => PLLSAI2PEN_A::Disabled,
            true => PLLSAI2PEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2PEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2PEN_A::Enabled
    }
}
///Field `PLLSAI2PEN` writer - SAI2PLL PLLSAI2CLK output enable
pub type PLLSAI2PEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLLSAI2CFGR_SPEC, PLLSAI2PEN_A, O>;
impl<'a, const O: u8> PLLSAI2PEN_W<'a, O> {
    ///PLLSAI2CLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2PEN_A::Disabled)
    }
    ///PLLSAI2CLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2PEN_A::Enabled)
    }
}
///Field `PLLSAI2P` reader - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
pub type PLLSAI2P_R = crate::BitReader<PLLSAI2P_A>;
///SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2P_A {
    ///0: PLLSAI2P = 7
    Div7 = 0,
    ///1: PLLSAI2P = 17
    Div17 = 1,
}
impl From<PLLSAI2P_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2P_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2P_A {
        match self.bits {
            false => PLLSAI2P_A::Div7,
            true => PLLSAI2P_A::Div17,
        }
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI2P_A::Div7
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI2P_A::Div17
    }
}
///Field `PLLSAI2P` writer - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
pub type PLLSAI2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI2CFGR_SPEC, PLLSAI2P_A, O>;
impl<'a, const O: u8> PLLSAI2P_W<'a, O> {
    ///PLLSAI2P = 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAI2P_A::Div7)
    }
    ///PLLSAI2P = 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAI2P_A::Div17)
    }
}
///Field `PLLSAI2QEN` reader - PLLSAI2 division factor for PLLDISCLK
pub type PLLSAI2QEN_R = crate::BitReader<PLLSAI2QEN_A>;
///PLLSAI2 division factor for PLLDISCLK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2QEN_A {
    ///0: PLLDSICLK output disable
    Disabled = 0,
    ///1: PLLDSICLK output enabled
    Enabled = 1,
}
impl From<PLLSAI2QEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2QEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2QEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2QEN_A {
        match self.bits {
            false => PLLSAI2QEN_A::Disabled,
            true => PLLSAI2QEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2QEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2QEN_A::Enabled
    }
}
///Field `PLLSAI2QEN` writer - PLLSAI2 division factor for PLLDISCLK
pub type PLLSAI2QEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLLSAI2CFGR_SPEC, PLLSAI2QEN_A, O>;
impl<'a, const O: u8> PLLSAI2QEN_W<'a, O> {
    ///PLLDSICLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2QEN_A::Disabled)
    }
    ///PLLDSICLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2QEN_A::Enabled)
    }
}
///Field `PLLSAI2Q` reader - SAI2PLL PLLSAI2CLK output enable
pub type PLLSAI2Q_R = crate::FieldReader<u8, PLLSAI2Q_A>;
///SAI2PLL PLLSAI2CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2Q_A {
    ///0: PLLSAI2x = 2
    Div2 = 0,
    ///1: PLLSAI2x = 4
    Div4 = 1,
    ///2: PLLSAI2x = 6
    Div6 = 2,
    ///3: PLLSAI2x = 8
    Div8 = 3,
}
impl From<PLLSAI2Q_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2Q_A) -> Self {
        variant as _
    }
}
impl PLLSAI2Q_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2Q_A {
        match self.bits {
            0 => PLLSAI2Q_A::Div2,
            1 => PLLSAI2Q_A::Div4,
            2 => PLLSAI2Q_A::Div6,
            3 => PLLSAI2Q_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2Q_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2Q_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2Q_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2Q_A::Div8
    }
}
///Field `PLLSAI2Q` writer - SAI2PLL PLLSAI2CLK output enable
pub type PLLSAI2Q_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLSAI2CFGR_SPEC, u8, PLLSAI2Q_A, 2, O>;
impl<'a, const O: u8> PLLSAI2Q_W<'a, O> {
    ///PLLSAI2x = 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::Div2)
    }
    ///PLLSAI2x = 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::Div4)
    }
    ///PLLSAI2x = 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::Div6)
    }
    ///PLLSAI2x = 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::Div8)
    }
}
///Field `PLLSAI2REN` reader - PLLSAI2 PLLADC2CLK output enable
pub type PLLSAI2REN_R = crate::BitReader<PLLSAI2REN_A>;
///PLLSAI2 PLLADC2CLK output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2REN_A {
    ///0: PLLLCDCLK output disable
    Disabled = 0,
    ///1: PLLLCDCLK output enabled
    Enabled = 1,
}
impl From<PLLSAI2REN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2REN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2REN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2REN_A {
        match self.bits {
            false => PLLSAI2REN_A::Disabled,
            true => PLLSAI2REN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2REN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2REN_A::Enabled
    }
}
///Field `PLLSAI2REN` writer - PLLSAI2 PLLADC2CLK output enable
pub type PLLSAI2REN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLLSAI2CFGR_SPEC, PLLSAI2REN_A, O>;
impl<'a, const O: u8> PLLSAI2REN_W<'a, O> {
    ///PLLLCDCLK output disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::Disabled)
    }
    ///PLLLCDCLK output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::Enabled)
    }
}
///Field `PLLSAI2R` reader - PLLSAI2 division factor for PLLADC2CLK (ADC clock)
pub use PLLSAI2Q_R as PLLSAI2R_R;
///Field `PLLSAI2R` writer - PLLSAI2 division factor for PLLADC2CLK (ADC clock)
pub use PLLSAI2Q_W as PLLSAI2R_W;
///Field `PLLSAI2PDIV` reader - PLLSAI2 division factor for PLLSAI2CLK
pub type PLLSAI2PDIV_R = crate::FieldReader<u8, PLLSAI2PDIV_A>;
///PLLSAI2 division factor for PLLSAI2CLK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2PDIV_A {
    ///0: PLLSAI2CLK is controlled by the bit PLLSAI2P
    Pllsai1p = 0,
    ///2: PLLSAI2CLK = VCOSAI2 / 2
    Div2 = 2,
    ///3: PLLSAI2CLK = VCOSAI2 / 3
    Div3 = 3,
    ///4: PLLSAI2CLK = VCOSAI2 / 4
    Div4 = 4,
    ///5: PLLSAI2CLK = VCOSAI2 / 5
    Div5 = 5,
    ///6: PLLSAI2CLK = VCOSAI2 / 6
    Div6 = 6,
    ///7: PLLSAI2CLK = VCOSAI2 / 7
    Div7 = 7,
    ///8: PLLSAI2CLK = VCOSAI2 / 8
    Div8 = 8,
    ///9: PLLSAI2CLK = VCOSAI2 / 9
    Div9 = 9,
    ///10: PLLSAI2CLK = VCOSAI2 / 10
    Div10 = 10,
    ///11: PLLSAI2CLK = VCOSAI2 / 11
    Div11 = 11,
    ///12: PLLSAI2CLK = VCOSAI2 / 12
    Div12 = 12,
    ///13: PLLSAI2CLK = VCOSAI2 / 13
    Div13 = 13,
    ///14: PLLSAI2CLK = VCOSAI2 / 14
    Div14 = 14,
    ///15: PLLSAI2CLK = VCOSAI2 / 15
    Div15 = 15,
    ///16: PLLSAI2CLK = VCOSAI2 / 16
    Div16 = 16,
    ///17: PLLSAI2CLK = VCOSAI2 / 17
    Div17 = 17,
    ///18: PLLSAI2CLK = VCOSAI2 / 18
    Div18 = 18,
    ///19: PLLSAI2CLK = VCOSAI2 / 19
    Div19 = 19,
    ///20: PLLSAI2CLK = VCOSAI2 / 20
    Div20 = 20,
    ///21: PLLSAI2CLK = VCOSAI2 / 21
    Div21 = 21,
    ///22: PLLSAI2CLK = VCOSAI2 / 22
    Div22 = 22,
    ///23: PLLSAI2CLK = VCOSAI2 / 23
    Div23 = 23,
    ///24: PLLSAI2CLK = VCOSAI2 / 24
    Div24 = 24,
    ///25: PLLSAI2CLK = VCOSAI2 / 25
    Div25 = 25,
    ///26: PLLSAI2CLK = VCOSAI2 / 26
    Div26 = 26,
    ///27: PLLSAI2CLK = VCOSAI2 / 27
    Div27 = 27,
    ///28: PLLSAI2CLK = VCOSAI2 / 28
    Div28 = 28,
    ///29: PLLSAI2CLK = VCOSAI2 / 29
    Div29 = 29,
    ///30: PLLSAI2CLK = VCOSAI2 / 30
    Div30 = 30,
    ///31: PLLSAI2CLK = VCOSAI2 / 31
    Div31 = 31,
}
impl From<PLLSAI2PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2PDIV_A) -> Self {
        variant as _
    }
}
impl PLLSAI2PDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLSAI2PDIV_A> {
        match self.bits {
            0 => Some(PLLSAI2PDIV_A::Pllsai1p),
            2 => Some(PLLSAI2PDIV_A::Div2),
            3 => Some(PLLSAI2PDIV_A::Div3),
            4 => Some(PLLSAI2PDIV_A::Div4),
            5 => Some(PLLSAI2PDIV_A::Div5),
            6 => Some(PLLSAI2PDIV_A::Div6),
            7 => Some(PLLSAI2PDIV_A::Div7),
            8 => Some(PLLSAI2PDIV_A::Div8),
            9 => Some(PLLSAI2PDIV_A::Div9),
            10 => Some(PLLSAI2PDIV_A::Div10),
            11 => Some(PLLSAI2PDIV_A::Div11),
            12 => Some(PLLSAI2PDIV_A::Div12),
            13 => Some(PLLSAI2PDIV_A::Div13),
            14 => Some(PLLSAI2PDIV_A::Div14),
            15 => Some(PLLSAI2PDIV_A::Div15),
            16 => Some(PLLSAI2PDIV_A::Div16),
            17 => Some(PLLSAI2PDIV_A::Div17),
            18 => Some(PLLSAI2PDIV_A::Div18),
            19 => Some(PLLSAI2PDIV_A::Div19),
            20 => Some(PLLSAI2PDIV_A::Div20),
            21 => Some(PLLSAI2PDIV_A::Div21),
            22 => Some(PLLSAI2PDIV_A::Div22),
            23 => Some(PLLSAI2PDIV_A::Div23),
            24 => Some(PLLSAI2PDIV_A::Div24),
            25 => Some(PLLSAI2PDIV_A::Div25),
            26 => Some(PLLSAI2PDIV_A::Div26),
            27 => Some(PLLSAI2PDIV_A::Div27),
            28 => Some(PLLSAI2PDIV_A::Div28),
            29 => Some(PLLSAI2PDIV_A::Div29),
            30 => Some(PLLSAI2PDIV_A::Div30),
            31 => Some(PLLSAI2PDIV_A::Div31),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pllsai1p`
    #[inline(always)]
    pub fn is_pllsai1p(&self) -> bool {
        *self == PLLSAI2PDIV_A::Pllsai1p
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div16
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div17
    }
    ///Checks if the value of the field is `Div18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div18
    }
    ///Checks if the value of the field is `Div19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div19
    }
    ///Checks if the value of the field is `Div20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div20
    }
    ///Checks if the value of the field is `Div21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div21
    }
    ///Checks if the value of the field is `Div22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div22
    }
    ///Checks if the value of the field is `Div23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div23
    }
    ///Checks if the value of the field is `Div24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div24
    }
    ///Checks if the value of the field is `Div25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div25
    }
    ///Checks if the value of the field is `Div26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div26
    }
    ///Checks if the value of the field is `Div27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div27
    }
    ///Checks if the value of the field is `Div28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div28
    }
    ///Checks if the value of the field is `Div29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div29
    }
    ///Checks if the value of the field is `Div30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div30
    }
    ///Checks if the value of the field is `Div31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAI2PDIV_A::Div31
    }
}
///Field `PLLSAI2PDIV` writer - PLLSAI2 division factor for PLLSAI2CLK
pub type PLLSAI2PDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLLSAI2CFGR_SPEC, u8, PLLSAI2PDIV_A, 5, O>;
impl<'a, const O: u8> PLLSAI2PDIV_W<'a, O> {
    ///PLLSAI2CLK is controlled by the bit PLLSAI2P
    #[inline(always)]
    pub fn pllsai1p(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Pllsai1p)
    }
    ///PLLSAI2CLK = VCOSAI2 / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div2)
    }
    ///PLLSAI2CLK = VCOSAI2 / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div3)
    }
    ///PLLSAI2CLK = VCOSAI2 / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div4)
    }
    ///PLLSAI2CLK = VCOSAI2 / 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div5)
    }
    ///PLLSAI2CLK = VCOSAI2 / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div6)
    }
    ///PLLSAI2CLK = VCOSAI2 / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div7)
    }
    ///PLLSAI2CLK = VCOSAI2 / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div8)
    }
    ///PLLSAI2CLK = VCOSAI2 / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div9)
    }
    ///PLLSAI2CLK = VCOSAI2 / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div10)
    }
    ///PLLSAI2CLK = VCOSAI2 / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div11)
    }
    ///PLLSAI2CLK = VCOSAI2 / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div12)
    }
    ///PLLSAI2CLK = VCOSAI2 / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div13)
    }
    ///PLLSAI2CLK = VCOSAI2 / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div14)
    }
    ///PLLSAI2CLK = VCOSAI2 / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div15)
    }
    ///PLLSAI2CLK = VCOSAI2 / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div16)
    }
    ///PLLSAI2CLK = VCOSAI2 / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div17)
    }
    ///PLLSAI2CLK = VCOSAI2 / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div18)
    }
    ///PLLSAI2CLK = VCOSAI2 / 19
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div19)
    }
    ///PLLSAI2CLK = VCOSAI2 / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div20)
    }
    ///PLLSAI2CLK = VCOSAI2 / 21
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div21)
    }
    ///PLLSAI2CLK = VCOSAI2 / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div22)
    }
    ///PLLSAI2CLK = VCOSAI2 / 23
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div23)
    }
    ///PLLSAI2CLK = VCOSAI2 / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div24)
    }
    ///PLLSAI2CLK = VCOSAI2 / 25
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div25)
    }
    ///PLLSAI2CLK = VCOSAI2 / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div26)
    }
    ///PLLSAI2CLK = VCOSAI2 / 27
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div27)
    }
    ///PLLSAI2CLK = VCOSAI2 / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div28)
    }
    ///PLLSAI2CLK = VCOSAI2 / 29
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div29)
    }
    ///PLLSAI2CLK = VCOSAI2 / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div30)
    }
    ///PLLSAI2CLK = VCOSAI2 / 31
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLSAI2PDIV_A::Div31)
    }
}
impl R {
    ///Bits 4:7 - Division factor for PLLSAI2 input clock
    #[inline(always)]
    pub fn pllsai2m(&self) -> PLLSAI2M_R {
        PLLSAI2M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - SAI2PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - PLLSAI2 division factor for PLLDISCLK
    #[inline(always)]
    pub fn pllsai2qen(&self) -> PLLSAI2QEN_R {
        PLLSAI2QEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2q(&self) -> PLLSAI2Q_R {
        PLLSAI2Q_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - PLLSAI2 PLLADC2CLK output enable
    #[inline(always)]
    pub fn pllsai2ren(&self) -> PLLSAI2REN_R {
        PLLSAI2REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai2r(&self) -> PLLSAI2R_R {
        PLLSAI2R_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 4:7 - Division factor for PLLSAI2 input clock
    #[inline(always)]
    #[must_use]
    pub fn pllsai2m(&mut self) -> PLLSAI2M_W<4> {
        PLLSAI2M_W::new(self)
    }
    ///Bits 8:14 - SAI2PLL multiplication factor for VCO
    #[inline(always)]
    #[must_use]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W<8> {
        PLLSAI2N_W::new(self)
    }
    ///Bit 16 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W<16> {
        PLLSAI2PEN_W::new(self)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    #[must_use]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W<17> {
        PLLSAI2P_W::new(self)
    }
    ///Bit 20 - PLLSAI2 division factor for PLLDISCLK
    #[inline(always)]
    #[must_use]
    pub fn pllsai2qen(&mut self) -> PLLSAI2QEN_W<20> {
        PLLSAI2QEN_W::new(self)
    }
    ///Bits 21:22 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai2q(&mut self) -> PLLSAI2Q_W<21> {
        PLLSAI2Q_W::new(self)
    }
    ///Bit 24 - PLLSAI2 PLLADC2CLK output enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai2ren(&mut self) -> PLLSAI2REN_W<24> {
        PLLSAI2REN_W::new(self)
    }
    ///Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)
    #[inline(always)]
    #[must_use]
    pub fn pllsai2r(&mut self) -> PLLSAI2R_W<25> {
        PLLSAI2R_W::new(self)
    }
    ///Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK
    #[inline(always)]
    #[must_use]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W<27> {
        PLLSAI2PDIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PLLSAI2 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllsai2cfgr](index.html) module
pub struct PLLSAI2CFGR_SPEC;
impl crate::RegisterSpec for PLLSAI2CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pllsai2cfgr::R](R) reader structure
impl crate::Readable for PLLSAI2CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pllsai2cfgr::W](W) writer structure
impl crate::Writable for PLLSAI2CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLLSAI2CFGR to value 0x1000
impl crate::Resettable for PLLSAI2CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
