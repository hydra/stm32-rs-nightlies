///Register `PRAR_CUR` reader
pub struct R(crate::R<PRAR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRAR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRAR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRAR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRAR_CUR` writer
pub struct W(crate::W<PRAR_CUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRAR_CUR_SPEC>;
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
impl From<crate::W<PRAR_CUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRAR_CUR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PROT_AREA_START` reader - Bank 1 PCROP area start status bits
pub type PROT_AREA_START_R = crate::FieldReader<u16, u16>;
///Field `PROT_AREA_START` writer - Bank 1 PCROP area start status bits
pub type PROT_AREA_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRAR_CUR_SPEC, u16, u16, 12, O>;
///Field `PROT_AREA_END` reader - Bank 1 PCROP area end status bits
pub type PROT_AREA_END_R = crate::FieldReader<u16, u16>;
///Field `PROT_AREA_END` writer - Bank 1 PCROP area end status bits
pub type PROT_AREA_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRAR_CUR_SPEC, u16, u16, 12, O>;
///Field `DMEP` reader - Bank 1 PCROP protected erase enable option status bit
pub type DMEP_R = crate::BitReader<bool>;
///Field `DMEP` writer - Bank 1 PCROP protected erase enable option status bit
pub type DMEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRAR_CUR_SPEC, bool, O>;
impl R {
    ///Bits 0:11 - Bank 1 PCROP area start status bits
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Bank 1 PCROP area end status bits
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Bank 1 PCROP protected erase enable option status bit
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Bank 1 PCROP area start status bits
    #[inline(always)]
    #[must_use]
    pub fn prot_area_start(&mut self) -> PROT_AREA_START_W<0> {
        PROT_AREA_START_W::new(self)
    }
    ///Bits 16:27 - Bank 1 PCROP area end status bits
    #[inline(always)]
    #[must_use]
    pub fn prot_area_end(&mut self) -> PROT_AREA_END_W<16> {
        PROT_AREA_END_W::new(self)
    }
    ///Bit 31 - Bank 1 PCROP protected erase enable option status bit
    #[inline(always)]
    #[must_use]
    pub fn dmep(&mut self) -> DMEP_W<31> {
        DMEP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH protection address for bank 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [prar_cur](index.html) module
pub struct PRAR_CUR_SPEC;
impl crate::RegisterSpec for PRAR_CUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [prar_cur::R](R) reader structure
impl crate::Readable for PRAR_CUR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [prar_cur::W](W) writer structure
impl crate::Writable for PRAR_CUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRAR_CUR to value 0
impl crate::Resettable for PRAR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
