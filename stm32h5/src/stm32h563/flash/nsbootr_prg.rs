///Register `NSBOOTR_PRG` reader
pub struct R(crate::R<NSBOOTR_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSBOOTR_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSBOOTR_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSBOOTR_PRG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `NSBOOTR_PRG` writer
pub struct W(crate::W<NSBOOTR_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSBOOTR_PRG_SPEC>;
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
impl From<crate::W<NSBOOTR_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSBOOTR_PRG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NSBOOT_LOCK` reader - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
pub type NSBOOT_LOCK_R = crate::FieldReader<u8, u8>;
///Field `NSBOOT_LOCK` writer - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
pub type NSBOOT_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NSBOOTR_PRG_SPEC, u8, u8, 8, O>;
///Field `NSBOOTADD` reader - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
pub type NSBOOTADD_R = crate::FieldReader<u32, u32>;
///Field `NSBOOTADD` writer - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
pub type NSBOOTADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NSBOOTR_PRG_SPEC, u32, u32, 24, O>;
impl R {
    ///Bits 0:7 - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
    #[inline(always)]
    pub fn nsboot_lock(&self) -> NSBOOT_LOCK_R {
        NSBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31 - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
    #[inline(always)]
    pub fn nsbootadd(&self) -> NSBOOTADD_R {
        NSBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:7 - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
    #[inline(always)]
    #[must_use]
    pub fn nsboot_lock(&mut self) -> NSBOOT_LOCK_W<0> {
        NSBOOT_LOCK_W::new(self)
    }
    ///Bits 8:31 - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd(&mut self) -> NSBOOTADD_W<8> {
        NSBOOTADD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH non-secure boot register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [nsbootr_prg](index.html) module
pub struct NSBOOTR_PRG_SPEC;
impl crate::RegisterSpec for NSBOOTR_PRG_SPEC {
    type Ux = u32;
}
///`read()` method returns [nsbootr_prg::R](R) reader structure
impl crate::Readable for NSBOOTR_PRG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [nsbootr_prg::W](W) writer structure
impl crate::Writable for NSBOOTR_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets NSBOOTR_PRG to value 0
impl crate::Resettable for NSBOOTR_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
