///Register `GICH_VMCR` reader
pub struct R(crate::R<GICH_VMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_VMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_VMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_VMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICH_VMCR` writer
pub struct W(crate::W<GICH_VMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICH_VMCR_SPEC>;
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
impl From<crate::W<GICH_VMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICH_VMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VMGRP0EN` reader - VMGRP0EN
pub type VMGRP0EN_R = crate::BitReader<bool>;
///Field `VMGRP0EN` writer - VMGRP0EN
pub type VMGRP0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_VMCR_SPEC, bool, O>;
///Field `VMGRP1EN` reader - VMGRP1EN
pub type VMGRP1EN_R = crate::BitReader<bool>;
///Field `VMGRP1EN` writer - VMGRP1EN
pub type VMGRP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_VMCR_SPEC, bool, O>;
///Field `VMACKCTL` reader - VMACKCTL
pub type VMACKCTL_R = crate::BitReader<bool>;
///Field `VMACKCTL` writer - VMACKCTL
pub type VMACKCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_VMCR_SPEC, bool, O>;
///Field `VMFIQEN` reader - VMFIQEN
pub type VMFIQEN_R = crate::BitReader<bool>;
///Field `VMFIQEN` writer - VMFIQEN
pub type VMFIQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_VMCR_SPEC, bool, O>;
///Field `VMCBPR` reader - VMCBPR
pub type VMCBPR_R = crate::BitReader<bool>;
///Field `VMCBPR` writer - VMCBPR
pub type VMCBPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_VMCR_SPEC, bool, O>;
///Field `VEM` reader - VEM
pub type VEM_R = crate::BitReader<bool>;
///Field `VEM` writer - VEM
pub type VEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_VMCR_SPEC, bool, O>;
///Field `VMABP` reader - VMABP
pub type VMABP_R = crate::FieldReader<u8, u8>;
///Field `VMABP` writer - VMABP
pub type VMABP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_VMCR_SPEC, u8, u8, 3, O>;
///Field `VMBP` reader - VMBP
pub type VMBP_R = crate::FieldReader<u8, u8>;
///Field `VMBP` writer - VMBP
pub type VMBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_VMCR_SPEC, u8, u8, 3, O>;
///Field `VMPRIMASK` reader - VMPRIMASK
pub type VMPRIMASK_R = crate::FieldReader<u8, u8>;
///Field `VMPRIMASK` writer - VMPRIMASK
pub type VMPRIMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_VMCR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bit 0 - VMGRP0EN
    #[inline(always)]
    pub fn vmgrp0en(&self) -> VMGRP0EN_R {
        VMGRP0EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VMGRP1EN
    #[inline(always)]
    pub fn vmgrp1en(&self) -> VMGRP1EN_R {
        VMGRP1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VMACKCTL
    #[inline(always)]
    pub fn vmackctl(&self) -> VMACKCTL_R {
        VMACKCTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VMFIQEN
    #[inline(always)]
    pub fn vmfiqen(&self) -> VMFIQEN_R {
        VMFIQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VMCBPR
    #[inline(always)]
    pub fn vmcbpr(&self) -> VMCBPR_R {
        VMCBPR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - VEM
    #[inline(always)]
    pub fn vem(&self) -> VEM_R {
        VEM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 18:20 - VMABP
    #[inline(always)]
    pub fn vmabp(&self) -> VMABP_R {
        VMABP_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - VMBP
    #[inline(always)]
    pub fn vmbp(&self) -> VMBP_R {
        VMBP_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 27:31 - VMPRIMASK
    #[inline(always)]
    pub fn vmprimask(&self) -> VMPRIMASK_R {
        VMPRIMASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - VMGRP0EN
    #[inline(always)]
    #[must_use]
    pub fn vmgrp0en(&mut self) -> VMGRP0EN_W<0> {
        VMGRP0EN_W::new(self)
    }
    ///Bit 1 - VMGRP1EN
    #[inline(always)]
    #[must_use]
    pub fn vmgrp1en(&mut self) -> VMGRP1EN_W<1> {
        VMGRP1EN_W::new(self)
    }
    ///Bit 2 - VMACKCTL
    #[inline(always)]
    #[must_use]
    pub fn vmackctl(&mut self) -> VMACKCTL_W<2> {
        VMACKCTL_W::new(self)
    }
    ///Bit 3 - VMFIQEN
    #[inline(always)]
    #[must_use]
    pub fn vmfiqen(&mut self) -> VMFIQEN_W<3> {
        VMFIQEN_W::new(self)
    }
    ///Bit 4 - VMCBPR
    #[inline(always)]
    #[must_use]
    pub fn vmcbpr(&mut self) -> VMCBPR_W<4> {
        VMCBPR_W::new(self)
    }
    ///Bit 9 - VEM
    #[inline(always)]
    #[must_use]
    pub fn vem(&mut self) -> VEM_W<9> {
        VEM_W::new(self)
    }
    ///Bits 18:20 - VMABP
    #[inline(always)]
    #[must_use]
    pub fn vmabp(&mut self) -> VMABP_W<18> {
        VMABP_W::new(self)
    }
    ///Bits 21:23 - VMBP
    #[inline(always)]
    #[must_use]
    pub fn vmbp(&mut self) -> VMBP_W<21> {
        VMBP_W::new(self)
    }
    ///Bits 27:31 - VMPRIMASK
    #[inline(always)]
    #[must_use]
    pub fn vmprimask(&mut self) -> VMPRIMASK_W<27> {
        VMPRIMASK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GICH virtual machine control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gich_vmcr](index.html) module
pub struct GICH_VMCR_SPEC;
impl crate::RegisterSpec for GICH_VMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gich_vmcr::R](R) reader structure
impl crate::Readable for GICH_VMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gich_vmcr::W](W) writer structure
impl crate::Writable for GICH_VMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICH_VMCR to value 0x004d_0000
impl crate::Resettable for GICH_VMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x004d_0000;
}
