///Register `GICD_ITARGETSR20` reader
pub struct R(crate::R<GICD_ITARGETSR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR20_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICD_ITARGETSR20` writer
pub struct W(crate::W<GICD_ITARGETSR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR20_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR20_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CPU_TARGETS0` reader - CPU_TARGETS0
pub type CPU_TARGETS0_R = crate::FieldReader<u8, u8>;
///Field `CPU_TARGETS0` writer - CPU_TARGETS0
pub type CPU_TARGETS0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR20_SPEC, u8, u8, 2, O>;
///Field `CPU_TARGETS1` reader - CPU_TARGETS1
pub type CPU_TARGETS1_R = crate::FieldReader<u8, u8>;
///Field `CPU_TARGETS1` writer - CPU_TARGETS1
pub type CPU_TARGETS1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR20_SPEC, u8, u8, 2, O>;
///Field `CPU_TARGETS2` reader - CPU_TARGETS2
pub type CPU_TARGETS2_R = crate::FieldReader<u8, u8>;
///Field `CPU_TARGETS2` writer - CPU_TARGETS2
pub type CPU_TARGETS2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR20_SPEC, u8, u8, 2, O>;
///Field `CPU_TARGETS3` reader - CPU_TARGETS3
pub type CPU_TARGETS3_R = crate::FieldReader<u8, u8>;
///Field `CPU_TARGETS3` writer - CPU_TARGETS3
pub type CPU_TARGETS3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR20_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - CPU_TARGETS0
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - CPU_TARGETS1
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - CPU_TARGETS2
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:25 - CPU_TARGETS3
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - CPU_TARGETS0
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets0(&mut self) -> CPU_TARGETS0_W<0> {
        CPU_TARGETS0_W::new(self)
    }
    ///Bits 8:9 - CPU_TARGETS1
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets1(&mut self) -> CPU_TARGETS1_W<8> {
        CPU_TARGETS1_W::new(self)
    }
    ///Bits 16:17 - CPU_TARGETS2
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets2(&mut self) -> CPU_TARGETS2_W<16> {
        CPU_TARGETS2_W::new(self)
    }
    ///Bits 24:25 - CPU_TARGETS3
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets3(&mut self) -> CPU_TARGETS3_W<24> {
        CPU_TARGETS3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GICD interrupt processor target register 20
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_itargetsr20](index.html) module
pub struct GICD_ITARGETSR20_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR20_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_itargetsr20::R](R) reader structure
impl crate::Readable for GICD_ITARGETSR20_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicd_itargetsr20::W](W) writer structure
impl crate::Writable for GICD_ITARGETSR20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICD_ITARGETSR20 to value 0
impl crate::Resettable for GICD_ITARGETSR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
