///Register `RXF0S` reader
pub struct R(crate::R<RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RXF0S` writer
pub struct W(crate::W<RXF0S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0S_SPEC>;
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
impl From<crate::W<RXF0S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0S_SPEC>) -> Self {
        W(writer)
    }
}
///Field `F0FL` reader - F0FL
pub type F0FL_R = crate::FieldReader<u8, u8>;
///Field `F0FL` writer - F0FL
pub type F0FL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXF0S_SPEC, u8, u8, 7, O>;
///Field `F0GI` reader - F0GI
pub type F0GI_R = crate::FieldReader<u8, u8>;
///Field `F0GI` writer - F0GI
pub type F0GI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXF0S_SPEC, u8, u8, 6, O>;
///Field `F0PI` reader - F0PI
pub type F0PI_R = crate::FieldReader<u8, u8>;
///Field `F0PI` writer - F0PI
pub type F0PI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXF0S_SPEC, u8, u8, 6, O>;
///Field `F0F` reader - F0F
pub type F0F_R = crate::BitReader<bool>;
///Field `F0F` writer - F0F
pub type F0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXF0S_SPEC, bool, O>;
///Field `RF0L` reader - RF0L
pub type RF0L_R = crate::BitReader<bool>;
///Field `RF0L` writer - RF0L
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXF0S_SPEC, bool, O>;
impl R {
    ///Bits 0:6 - F0FL
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:13 - F0GI
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - F0PI
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bit 24 - F0F
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - RF0L
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - F0FL
    #[inline(always)]
    #[must_use]
    pub fn f0fl(&mut self) -> F0FL_W<0> {
        F0FL_W::new(self)
    }
    ///Bits 8:13 - F0GI
    #[inline(always)]
    #[must_use]
    pub fn f0gi(&mut self) -> F0GI_W<8> {
        F0GI_W::new(self)
    }
    ///Bits 16:21 - F0PI
    #[inline(always)]
    #[must_use]
    pub fn f0pi(&mut self) -> F0PI_W<16> {
        F0PI_W::new(self)
    }
    ///Bit 24 - F0F
    #[inline(always)]
    #[must_use]
    pub fn f0f(&mut self) -> F0F_W<24> {
        F0F_W::new(self)
    }
    ///Bit 25 - RF0L
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<25> {
        RF0L_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Rx FIFO 0 Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxf0s](index.html) module
pub struct RXF0S_SPEC;
impl crate::RegisterSpec for RXF0S_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxf0s::R](R) reader structure
impl crate::Readable for RXF0S_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rxf0s::W](W) writer structure
impl crate::Writable for RXF0S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RXF0S to value 0
impl crate::Resettable for RXF0S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
