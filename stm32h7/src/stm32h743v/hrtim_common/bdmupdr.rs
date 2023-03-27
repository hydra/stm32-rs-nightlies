///Register `BDMUPDR` reader
pub struct R(crate::R<BDMUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDMUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDMUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDMUPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDMUPDR` writer
pub struct W(crate::W<BDMUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDMUPDR_SPEC>;
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
impl From<crate::W<BDMUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDMUPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCR` reader - MCR
pub type MCR_R = crate::BitReader<bool>;
///Field `MCR` writer - MCR
pub type MCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MICR` reader - MICR
pub type MICR_R = crate::BitReader<bool>;
///Field `MICR` writer - MICR
pub type MICR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MDIER` reader - MDIER
pub type MDIER_R = crate::BitReader<bool>;
///Field `MDIER` writer - MDIER
pub type MDIER_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MCNT` reader - MCNT
pub type MCNT_R = crate::BitReader<bool>;
///Field `MCNT` writer - MCNT
pub type MCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MPER` reader - MPER
pub type MPER_R = crate::BitReader<bool>;
///Field `MPER` writer - MPER
pub type MPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MREP` reader - MREP
pub type MREP_R = crate::BitReader<bool>;
///Field `MREP` writer - MREP
pub type MREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MCMP1` reader - MCMP1
pub type MCMP1_R = crate::BitReader<bool>;
///Field `MCMP1` writer - MCMP1
pub type MCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MCMP2` reader - MCMP2
pub type MCMP2_R = crate::BitReader<bool>;
///Field `MCMP2` writer - MCMP2
pub type MCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MCMP3` reader - MCMP3
pub type MCMP3_R = crate::BitReader<bool>;
///Field `MCMP3` writer - MCMP3
pub type MCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
///Field `MCMP4` reader - MCMP4
pub type MCMP4_R = crate::BitReader<bool>;
///Field `MCMP4` writer - MCMP4
pub type MCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPDR_SPEC, bool, O>;
impl R {
    ///Bit 0 - MCR
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MICR
    #[inline(always)]
    pub fn micr(&self) -> MICR_R {
        MICR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MDIER
    #[inline(always)]
    pub fn mdier(&self) -> MDIER_R {
        MDIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCNT
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MPER
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MREP
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MCMP1
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MCMP2
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MCMP3
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MCMP4
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MCR
    #[inline(always)]
    #[must_use]
    pub fn mcr(&mut self) -> MCR_W<0> {
        MCR_W::new(self)
    }
    ///Bit 1 - MICR
    #[inline(always)]
    #[must_use]
    pub fn micr(&mut self) -> MICR_W<1> {
        MICR_W::new(self)
    }
    ///Bit 2 - MDIER
    #[inline(always)]
    #[must_use]
    pub fn mdier(&mut self) -> MDIER_W<2> {
        MDIER_W::new(self)
    }
    ///Bit 3 - MCNT
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<3> {
        MCNT_W::new(self)
    }
    ///Bit 4 - MPER
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MPER_W<4> {
        MPER_W::new(self)
    }
    ///Bit 5 - MREP
    #[inline(always)]
    #[must_use]
    pub fn mrep(&mut self) -> MREP_W<5> {
        MREP_W::new(self)
    }
    ///Bit 6 - MCMP1
    #[inline(always)]
    #[must_use]
    pub fn mcmp1(&mut self) -> MCMP1_W<6> {
        MCMP1_W::new(self)
    }
    ///Bit 7 - MCMP2
    #[inline(always)]
    #[must_use]
    pub fn mcmp2(&mut self) -> MCMP2_W<7> {
        MCMP2_W::new(self)
    }
    ///Bit 8 - MCMP3
    #[inline(always)]
    #[must_use]
    pub fn mcmp3(&mut self) -> MCMP3_W<8> {
        MCMP3_W::new(self)
    }
    ///Bit 9 - MCMP4
    #[inline(always)]
    #[must_use]
    pub fn mcmp4(&mut self) -> MCMP4_W<9> {
        MCMP4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///BDMUPDR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdmupdr](index.html) module
pub struct BDMUPDR_SPEC;
impl crate::RegisterSpec for BDMUPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdmupdr::R](R) reader structure
impl crate::Readable for BDMUPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdmupdr::W](W) writer structure
impl crate::Writable for BDMUPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDMUPDR to value 0
impl crate::Resettable for BDMUPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
