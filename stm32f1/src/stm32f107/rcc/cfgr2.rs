///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PREDIV1` reader - PREDIV1 division factor
pub type PREDIV1_R = crate::FieldReader<u8, PREDIV1_A>;
///PREDIV1 division factor
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREDIV1_A {
    ///0: PREDIV input clock not divided
    Div1 = 0,
    ///1: PREDIV input clock divided by 2
    Div2 = 1,
    ///2: PREDIV input clock divided by 3
    Div3 = 2,
    ///3: PREDIV input clock divided by 4
    Div4 = 3,
    ///4: PREDIV input clock divided by 5
    Div5 = 4,
    ///5: PREDIV input clock divided by 6
    Div6 = 5,
    ///6: PREDIV input clock divided by 7
    Div7 = 6,
    ///7: PREDIV input clock divided by 8
    Div8 = 7,
    ///8: PREDIV input clock divided by 9
    Div9 = 8,
    ///9: PREDIV input clock divided by 10
    Div10 = 9,
    ///10: PREDIV input clock divided by 11
    Div11 = 10,
    ///11: PREDIV input clock divided by 12
    Div12 = 11,
    ///12: PREDIV input clock divided by 13
    Div13 = 12,
    ///13: PREDIV input clock divided by 14
    Div14 = 13,
    ///14: PREDIV input clock divided by 15
    Div15 = 14,
    ///15: PREDIV input clock divided by 16
    Div16 = 15,
}
impl From<PREDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV1_A) -> Self {
        variant as _
    }
}
impl PREDIV1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PREDIV1_A {
        match self.bits {
            0 => PREDIV1_A::Div1,
            1 => PREDIV1_A::Div2,
            2 => PREDIV1_A::Div3,
            3 => PREDIV1_A::Div4,
            4 => PREDIV1_A::Div5,
            5 => PREDIV1_A::Div6,
            6 => PREDIV1_A::Div7,
            7 => PREDIV1_A::Div8,
            8 => PREDIV1_A::Div9,
            9 => PREDIV1_A::Div10,
            10 => PREDIV1_A::Div11,
            11 => PREDIV1_A::Div12,
            12 => PREDIV1_A::Div13,
            13 => PREDIV1_A::Div14,
            14 => PREDIV1_A::Div15,
            15 => PREDIV1_A::Div16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV1_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV1_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV1_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV1_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV1_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV1_A::Div6
    }
    ///Checks if the value of the field is `Div7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV1_A::Div7
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV1_A::Div8
    }
    ///Checks if the value of the field is `Div9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV1_A::Div9
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV1_A::Div10
    }
    ///Checks if the value of the field is `Div11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV1_A::Div11
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV1_A::Div12
    }
    ///Checks if the value of the field is `Div13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV1_A::Div13
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV1_A::Div14
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV1_A::Div15
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV1_A::Div16
    }
}
///Field `PREDIV1` writer - PREDIV1 division factor
pub type PREDIV1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, PREDIV1_A, 4, O>;
impl<'a, const O: u8> PREDIV1_W<'a, O> {
    ///PREDIV input clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div1)
    }
    ///PREDIV input clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div2)
    }
    ///PREDIV input clock divided by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div3)
    }
    ///PREDIV input clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div4)
    }
    ///PREDIV input clock divided by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div5)
    }
    ///PREDIV input clock divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div6)
    }
    ///PREDIV input clock divided by 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div7)
    }
    ///PREDIV input clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div8)
    }
    ///PREDIV input clock divided by 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div9)
    }
    ///PREDIV input clock divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div10)
    }
    ///PREDIV input clock divided by 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div11)
    }
    ///PREDIV input clock divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div12)
    }
    ///PREDIV input clock divided by 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div13)
    }
    ///PREDIV input clock divided by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div14)
    }
    ///PREDIV input clock divided by 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div15)
    }
    ///PREDIV input clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV1_A::Div16)
    }
}
///Field `PREDIV2` reader - PREDIV2 division factor
pub use PREDIV1_R as PREDIV2_R;
///Field `PREDIV2` writer - PREDIV2 division factor
pub use PREDIV1_W as PREDIV2_W;
///Field `PLL2MUL` reader - PLL2 Multiplication Factor
pub type PLL2MUL_R = crate::FieldReader<u8, PLL2MUL_A>;
///PLL2 Multiplication Factor
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2MUL_A {
    ///6: PLL clock entry x8
    Mul8 = 6,
    ///7: PLL clock entry x9
    Mul9 = 7,
    ///8: PLL clock entry x10
    Mul10 = 8,
    ///9: PLL clock entry x11
    Mul11 = 9,
    ///10: PLL clock entry x12
    Mul12 = 10,
    ///11: PLL clock entry x13
    Mul13 = 11,
    ///12: PLL clock entry x14
    Mul14 = 12,
    ///14: PLL clock entry x16
    Mul16 = 14,
    ///15: PLL clock entry x20
    Mul20 = 15,
}
impl From<PLL2MUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2MUL_A) -> Self {
        variant as _
    }
}
impl PLL2MUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PLL2MUL_A> {
        match self.bits {
            6 => Some(PLL2MUL_A::Mul8),
            7 => Some(PLL2MUL_A::Mul9),
            8 => Some(PLL2MUL_A::Mul10),
            9 => Some(PLL2MUL_A::Mul11),
            10 => Some(PLL2MUL_A::Mul12),
            11 => Some(PLL2MUL_A::Mul13),
            12 => Some(PLL2MUL_A::Mul14),
            14 => Some(PLL2MUL_A::Mul16),
            15 => Some(PLL2MUL_A::Mul20),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Mul8`
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLL2MUL_A::Mul8
    }
    ///Checks if the value of the field is `Mul9`
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLL2MUL_A::Mul9
    }
    ///Checks if the value of the field is `Mul10`
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == PLL2MUL_A::Mul10
    }
    ///Checks if the value of the field is `Mul11`
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == PLL2MUL_A::Mul11
    }
    ///Checks if the value of the field is `Mul12`
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLL2MUL_A::Mul12
    }
    ///Checks if the value of the field is `Mul13`
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == PLL2MUL_A::Mul13
    }
    ///Checks if the value of the field is `Mul14`
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == PLL2MUL_A::Mul14
    }
    ///Checks if the value of the field is `Mul16`
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLL2MUL_A::Mul16
    }
    ///Checks if the value of the field is `Mul20`
    #[inline(always)]
    pub fn is_mul20(&self) -> bool {
        *self == PLL2MUL_A::Mul20
    }
}
///Field `PLL2MUL` writer - PLL2 Multiplication Factor
pub type PLL2MUL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, PLL2MUL_A, 4, O>;
impl<'a, const O: u8> PLL2MUL_W<'a, O> {
    ///PLL clock entry x8
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul8)
    }
    ///PLL clock entry x9
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul9)
    }
    ///PLL clock entry x10
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul10)
    }
    ///PLL clock entry x11
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul11)
    }
    ///PLL clock entry x12
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul12)
    }
    ///PLL clock entry x13
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul13)
    }
    ///PLL clock entry x14
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul14)
    }
    ///PLL clock entry x16
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul16)
    }
    ///PLL clock entry x20
    #[inline(always)]
    pub fn mul20(self) -> &'a mut W {
        self.variant(PLL2MUL_A::Mul20)
    }
}
///Field `PLL3MUL` reader - PLL3 Multiplication Factor
pub use PLL2MUL_R as PLL3MUL_R;
///Field `PLL3MUL` writer - PLL3 Multiplication Factor
pub use PLL2MUL_W as PLL3MUL_W;
///Field `PREDIV1SRC` reader - PREDIV1 entry clock source
pub type PREDIV1SRC_R = crate::BitReader<PREDIV1SRC_A>;
///PREDIV1 entry clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREDIV1SRC_A {
    ///0: HSE oscillator clock selected as PREDIV1 clock entry
    Hse = 0,
    ///1: PLL2 selected as PREDIV1 clock entry
    Pll2 = 1,
}
impl From<PREDIV1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PREDIV1SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl PREDIV1SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PREDIV1SRC_A {
        match self.bits {
            false => PREDIV1SRC_A::Hse,
            true => PREDIV1SRC_A::Pll2,
        }
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PREDIV1SRC_A::Hse
    }
    ///Checks if the value of the field is `Pll2`
    #[inline(always)]
    pub fn is_pll2(&self) -> bool {
        *self == PREDIV1SRC_A::Pll2
    }
}
///Field `PREDIV1SRC` writer - PREDIV1 entry clock source
pub type PREDIV1SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, PREDIV1SRC_A, O>;
impl<'a, const O: u8> PREDIV1SRC_W<'a, O> {
    ///HSE oscillator clock selected as PREDIV1 clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PREDIV1SRC_A::Hse)
    }
    ///PLL2 selected as PREDIV1 clock entry
    #[inline(always)]
    pub fn pll2(self) -> &'a mut W {
        self.variant(PREDIV1SRC_A::Pll2)
    }
}
///Field `I2S2SRC` reader - I2S2 clock source
pub type I2S2SRC_R = crate::BitReader<I2S2SRC_A>;
///I2S2 clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S2SRC_A {
    ///0: System clock (SYSCLK) selected as I2S clock entry
    Sysclk = 0,
    ///1: PLL3 VCO clock selected as I2S clock entry
    Pll3 = 1,
}
impl From<I2S2SRC_A> for bool {
    #[inline(always)]
    fn from(variant: I2S2SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S2SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2S2SRC_A {
        match self.bits {
            false => I2S2SRC_A::Sysclk,
            true => I2S2SRC_A::Pll3,
        }
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2S2SRC_A::Sysclk
    }
    ///Checks if the value of the field is `Pll3`
    #[inline(always)]
    pub fn is_pll3(&self) -> bool {
        *self == I2S2SRC_A::Pll3
    }
}
///Field `I2S2SRC` writer - I2S2 clock source
pub type I2S2SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2S2SRC_A, O>;
impl<'a, const O: u8> I2S2SRC_W<'a, O> {
    ///System clock (SYSCLK) selected as I2S clock entry
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2S2SRC_A::Sysclk)
    }
    ///PLL3 VCO clock selected as I2S clock entry
    #[inline(always)]
    pub fn pll3(self) -> &'a mut W {
        self.variant(I2S2SRC_A::Pll3)
    }
}
///Field `I2S3SRC` reader - I2S3 clock source
pub use I2S2SRC_R as I2S3SRC_R;
///Field `I2S3SRC` writer - I2S3 clock source
pub use I2S2SRC_W as I2S3SRC_W;
impl R {
    ///Bits 0:3 - PREDIV1 division factor
    #[inline(always)]
    pub fn prediv1(&self) -> PREDIV1_R {
        PREDIV1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PREDIV2 division factor
    #[inline(always)]
    pub fn prediv2(&self) -> PREDIV2_R {
        PREDIV2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - PLL2 Multiplication Factor
    #[inline(always)]
    pub fn pll2mul(&self) -> PLL2MUL_R {
        PLL2MUL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PLL3 Multiplication Factor
    #[inline(always)]
    pub fn pll3mul(&self) -> PLL3MUL_R {
        PLL3MUL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - PREDIV1 entry clock source
    #[inline(always)]
    pub fn prediv1src(&self) -> PREDIV1SRC_R {
        PREDIV1SRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - I2S2 clock source
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I2S3 clock source
    #[inline(always)]
    pub fn i2s3src(&self) -> I2S3SRC_R {
        I2S3SRC_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - PREDIV1 division factor
    #[inline(always)]
    #[must_use]
    pub fn prediv1(&mut self) -> PREDIV1_W<0> {
        PREDIV1_W::new(self)
    }
    ///Bits 4:7 - PREDIV2 division factor
    #[inline(always)]
    #[must_use]
    pub fn prediv2(&mut self) -> PREDIV2_W<4> {
        PREDIV2_W::new(self)
    }
    ///Bits 8:11 - PLL2 Multiplication Factor
    #[inline(always)]
    #[must_use]
    pub fn pll2mul(&mut self) -> PLL2MUL_W<8> {
        PLL2MUL_W::new(self)
    }
    ///Bits 12:15 - PLL3 Multiplication Factor
    #[inline(always)]
    #[must_use]
    pub fn pll3mul(&mut self) -> PLL3MUL_W<12> {
        PLL3MUL_W::new(self)
    }
    ///Bit 16 - PREDIV1 entry clock source
    #[inline(always)]
    #[must_use]
    pub fn prediv1src(&mut self) -> PREDIV1SRC_W<16> {
        PREDIV1SRC_W::new(self)
    }
    ///Bit 17 - I2S2 clock source
    #[inline(always)]
    #[must_use]
    pub fn i2s2src(&mut self) -> I2S2SRC_W<17> {
        I2S2SRC_W::new(self)
    }
    ///Bit 18 - I2S3 clock source
    #[inline(always)]
    #[must_use]
    pub fn i2s3src(&mut self) -> I2S3SRC_W<18> {
        I2S3SRC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock configuration register2 (RCC_CFGR2)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
