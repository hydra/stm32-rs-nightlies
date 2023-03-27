///Register `FLASH_SECCR` reader
pub struct R(crate::R<FLASH_SECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SECCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SECCR` writer
pub struct W(crate::W<FLASH_SECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SECCR_SPEC>;
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
impl From<crate::W<FLASH_SECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SECCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PG` reader - Secure programming
pub type PG_R = crate::BitReader<bool>;
///Field `PG` writer - Secure programming
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `PER` reader - Secure page erase
pub type PER_R = crate::BitReader<bool>;
///Field `PER` writer - Secure page erase
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `MER1` reader - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
pub type MER1_R = crate::BitReader<bool>;
///Field `MER1` writer - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
pub type MER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `PNB` reader - Secure page number selection These bits select the page to erase: ...
pub type PNB_R = crate::FieldReader<u8, u8>;
///Field `PNB` writer - Secure page number selection These bits select the page to erase: ...
pub type PNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_SECCR_SPEC, u8, u8, 7, O>;
///Field `BKER` reader - Secure bank selection for page erase
pub type BKER_R = crate::BitReader<bool>;
///Field `BKER` writer - Secure bank selection for page erase
pub type BKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `BWR` reader - Secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_R = crate::BitReader<bool>;
///Field `BWR` writer - Secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `MER2` reader - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
pub type MER2_R = crate::BitReader<bool>;
///Field `MER2` writer - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
pub type MER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `STRT` reader - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
pub type STRT_R = crate::BitReader<bool>;
///Field `STRT` writer - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `EOPIE` reader - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
pub type EOPIE_R = crate::BitReader<bool>;
///Field `EOPIE` writer - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `ERRIE` reader - Secure error interrupt enable
pub type ERRIE_R = crate::BitReader<bool>;
///Field `ERRIE` writer - Secure error interrupt enable
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `RDERRIE` reader - Secure PCROP read error interrupt enable
pub type RDERRIE_R = crate::BitReader<bool>;
///Field `RDERRIE` writer - Secure PCROP read error interrupt enable
pub type RDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `INV` reader - Flash memory security state invert This bit inverts the Flash memory security state.
pub type INV_R = crate::BitReader<bool>;
///Field `INV` writer - Flash memory security state invert This bit inverts the Flash memory security state.
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
///Field `LOCK` reader - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_R = crate::BitReader<bool>;
///Field `LOCK` writer - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Secure programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Secure page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Secure page number selection These bits select the page to erase: ...
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 11 - Secure bank selection for page erase
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Secure error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Secure PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - Flash memory security state invert This bit inverts the Flash memory security state.
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Secure programming
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<0> {
        PG_W::new(self)
    }
    ///Bit 1 - Secure page erase
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<1> {
        PER_W::new(self)
    }
    ///Bit 2 - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> MER1_W<2> {
        MER1_W::new(self)
    }
    ///Bits 3:9 - Secure page number selection These bits select the page to erase: ...
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<3> {
        PNB_W::new(self)
    }
    ///Bit 11 - Secure bank selection for page erase
    #[inline(always)]
    #[must_use]
    pub fn bker(&mut self) -> BKER_W<11> {
        BKER_W::new(self)
    }
    ///Bit 14 - Secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    #[must_use]
    pub fn bwr(&mut self) -> BWR_W<14> {
        BWR_W::new(self)
    }
    ///Bit 15 - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    #[must_use]
    pub fn mer2(&mut self) -> MER2_W<15> {
        MER2_W::new(self)
    }
    ///Bit 16 - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<16> {
        STRT_W::new(self)
    }
    ///Bit 24 - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<24> {
        EOPIE_W::new(self)
    }
    ///Bit 25 - Secure error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<25> {
        ERRIE_W::new(self)
    }
    ///Bit 26 - Secure PCROP read error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RDERRIE_W<26> {
        RDERRIE_W::new(self)
    }
    ///Bit 29 - Flash memory security state invert This bit inverts the Flash memory security state.
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<29> {
        INV_W::new(self)
    }
    ///Bit 31 - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
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
///FLASH secure control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_seccr](index.html) module
pub struct FLASH_SECCR_SPEC;
impl crate::RegisterSpec for FLASH_SECCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_seccr::R](R) reader structure
impl crate::Readable for FLASH_SECCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_seccr::W](W) writer structure
impl crate::Writable for FLASH_SECCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SECCR to value 0x8000_0000
impl crate::Resettable for FLASH_SECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
