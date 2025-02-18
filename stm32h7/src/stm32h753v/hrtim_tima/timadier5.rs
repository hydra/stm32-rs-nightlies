///Register `TIMADIER5` reader
pub struct R(crate::R<TIMADIER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMADIER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMADIER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMADIER5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMADIER5` writer
pub struct W(crate::W<TIMADIER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMADIER5_SPEC>;
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
impl From<crate::W<TIMADIER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMADIER5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMP1IE` reader - CMP1IE
pub type CMP1IE_R = crate::BitReader<bool>;
///Field `CMP1IE` writer - CMP1IE
pub type CMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CMP2IE` reader - CMP2IE
pub type CMP2IE_R = crate::BitReader<bool>;
///Field `CMP2IE` writer - CMP2IE
pub type CMP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CMP3IE` reader - CMP3IE
pub type CMP3IE_R = crate::BitReader<bool>;
///Field `CMP3IE` writer - CMP3IE
pub type CMP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CMP4IE` reader - CMP4IE
pub type CMP4IE_R = crate::BitReader<bool>;
///Field `CMP4IE` writer - CMP4IE
pub type CMP4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `REPIE` reader - REPIE
pub type REPIE_R = crate::BitReader<bool>;
///Field `REPIE` writer - REPIE
pub type REPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `UPDIE` reader - UPDIE
pub type UPDIE_R = crate::BitReader<bool>;
///Field `UPDIE` writer - UPDIE
pub type UPDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CPT1IE` reader - CPT1IE
pub type CPT1IE_R = crate::BitReader<bool>;
///Field `CPT1IE` writer - CPT1IE
pub type CPT1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CPT2IE` reader - CPT2IE
pub type CPT2IE_R = crate::BitReader<bool>;
///Field `CPT2IE` writer - CPT2IE
pub type CPT2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `SET1xIE` reader - SET1xIE
pub type SET1X_IE_R = crate::BitReader<bool>;
///Field `SET1xIE` writer - SET1xIE
pub type SET1X_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `RSTx1IE` reader - RSTx1IE
pub type RSTX1IE_R = crate::BitReader<bool>;
///Field `RSTx1IE` writer - RSTx1IE
pub type RSTX1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `SETx2IE` reader - SETx2IE
pub type SETX2IE_R = crate::BitReader<bool>;
///Field `SETx2IE` writer - SETx2IE
pub type SETX2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `RSTx2IE` reader - RSTx2IE
pub type RSTX2IE_R = crate::BitReader<bool>;
///Field `RSTx2IE` writer - RSTx2IE
pub type RSTX2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `RSTIE` reader - RSTIE
pub type RSTIE_R = crate::BitReader<bool>;
///Field `RSTIE` writer - RSTIE
pub type RSTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `DLYPRTIE` reader - DLYPRTIE
pub type DLYPRTIE_R = crate::BitReader<bool>;
///Field `DLYPRTIE` writer - DLYPRTIE
pub type DLYPRTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CMP1DE` reader - CMP1DE
pub type CMP1DE_R = crate::BitReader<bool>;
///Field `CMP1DE` writer - CMP1DE
pub type CMP1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CMP2DE` reader - CMP2DE
pub type CMP2DE_R = crate::BitReader<bool>;
///Field `CMP2DE` writer - CMP2DE
pub type CMP2DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CMP3DE` reader - CMP3DE
pub type CMP3DE_R = crate::BitReader<bool>;
///Field `CMP3DE` writer - CMP3DE
pub type CMP3DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CMP4DE` reader - CMP4DE
pub type CMP4DE_R = crate::BitReader<bool>;
///Field `CMP4DE` writer - CMP4DE
pub type CMP4DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `REPDE` reader - REPDE
pub type REPDE_R = crate::BitReader<bool>;
///Field `REPDE` writer - REPDE
pub type REPDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `UPDDE` reader - UPDDE
pub type UPDDE_R = crate::BitReader<bool>;
///Field `UPDDE` writer - UPDDE
pub type UPDDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CPT1DE` reader - CPT1DE
pub type CPT1DE_R = crate::BitReader<bool>;
///Field `CPT1DE` writer - CPT1DE
pub type CPT1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `CPT2DE` reader - CPT2DE
pub type CPT2DE_R = crate::BitReader<bool>;
///Field `CPT2DE` writer - CPT2DE
pub type CPT2DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `SET1xDE` reader - SET1xDE
pub type SET1X_DE_R = crate::BitReader<bool>;
///Field `SET1xDE` writer - SET1xDE
pub type SET1X_DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `RSTx1DE` reader - RSTx1DE
pub type RSTX1DE_R = crate::BitReader<bool>;
///Field `RSTx1DE` writer - RSTx1DE
pub type RSTX1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `SETx2DE` reader - SETx2DE
pub type SETX2DE_R = crate::BitReader<bool>;
///Field `SETx2DE` writer - SETx2DE
pub type SETX2DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `RSTx2DE` reader - RSTx2DE
pub type RSTX2DE_R = crate::BitReader<bool>;
///Field `RSTx2DE` writer - RSTx2DE
pub type RSTX2DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `RSTDE` reader - RSTDE
pub type RSTDE_R = crate::BitReader<bool>;
///Field `RSTDE` writer - RSTDE
pub type RSTDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
///Field `DLYPRTDE` reader - DLYPRTDE
pub type DLYPRTDE_R = crate::BitReader<bool>;
///Field `DLYPRTDE` writer - DLYPRTDE
pub type DLYPRTDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER5_SPEC, bool, O>;
impl R {
    ///Bit 0 - CMP1IE
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMP3IE_R {
        CMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMP4IE_R {
        CMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    pub fn cpt1ie(&self) -> CPT1IE_R {
        CPT1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    pub fn cpt2ie(&self) -> CPT2IE_R {
        CPT2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SET1xIE
    #[inline(always)]
    pub fn set1x_ie(&self) -> SET1X_IE_R {
        SET1X_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    pub fn rstx1ie(&self) -> RSTX1IE_R {
        RSTX1IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SETx2IE
    #[inline(always)]
    pub fn setx2ie(&self) -> SETX2IE_R {
        SETX2IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    pub fn rstx2ie(&self) -> RSTX2IE_R {
        RSTX2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    pub fn dlyprtie(&self) -> DLYPRTIE_R {
        DLYPRTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    pub fn cmp1de(&self) -> CMP1DE_R {
        CMP1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    pub fn cmp2de(&self) -> CMP2DE_R {
        CMP2DE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    pub fn cmp3de(&self) -> CMP3DE_R {
        CMP3DE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    pub fn cmp4de(&self) -> CMP4DE_R {
        CMP4DE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    pub fn cpt1de(&self) -> CPT1DE_R {
        CPT1DE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    pub fn cpt2de(&self) -> CPT2DE_R {
        CPT2DE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SET1xDE
    #[inline(always)]
    pub fn set1x_de(&self) -> SET1X_DE_R {
        SET1X_DE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    pub fn rstx1de(&self) -> RSTX1DE_R {
        RSTX1DE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SETx2DE
    #[inline(always)]
    pub fn setx2de(&self) -> SETX2DE_R {
        SETX2DE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    pub fn rstx2de(&self) -> RSTX2DE_R {
        RSTX2DE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    pub fn rstde(&self) -> RSTDE_R {
        RSTDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    pub fn dlyprtde(&self) -> DLYPRTDE_R {
        DLYPRTDE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CMP1IE
    #[inline(always)]
    #[must_use]
    pub fn cmp1ie(&mut self) -> CMP1IE_W<0> {
        CMP1IE_W::new(self)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    #[must_use]
    pub fn cmp2ie(&mut self) -> CMP2IE_W<1> {
        CMP2IE_W::new(self)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    #[must_use]
    pub fn cmp3ie(&mut self) -> CMP3IE_W<2> {
        CMP3IE_W::new(self)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    #[must_use]
    pub fn cmp4ie(&mut self) -> CMP4IE_W<3> {
        CMP4IE_W::new(self)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    #[must_use]
    pub fn repie(&mut self) -> REPIE_W<4> {
        REPIE_W::new(self)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    #[must_use]
    pub fn updie(&mut self) -> UPDIE_W<6> {
        UPDIE_W::new(self)
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    #[must_use]
    pub fn cpt1ie(&mut self) -> CPT1IE_W<7> {
        CPT1IE_W::new(self)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    #[must_use]
    pub fn cpt2ie(&mut self) -> CPT2IE_W<8> {
        CPT2IE_W::new(self)
    }
    ///Bit 9 - SET1xIE
    #[inline(always)]
    #[must_use]
    pub fn set1x_ie(&mut self) -> SET1X_IE_W<9> {
        SET1X_IE_W::new(self)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    #[must_use]
    pub fn rstx1ie(&mut self) -> RSTX1IE_W<10> {
        RSTX1IE_W::new(self)
    }
    ///Bit 11 - SETx2IE
    #[inline(always)]
    #[must_use]
    pub fn setx2ie(&mut self) -> SETX2IE_W<11> {
        SETX2IE_W::new(self)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    #[must_use]
    pub fn rstx2ie(&mut self) -> RSTX2IE_W<12> {
        RSTX2IE_W::new(self)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RSTIE_W<13> {
        RSTIE_W::new(self)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    #[must_use]
    pub fn dlyprtie(&mut self) -> DLYPRTIE_W<14> {
        DLYPRTIE_W::new(self)
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    #[must_use]
    pub fn cmp1de(&mut self) -> CMP1DE_W<16> {
        CMP1DE_W::new(self)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    #[must_use]
    pub fn cmp2de(&mut self) -> CMP2DE_W<17> {
        CMP2DE_W::new(self)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    #[must_use]
    pub fn cmp3de(&mut self) -> CMP3DE_W<18> {
        CMP3DE_W::new(self)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    #[must_use]
    pub fn cmp4de(&mut self) -> CMP4DE_W<19> {
        CMP4DE_W::new(self)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    #[must_use]
    pub fn repde(&mut self) -> REPDE_W<20> {
        REPDE_W::new(self)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    #[must_use]
    pub fn updde(&mut self) -> UPDDE_W<22> {
        UPDDE_W::new(self)
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    #[must_use]
    pub fn cpt1de(&mut self) -> CPT1DE_W<23> {
        CPT1DE_W::new(self)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    #[must_use]
    pub fn cpt2de(&mut self) -> CPT2DE_W<24> {
        CPT2DE_W::new(self)
    }
    ///Bit 25 - SET1xDE
    #[inline(always)]
    #[must_use]
    pub fn set1x_de(&mut self) -> SET1X_DE_W<25> {
        SET1X_DE_W::new(self)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    #[must_use]
    pub fn rstx1de(&mut self) -> RSTX1DE_W<26> {
        RSTX1DE_W::new(self)
    }
    ///Bit 27 - SETx2DE
    #[inline(always)]
    #[must_use]
    pub fn setx2de(&mut self) -> SETX2DE_W<27> {
        SETX2DE_W::new(self)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    #[must_use]
    pub fn rstx2de(&mut self) -> RSTX2DE_W<28> {
        RSTX2DE_W::new(self)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    #[must_use]
    pub fn rstde(&mut self) -> RSTDE_W<29> {
        RSTDE_W::new(self)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    #[must_use]
    pub fn dlyprtde(&mut self) -> DLYPRTDE_W<30> {
        DLYPRTDE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIMxDIER5
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timadier5](index.html) module
pub struct TIMADIER5_SPEC;
impl crate::RegisterSpec for TIMADIER5_SPEC {
    type Ux = u32;
}
///`read()` method returns [timadier5::R](R) reader structure
impl crate::Readable for TIMADIER5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timadier5::W](W) writer structure
impl crate::Writable for TIMADIER5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMADIER5 to value 0
impl crate::Resettable for TIMADIER5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
