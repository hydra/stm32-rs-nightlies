///Register `AFRH` reader
pub struct R(crate::R<AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AFRH` writer
pub struct W(crate::W<AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRH_SPEC>;
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
impl From<crate::W<AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AFRH8` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH8_R = crate::FieldReader<u8, AFRH8_A>;
///Alternate function selection for port x bit y (y = 8..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFRH8_A {
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
impl From<AFRH8_A> for u8 {
    #[inline(always)]
    fn from(variant: AFRH8_A) -> Self {
        variant as _
    }
}
impl AFRH8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFRH8_A {
        match self.bits {
            0 => AFRH8_A::Af0,
            1 => AFRH8_A::Af1,
            2 => AFRH8_A::Af2,
            3 => AFRH8_A::Af3,
            4 => AFRH8_A::Af4,
            5 => AFRH8_A::Af5,
            6 => AFRH8_A::Af6,
            7 => AFRH8_A::Af7,
            8 => AFRH8_A::Af8,
            9 => AFRH8_A::Af9,
            10 => AFRH8_A::Af10,
            11 => AFRH8_A::Af11,
            12 => AFRH8_A::Af12,
            13 => AFRH8_A::Af13,
            14 => AFRH8_A::Af14,
            15 => AFRH8_A::Af15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Af0`
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRH8_A::Af0
    }
    ///Checks if the value of the field is `Af1`
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRH8_A::Af1
    }
    ///Checks if the value of the field is `Af2`
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRH8_A::Af2
    }
    ///Checks if the value of the field is `Af3`
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRH8_A::Af3
    }
    ///Checks if the value of the field is `Af4`
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRH8_A::Af4
    }
    ///Checks if the value of the field is `Af5`
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRH8_A::Af5
    }
    ///Checks if the value of the field is `Af6`
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRH8_A::Af6
    }
    ///Checks if the value of the field is `Af7`
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRH8_A::Af7
    }
    ///Checks if the value of the field is `Af8`
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFRH8_A::Af8
    }
    ///Checks if the value of the field is `Af9`
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFRH8_A::Af9
    }
    ///Checks if the value of the field is `Af10`
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFRH8_A::Af10
    }
    ///Checks if the value of the field is `Af11`
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFRH8_A::Af11
    }
    ///Checks if the value of the field is `Af12`
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFRH8_A::Af12
    }
    ///Checks if the value of the field is `Af13`
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFRH8_A::Af13
    }
    ///Checks if the value of the field is `Af14`
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFRH8_A::Af14
    }
    ///Checks if the value of the field is `Af15`
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFRH8_A::Af15
    }
}
///Field `AFRH8` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH8_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AFRH_SPEC, u8, AFRH8_A, 4, O>;
impl<'a, const O: u8> AFRH8_W<'a, O> {
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH8_A::Af0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH8_A::Af1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH8_A::Af2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH8_A::Af3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH8_A::Af4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH8_A::Af5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH8_A::Af6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH8_A::Af7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH8_A::Af8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH8_A::Af9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH8_A::Af10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH8_A::Af11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH8_A::Af12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH8_A::Af13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH8_A::Af14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH8_A::Af15)
    }
}
///Field `AFRH9` reader - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_R as AFRH9_R;
///Field `AFRH10` reader - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_R as AFRH10_R;
///Field `AFRH11` reader - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_R as AFRH11_R;
///Field `AFRH12` reader - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_R as AFRH12_R;
///Field `AFRH13` reader - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_R as AFRH13_R;
///Field `AFRH14` reader - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_R as AFRH14_R;
///Field `AFRH15` reader - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_R as AFRH15_R;
///Field `AFRH9` writer - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_W as AFRH9_W;
///Field `AFRH10` writer - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_W as AFRH10_W;
///Field `AFRH11` writer - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_W as AFRH11_W;
///Field `AFRH12` writer - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_W as AFRH12_W;
///Field `AFRH13` writer - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_W as AFRH13_W;
///Field `AFRH14` writer - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_W as AFRH14_W;
///Field `AFRH15` writer - Alternate function selection for port x bit y (y = 8..15)
pub use AFRH8_W as AFRH15_W;
impl R {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh8(&self) -> AFRH8_R {
        AFRH8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh9(&self) -> AFRH9_R {
        AFRH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh10(&self) -> AFRH10_R {
        AFRH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh11(&self) -> AFRH11_R {
        AFRH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh12(&self) -> AFRH12_R {
        AFRH12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh13(&self) -> AFRH13_R {
        AFRH13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh14(&self) -> AFRH14_R {
        AFRH14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh15(&self) -> AFRH15_R {
        AFRH15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh8(&mut self) -> AFRH8_W<0> {
        AFRH8_W::new(self)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh9(&mut self) -> AFRH9_W<4> {
        AFRH9_W::new(self)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh10(&mut self) -> AFRH10_W<8> {
        AFRH10_W::new(self)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh11(&mut self) -> AFRH11_W<12> {
        AFRH11_W::new(self)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh12(&mut self) -> AFRH12_W<16> {
        AFRH12_W::new(self)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh13(&mut self) -> AFRH13_W<20> {
        AFRH13_W::new(self)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh14(&mut self) -> AFRH14_W<24> {
        AFRH14_W::new(self)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afrh15(&mut self) -> AFRH15_W<28> {
        AFRH15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO alternate function high register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrh](index.html) module
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [afrh::R](R) reader structure
impl crate::Readable for AFRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [afrh::W](W) writer structure
impl crate::Writable for AFRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
