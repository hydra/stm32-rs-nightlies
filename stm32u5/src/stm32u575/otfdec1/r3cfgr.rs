///Register `R3CFGR` reader
pub struct R(crate::R<R3CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R3CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R3CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R3CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `R3CFGR` writer
pub struct W(crate::W<R3CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R3CFGR_SPEC>;
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
impl From<crate::W<R3CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R3CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REG_EN` reader - region on-the-fly decryption enable
pub type REG_EN_R = crate::BitReader<bool>;
///Field `REG_EN` writer - region on-the-fly decryption enable
pub type REG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, R3CFGR_SPEC, bool, O>;
///Field `CONFIGLOCK` reader - region config lock
pub type CONFIGLOCK_R = crate::BitReader<bool>;
///Field `CONFIGLOCK` writer - region config lock
pub type CONFIGLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, R3CFGR_SPEC, bool, O>;
///Field `KEYLOCK` reader - region key lock
pub type KEYLOCK_R = crate::BitReader<bool>;
///Field `KEYLOCK` writer - region key lock
pub type KEYLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, R3CFGR_SPEC, bool, O>;
///Field `MODE` reader - operating mode
pub type MODE_R = crate::FieldReader<u8, u8>;
///Field `MODE` writer - operating mode
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, R3CFGR_SPEC, u8, u8, 2, O>;
///Field `KEYCRC` reader - region key 8-bit CRC
pub type KEYCRC_R = crate::FieldReader<u8, u8>;
///Field `REGx_VERSION` reader - region firmware version
pub type REGX_VERSION_R = crate::FieldReader<u16, u16>;
///Field `REGx_VERSION` writer - region firmware version
pub type REGX_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R3CFGR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bit 0 - region on-the-fly decryption enable
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - region config lock
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - region key lock
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:15 - region key 8-bit CRC
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - region firmware version
    #[inline(always)]
    pub fn regx_version(&self) -> REGX_VERSION_R {
        REGX_VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - region on-the-fly decryption enable
    #[inline(always)]
    #[must_use]
    pub fn reg_en(&mut self) -> REG_EN_W<0> {
        REG_EN_W::new(self)
    }
    ///Bit 1 - region config lock
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<1> {
        CONFIGLOCK_W::new(self)
    }
    ///Bit 2 - region key lock
    #[inline(always)]
    #[must_use]
    pub fn keylock(&mut self) -> KEYLOCK_W<2> {
        KEYLOCK_W::new(self)
    }
    ///Bits 4:5 - operating mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    ///Bits 16:31 - region firmware version
    #[inline(always)]
    #[must_use]
    pub fn regx_version(&mut self) -> REGX_VERSION_W<16> {
        REGX_VERSION_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTFDEC region x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r3cfgr](index.html) module
pub struct R3CFGR_SPEC;
impl crate::RegisterSpec for R3CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [r3cfgr::R](R) reader structure
impl crate::Readable for R3CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [r3cfgr::W](W) writer structure
impl crate::Writable for R3CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets R3CFGR to value 0
impl crate::Resettable for R3CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
