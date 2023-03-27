///Register `BDMUPR` reader
pub struct R(crate::R<BDMUPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDMUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDMUPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDMUPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDMUPR` writer
pub struct W(crate::W<BDMUPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDMUPR_SPEC>;
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
impl From<crate::W<BDMUPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDMUPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCR` reader - MCR
pub type MCR_R = crate::BitReader<MCR_A>;
///MCR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCR_A {
    ///0: Register not updated by burst DMA access
    NotUpdated = 0,
    ///1: Register updated by burst DMA access
    Updated = 1,
}
impl From<MCR_A> for bool {
    #[inline(always)]
    fn from(variant: MCR_A) -> Self {
        variant as u8 != 0
    }
}
impl MCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCR_A {
        match self.bits {
            false => MCR_A::NotUpdated,
            true => MCR_A::Updated,
        }
    }
    ///Checks if the value of the field is `NotUpdated`
    #[inline(always)]
    pub fn is_not_updated(&self) -> bool {
        *self == MCR_A::NotUpdated
    }
    ///Checks if the value of the field is `Updated`
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == MCR_A::Updated
    }
}
///Field `MCR` writer - MCR
pub type MCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDMUPR_SPEC, MCR_A, O>;
impl<'a, const O: u8> MCR_W<'a, O> {
    ///Register not updated by burst DMA access
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCR_A::NotUpdated)
    }
    ///Register updated by burst DMA access
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCR_A::Updated)
    }
}
///Field `MICR` reader - MICR
pub use MCR_R as MICR_R;
///Field `MDIER` reader - MDIER
pub use MCR_R as MDIER_R;
///Field `MCNT` reader - MCNT
pub use MCR_R as MCNT_R;
///Field `MPER` reader - MPER
pub use MCR_R as MPER_R;
///Field `MREP` reader - MREP
pub use MCR_R as MREP_R;
///Field `MCMP1` reader - MCMP1
pub use MCR_R as MCMP1_R;
///Field `MCMP2` reader - MCMP2
pub use MCR_R as MCMP2_R;
///Field `MCMP3` reader - MCMP3
pub use MCR_R as MCMP3_R;
///Field `MCMP4` reader - MCMP4
pub use MCR_R as MCMP4_R;
///Field `MICR` writer - MICR
pub use MCR_W as MICR_W;
///Field `MDIER` writer - MDIER
pub use MCR_W as MDIER_W;
///Field `MCNT` writer - MCNT
pub use MCR_W as MCNT_W;
///Field `MPER` writer - MPER
pub use MCR_W as MPER_W;
///Field `MREP` writer - MREP
pub use MCR_W as MREP_W;
///Field `MCMP1` writer - MCMP1
pub use MCR_W as MCMP1_W;
///Field `MCMP2` writer - MCMP2
pub use MCR_W as MCMP2_W;
///Field `MCMP3` writer - MCMP3
pub use MCR_W as MCMP3_W;
///Field `MCMP4` writer - MCMP4
pub use MCR_W as MCMP4_W;
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
///For information about available fields see [bdmupr](index.html) module
pub struct BDMUPR_SPEC;
impl crate::RegisterSpec for BDMUPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdmupr::R](R) reader structure
impl crate::Readable for BDMUPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdmupr::W](W) writer structure
impl crate::Writable for BDMUPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDMUPR to value 0
impl crate::Resettable for BDMUPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
