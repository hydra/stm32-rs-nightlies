///Register `VDSSA` reader
pub struct R(crate::R<VDSSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDSSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDSSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDSSA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VDSSA` writer
pub struct W(crate::W<VDSSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDSSA_SPEC>;
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
impl From<crate::W<VDSSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDSSA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADD` reader - Volatile data segment start address
pub type ADD_R = crate::FieldReader<u16, u16>;
///Field `ADD` writer - Volatile data segment start address
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VDSSA_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 6:17 - Volatile data segment start address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 6:17 - Volatile data segment start address
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<6> {
        ADD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Volatile data segment start address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vdssa](index.html) module
pub struct VDSSA_SPEC;
impl crate::RegisterSpec for VDSSA_SPEC {
    type Ux = u32;
}
///`read()` method returns [vdssa::R](R) reader structure
impl crate::Readable for VDSSA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vdssa::W](W) writer structure
impl crate::Writable for VDSSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VDSSA to value 0
impl crate::Resettable for VDSSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
