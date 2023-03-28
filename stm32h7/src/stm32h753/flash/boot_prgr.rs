///Register `BOOT_PRGR` reader
pub struct R(crate::R<BOOT_PRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_PRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_PRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_PRGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BOOT_PRGR` writer
pub struct W(crate::W<BOOT_PRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_PRGR_SPEC>;
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
impl From<crate::W<BOOT_PRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_PRGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BOOT_ADD0` reader - Boot address 0
pub type BOOT_ADD0_R = crate::FieldReader<u16, u16>;
///Field `BOOT_ADD0` writer - Boot address 0
pub type BOOT_ADD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_PRGR_SPEC, u16, u16, 16, O>;
///Field `BOOT_ADD1` reader - Boot address 1
pub type BOOT_ADD1_R = crate::FieldReader<u16, u16>;
///Field `BOOT_ADD1` writer - Boot address 1
pub type BOOT_ADD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_PRGR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Boot address 0
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Boot address 1
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Boot address 0
    #[inline(always)]
    #[must_use]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W<0> {
        BOOT_ADD0_W::new(self)
    }
    ///Bits 16:31 - Boot address 1
    #[inline(always)]
    #[must_use]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W<16> {
        BOOT_ADD1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH register with boot address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [boot_prgr](index.html) module
pub struct BOOT_PRGR_SPEC;
impl crate::RegisterSpec for BOOT_PRGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [boot_prgr::R](R) reader structure
impl crate::Readable for BOOT_PRGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [boot_prgr::W](W) writer structure
impl crate::Writable for BOOT_PRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BOOT_PRGR to value 0
impl crate::Resettable for BOOT_PRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
