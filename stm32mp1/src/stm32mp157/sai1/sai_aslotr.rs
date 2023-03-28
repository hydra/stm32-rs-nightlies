///Register `SAI_ASLOTR` reader
pub struct R(crate::R<SAI_ASLOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ASLOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ASLOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ASLOTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SAI_ASLOTR` writer
pub struct W(crate::W<SAI_ASLOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ASLOTR_SPEC>;
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
impl From<crate::W<SAI_ASLOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ASLOTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FBOFF` reader - FBOFF
pub type FBOFF_R = crate::FieldReader<u8, u8>;
///Field `FBOFF` writer - FBOFF
pub type FBOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ASLOTR_SPEC, u8, u8, 5, O>;
///Field `SLOTSZ` reader - SLOTSZ
pub type SLOTSZ_R = crate::FieldReader<u8, u8>;
///Field `SLOTSZ` writer - SLOTSZ
pub type SLOTSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ASLOTR_SPEC, u8, u8, 2, O>;
///Field `NBSLOT` reader - NBSLOT
pub type NBSLOT_R = crate::FieldReader<u8, u8>;
///Field `NBSLOT` writer - NBSLOT
pub type NBSLOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ASLOTR_SPEC, u8, u8, 4, O>;
///Field `SLOTEN` reader - SLOTEN
pub type SLOTEN_R = crate::FieldReader<u16, u16>;
///Field `SLOTEN` writer - SLOTEN
pub type SLOTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ASLOTR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:4 - FBOFF
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - SLOTSZ
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - NBSLOT
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - SLOTEN
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:4 - FBOFF
    #[inline(always)]
    #[must_use]
    pub fn fboff(&mut self) -> FBOFF_W<0> {
        FBOFF_W::new(self)
    }
    ///Bits 6:7 - SLOTSZ
    #[inline(always)]
    #[must_use]
    pub fn slotsz(&mut self) -> SLOTSZ_W<6> {
        SLOTSZ_W::new(self)
    }
    ///Bits 8:11 - NBSLOT
    #[inline(always)]
    #[must_use]
    pub fn nbslot(&mut self) -> NBSLOT_W<8> {
        NBSLOT_W::new(self)
    }
    ///Bits 16:31 - SLOTEN
    #[inline(always)]
    #[must_use]
    pub fn sloten(&mut self) -> SLOTEN_W<16> {
        SLOTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register has no meaning in and SPDIF audio protocol
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sai_aslotr](index.html) module
pub struct SAI_ASLOTR_SPEC;
impl crate::RegisterSpec for SAI_ASLOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sai_aslotr::R](R) reader structure
impl crate::Readable for SAI_ASLOTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sai_aslotr::W](W) writer structure
impl crate::Writable for SAI_ASLOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SAI_ASLOTR to value 0
impl crate::Resettable for SAI_ASLOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
