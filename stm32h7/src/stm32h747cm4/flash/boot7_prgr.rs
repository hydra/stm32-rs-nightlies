///Register `BOOT7_PRGR` reader
pub struct R(crate::R<BOOT7_PRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT7_PRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT7_PRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT7_PRGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BOOT7_PRGR` writer
pub struct W(crate::W<BOOT7_PRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT7_PRGR_SPEC>;
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
impl From<crate::W<BOOT7_PRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT7_PRGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BOOT_CM7_ADD0` reader - Arm Cortex-M7 boot address 0 configuration
pub type BOOT_CM7_ADD0_R = crate::FieldReader<u16, u16>;
///Field `BOOT_CM7_ADD0` writer - Arm Cortex-M7 boot address 0 configuration
pub type BOOT_CM7_ADD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT7_PRGR_SPEC, u16, u16, 16, O>;
///Field `BOOT_CM7_ADD1` reader - Arm Cortex-M7 boot address 1 configuration
pub type BOOT_CM7_ADD1_R = crate::FieldReader<u16, u16>;
///Field `BOOT_CM7_ADD1` writer - Arm Cortex-M7 boot address 1 configuration
pub type BOOT_CM7_ADD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT7_PRGR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Arm Cortex-M7 boot address 0 configuration
    #[inline(always)]
    pub fn boot_cm7_add0(&self) -> BOOT_CM7_ADD0_R {
        BOOT_CM7_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Arm Cortex-M7 boot address 1 configuration
    #[inline(always)]
    pub fn boot_cm7_add1(&self) -> BOOT_CM7_ADD1_R {
        BOOT_CM7_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Arm Cortex-M7 boot address 0 configuration
    #[inline(always)]
    #[must_use]
    pub fn boot_cm7_add0(&mut self) -> BOOT_CM7_ADD0_W<0> {
        BOOT_CM7_ADD0_W::new(self)
    }
    ///Bits 16:31 - Arm Cortex-M7 boot address 1 configuration
    #[inline(always)]
    #[must_use]
    pub fn boot_cm7_add1(&mut self) -> BOOT_CM7_ADD1_W<16> {
        BOOT_CM7_ADD1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH register boot address for Arm Cortex-M7 core
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [boot7_prgr](index.html) module
pub struct BOOT7_PRGR_SPEC;
impl crate::RegisterSpec for BOOT7_PRGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [boot7_prgr::R](R) reader structure
impl crate::Readable for BOOT7_PRGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [boot7_prgr::W](W) writer structure
impl crate::Writable for BOOT7_PRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BOOT7_PRGR to value 0
impl crate::Resettable for BOOT7_PRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
