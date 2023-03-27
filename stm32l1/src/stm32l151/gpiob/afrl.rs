///Register `AFRL` reader
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AFRL` writer
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AFRL0` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL0_R = crate::FieldReader<u8, AFRL0_A>;
///Alternate function selection for port x bit y (y = 0..7)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFRL0_A {
    ///0: AF0
    Af0 = 0,
    ///1: AF1
    Af1 = 1,
    ///2: AF2
    Af2 = 2,
    ///3: AF3
    Af3 = 3,
    ///4: AF4
    Af4 = 4,
    ///5: AF5
    Af5 = 5,
    ///6: AF6
    Af6 = 6,
    ///7: AF7
    Af7 = 7,
    ///8: AF8
    Af8 = 8,
    ///9: AF9
    Af9 = 9,
    ///10: AF10
    Af10 = 10,
    ///11: AF11
    Af11 = 11,
    ///12: AF12
    Af12 = 12,
    ///13: AF13
    Af13 = 13,
    ///14: AF14
    Af14 = 14,
    ///15: AF15
    Af15 = 15,
}
impl From<AFRL0_A> for u8 {
    #[inline(always)]
    fn from(variant: AFRL0_A) -> Self {
        variant as _
    }
}
impl AFRL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFRL0_A {
        match self.bits {
            0 => AFRL0_A::Af0,
            1 => AFRL0_A::Af1,
            2 => AFRL0_A::Af2,
            3 => AFRL0_A::Af3,
            4 => AFRL0_A::Af4,
            5 => AFRL0_A::Af5,
            6 => AFRL0_A::Af6,
            7 => AFRL0_A::Af7,
            8 => AFRL0_A::Af8,
            9 => AFRL0_A::Af9,
            10 => AFRL0_A::Af10,
            11 => AFRL0_A::Af11,
            12 => AFRL0_A::Af12,
            13 => AFRL0_A::Af13,
            14 => AFRL0_A::Af14,
            15 => AFRL0_A::Af15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Af0`
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRL0_A::Af0
    }
    ///Checks if the value of the field is `Af1`
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRL0_A::Af1
    }
    ///Checks if the value of the field is `Af2`
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRL0_A::Af2
    }
    ///Checks if the value of the field is `Af3`
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRL0_A::Af3
    }
    ///Checks if the value of the field is `Af4`
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRL0_A::Af4
    }
    ///Checks if the value of the field is `Af5`
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRL0_A::Af5
    }
    ///Checks if the value of the field is `Af6`
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRL0_A::Af6
    }
    ///Checks if the value of the field is `Af7`
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRL0_A::Af7
    }
    ///Checks if the value of the field is `Af8`
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFRL0_A::Af8
    }
    ///Checks if the value of the field is `Af9`
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFRL0_A::Af9
    }
    ///Checks if the value of the field is `Af10`
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFRL0_A::Af10
    }
    ///Checks if the value of the field is `Af11`
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFRL0_A::Af11
    }
    ///Checks if the value of the field is `Af12`
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFRL0_A::Af12
    }
    ///Checks if the value of the field is `Af13`
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFRL0_A::Af13
    }
    ///Checks if the value of the field is `Af14`
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFRL0_A::Af14
    }
    ///Checks if the value of the field is `Af15`
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFRL0_A::Af15
    }
}
///Field `AFRL0` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AFRL_SPEC, u8, AFRL0_A, 4, O>;
impl<'a, const O: u8> AFRL0_W<'a, O> {
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL0_A::Af0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL0_A::Af1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL0_A::Af2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL0_A::Af3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL0_A::Af4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL0_A::Af5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL0_A::Af6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL0_A::Af7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL0_A::Af8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL0_A::Af9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL0_A::Af10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL0_A::Af11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL0_A::Af12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL0_A::Af13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL0_A::Af14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL0_A::Af15)
    }
}
///Field `AFRL1` reader - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_R as AFRL1_R;
///Field `AFRL2` reader - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_R as AFRL2_R;
///Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_R as AFRL3_R;
///Field `AFRL4` reader - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_R as AFRL4_R;
///Field `AFRL5` reader - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_R as AFRL5_R;
///Field `AFRL6` reader - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_R as AFRL6_R;
///Field `AFRL7` reader - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_R as AFRL7_R;
///Field `AFRL1` writer - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_W as AFRL1_W;
///Field `AFRL2` writer - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_W as AFRL2_W;
///Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_W as AFRL3_W;
///Field `AFRL4` writer - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_W as AFRL4_W;
///Field `AFRL5` writer - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_W as AFRL5_W;
///Field `AFRL6` writer - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_W as AFRL6_W;
///Field `AFRL7` writer - Alternate function selection for port x bit y (y = 0..7)
pub use AFRL0_W as AFRL7_W;
impl R {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl0(&self) -> AFRL0_R {
        AFRL0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl1(&self) -> AFRL1_R {
        AFRL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl2(&self) -> AFRL2_R {
        AFRL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl4(&self) -> AFRL4_R {
        AFRL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl5(&self) -> AFRL5_R {
        AFRL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl6(&self) -> AFRL6_R {
        AFRL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl7(&self) -> AFRL7_R {
        AFRL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl0(&mut self) -> AFRL0_W<0> {
        AFRL0_W::new(self)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl1(&mut self) -> AFRL1_W<4> {
        AFRL1_W::new(self)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl2(&mut self) -> AFRL2_W<8> {
        AFRL2_W::new(self)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl3(&mut self) -> AFRL3_W<12> {
        AFRL3_W::new(self)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl4(&mut self) -> AFRL4_W<16> {
        AFRL4_W::new(self)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl5(&mut self) -> AFRL5_W<20> {
        AFRL5_W::new(self)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl6(&mut self) -> AFRL6_W<24> {
        AFRL6_W::new(self)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    #[must_use]
    pub fn afrl7(&mut self) -> AFRL7_W<28> {
        AFRL7_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AFRL
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrl](index.html) module
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [afrl::R](R) reader structure
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [afrl::W](W) writer structure
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AFRL to value 0
impl crate::Resettable for AFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
