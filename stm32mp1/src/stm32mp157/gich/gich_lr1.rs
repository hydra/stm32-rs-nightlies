///Register `GICH_LR1` reader
pub struct R(crate::R<GICH_LR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_LR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_LR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_LR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICH_LR1` writer
pub struct W(crate::W<GICH_LR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICH_LR1_SPEC>;
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
impl From<crate::W<GICH_LR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICH_LR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VIRTUALID` reader - VIRTUALID
pub type VIRTUALID_R = crate::FieldReader<u16, u16>;
///Field `VIRTUALID` writer - VIRTUALID
pub type VIRTUALID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_LR1_SPEC, u16, u16, 10, O>;
///Field `PHYSICALID` reader - PHYSICALID
pub type PHYSICALID_R = crate::FieldReader<u16, u16>;
///Field `PHYSICALID` writer - PHYSICALID
pub type PHYSICALID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICH_LR1_SPEC, u16, u16, 10, O>;
///Field `PRIORITY` reader - PRIORITY
pub type PRIORITY_R = crate::FieldReader<u8, u8>;
///Field `PRIORITY` writer - PRIORITY
pub type PRIORITY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_LR1_SPEC, u8, u8, 5, O>;
///Field `STATE` reader - STATE
pub type STATE_R = crate::FieldReader<u8, u8>;
///Field `STATE` writer - STATE
pub type STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_LR1_SPEC, u8, u8, 2, O>;
///Field `GRP1` reader - GRP1
pub type GRP1_R = crate::BitReader<bool>;
///Field `GRP1` writer - GRP1
pub type GRP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_LR1_SPEC, bool, O>;
///Field `HW` reader - HW
pub type HW_R = crate::BitReader<bool>;
///Field `HW` writer - HW
pub type HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_LR1_SPEC, bool, O>;
impl R {
    ///Bits 0:9 - VIRTUALID
    #[inline(always)]
    pub fn virtualid(&self) -> VIRTUALID_R {
        VIRTUALID_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - PHYSICALID
    #[inline(always)]
    pub fn physicalid(&self) -> PHYSICALID_R {
        PHYSICALID_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 23:27 - PRIORITY
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    ///Bits 28:29 - STATE
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - GRP1
    #[inline(always)]
    pub fn grp1(&self) -> GRP1_R {
        GRP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - HW
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:9 - VIRTUALID
    #[inline(always)]
    #[must_use]
    pub fn virtualid(&mut self) -> VIRTUALID_W<0> {
        VIRTUALID_W::new(self)
    }
    ///Bits 10:19 - PHYSICALID
    #[inline(always)]
    #[must_use]
    pub fn physicalid(&mut self) -> PHYSICALID_W<10> {
        PHYSICALID_W::new(self)
    }
    ///Bits 23:27 - PRIORITY
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<23> {
        PRIORITY_W::new(self)
    }
    ///Bits 28:29 - STATE
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<28> {
        STATE_W::new(self)
    }
    ///Bit 30 - GRP1
    #[inline(always)]
    #[must_use]
    pub fn grp1(&mut self) -> GRP1_W<30> {
        GRP1_W::new(self)
    }
    ///Bit 31 - HW
    #[inline(always)]
    #[must_use]
    pub fn hw(&mut self) -> HW_W<31> {
        HW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GICH list register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gich_lr1](index.html) module
pub struct GICH_LR1_SPEC;
impl crate::RegisterSpec for GICH_LR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [gich_lr1::R](R) reader structure
impl crate::Readable for GICH_LR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gich_lr1::W](W) writer structure
impl crate::Writable for GICH_LR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICH_LR1 to value 0
impl crate::Resettable for GICH_LR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
