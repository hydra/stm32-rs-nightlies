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
///Field `MODER0` reader - MODER0
pub type MODER0_R = crate::FieldReader<u8, MODER0_A>;
///MODER0
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
///Field `MODER0` writer - MODER0
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
///Field `MODER1` reader - MODER1
pub use MODER0_R as MODER1_R;
///Field `MODER2` reader - MODER2
pub use MODER0_R as MODER2_R;
///Field `MODER3` reader - MODER3
pub use MODER0_R as MODER3_R;
///Field `MODER4` reader - MODER4
pub use MODER0_R as MODER4_R;
///Field `MODER5` reader - MODER5
pub use MODER0_R as MODER5_R;
///Field `MODER6` reader - MODER6
pub use MODER0_R as MODER6_R;
///Field `MODER7` reader - MODER7
pub use MODER0_R as MODER7_R;
///Field `MODER8` reader - MODER8
pub use MODER0_R as MODER8_R;
///Field `MODER9` reader - MODER9
pub use MODER0_R as MODER9_R;
///Field `MODER10` reader - MODER10
pub use MODER0_R as MODER10_R;
///Field `MODER11` reader - MODER11
pub use MODER0_R as MODER11_R;
///Field `MODER12` reader - MODER12
pub use MODER0_R as MODER12_R;
///Field `MODER13` reader - MODER13
pub use MODER0_R as MODER13_R;
///Field `MODER14` reader - MODER14
pub use MODER0_R as MODER14_R;
///Field `MODER15` reader - MODER15
pub use MODER0_R as MODER15_R;
///Field `MODER1` writer - MODER1
pub use MODER0_W as MODER1_W;
///Field `MODER2` writer - MODER2
pub use MODER0_W as MODER2_W;
///Field `MODER3` writer - MODER3
pub use MODER0_W as MODER3_W;
///Field `MODER4` writer - MODER4
pub use MODER0_W as MODER4_W;
///Field `MODER5` writer - MODER5
pub use MODER0_W as MODER5_W;
///Field `MODER6` writer - MODER6
pub use MODER0_W as MODER6_W;
///Field `MODER7` writer - MODER7
pub use MODER0_W as MODER7_W;
///Field `MODER8` writer - MODER8
pub use MODER0_W as MODER8_W;
///Field `MODER9` writer - MODER9
pub use MODER0_W as MODER9_W;
///Field `MODER10` writer - MODER10
pub use MODER0_W as MODER10_W;
///Field `MODER11` writer - MODER11
pub use MODER0_W as MODER11_W;
///Field `MODER12` writer - MODER12
pub use MODER0_W as MODER12_W;
///Field `MODER13` writer - MODER13
pub use MODER0_W as MODER13_W;
///Field `MODER14` writer - MODER14
pub use MODER0_W as MODER14_W;
///Field `MODER15` writer - MODER15
pub use MODER0_W as MODER15_W;
impl R {
    ///Bits 0:1 - MODER0
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - MODER1
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - MODER2
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - MODER3
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - MODER4
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - MODER5
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - MODER6
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - MODER7
    #[inline(always)]
    pub fn moder7(&self) -> MODER7_R {
        MODER7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - MODER8
    #[inline(always)]
    pub fn moder8(&self) -> MODER8_R {
        MODER8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - MODER9
    #[inline(always)]
    pub fn moder9(&self) -> MODER9_R {
        MODER9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - MODER10
    #[inline(always)]
    pub fn moder10(&self) -> MODER10_R {
        MODER10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - MODER11
    #[inline(always)]
    pub fn moder11(&self) -> MODER11_R {
        MODER11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - MODER12
    #[inline(always)]
    pub fn moder12(&self) -> MODER12_R {
        MODER12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - MODER13
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - MODER14
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - MODER15
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - MODER0
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER0_W<0> {
        MODER0_W::new(self)
    }
    ///Bits 2:3 - MODER1
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER1_W<2> {
        MODER1_W::new(self)
    }
    ///Bits 4:5 - MODER2
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER2_W<4> {
        MODER2_W::new(self)
    }
    ///Bits 6:7 - MODER3
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<6> {
        MODER3_W::new(self)
    }
    ///Bits 8:9 - MODER4
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER4_W<8> {
        MODER4_W::new(self)
    }
    ///Bits 10:11 - MODER5
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER5_W<10> {
        MODER5_W::new(self)
    }
    ///Bits 12:13 - MODER6
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER6_W<12> {
        MODER6_W::new(self)
    }
    ///Bits 14:15 - MODER7
    #[inline(always)]
    #[must_use]
    pub fn moder7(&mut self) -> MODER7_W<14> {
        MODER7_W::new(self)
    }
    ///Bits 16:17 - MODER8
    #[inline(always)]
    #[must_use]
    pub fn moder8(&mut self) -> MODER8_W<16> {
        MODER8_W::new(self)
    }
    ///Bits 18:19 - MODER9
    #[inline(always)]
    #[must_use]
    pub fn moder9(&mut self) -> MODER9_W<18> {
        MODER9_W::new(self)
    }
    ///Bits 20:21 - MODER10
    #[inline(always)]
    #[must_use]
    pub fn moder10(&mut self) -> MODER10_W<20> {
        MODER10_W::new(self)
    }
    ///Bits 22:23 - MODER11
    #[inline(always)]
    #[must_use]
    pub fn moder11(&mut self) -> MODER11_W<22> {
        MODER11_W::new(self)
    }
    ///Bits 24:25 - MODER12
    #[inline(always)]
    #[must_use]
    pub fn moder12(&mut self) -> MODER12_W<24> {
        MODER12_W::new(self)
    }
    ///Bits 26:27 - MODER13
    #[inline(always)]
    #[must_use]
    pub fn moder13(&mut self) -> MODER13_W<26> {
        MODER13_W::new(self)
    }
    ///Bits 28:29 - MODER14
    #[inline(always)]
    #[must_use]
    pub fn moder14(&mut self) -> MODER14_W<28> {
        MODER14_W::new(self)
    }
    ///Bits 30:31 - MODER15
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
///`reset()` method sets MODER to value 0xabff_ffff
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: Self::Ux = 0xabff_ffff;
}
