///Register `WCCR` reader
pub struct R(crate::R<WCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WCCR` writer
pub struct W(crate::W<WCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCCR_SPEC>;
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
impl From<crate::W<WCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IMODE` reader - Instruction mode
pub type IMODE_R = crate::FieldReader<u8, u8>;
///Field `IMODE` writer - Instruction mode
pub type IMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, u8, 3, O>;
///Field `IDTR` reader - Instruction double transfer rate
pub type IDTR_R = crate::BitReader<bool>;
///Field `IDTR` writer - Instruction double transfer rate
pub type IDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, bool, O>;
///Field `ISIZE` reader - Instruction size
pub type ISIZE_R = crate::FieldReader<u8, u8>;
///Field `ISIZE` writer - Instruction size
pub type ISIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, u8, 2, O>;
///Field `ADMODE` reader - Address mode
pub type ADMODE_R = crate::FieldReader<u8, u8>;
///Field `ADMODE` writer - Address mode
pub type ADMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, u8, 3, O>;
///Field `ADDTR` reader - Address double transfer rate
pub type ADDTR_R = crate::BitReader<bool>;
///Field `ADDTR` writer - Address double transfer rate
pub type ADDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, bool, O>;
///Field `ADSIZE` reader - Address size
pub type ADSIZE_R = crate::FieldReader<u8, u8>;
///Field `ADSIZE` writer - Address size
pub type ADSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, u8, 2, O>;
///Field `ABMODE` reader - Alternate byte mode
pub type ABMODE_R = crate::FieldReader<u8, u8>;
///Field `ABMODE` writer - Alternate byte mode
pub type ABMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, u8, 3, O>;
///Field `ABDTR` reader - Alternate bytes double transfer rate
pub type ABDTR_R = crate::BitReader<bool>;
///Field `ABDTR` writer - Alternate bytes double transfer rate
pub type ABDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, bool, O>;
///Field `ABSIZE` reader - Alternate bytes size
pub type ABSIZE_R = crate::FieldReader<u8, u8>;
///Field `ABSIZE` writer - Alternate bytes size
pub type ABSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, u8, 2, O>;
///Field `DMODE` reader - Data mode
pub type DMODE_R = crate::FieldReader<u8, u8>;
///Field `DMODE` writer - Data mode
pub type DMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, u8, 3, O>;
///Field `DDTR` reader - alternate bytes double transfer rate
pub type DDTR_R = crate::BitReader<bool>;
///Field `DDTR` writer - alternate bytes double transfer rate
pub type DDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, bool, O>;
///Field `DQSE` reader - DQS enable
pub type DQSE_R = crate::BitReader<bool>;
///Field `DQSE` writer - DQS enable
pub type DQSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, bool, O>;
///Field `SIOO` reader - Send instruction only once mode
pub type SIOO_R = crate::BitReader<bool>;
///Field `SIOO` writer - Send instruction only once mode
pub type SIOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - Instruction mode
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Instruction double transfer rate
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Instruction size
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:10 - Address mode
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - Address double transfer rate
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Address size
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - Alternate byte mode
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Alternate bytes double transfer rate
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Alternate bytes size
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:26 - Data mode
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - alternate bytes double transfer rate
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DQS enable
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Send instruction only once mode
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Instruction mode
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<0> {
        IMODE_W::new(self)
    }
    ///Bit 3 - Instruction double transfer rate
    #[inline(always)]
    #[must_use]
    pub fn idtr(&mut self) -> IDTR_W<3> {
        IDTR_W::new(self)
    }
    ///Bits 4:5 - Instruction size
    #[inline(always)]
    #[must_use]
    pub fn isize(&mut self) -> ISIZE_W<4> {
        ISIZE_W::new(self)
    }
    ///Bits 8:10 - Address mode
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<8> {
        ADMODE_W::new(self)
    }
    ///Bit 11 - Address double transfer rate
    #[inline(always)]
    #[must_use]
    pub fn addtr(&mut self) -> ADDTR_W<11> {
        ADDTR_W::new(self)
    }
    ///Bits 12:13 - Address size
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<12> {
        ADSIZE_W::new(self)
    }
    ///Bits 16:18 - Alternate byte mode
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<16> {
        ABMODE_W::new(self)
    }
    ///Bit 19 - Alternate bytes double transfer rate
    #[inline(always)]
    #[must_use]
    pub fn abdtr(&mut self) -> ABDTR_W<19> {
        ABDTR_W::new(self)
    }
    ///Bits 20:21 - Alternate bytes size
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<20> {
        ABSIZE_W::new(self)
    }
    ///Bits 24:26 - Data mode
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<24> {
        DMODE_W::new(self)
    }
    ///Bit 27 - alternate bytes double transfer rate
    #[inline(always)]
    #[must_use]
    pub fn ddtr(&mut self) -> DDTR_W<27> {
        DDTR_W::new(self)
    }
    ///Bit 29 - DQS enable
    #[inline(always)]
    #[must_use]
    pub fn dqse(&mut self) -> DQSE_W<29> {
        DQSE_W::new(self)
    }
    ///Bit 31 - Send instruction only once mode
    #[inline(always)]
    #[must_use]
    pub fn sioo(&mut self) -> SIOO_W<31> {
        SIOO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///write communication configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wccr](index.html) module
pub struct WCCR_SPEC;
impl crate::RegisterSpec for WCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wccr::R](R) reader structure
impl crate::Readable for WCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wccr::W](W) writer structure
impl crate::Writable for WCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WCCR to value 0
impl crate::Resettable for WCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
