///Register `GPIOJ_AFRH` reader
pub struct R(crate::R<GPIOJ_AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOJ_AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOJ_AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOJ_AFRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPIOJ_AFRH` writer
pub struct W(crate::W<GPIOJ_AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOJ_AFRH_SPEC>;
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
impl From<crate::W<GPIOJ_AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOJ_AFRH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AFR8` reader - AFR8
pub type AFR8_R = crate::FieldReader<u8, u8>;
///Field `AFR8` writer - AFR8
pub type AFR8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
///Field `AFR9` reader - AFR9
pub type AFR9_R = crate::FieldReader<u8, u8>;
///Field `AFR9` writer - AFR9
pub type AFR9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
///Field `AFR10` reader - AFR10
pub type AFR10_R = crate::FieldReader<u8, u8>;
///Field `AFR10` writer - AFR10
pub type AFR10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
///Field `AFR11` reader - AFR11
pub type AFR11_R = crate::FieldReader<u8, u8>;
///Field `AFR11` writer - AFR11
pub type AFR11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
///Field `AFR12` reader - AFR12
pub type AFR12_R = crate::FieldReader<u8, u8>;
///Field `AFR12` writer - AFR12
pub type AFR12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
///Field `AFR13` reader - AFR13
pub type AFR13_R = crate::FieldReader<u8, u8>;
///Field `AFR13` writer - AFR13
pub type AFR13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
///Field `AFR14` reader - AFR14
pub type AFR14_R = crate::FieldReader<u8, u8>;
///Field `AFR14` writer - AFR14
pub type AFR14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
///Field `AFR15` reader - AFR15
pub type AFR15_R = crate::FieldReader<u8, u8>;
///Field `AFR15` writer - AFR15
pub type AFR15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOJ_AFRH_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - AFR8
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AFR9
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - AFR10
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - AFR11
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - AFR12
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - AFR13
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - AFR14
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - AFR15
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - AFR8
    #[inline(always)]
    #[must_use]
    pub fn afr8(&mut self) -> AFR8_W<0> {
        AFR8_W::new(self)
    }
    ///Bits 4:7 - AFR9
    #[inline(always)]
    #[must_use]
    pub fn afr9(&mut self) -> AFR9_W<4> {
        AFR9_W::new(self)
    }
    ///Bits 8:11 - AFR10
    #[inline(always)]
    #[must_use]
    pub fn afr10(&mut self) -> AFR10_W<8> {
        AFR10_W::new(self)
    }
    ///Bits 12:15 - AFR11
    #[inline(always)]
    #[must_use]
    pub fn afr11(&mut self) -> AFR11_W<12> {
        AFR11_W::new(self)
    }
    ///Bits 16:19 - AFR12
    #[inline(always)]
    #[must_use]
    pub fn afr12(&mut self) -> AFR12_W<16> {
        AFR12_W::new(self)
    }
    ///Bits 20:23 - AFR13
    #[inline(always)]
    #[must_use]
    pub fn afr13(&mut self) -> AFR13_W<20> {
        AFR13_W::new(self)
    }
    ///Bits 24:27 - AFR14
    #[inline(always)]
    #[must_use]
    pub fn afr14(&mut self) -> AFR14_W<24> {
        AFR14_W::new(self)
    }
    ///Bits 28:31 - AFR15
    #[inline(always)]
    #[must_use]
    pub fn afr15(&mut self) -> AFR15_W<28> {
        AFR15_W::new(self)
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
///For information about available fields see [gpioj_afrh](index.html) module
pub struct GPIOJ_AFRH_SPEC;
impl crate::RegisterSpec for GPIOJ_AFRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioj_afrh::R](R) reader structure
impl crate::Readable for GPIOJ_AFRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpioj_afrh::W](W) writer structure
impl crate::Writable for GPIOJ_AFRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPIOJ_AFRH to value 0
impl crate::Resettable for GPIOJ_AFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
