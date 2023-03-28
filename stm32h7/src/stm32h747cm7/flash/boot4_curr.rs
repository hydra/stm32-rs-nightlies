///Register `BOOT4_CURR` reader
pub struct R(crate::R<BOOT4_CURR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT4_CURR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT4_CURR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT4_CURR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BOOT4_CURR` writer
pub struct W(crate::W<BOOT4_CURR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT4_CURR_SPEC>;
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
impl From<crate::W<BOOT4_CURR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT4_CURR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BOOT_CM4_ADD0` reader - Arm Cortex-M4 boot address 0
pub type BOOT_CM4_ADD0_R = crate::FieldReader<u16, u16>;
///Field `BOOT_CM4_ADD0` writer - Arm Cortex-M4 boot address 0
pub type BOOT_CM4_ADD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT4_CURR_SPEC, u16, u16, 16, O>;
///Field `BOOT_CM4_ADD1` reader - Arm Cortex-M4 boot address 1
pub type BOOT_CM4_ADD1_R = crate::FieldReader<u16, u16>;
///Field `BOOT_CM4_ADD1` writer - Arm Cortex-M4 boot address 1
pub type BOOT_CM4_ADD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT4_CURR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Arm Cortex-M4 boot address 0
    #[inline(always)]
    pub fn boot_cm4_add0(&self) -> BOOT_CM4_ADD0_R {
        BOOT_CM4_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Arm Cortex-M4 boot address 1
    #[inline(always)]
    pub fn boot_cm4_add1(&self) -> BOOT_CM4_ADD1_R {
        BOOT_CM4_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Arm Cortex-M4 boot address 0
    #[inline(always)]
    #[must_use]
    pub fn boot_cm4_add0(&mut self) -> BOOT_CM4_ADD0_W<0> {
        BOOT_CM4_ADD0_W::new(self)
    }
    ///Bits 16:31 - Arm Cortex-M4 boot address 1
    #[inline(always)]
    #[must_use]
    pub fn boot_cm4_add1(&mut self) -> BOOT_CM4_ADD1_W<16> {
        BOOT_CM4_ADD1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH register boot address for Arm Cortex-M4 core
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [boot4_curr](index.html) module
pub struct BOOT4_CURR_SPEC;
impl crate::RegisterSpec for BOOT4_CURR_SPEC {
    type Ux = u32;
}
///`read()` method returns [boot4_curr::R](R) reader structure
impl crate::Readable for BOOT4_CURR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [boot4_curr::W](W) writer structure
impl crate::Writable for BOOT4_CURR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BOOT4_CURR to value 0
impl crate::Resettable for BOOT4_CURR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
