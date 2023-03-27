///Register `OSPEEDR` reader
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OSPEEDR` writer
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OSPEEDR0` reader - OSPEEDR0
pub type OSPEEDR0_R = crate::FieldReader<u8, OSPEEDR0_A>;
///OSPEEDR0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEEDR0_A {
    ///0: Low speed
    LowSpeed = 0,
    ///1: Medium speed
    MediumSpeed = 1,
    ///2: High speed
    HighSpeed = 2,
    ///3: Very high speed
    VeryHighSpeed = 3,
}
impl From<OSPEEDR0_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEEDR0_A) -> Self {
        variant as _
    }
}
impl OSPEEDR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSPEEDR0_A {
        match self.bits {
            0 => OSPEEDR0_A::LowSpeed,
            1 => OSPEEDR0_A::MediumSpeed,
            2 => OSPEEDR0_A::HighSpeed,
            3 => OSPEEDR0_A::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `LowSpeed`
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEEDR0_A::LowSpeed
    }
    ///Checks if the value of the field is `MediumSpeed`
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEEDR0_A::MediumSpeed
    }
    ///Checks if the value of the field is `HighSpeed`
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEEDR0_A::HighSpeed
    }
    ///Checks if the value of the field is `VeryHighSpeed`
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEEDR0_A::VeryHighSpeed
    }
}
///Field `OSPEEDR0` writer - OSPEEDR0
pub type OSPEEDR0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OSPEEDR_SPEC, u8, OSPEEDR0_A, 2, O>;
impl<'a, const O: u8> OSPEEDR0_W<'a, O> {
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::LowSpeed)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::MediumSpeed)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::HighSpeed)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::VeryHighSpeed)
    }
}
///Field `OSPEEDR1` reader - OSPEEDR1
pub use OSPEEDR0_R as OSPEEDR1_R;
///Field `OSPEEDR2` reader - OSPEEDR2
pub use OSPEEDR0_R as OSPEEDR2_R;
///Field `OSPEEDR3` reader - OSPEEDR3
pub use OSPEEDR0_R as OSPEEDR3_R;
///Field `OSPEEDR4` reader - OSPEEDR4
pub use OSPEEDR0_R as OSPEEDR4_R;
///Field `OSPEEDR5` reader - OSPEEDR5
pub use OSPEEDR0_R as OSPEEDR5_R;
///Field `OSPEEDR6` reader - OSPEEDR6
pub use OSPEEDR0_R as OSPEEDR6_R;
///Field `OSPEEDR7` reader - OSPEEDR7
pub use OSPEEDR0_R as OSPEEDR7_R;
///Field `OSPEEDR8` reader - OSPEEDR8
pub use OSPEEDR0_R as OSPEEDR8_R;
///Field `OSPEEDR9` reader - OSPEEDR9
pub use OSPEEDR0_R as OSPEEDR9_R;
///Field `OSPEEDR10` reader - OSPEEDR10
pub use OSPEEDR0_R as OSPEEDR10_R;
///Field `OSPEEDR11` reader - OSPEEDR11
pub use OSPEEDR0_R as OSPEEDR11_R;
///Field `OSPEEDR12` reader - OSPEEDR12
pub use OSPEEDR0_R as OSPEEDR12_R;
///Field `OSPEEDR13` reader - OSPEEDR13
pub use OSPEEDR0_R as OSPEEDR13_R;
///Field `OSPEEDR14` reader - OSPEEDR14
pub use OSPEEDR0_R as OSPEEDR14_R;
///Field `OSPEEDR15` reader - OSPEEDR15
pub use OSPEEDR0_R as OSPEEDR15_R;
///Field `OSPEEDR1` writer - OSPEEDR1
pub use OSPEEDR0_W as OSPEEDR1_W;
///Field `OSPEEDR2` writer - OSPEEDR2
pub use OSPEEDR0_W as OSPEEDR2_W;
///Field `OSPEEDR3` writer - OSPEEDR3
pub use OSPEEDR0_W as OSPEEDR3_W;
///Field `OSPEEDR4` writer - OSPEEDR4
pub use OSPEEDR0_W as OSPEEDR4_W;
///Field `OSPEEDR5` writer - OSPEEDR5
pub use OSPEEDR0_W as OSPEEDR5_W;
///Field `OSPEEDR6` writer - OSPEEDR6
pub use OSPEEDR0_W as OSPEEDR6_W;
///Field `OSPEEDR7` writer - OSPEEDR7
pub use OSPEEDR0_W as OSPEEDR7_W;
///Field `OSPEEDR8` writer - OSPEEDR8
pub use OSPEEDR0_W as OSPEEDR8_W;
///Field `OSPEEDR9` writer - OSPEEDR9
pub use OSPEEDR0_W as OSPEEDR9_W;
///Field `OSPEEDR10` writer - OSPEEDR10
pub use OSPEEDR0_W as OSPEEDR10_W;
///Field `OSPEEDR11` writer - OSPEEDR11
pub use OSPEEDR0_W as OSPEEDR11_W;
///Field `OSPEEDR12` writer - OSPEEDR12
pub use OSPEEDR0_W as OSPEEDR12_W;
///Field `OSPEEDR13` writer - OSPEEDR13
pub use OSPEEDR0_W as OSPEEDR13_W;
///Field `OSPEEDR14` writer - OSPEEDR14
pub use OSPEEDR0_W as OSPEEDR14_W;
///Field `OSPEEDR15` writer - OSPEEDR15
pub use OSPEEDR0_W as OSPEEDR15_W;
impl R {
    ///Bits 0:1 - OSPEEDR0
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - OSPEEDR1
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - OSPEEDR2
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - OSPEEDR3
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - OSPEEDR4
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - OSPEEDR5
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - OSPEEDR6
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - OSPEEDR7
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR7_R {
        OSPEEDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - OSPEEDR8
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR8_R {
        OSPEEDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - OSPEEDR9
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR9_R {
        OSPEEDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - OSPEEDR10
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR10_R {
        OSPEEDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - OSPEEDR11
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR11_R {
        OSPEEDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - OSPEEDR12
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR12_R {
        OSPEEDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - OSPEEDR13
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - OSPEEDR14
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - OSPEEDR15
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - OSPEEDR0
    #[inline(always)]
    #[must_use]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<0> {
        OSPEEDR0_W::new(self)
    }
    ///Bits 2:3 - OSPEEDR1
    #[inline(always)]
    #[must_use]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<2> {
        OSPEEDR1_W::new(self)
    }
    ///Bits 4:5 - OSPEEDR2
    #[inline(always)]
    #[must_use]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<4> {
        OSPEEDR2_W::new(self)
    }
    ///Bits 6:7 - OSPEEDR3
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<6> {
        OSPEEDR3_W::new(self)
    }
    ///Bits 8:9 - OSPEEDR4
    #[inline(always)]
    #[must_use]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<8> {
        OSPEEDR4_W::new(self)
    }
    ///Bits 10:11 - OSPEEDR5
    #[inline(always)]
    #[must_use]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W<10> {
        OSPEEDR5_W::new(self)
    }
    ///Bits 12:13 - OSPEEDR6
    #[inline(always)]
    #[must_use]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W<12> {
        OSPEEDR6_W::new(self)
    }
    ///Bits 14:15 - OSPEEDR7
    #[inline(always)]
    #[must_use]
    pub fn ospeedr7(&mut self) -> OSPEEDR7_W<14> {
        OSPEEDR7_W::new(self)
    }
    ///Bits 16:17 - OSPEEDR8
    #[inline(always)]
    #[must_use]
    pub fn ospeedr8(&mut self) -> OSPEEDR8_W<16> {
        OSPEEDR8_W::new(self)
    }
    ///Bits 18:19 - OSPEEDR9
    #[inline(always)]
    #[must_use]
    pub fn ospeedr9(&mut self) -> OSPEEDR9_W<18> {
        OSPEEDR9_W::new(self)
    }
    ///Bits 20:21 - OSPEEDR10
    #[inline(always)]
    #[must_use]
    pub fn ospeedr10(&mut self) -> OSPEEDR10_W<20> {
        OSPEEDR10_W::new(self)
    }
    ///Bits 22:23 - OSPEEDR11
    #[inline(always)]
    #[must_use]
    pub fn ospeedr11(&mut self) -> OSPEEDR11_W<22> {
        OSPEEDR11_W::new(self)
    }
    ///Bits 24:25 - OSPEEDR12
    #[inline(always)]
    #[must_use]
    pub fn ospeedr12(&mut self) -> OSPEEDR12_W<24> {
        OSPEEDR12_W::new(self)
    }
    ///Bits 26:27 - OSPEEDR13
    #[inline(always)]
    #[must_use]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W<26> {
        OSPEEDR13_W::new(self)
    }
    ///Bits 28:29 - OSPEEDR14
    #[inline(always)]
    #[must_use]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W<28> {
        OSPEEDR14_W::new(self)
    }
    ///Bits 30:31 - OSPEEDR15
    #[inline(always)]
    #[must_use]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W<30> {
        OSPEEDR15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output speed register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ospeedr](index.html) module
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ospeedr::R](R) reader structure
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ospeedr::W](W) writer structure
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OSPEEDR to value 0xc0
impl crate::Resettable for OSPEEDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
