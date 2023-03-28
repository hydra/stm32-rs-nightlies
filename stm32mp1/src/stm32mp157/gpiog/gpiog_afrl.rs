///Register `GPIOG_AFRL` reader
pub struct R(crate::R<GPIOG_AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOG_AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOG_AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOG_AFRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPIOG_AFRL` writer
pub struct W(crate::W<GPIOG_AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOG_AFRL_SPEC>;
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
impl From<crate::W<GPIOG_AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOG_AFRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AFR0` reader - AFR0
pub type AFR0_R = crate::FieldReader<u8, u8>;
///Field `AFR0` writer - AFR0
pub type AFR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
///Field `AFR1` reader - AFR1
pub type AFR1_R = crate::FieldReader<u8, u8>;
///Field `AFR1` writer - AFR1
pub type AFR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
///Field `AFR2` reader - AFR2
pub type AFR2_R = crate::FieldReader<u8, u8>;
///Field `AFR2` writer - AFR2
pub type AFR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
///Field `AFR3` reader - AFR3
pub type AFR3_R = crate::FieldReader<u8, u8>;
///Field `AFR3` writer - AFR3
pub type AFR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
///Field `AFR4` reader - AFR4
pub type AFR4_R = crate::FieldReader<u8, u8>;
///Field `AFR4` writer - AFR4
pub type AFR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
///Field `AFR5` reader - AFR5
pub type AFR5_R = crate::FieldReader<u8, u8>;
///Field `AFR5` writer - AFR5
pub type AFR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
///Field `AFR6` reader - AFR6
pub type AFR6_R = crate::FieldReader<u8, u8>;
///Field `AFR6` writer - AFR6
pub type AFR6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
///Field `AFR7` reader - AFR7
pub type AFR7_R = crate::FieldReader<u8, u8>;
///Field `AFR7` writer - AFR7
pub type AFR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOG_AFRL_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - AFR0
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AFR1
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - AFR2
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - AFR3
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - AFR4
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - AFR5
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - AFR6
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - AFR7
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - AFR0
    #[inline(always)]
    #[must_use]
    pub fn afr0(&mut self) -> AFR0_W<0> {
        AFR0_W::new(self)
    }
    ///Bits 4:7 - AFR1
    #[inline(always)]
    #[must_use]
    pub fn afr1(&mut self) -> AFR1_W<4> {
        AFR1_W::new(self)
    }
    ///Bits 8:11 - AFR2
    #[inline(always)]
    #[must_use]
    pub fn afr2(&mut self) -> AFR2_W<8> {
        AFR2_W::new(self)
    }
    ///Bits 12:15 - AFR3
    #[inline(always)]
    #[must_use]
    pub fn afr3(&mut self) -> AFR3_W<12> {
        AFR3_W::new(self)
    }
    ///Bits 16:19 - AFR4
    #[inline(always)]
    #[must_use]
    pub fn afr4(&mut self) -> AFR4_W<16> {
        AFR4_W::new(self)
    }
    ///Bits 20:23 - AFR5
    #[inline(always)]
    #[must_use]
    pub fn afr5(&mut self) -> AFR5_W<20> {
        AFR5_W::new(self)
    }
    ///Bits 24:27 - AFR6
    #[inline(always)]
    #[must_use]
    pub fn afr6(&mut self) -> AFR6_W<24> {
        AFR6_W::new(self)
    }
    ///Bits 28:31 - AFR7
    #[inline(always)]
    #[must_use]
    pub fn afr7(&mut self) -> AFR7_W<28> {
        AFR7_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO alternate function low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpiog_afrl](index.html) module
pub struct GPIOG_AFRL_SPEC;
impl crate::RegisterSpec for GPIOG_AFRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpiog_afrl::R](R) reader structure
impl crate::Readable for GPIOG_AFRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpiog_afrl::W](W) writer structure
impl crate::Writable for GPIOG_AFRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPIOG_AFRL to value 0
impl crate::Resettable for GPIOG_AFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
