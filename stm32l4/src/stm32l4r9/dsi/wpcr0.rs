///Register `WPCR0` reader
pub struct R(crate::R<WPCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WPCR0` writer
pub struct W(crate::W<WPCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR0_SPEC>;
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
impl From<crate::W<WPCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UIX4` reader - Unit Interval multiplied by 4
pub type UIX4_R = crate::FieldReader<u8, u8>;
///Field `UIX4` writer - Unit Interval multiplied by 4
pub type UIX4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR0_SPEC, u8, u8, 6, O>;
///Field `SWCL` reader - Swap Clock Lane pins
pub type SWCL_R = crate::BitReader<bool>;
///Field `SWCL` writer - Swap Clock Lane pins
pub type SWCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `SWDL0` reader - Swap Data Lane 0 pins
pub type SWDL0_R = crate::BitReader<bool>;
///Field `SWDL0` writer - Swap Data Lane 0 pins
pub type SWDL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `SWDL1` reader - Swap Data Lane 1 pins
pub type SWDL1_R = crate::BitReader<bool>;
///Field `SWDL1` writer - Swap Data Lane 1 pins
pub type SWDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `HSICL` reader - Invert Hight-Speed data signal on Clock Lane
pub type HSICL_R = crate::BitReader<bool>;
///Field `HSICL` writer - Invert Hight-Speed data signal on Clock Lane
pub type HSICL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `HSIDL0` reader - Invert the Hight-Speed data signal on Data Lane 0
pub type HSIDL0_R = crate::BitReader<bool>;
///Field `HSIDL0` writer - Invert the Hight-Speed data signal on Data Lane 0
pub type HSIDL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `HSIDL1` reader - Invert the High-Speed data signal on Data Lane 1
pub type HSIDL1_R = crate::BitReader<bool>;
///Field `HSIDL1` writer - Invert the High-Speed data signal on Data Lane 1
pub type HSIDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `FTXSMCL` reader - Force in TX Stop Mode the Clock Lane
pub type FTXSMCL_R = crate::BitReader<bool>;
///Field `FTXSMCL` writer - Force in TX Stop Mode the Clock Lane
pub type FTXSMCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `FTXSMDL` reader - Force in TX Stop Mode the Data Lanes
pub type FTXSMDL_R = crate::BitReader<bool>;
///Field `FTXSMDL` writer - Force in TX Stop Mode the Data Lanes
pub type FTXSMDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `CDOFFDL` reader - Contention Detection OFF on Data Lanes
pub type CDOFFDL_R = crate::BitReader<bool>;
///Field `CDOFFDL` writer - Contention Detection OFF on Data Lanes
pub type CDOFFDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `TDDL` reader - Turn Disable Data Lanes
pub type TDDL_R = crate::BitReader<bool>;
///Field `TDDL` writer - Turn Disable Data Lanes
pub type TDDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `PDEN` reader - Pull-Down Enable
pub type PDEN_R = crate::BitReader<bool>;
///Field `PDEN` writer - Pull-Down Enable
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `TCLKPREPEN` reader - custom time for tCLK-PREPARE Enable
pub type TCLKPREPEN_R = crate::BitReader<bool>;
///Field `TCLKPREPEN` writer - custom time for tCLK-PREPARE Enable
pub type TCLKPREPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `TCLKZEROEN` reader - custom time for tCLK-ZERO Enable
pub type TCLKZEROEN_R = crate::BitReader<bool>;
///Field `TCLKZEROEN` writer - custom time for tCLK-ZERO Enable
pub type TCLKZEROEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `THSPREPEN` reader - custom time for tHS-PREPARE Enable
pub type THSPREPEN_R = crate::BitReader<bool>;
///Field `THSPREPEN` writer - custom time for tHS-PREPARE Enable
pub type THSPREPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `THSTRAILEN` reader - custom time for tHS-TRAIL Enable
pub type THSTRAILEN_R = crate::BitReader<bool>;
///Field `THSTRAILEN` writer - custom time for tHS-TRAIL Enable
pub type THSTRAILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `THSZEROEN` reader - custom time for tHS-ZERO Enable
pub type THSZEROEN_R = crate::BitReader<bool>;
///Field `THSZEROEN` writer - custom time for tHS-ZERO Enable
pub type THSZEROEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `TLPXDEN` reader - custom time for tLPX for Data lanes Enable
pub type TLPXDEN_R = crate::BitReader<bool>;
///Field `TLPXDEN` writer - custom time for tLPX for Data lanes Enable
pub type TLPXDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `THSEXITEN` reader - custom time for tHS-EXIT Enable
pub type THSEXITEN_R = crate::BitReader<bool>;
///Field `THSEXITEN` writer - custom time for tHS-EXIT Enable
pub type THSEXITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `TLPXCEN` reader - custom time for tLPX for Clock lane Enable
pub type TLPXCEN_R = crate::BitReader<bool>;
///Field `TLPXCEN` writer - custom time for tLPX for Clock lane Enable
pub type TLPXCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
///Field `TCLKPOSTEN` reader - custom time for tCLK-POST Enable
pub type TCLKPOSTEN_R = crate::BitReader<bool>;
///Field `TCLKPOSTEN` writer - custom time for tCLK-POST Enable
pub type TCLKPOSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
impl R {
    ///Bits 0:5 - Unit Interval multiplied by 4
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - Swap Clock Lane pins
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Swap Data Lane 0 pins
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Swap Data Lane 1 pins
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Invert Hight-Speed data signal on Clock Lane
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Invert the Hight-Speed data signal on Data Lane 0
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Invert the High-Speed data signal on Data Lane 1
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Force in TX Stop Mode the Clock Lane
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Force in TX Stop Mode the Data Lanes
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Contention Detection OFF on Data Lanes
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Turn Disable Data Lanes
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Pull-Down Enable
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - custom time for tCLK-PREPARE Enable
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - custom time for tCLK-ZERO Enable
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - custom time for tHS-PREPARE Enable
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - custom time for tHS-TRAIL Enable
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - custom time for tHS-ZERO Enable
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - custom time for tLPX for Data lanes Enable
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - custom time for tHS-EXIT Enable
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - custom time for tLPX for Clock lane Enable
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - custom time for tCLK-POST Enable
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - Unit Interval multiplied by 4
    #[inline(always)]
    #[must_use]
    pub fn uix4(&mut self) -> UIX4_W<0> {
        UIX4_W::new(self)
    }
    ///Bit 6 - Swap Clock Lane pins
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<6> {
        SWCL_W::new(self)
    }
    ///Bit 7 - Swap Data Lane 0 pins
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<7> {
        SWDL0_W::new(self)
    }
    ///Bit 8 - Swap Data Lane 1 pins
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<8> {
        SWDL1_W::new(self)
    }
    ///Bit 9 - Invert Hight-Speed data signal on Clock Lane
    #[inline(always)]
    #[must_use]
    pub fn hsicl(&mut self) -> HSICL_W<9> {
        HSICL_W::new(self)
    }
    ///Bit 10 - Invert the Hight-Speed data signal on Data Lane 0
    #[inline(always)]
    #[must_use]
    pub fn hsidl0(&mut self) -> HSIDL0_W<10> {
        HSIDL0_W::new(self)
    }
    ///Bit 11 - Invert the High-Speed data signal on Data Lane 1
    #[inline(always)]
    #[must_use]
    pub fn hsidl1(&mut self) -> HSIDL1_W<11> {
        HSIDL1_W::new(self)
    }
    ///Bit 12 - Force in TX Stop Mode the Clock Lane
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<12> {
        FTXSMCL_W::new(self)
    }
    ///Bit 13 - Force in TX Stop Mode the Data Lanes
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<13> {
        FTXSMDL_W::new(self)
    }
    ///Bit 14 - Contention Detection OFF on Data Lanes
    #[inline(always)]
    #[must_use]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<14> {
        CDOFFDL_W::new(self)
    }
    ///Bit 16 - Turn Disable Data Lanes
    #[inline(always)]
    #[must_use]
    pub fn tddl(&mut self) -> TDDL_W<16> {
        TDDL_W::new(self)
    }
    ///Bit 18 - Pull-Down Enable
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<18> {
        PDEN_W::new(self)
    }
    ///Bit 19 - custom time for tCLK-PREPARE Enable
    #[inline(always)]
    #[must_use]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W<19> {
        TCLKPREPEN_W::new(self)
    }
    ///Bit 20 - custom time for tCLK-ZERO Enable
    #[inline(always)]
    #[must_use]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W<20> {
        TCLKZEROEN_W::new(self)
    }
    ///Bit 21 - custom time for tHS-PREPARE Enable
    #[inline(always)]
    #[must_use]
    pub fn thsprepen(&mut self) -> THSPREPEN_W<21> {
        THSPREPEN_W::new(self)
    }
    ///Bit 22 - custom time for tHS-TRAIL Enable
    #[inline(always)]
    #[must_use]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W<22> {
        THSTRAILEN_W::new(self)
    }
    ///Bit 23 - custom time for tHS-ZERO Enable
    #[inline(always)]
    #[must_use]
    pub fn thszeroen(&mut self) -> THSZEROEN_W<23> {
        THSZEROEN_W::new(self)
    }
    ///Bit 24 - custom time for tLPX for Data lanes Enable
    #[inline(always)]
    #[must_use]
    pub fn tlpxden(&mut self) -> TLPXDEN_W<24> {
        TLPXDEN_W::new(self)
    }
    ///Bit 25 - custom time for tHS-EXIT Enable
    #[inline(always)]
    #[must_use]
    pub fn thsexiten(&mut self) -> THSEXITEN_W<25> {
        THSEXITEN_W::new(self)
    }
    ///Bit 26 - custom time for tLPX for Clock lane Enable
    #[inline(always)]
    #[must_use]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W<26> {
        TLPXCEN_W::new(self)
    }
    ///Bit 27 - custom time for tCLK-POST Enable
    #[inline(always)]
    #[must_use]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W<27> {
        TCLKPOSTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Wrapper PHY Configuration Register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wpcr0](index.html) module
pub struct WPCR0_SPEC;
impl crate::RegisterSpec for WPCR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [wpcr0::R](R) reader structure
impl crate::Readable for WPCR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wpcr0::W](W) writer structure
impl crate::Writable for WPCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WPCR0 to value 0
impl crate::Resettable for WPCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
