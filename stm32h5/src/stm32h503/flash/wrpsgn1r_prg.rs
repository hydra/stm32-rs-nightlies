///Register `WRPSGN1R_PRG` reader
pub struct R(crate::R<WRPSGN1R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPSGN1R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPSGN1R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPSGN1R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WRPSGN1R_PRG` writer
pub struct W(crate::W<WRPSGN1R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPSGN1R_PRG_SPEC>;
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
impl From<crate::W<WRPSGN1R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPSGN1R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WRPSG1` reader - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
pub type WRPSG1_R = crate::FieldReader<u8, u8>;
///Field `WRPSG1` writer - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
pub type WRPSG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPSGN1R_PRG_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
    #[inline(always)]
    #[must_use]
    pub fn wrpsg1(&mut self) -> WRPSG1_W<0> {
        WRPSG1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH write sector protection for Bank1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrpsgn1r_prg](index.html) module
pub struct WRPSGN1R_PRG_SPEC;
impl crate::RegisterSpec for WRPSGN1R_PRG_SPEC {
    type Ux = u32;
}
///`read()` method returns [wrpsgn1r_prg::R](R) reader structure
impl crate::Readable for WRPSGN1R_PRG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wrpsgn1r_prg::W](W) writer structure
impl crate::Writable for WRPSGN1R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WRPSGN1R_PRG to value 0
impl crate::Resettable for WRPSGN1R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
