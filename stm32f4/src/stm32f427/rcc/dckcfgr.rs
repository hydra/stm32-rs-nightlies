///Register `DCKCFGR` reader
pub struct R(crate::R<DCKCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCKCFGR` writer
pub struct W(crate::W<DCKCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR_SPEC>;
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
impl From<crate::W<DCKCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLI2SDIVQ` reader - PLLI2S division factor for SAI1 clock
pub type PLLI2SDIVQ_R = crate::FieldReader<u8, PLLI2SDIVQ_A>;
///PLLI2S division factor for SAI1 clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLI2SDIVQ_A {
    ///0: PLLI2SDIVQ = /1
    Div1 = 0,
    ///1: PLLI2SDIVQ = /2
    Div2 = 1,
    ///2: PLLI2SDIVQ = /3
    Div3 = 2,
    ///3: PLLI2SDIVQ = /4
    Div4 = 3,
    ///4: PLLI2SDIVQ = /5
    Div5 = 4,
    ///5: PLLI2SDIVQ = /6
    Div6 = 5,
    ///6: PLLI2SDIVQ = /7
    Div7 = 6,
    ///7: PLLI2SDIVQ = /8
    Div8 = 7,
    ///8: PLLI2SDIVQ = /9
    Div9 = 8,
    ///9: PLLI2SDIVQ = /10
    Div10 = 9,
    ///10: PLLI2SDIVQ = /11
    Div11 = 10,
    ///11: PLLI2SDIVQ = /12
    Div12 = 11,
    ///12: PLLI2SDIVQ = /13
    Div13 = 12,
    ///13: PLLI2SDIVQ = /14
    Div14 = 13,
    ///14: PLLI2SDIVQ = /15
    Div15 = 14,
    ///15: PLLI2SDIVQ = /16
    Div16 = 15,
    ///16: PLLI2SDIVQ = /17
    Div17 = 16,
    ///17: PLLI2SDIVQ = /18
    Div18 = 17,
    ///18: PLLI2SDIVQ = /19
    Div19 = 18,
    ///19: PLLI2SDIVQ = /20
    Div20 = 19,
    ///20: PLLI2SDIVQ = /21
    Div21 = 20,
    ///21: PLLI2SDIVQ = /22
    Div22 = 21,
    ///22: PLLI2SDIVQ = /23
    Div23 = 22,
    ///23: PLLI2SDIVQ = /24
    Div24 = 23,
    ///24: PLLI2SDIVQ = /25
    Div25 = 24,
    ///25: PLLI2SDIVQ = /26
    Div26 = 25,
    ///26: PLLI2SDIVQ = /27
    Div27 = 26,
    ///27: PLLI2SDIVQ = /28
    Div28 = 27,
    ///28: PLLI2SDIVQ = /29
    Div29 = 28,
    ///29: PLLI2SDIVQ = /30
    Div30 = 29,
    ///30: PLLI2SDIVQ = /31
    Div31 = 30,
    ///31: PLLI2SDIVQ = /32
    Div32 = 31,
}
impl From<PLLI2SDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SDIVQ_A) -> Self {
        variant as _
    }
}
impl PLLI2SDIVQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SDIVQ_A {
        match self.bits {
            0 => PLLI2SDIVQ_A::Div1,
            1 => PLLI2SDIVQ_A::Div2,
            2 => PLLI2SDIVQ_A::Div3,
            3 => PLLI2SDIVQ_A::Div4,
            4 => PLLI2SDIVQ_A::Div5,
            5 => PLLI2SDIVQ_A::Div6,
            6 => PLLI2SDIVQ_A::Div7,
            7 => PLLI2SDIVQ_A::Div8,
            8 => PLLI2SDIVQ_A::Div9,
            9 => PLLI2SDIVQ_A::Div10,
            10 => PLLI2SDIVQ_A::Div11,
            11 => PLLI2SDIVQ_A::Div12,
            12 => PLLI2SDIVQ_A::Div13,
            13 => PLLI2SDIVQ_A::Div14,
            14 => PLLI2SDIVQ_A::Div15,
            15 => PLLI2SDIVQ_A::Div16,
            16 => PLLI2SDIVQ_A::Div17,
            17 => PLLI2SDIVQ_A::Div18,
            18 => PLLI2SDIVQ_A::Div19,
            19 => PLLI2SDIVQ_A::Div20,
            20 => PLLI2SDIVQ_A::Div21,
            21 => PLLI2SDIVQ_A::Div22,
            22 => PLLI2SDIVQ_A::Div23,
            23 => PLLI2SDIVQ_A::Div24,
            24 => PLLI2SDIVQ_A::Div25,
            25 => PLLI2SDIVQ_A::Div26,
            26 => PLLI2SDIVQ_A::Div27,
            27 => PLLI2SDIVQ_A::Div28,
            28 => PLLI2SDIVQ_A::Div29,
            29 => PLLI2SDIVQ_A::Div30,
            30 => PLLI2SDIVQ_A::Div31,
            31 => PLLI2SDIVQ_A::Div32,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div16
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div17
    }
    ///Checks if the value of the field is `Div18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div18
    }
    ///Checks if the value of the field is `Div19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div19
    }
    ///Checks if the value of the field is `Div20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div20
    }
    ///Checks if the value of the field is `Div21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div21
    }
    ///Checks if the value of the field is `Div22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div22
    }
    ///Checks if the value of the field is `Div23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div23
    }
    ///Checks if the value of the field is `Div24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div24
    }
    ///Checks if the value of the field is `Div25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div25
    }
    ///Checks if the value of the field is `Div26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div26
    }
    ///Checks if the value of the field is `Div27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div27
    }
    ///Checks if the value of the field is `Div28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div28
    }
    ///Checks if the value of the field is `Div29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div29
    }
    ///Checks if the value of the field is `Div30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div30
    }
    ///Checks if the value of the field is `Div31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div31
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLI2SDIVQ_A::Div32
    }
}
///Field `PLLI2SDIVQ` writer - PLLI2S division factor for SAI1 clock
pub type PLLI2SDIVQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR_SPEC, u8, PLLI2SDIVQ_A, 5, O>;
impl<'a, const O: u8> PLLI2SDIVQ_W<'a, O> {
    ///PLLI2SDIVQ = /1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div1)
    }
    ///PLLI2SDIVQ = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div2)
    }
    ///PLLI2SDIVQ = /3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div3)
    }
    ///PLLI2SDIVQ = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div4)
    }
    ///PLLI2SDIVQ = /5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div5)
    }
    ///PLLI2SDIVQ = /6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div6)
    }
    ///PLLI2SDIVQ = /7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div7)
    }
    ///PLLI2SDIVQ = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div8)
    }
    ///PLLI2SDIVQ = /9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div9)
    }
    ///PLLI2SDIVQ = /10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div10)
    }
    ///PLLI2SDIVQ = /11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div11)
    }
    ///PLLI2SDIVQ = /12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div12)
    }
    ///PLLI2SDIVQ = /13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div13)
    }
    ///PLLI2SDIVQ = /14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div14)
    }
    ///PLLI2SDIVQ = /15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div15)
    }
    ///PLLI2SDIVQ = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div16)
    }
    ///PLLI2SDIVQ = /17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div17)
    }
    ///PLLI2SDIVQ = /18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div18)
    }
    ///PLLI2SDIVQ = /19
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div19)
    }
    ///PLLI2SDIVQ = /20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div20)
    }
    ///PLLI2SDIVQ = /21
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div21)
    }
    ///PLLI2SDIVQ = /22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div22)
    }
    ///PLLI2SDIVQ = /23
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div23)
    }
    ///PLLI2SDIVQ = /24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div24)
    }
    ///PLLI2SDIVQ = /25
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div25)
    }
    ///PLLI2SDIVQ = /26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div26)
    }
    ///PLLI2SDIVQ = /27
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div27)
    }
    ///PLLI2SDIVQ = /28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div28)
    }
    ///PLLI2SDIVQ = /29
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div29)
    }
    ///PLLI2SDIVQ = /30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div30)
    }
    ///PLLI2SDIVQ = /31
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div31)
    }
    ///PLLI2SDIVQ = /32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::Div32)
    }
}
///Field `PLLSAIDIVQ` reader - PLLSAI division factor for SAI1 clock
pub type PLLSAIDIVQ_R = crate::FieldReader<u8, PLLSAIDIVQ_A>;
///PLLSAI division factor for SAI1 clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIDIVQ_A {
    ///0: PLLSAIDIVQ = /1
    Div1 = 0,
    ///1: PLLSAIDIVQ = /2
    Div2 = 1,
    ///2: PLLSAIDIVQ = /3
    Div3 = 2,
    ///3: PLLSAIDIVQ = /4
    Div4 = 3,
    ///4: PLLSAIDIVQ = /5
    Div5 = 4,
    ///5: PLLSAIDIVQ = /6
    Div6 = 5,
    ///6: PLLSAIDIVQ = /7
    Div7 = 6,
    ///7: PLLSAIDIVQ = /8
    Div8 = 7,
    ///8: PLLSAIDIVQ = /9
    Div9 = 8,
    ///9: PLLSAIDIVQ = /10
    Div10 = 9,
    ///10: PLLSAIDIVQ = /11
    Div11 = 10,
    ///11: PLLSAIDIVQ = /12
    Div12 = 11,
    ///12: PLLSAIDIVQ = /13
    Div13 = 12,
    ///13: PLLSAIDIVQ = /14
    Div14 = 13,
    ///14: PLLSAIDIVQ = /15
    Div15 = 14,
    ///15: PLLSAIDIVQ = /16
    Div16 = 15,
    ///16: PLLSAIDIVQ = /17
    Div17 = 16,
    ///17: PLLSAIDIVQ = /18
    Div18 = 17,
    ///18: PLLSAIDIVQ = /19
    Div19 = 18,
    ///19: PLLSAIDIVQ = /20
    Div20 = 19,
    ///20: PLLSAIDIVQ = /21
    Div21 = 20,
    ///21: PLLSAIDIVQ = /22
    Div22 = 21,
    ///22: PLLSAIDIVQ = /23
    Div23 = 22,
    ///23: PLLSAIDIVQ = /24
    Div24 = 23,
    ///24: PLLSAIDIVQ = /25
    Div25 = 24,
    ///25: PLLSAIDIVQ = /26
    Div26 = 25,
    ///26: PLLSAIDIVQ = /27
    Div27 = 26,
    ///27: PLLSAIDIVQ = /28
    Div28 = 27,
    ///28: PLLSAIDIVQ = /29
    Div29 = 28,
    ///29: PLLSAIDIVQ = /30
    Div30 = 29,
    ///30: PLLSAIDIVQ = /31
    Div31 = 30,
    ///31: PLLSAIDIVQ = /32
    Div32 = 31,
}
impl From<PLLSAIDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVQ_A) -> Self {
        variant as _
    }
}
impl PLLSAIDIVQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVQ_A {
        match self.bits {
            0 => PLLSAIDIVQ_A::Div1,
            1 => PLLSAIDIVQ_A::Div2,
            2 => PLLSAIDIVQ_A::Div3,
            3 => PLLSAIDIVQ_A::Div4,
            4 => PLLSAIDIVQ_A::Div5,
            5 => PLLSAIDIVQ_A::Div6,
            6 => PLLSAIDIVQ_A::Div7,
            7 => PLLSAIDIVQ_A::Div8,
            8 => PLLSAIDIVQ_A::Div9,
            9 => PLLSAIDIVQ_A::Div10,
            10 => PLLSAIDIVQ_A::Div11,
            11 => PLLSAIDIVQ_A::Div12,
            12 => PLLSAIDIVQ_A::Div13,
            13 => PLLSAIDIVQ_A::Div14,
            14 => PLLSAIDIVQ_A::Div15,
            15 => PLLSAIDIVQ_A::Div16,
            16 => PLLSAIDIVQ_A::Div17,
            17 => PLLSAIDIVQ_A::Div18,
            18 => PLLSAIDIVQ_A::Div19,
            19 => PLLSAIDIVQ_A::Div20,
            20 => PLLSAIDIVQ_A::Div21,
            21 => PLLSAIDIVQ_A::Div22,
            22 => PLLSAIDIVQ_A::Div23,
            23 => PLLSAIDIVQ_A::Div24,
            24 => PLLSAIDIVQ_A::Div25,
            25 => PLLSAIDIVQ_A::Div26,
            26 => PLLSAIDIVQ_A::Div27,
            27 => PLLSAIDIVQ_A::Div28,
            28 => PLLSAIDIVQ_A::Div29,
            29 => PLLSAIDIVQ_A::Div30,
            30 => PLLSAIDIVQ_A::Div31,
            31 => PLLSAIDIVQ_A::Div32,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div16
    }
    ///Checks if the value of the field is `Div17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div17
    }
    ///Checks if the value of the field is `Div18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div18
    }
    ///Checks if the value of the field is `Div19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div19
    }
    ///Checks if the value of the field is `Div20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div20
    }
    ///Checks if the value of the field is `Div21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div21
    }
    ///Checks if the value of the field is `Div22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div22
    }
    ///Checks if the value of the field is `Div23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div23
    }
    ///Checks if the value of the field is `Div24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div24
    }
    ///Checks if the value of the field is `Div25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div25
    }
    ///Checks if the value of the field is `Div26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div26
    }
    ///Checks if the value of the field is `Div27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div27
    }
    ///Checks if the value of the field is `Div28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div28
    }
    ///Checks if the value of the field is `Div29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div29
    }
    ///Checks if the value of the field is `Div30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div30
    }
    ///Checks if the value of the field is `Div31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div31
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLSAIDIVQ_A::Div32
    }
}
///Field `PLLSAIDIVQ` writer - PLLSAI division factor for SAI1 clock
pub type PLLSAIDIVQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR_SPEC, u8, PLLSAIDIVQ_A, 5, O>;
impl<'a, const O: u8> PLLSAIDIVQ_W<'a, O> {
    ///PLLSAIDIVQ = /1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div1)
    }
    ///PLLSAIDIVQ = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div2)
    }
    ///PLLSAIDIVQ = /3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div3)
    }
    ///PLLSAIDIVQ = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div4)
    }
    ///PLLSAIDIVQ = /5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div5)
    }
    ///PLLSAIDIVQ = /6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div6)
    }
    ///PLLSAIDIVQ = /7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div7)
    }
    ///PLLSAIDIVQ = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div8)
    }
    ///PLLSAIDIVQ = /9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div9)
    }
    ///PLLSAIDIVQ = /10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div10)
    }
    ///PLLSAIDIVQ = /11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div11)
    }
    ///PLLSAIDIVQ = /12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div12)
    }
    ///PLLSAIDIVQ = /13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div13)
    }
    ///PLLSAIDIVQ = /14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div14)
    }
    ///PLLSAIDIVQ = /15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div15)
    }
    ///PLLSAIDIVQ = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div16)
    }
    ///PLLSAIDIVQ = /17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div17)
    }
    ///PLLSAIDIVQ = /18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div18)
    }
    ///PLLSAIDIVQ = /19
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div19)
    }
    ///PLLSAIDIVQ = /20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div20)
    }
    ///PLLSAIDIVQ = /21
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div21)
    }
    ///PLLSAIDIVQ = /22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div22)
    }
    ///PLLSAIDIVQ = /23
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div23)
    }
    ///PLLSAIDIVQ = /24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div24)
    }
    ///PLLSAIDIVQ = /25
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div25)
    }
    ///PLLSAIDIVQ = /26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div26)
    }
    ///PLLSAIDIVQ = /27
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div27)
    }
    ///PLLSAIDIVQ = /28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div28)
    }
    ///PLLSAIDIVQ = /29
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div29)
    }
    ///PLLSAIDIVQ = /30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div30)
    }
    ///PLLSAIDIVQ = /31
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div31)
    }
    ///PLLSAIDIVQ = /32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::Div32)
    }
}
///Field `PLLSAIDIVR` reader - division factor for LCD_CLK
pub type PLLSAIDIVR_R = crate::FieldReader<u8, PLLSAIDIVR_A>;
///division factor for LCD_CLK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIDIVR_A {
    ///0: PLLSAIDIVR = /2
    Div2 = 0,
    ///1: PLLSAIDIVR = /4
    Div4 = 1,
    ///2: PLLSAIDIVR = /8
    Div8 = 2,
    ///3: PLLSAIDIVR = /16
    Div16 = 3,
}
impl From<PLLSAIDIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVR_A) -> Self {
        variant as _
    }
}
impl PLLSAIDIVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVR_A {
        match self.bits {
            0 => PLLSAIDIVR_A::Div2,
            1 => PLLSAIDIVR_A::Div4,
            2 => PLLSAIDIVR_A::Div8,
            3 => PLLSAIDIVR_A::Div16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVR_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVR_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVR_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVR_A::Div16
    }
}
///Field `PLLSAIDIVR` writer - division factor for LCD_CLK
pub type PLLSAIDIVR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR_SPEC, u8, PLLSAIDIVR_A, 2, O>;
impl<'a, const O: u8> PLLSAIDIVR_W<'a, O> {
    ///PLLSAIDIVR = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div2)
    }
    ///PLLSAIDIVR = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div4)
    }
    ///PLLSAIDIVR = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div8)
    }
    ///PLLSAIDIVR = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::Div16)
    }
}
///Field `SAI1ASRC` reader - SAI1-A clock source selection
pub type SAI1ASRC_R = crate::FieldReader<u8, SAI1ASRC_A>;
///SAI1-A clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1ASRC_A {
    ///0: SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    Pllsai = 0,
    ///1: SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    Plli2s = 1,
    ///2: SAI1-A clock frequency = Alternate function input frequency
    I2sCkin = 2,
}
impl From<SAI1ASRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1ASRC_A) -> Self {
        variant as _
    }
}
impl SAI1ASRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI1ASRC_A> {
        match self.bits {
            0 => Some(SAI1ASRC_A::Pllsai),
            1 => Some(SAI1ASRC_A::Plli2s),
            2 => Some(SAI1ASRC_A::I2sCkin),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pllsai`
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1ASRC_A::Pllsai
    }
    ///Checks if the value of the field is `Plli2s`
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1ASRC_A::Plli2s
    }
    ///Checks if the value of the field is `I2sCkin`
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1ASRC_A::I2sCkin
    }
}
///Field `SAI1ASRC` writer - SAI1-A clock source selection
pub type SAI1ASRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCKCFGR_SPEC, u8, SAI1ASRC_A, 2, O>;
impl<'a, const O: u8> SAI1ASRC_W<'a, O> {
    ///SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::Pllsai)
    }
    ///SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::Plli2s)
    }
    ///SAI1-A clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::I2sCkin)
    }
}
///Field `SAI1BSRC` reader - SAI1-B clock source selection
pub type SAI1BSRC_R = crate::FieldReader<u8, SAI1BSRC_A>;
///SAI1-B clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1BSRC_A {
    ///0: SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    Pllsai = 0,
    ///1: SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    Plli2s = 1,
    ///2: SAI1-B clock frequency = Alternate function input frequency
    I2sCkin = 2,
}
impl From<SAI1BSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1BSRC_A) -> Self {
        variant as _
    }
}
impl SAI1BSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI1BSRC_A> {
        match self.bits {
            0 => Some(SAI1BSRC_A::Pllsai),
            1 => Some(SAI1BSRC_A::Plli2s),
            2 => Some(SAI1BSRC_A::I2sCkin),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pllsai`
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1BSRC_A::Pllsai
    }
    ///Checks if the value of the field is `Plli2s`
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1BSRC_A::Plli2s
    }
    ///Checks if the value of the field is `I2sCkin`
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1BSRC_A::I2sCkin
    }
}
///Field `SAI1BSRC` writer - SAI1-B clock source selection
pub type SAI1BSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCKCFGR_SPEC, u8, SAI1BSRC_A, 2, O>;
impl<'a, const O: u8> SAI1BSRC_W<'a, O> {
    ///SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::Pllsai)
    }
    ///SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::Plli2s)
    }
    ///SAI1-B clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::I2sCkin)
    }
}
///Field `TIMPRE` reader - Timers clocks prescalers selection
pub type TIMPRE_R = crate::BitReader<TIMPRE_A>;
///Timers clocks prescalers selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE_A {
    ///0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    Mul2 = 0,
    ///1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    Mul4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::Mul2,
            true => TIMPRE_A::Mul4,
        }
    }
    ///Checks if the value of the field is `Mul2`
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == TIMPRE_A::Mul2
    }
    ///Checks if the value of the field is `Mul4`
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == TIMPRE_A::Mul4
    }
}
///Field `TIMPRE` writer - Timers clocks prescalers selection
pub type TIMPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR_SPEC, TIMPRE_A, O>;
impl<'a, const O: u8> TIMPRE_W<'a, O> {
    ///If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::Mul2)
    }
    ///If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::Mul4)
    }
}
impl R {
    ///Bits 0:4 - PLLI2S division factor for SAI1 clock
    #[inline(always)]
    pub fn plli2sdivq(&self) -> PLLI2SDIVQ_R {
        PLLI2SDIVQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - PLLSAI division factor for SAI1 clock
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:17 - division factor for LCD_CLK
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PLLSAIDIVR_R {
        PLLSAIDIVR_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - SAI1-A clock source selection
    #[inline(always)]
    pub fn sai1asrc(&self) -> SAI1ASRC_R {
        SAI1ASRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SAI1-B clock source selection
    #[inline(always)]
    pub fn sai1bsrc(&self) -> SAI1BSRC_R {
        SAI1BSRC_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - PLLI2S division factor for SAI1 clock
    #[inline(always)]
    #[must_use]
    pub fn plli2sdivq(&mut self) -> PLLI2SDIVQ_W<0> {
        PLLI2SDIVQ_W::new(self)
    }
    ///Bits 8:12 - PLLSAI division factor for SAI1 clock
    #[inline(always)]
    #[must_use]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W<8> {
        PLLSAIDIVQ_W::new(self)
    }
    ///Bits 16:17 - division factor for LCD_CLK
    #[inline(always)]
    #[must_use]
    pub fn pllsaidivr(&mut self) -> PLLSAIDIVR_W<16> {
        PLLSAIDIVR_W::new(self)
    }
    ///Bits 20:21 - SAI1-A clock source selection
    #[inline(always)]
    #[must_use]
    pub fn sai1asrc(&mut self) -> SAI1ASRC_W<20> {
        SAI1ASRC_W::new(self)
    }
    ///Bits 22:23 - SAI1-B clock source selection
    #[inline(always)]
    #[must_use]
    pub fn sai1bsrc(&mut self) -> SAI1BSRC_W<22> {
        SAI1BSRC_W::new(self)
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<24> {
        TIMPRE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC Dedicated Clock Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dckcfgr](index.html) module
pub struct DCKCFGR_SPEC;
impl crate::RegisterSpec for DCKCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dckcfgr::R](R) reader structure
impl crate::Readable for DCKCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dckcfgr::W](W) writer structure
impl crate::Writable for DCKCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCKCFGR to value 0
impl crate::Resettable for DCKCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
