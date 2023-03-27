///Register `FLASH_NSCR` reader
pub struct R(crate::R<FLASH_NSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_NSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_NSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_NSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_NSCR` writer
pub struct W(crate::W<FLASH_NSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_NSCR_SPEC>;
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
impl From<crate::W<FLASH_NSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_NSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PG` reader - Non-secure programming
pub type PG_R = crate::BitReader<bool>;
///Field `PG` writer - Non-secure programming
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `PER` reader - Non-secure page erase
pub type PER_R = crate::BitReader<bool>;
///Field `PER` writer - Non-secure page erase
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `MER1` reader - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
pub type MER1_R = crate::BitReader<bool>;
///Field `MER1` writer - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
pub type MER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `PNB` reader - Non-secure page number selection These bits select the page to erase. ...
pub type PNB_R = crate::FieldReader<u8, u8>;
///Field `PNB` writer - Non-secure page number selection These bits select the page to erase. ...
pub type PNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_NSCR_SPEC, u8, u8, 7, O>;
///Field `BKER` reader - Non-secure bank selection for page erase
pub type BKER_R = crate::BitReader<bool>;
///Field `BKER` writer - Non-secure bank selection for page erase
pub type BKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `BWR` reader - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_R = crate::BitReader<bool>;
///Field `BWR` writer - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `MER2` reader - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
pub type MER2_R = crate::BitReader<bool>;
///Field `MER2` writer - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
pub type MER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `STRT` reader - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type STRT_R = crate::BitReader<bool>;
///Field `STRT` writer - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `OPTSTRT` reader - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type OPTSTRT_R = crate::BitReader<bool>;
///Field `OPTSTRT` writer - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type OPTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `EOPIE` reader - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
pub type EOPIE_R = crate::BitReader<bool>;
///Field `EOPIE` writer - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `ERRIE` reader - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
pub type ERRIE_R = crate::BitReader<bool>;
///Field `ERRIE` writer - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `OBL_LAUNCH` reader - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
pub type OBL_LAUNCH_R = crate::BitReader<bool>;
///Field `OBL_LAUNCH` writer - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
pub type OBL_LAUNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `OPTLOCK` reader - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_R = crate::BitReader<bool>;
///Field `OPTLOCK` writer - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
///Field `LOCK` reader - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_R = crate::BitReader<bool>;
///Field `LOCK` writer - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Non-secure programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Non-secure page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Non-secure page number selection These bits select the page to erase. ...
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 11 - Non-secure bank selection for page erase
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Non-secure programming
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<0> {
        PG_W::new(self)
    }
    ///Bit 1 - Non-secure page erase
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<1> {
        PER_W::new(self)
    }
    ///Bit 2 - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> MER1_W<2> {
        MER1_W::new(self)
    }
    ///Bits 3:9 - Non-secure page number selection These bits select the page to erase. ...
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<3> {
        PNB_W::new(self)
    }
    ///Bit 11 - Non-secure bank selection for page erase
    #[inline(always)]
    #[must_use]
    pub fn bker(&mut self) -> BKER_W<11> {
        BKER_W::new(self)
    }
    ///Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    #[must_use]
    pub fn bwr(&mut self) -> BWR_W<14> {
        BWR_W::new(self)
    }
    ///Bit 15 - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    #[must_use]
    pub fn mer2(&mut self) -> MER2_W<15> {
        MER2_W::new(self)
    }
    ///Bit 16 - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<16> {
        STRT_W::new(self)
    }
    ///Bit 17 - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<17> {
        OPTSTRT_W::new(self)
    }
    ///Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<24> {
        EOPIE_W::new(self)
    }
    ///Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<25> {
        ERRIE_W::new(self)
    }
    ///Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<27> {
        OBL_LAUNCH_W::new(self)
    }
    ///Bit 30 - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<30> {
        OPTLOCK_W::new(self)
    }
    ///Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH non-secure control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_nscr](index.html) module
pub struct FLASH_NSCR_SPEC;
impl crate::RegisterSpec for FLASH_NSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_nscr::R](R) reader structure
impl crate::Readable for FLASH_NSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_nscr::W](W) writer structure
impl crate::Writable for FLASH_NSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_NSCR to value 0xc000_0000
impl crate::Resettable for FLASH_NSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
