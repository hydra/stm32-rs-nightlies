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
///Field `AFSEL0` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL0_R = crate::FieldReader<u8, u8>;
///Field `AFSEL0` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
///Field `AFSEL1` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL1_R = crate::FieldReader<u8, u8>;
///Field `AFSEL1` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
///Field `AFSEL2` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL2_R = crate::FieldReader<u8, u8>;
///Field `AFSEL2` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
///Field `AFSEL3` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL3_R = crate::FieldReader<u8, u8>;
///Field `AFSEL3` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
///Field `AFSEL4` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL4_R = crate::FieldReader<u8, u8>;
///Field `AFSEL4` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
///Field `AFSEL5` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL5_R = crate::FieldReader<u8, u8>;
///Field `AFSEL5` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
///Field `AFSEL6` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL6_R = crate::FieldReader<u8, u8>;
///Field `AFSEL6` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
///Field `AFSEL7` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL7_R = crate::FieldReader<u8, u8>;
///Field `AFSEL7` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
pub type AFSEL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> AFSEL0_W<0> {
        AFSEL0_W::new(self)
    }
    ///Bits 4:7 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> AFSEL1_W<4> {
        AFSEL1_W::new(self)
    }
    ///Bits 8:11 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel2(&mut self) -> AFSEL2_W<8> {
        AFSEL2_W::new(self)
    }
    ///Bits 12:15 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> AFSEL3_W<12> {
        AFSEL3_W::new(self)
    }
    ///Bits 16:19 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel4(&mut self) -> AFSEL4_W<16> {
        AFSEL4_W::new(self)
    }
    ///Bits 20:23 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel5(&mut self) -> AFSEL5_W<20> {
        AFSEL5_W::new(self)
    }
    ///Bits 24:27 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel6(&mut self) -> AFSEL6_W<24> {
        AFSEL6_W::new(self)
    }
    ///Bits 28:31 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    #[must_use]
    pub fn afsel7(&mut self) -> AFSEL7_W<28> {
        AFSEL7_W::new(self)
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
