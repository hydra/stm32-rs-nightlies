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
///Field `MODE0` reader - Port x configuration bits (y = 0..15)
pub type MODE0_R = crate::FieldReader<u8, MODE0_A>;
///Port x configuration bits (y = 0..15)
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0_A {
    ///0: Input mode
    Input = 0,
    ///1: General purpose output mode
    Output = 1,
    ///2: Alternate function mode
    Alternate = 2,
    ///3: Analog mode
    Analog = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
impl MODE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::Input,
            1 => MODE0_A::Output,
            2 => MODE0_A::Alternate,
            3 => MODE0_A::Analog,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Input`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0_A::Input
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE0_A::Output
    }
    ///Checks if the value of the field is `Alternate`
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODE0_A::Alternate
    }
    ///Checks if the value of the field is `Analog`
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODE0_A::Analog
    }
}
///Field `MODE0` writer - Port x configuration bits (y = 0..15)
pub type MODE0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MODER_SPEC, u8, MODE0_A, 2, O>;
impl<'a, const O: u8> MODE0_W<'a, O> {
    ///Input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0_A::Input)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0_A::Output)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE0_A::Alternate)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE0_A::Analog)
    }
}
///Field `MODE1` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE1_R;
///Field `MODE2` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE2_R;
///Field `MODE3` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE3_R;
///Field `MODE4` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE4_R;
///Field `MODE5` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE5_R;
///Field `MODE6` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE6_R;
///Field `MODE7` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE7_R;
///Field `MODE8` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE8_R;
///Field `MODE9` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE9_R;
///Field `MODE10` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE10_R;
///Field `MODE11` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE11_R;
///Field `MODE12` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE12_R;
///Field `MODE13` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE13_R;
///Field `MODE14` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE14_R;
///Field `MODE15` reader - Port x configuration bits (y = 0..15)
pub use MODE0_R as MODE15_R;
///Field `MODE1` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE1_W;
///Field `MODE2` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE2_W;
///Field `MODE3` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE3_W;
///Field `MODE4` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE4_W;
///Field `MODE5` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE5_W;
///Field `MODE6` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE6_W;
///Field `MODE7` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE7_W;
///Field `MODE8` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE8_W;
///Field `MODE9` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE9_W;
///Field `MODE10` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE10_W;
///Field `MODE11` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE11_W;
///Field `MODE12` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE12_W;
///Field `MODE13` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE13_W;
///Field `MODE14` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE14_W;
///Field `MODE15` writer - Port x configuration bits (y = 0..15)
pub use MODE0_W as MODE15_W;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<0> {
        MODE0_W::new(self)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<2> {
        MODE1_W::new(self)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<4> {
        MODE2_W::new(self)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<6> {
        MODE3_W::new(self)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<8> {
        MODE4_W::new(self)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<10> {
        MODE5_W::new(self)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<12> {
        MODE6_W::new(self)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<14> {
        MODE7_W::new(self)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<16> {
        MODE8_W::new(self)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<18> {
        MODE9_W::new(self)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE10_W<20> {
        MODE10_W::new(self)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE11_W<22> {
        MODE11_W::new(self)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE12_W<24> {
        MODE12_W::new(self)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE13_W<26> {
        MODE13_W::new(self)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE14_W<28> {
        MODE14_W::new(self)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE15_W<30> {
        MODE15_W::new(self)
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
///`reset()` method sets MODER to value 0xebff_fcff
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: Self::Ux = 0xebff_fcff;
}
