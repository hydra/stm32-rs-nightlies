///Register `BSLOTR` reader
pub struct R(crate::R<BSLOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSLOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSLOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSLOTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BSLOTR` writer
pub struct W(crate::W<BSLOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSLOTR_SPEC>;
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
impl From<crate::W<BSLOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSLOTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FBOFF` reader - First bit offset
pub type FBOFF_R = crate::FieldReader<u8, u8>;
///Field `FBOFF` writer - First bit offset
pub type FBOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSLOTR_SPEC, u8, u8, 5, O>;
///Field `SLOTSZ` reader - Slot size
pub type SLOTSZ_R = crate::FieldReader<u8, u8>;
///Field `SLOTSZ` writer - Slot size
pub type SLOTSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSLOTR_SPEC, u8, u8, 2, O>;
///Field `NBSLOT` reader - Number of slots in an audio frame
pub type NBSLOT_R = crate::FieldReader<u8, u8>;
///Field `NBSLOT` writer - Number of slots in an audio frame
pub type NBSLOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSLOTR_SPEC, u8, u8, 4, O>;
///Field `SLOTEN` reader - Slot enable
pub type SLOTEN_R = crate::FieldReader<u16, u16>;
///Field `SLOTEN` writer - Slot enable
pub type SLOTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSLOTR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - Slot enable
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    #[must_use]
    pub fn fboff(&mut self) -> FBOFF_W<0> {
        FBOFF_W::new(self)
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    #[must_use]
    pub fn slotsz(&mut self) -> SLOTSZ_W<6> {
        SLOTSZ_W::new(self)
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    #[must_use]
    pub fn nbslot(&mut self) -> NBSLOT_W<8> {
        NBSLOT_W::new(self)
    }
    ///Bits 16:31 - Slot enable
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
///B Slot register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bslotr](index.html) module
pub struct BSLOTR_SPEC;
impl crate::RegisterSpec for BSLOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bslotr::R](R) reader structure
impl crate::Readable for BSLOTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bslotr::W](W) writer structure
impl crate::Writable for BSLOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BSLOTR to value 0
impl crate::Resettable for BSLOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
