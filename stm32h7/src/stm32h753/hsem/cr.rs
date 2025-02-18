///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MASTERID` reader - MasterID of semaphores to be cleared
pub type MASTERID_R = crate::FieldReader<u8, u8>;
///Field `MASTERID` writer - MasterID of semaphores to be cleared
pub type MASTERID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 8, O>;
///Field `KEY` reader - Semaphore clear Key
pub type KEY_R = crate::FieldReader<u16, u16>;
///Field `KEY` writer - Semaphore clear Key
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 8:15 - MasterID of semaphores to be cleared
    #[inline(always)]
    pub fn masterid(&self) -> MASTERID_R {
        MASTERID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - Semaphore clear Key
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 8:15 - MasterID of semaphores to be cleared
    #[inline(always)]
    #[must_use]
    pub fn masterid(&mut self) -> MASTERID_W<8> {
        MASTERID_W::new(self)
    }
    ///Bits 16:31 - Semaphore clear Key
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HSEM Clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
