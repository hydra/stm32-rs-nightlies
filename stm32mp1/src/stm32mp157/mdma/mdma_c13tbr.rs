///Register `MDMA_C13TBR` reader
pub struct R(crate::R<MDMA_C13TBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C13TBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C13TBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C13TBR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDMA_C13TBR` writer
pub struct W(crate::W<MDMA_C13TBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C13TBR_SPEC>;
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
impl From<crate::W<MDMA_C13TBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C13TBR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSEL` reader - TSEL
pub type TSEL_R = crate::FieldReader<u8, u8>;
///Field `TSEL` writer - TSEL
pub type TSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C13TBR_SPEC, u8, u8, 6, O>;
///Field `SBUS` reader - SBUS
pub type SBUS_R = crate::BitReader<bool>;
///Field `SBUS` writer - SBUS
pub type SBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C13TBR_SPEC, bool, O>;
///Field `DBUS` reader - DBUS
pub type DBUS_R = crate::BitReader<bool>;
///Field `DBUS` writer - DBUS
pub type DBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C13TBR_SPEC, bool, O>;
impl R {
    ///Bits 0:5 - TSEL
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 16 - SBUS
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DBUS
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - TSEL
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TSEL_W<0> {
        TSEL_W::new(self)
    }
    ///Bit 16 - SBUS
    #[inline(always)]
    #[must_use]
    pub fn sbus(&mut self) -> SBUS_W<16> {
        SBUS_W::new(self)
    }
    ///Bit 17 - DBUS
    #[inline(always)]
    #[must_use]
    pub fn dbus(&mut self) -> DBUS_W<17> {
        DBUS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
///+ 0x18).
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdma_c13tbr](index.html) module
pub struct MDMA_C13TBR_SPEC;
impl crate::RegisterSpec for MDMA_C13TBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdma_c13tbr::R](R) reader structure
impl crate::Readable for MDMA_C13TBR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdma_c13tbr::W](W) writer structure
impl crate::Writable for MDMA_C13TBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDMA_C13TBR to value 0
impl crate::Resettable for MDMA_C13TBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
