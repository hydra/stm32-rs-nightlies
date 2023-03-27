///Register `MODER` reader
pub struct R(crate::R<MODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MODER` writer
pub struct W(crate::W<MODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODER_SPEC>;
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
impl From<crate::W<MODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MODER0` reader - Port x configuration bits (y = 0..15)
pub type MODER0_R = crate::FieldReader<u8, MODER0_A>;
///Port x configuration bits (y = 0..15)
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODER0_A {
    ///0: Input mode (reset state)
    Input = 0,
    ///1: General purpose output mode
    Output = 1,
    ///2: Alternate function mode
    Alternate = 2,
    ///3: Analog mode
    Analog = 3,
}
impl From<MODER0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODER0_A) -> Self {
        variant as _
    }
}
impl MODER0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODER0_A {
        match self.bits {
            0 => MODER0_A::Input,
            1 => MODER0_A::Output,
            2 => MODER0_A::Alternate,
            3 => MODER0_A::Analog,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Input`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODER0_A::Input
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODER0_A::Output
    }
    ///Checks if the value of the field is `Alternate`
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODER0_A::Alternate
    }
    ///Checks if the value of the field is `Analog`
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODER0_A::Analog
    }
}
///Field `MODER0` writer - Port x configuration bits (y = 0..15)
pub type MODER0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODER_SPEC, u8, MODER0_A, 2, O>;
impl<'a, const O: u8> MODER0_W<'a, O> {
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER0_A::Input)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER0_A::Output)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER0_A::Alternate)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER0_A::Analog)
    }
}
///Field `MODER1` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER1_R;
///Field `MODER2` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER2_R;
///Field `MODER3` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER3_R;
///Field `MODER4` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER4_R;
///Field `MODER5` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER5_R;
///Field `MODER6` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER6_R;
///Field `MODER13` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER13_R;
///Field `MODER14` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER14_R;
///Field `MODER15` reader - Port x configuration bits (y = 0..15)
pub use MODER0_R as MODER15_R;
///Field `MODER1` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER1_W;
///Field `MODER2` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER2_W;
///Field `MODER3` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER3_W;
///Field `MODER4` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER4_W;
///Field `MODER5` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER5_W;
///Field `MODER6` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER6_W;
///Field `MODER13` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER13_W;
///Field `MODER14` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER14_W;
///Field `MODER15` writer - Port x configuration bits (y = 0..15)
pub use MODER0_W as MODER15_W;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER0_W<0> {
        MODER0_W::new(self)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER1_W<2> {
        MODER1_W::new(self)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER2_W<4> {
        MODER2_W::new(self)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<6> {
        MODER3_W::new(self)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER4_W<8> {
        MODER4_W::new(self)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER5_W<10> {
        MODER5_W::new(self)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER6_W<12> {
        MODER6_W::new(self)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder13(&mut self) -> MODER13_W<26> {
        MODER13_W::new(self)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder14(&mut self) -> MODER14_W<28> {
        MODER14_W::new(self)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn moder15(&mut self) -> MODER15_W<30> {
        MODER15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [moder](index.html) module
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
///`read()` method returns [moder::R](R) reader structure
impl crate::Readable for MODER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [moder::W](W) writer structure
impl crate::Writable for MODER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MODER to value 0xfc00_3fff
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: Self::Ux = 0xfc00_3fff;
}
