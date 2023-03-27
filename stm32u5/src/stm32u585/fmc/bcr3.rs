///Register `BCR3` reader
pub struct R(crate::R<BCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BCR3` writer
pub struct W(crate::W<BCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR3_SPEC>;
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
impl From<crate::W<BCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MBKEN` reader - Memory bank enable bit
pub type MBKEN_R = crate::BitReader<bool>;
///Field `MBKEN` writer - Memory bank enable bit
pub type MBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `MUXEN` reader - Address/data multiplexing enable bit
pub type MUXEN_R = crate::BitReader<bool>;
///Field `MUXEN` writer - Address/data multiplexing enable bit
pub type MUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `MTYP` reader - Memory type
pub type MTYP_R = crate::FieldReader<u8, u8>;
///Field `MTYP` writer - Memory type
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR3_SPEC, u8, u8, 2, O>;
///Field `MWID` reader - Memory data bus width
pub type MWID_R = crate::FieldReader<u8, u8>;
///Field `MWID` writer - Memory data bus width
pub type MWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR3_SPEC, u8, u8, 2, O>;
///Field `FACCEN` reader - Flash access enable
pub type FACCEN_R = crate::BitReader<bool>;
///Field `FACCEN` writer - Flash access enable
pub type FACCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `BURSTEN` reader - Burst enable bit
pub type BURSTEN_R = crate::BitReader<bool>;
///Field `BURSTEN` writer - Burst enable bit
pub type BURSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `WAITPOL` reader - Wait signal polarity bit
pub type WAITPOL_R = crate::BitReader<bool>;
///Field `WAITPOL` writer - Wait signal polarity bit
pub type WAITPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `WAITCFG` reader - Wait timing configuration
pub type WAITCFG_R = crate::BitReader<bool>;
///Field `WAITCFG` writer - Wait timing configuration
pub type WAITCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `WREN` reader - Write enable bit
pub type WREN_R = crate::BitReader<bool>;
///Field `WREN` writer - Write enable bit
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `WAITEN` reader - Wait enable bit
pub type WAITEN_R = crate::BitReader<bool>;
///Field `WAITEN` writer - Wait enable bit
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `EXTMOD` reader - Extended mode enable
pub type EXTMOD_R = crate::BitReader<bool>;
///Field `EXTMOD` writer - Extended mode enable
pub type EXTMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers
pub type ASYNCWAIT_R = crate::BitReader<bool>;
///Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers
pub type ASYNCWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `CPSIZE` reader - CRAM Page Size
pub type CPSIZE_R = crate::FieldReader<u8, u8>;
///Field `CPSIZE` writer - CRAM Page Size
pub type CPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR3_SPEC, u8, u8, 3, O>;
///Field `CBURSTRW` reader - Write burst enable
pub type CBURSTRW_R = crate::BitReader<bool>;
///Field `CBURSTRW` writer - Write burst enable
pub type CBURSTRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `CCLKEN` reader - Continuous clock enable
pub type CCLKEN_R = crate::BitReader<bool>;
///Field `CCLKEN` writer - Continuous clock enable
pub type CCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `WFDIS` reader - Write FIFO disable
pub type WFDIS_R = crate::BitReader<bool>;
///Field `WFDIS` writer - Write FIFO disable
pub type WFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
///Field `NBLSET` reader - Byte lane (NBL) setup
pub type NBLSET_R = crate::FieldReader<u8, u8>;
///Field `NBLSET` writer - Byte lane (NBL) setup
pub type NBLSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR3_SPEC, u8, u8, 2, O>;
///Field `FMCEN` reader - FMC controller enable
pub type FMCEN_R = crate::BitReader<bool>;
///Field `FMCEN` writer - FMC controller enable
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - Memory bank enable bit
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Flash access enable
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - CRAM Page Size
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Continuous clock enable
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Write FIFO disable
    #[inline(always)]
    pub fn wfdis(&self) -> WFDIS_R {
        WFDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&self) -> NBLSET_R {
        NBLSET_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 31 - FMC controller enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Memory bank enable bit
    #[inline(always)]
    #[must_use]
    pub fn mbken(&mut self) -> MBKEN_W<0> {
        MBKEN_W::new(self)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<1> {
        MUXEN_W::new(self)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<2> {
        MTYP_W::new(self)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<4> {
        MWID_W::new(self)
    }
    ///Bit 6 - Flash access enable
    #[inline(always)]
    #[must_use]
    pub fn faccen(&mut self) -> FACCEN_W<6> {
        FACCEN_W::new(self)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    #[must_use]
    pub fn bursten(&mut self) -> BURSTEN_W<8> {
        BURSTEN_W::new(self)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    #[must_use]
    pub fn waitpol(&mut self) -> WAITPOL_W<9> {
        WAITPOL_W::new(self)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    #[must_use]
    pub fn waitcfg(&mut self) -> WAITCFG_W<11> {
        WAITCFG_W::new(self)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<12> {
        WREN_W::new(self)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<13> {
        WAITEN_W::new(self)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    #[must_use]
    pub fn extmod(&mut self) -> EXTMOD_W<14> {
        EXTMOD_W::new(self)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<15> {
        ASYNCWAIT_W::new(self)
    }
    ///Bits 16:18 - CRAM Page Size
    #[inline(always)]
    #[must_use]
    pub fn cpsize(&mut self) -> CPSIZE_W<16> {
        CPSIZE_W::new(self)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    #[must_use]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<19> {
        CBURSTRW_W::new(self)
    }
    ///Bit 20 - Continuous clock enable
    #[inline(always)]
    #[must_use]
    pub fn cclken(&mut self) -> CCLKEN_W<20> {
        CCLKEN_W::new(self)
    }
    ///Bit 21 - Write FIFO disable
    #[inline(always)]
    #[must_use]
    pub fn wfdis(&mut self) -> WFDIS_W<21> {
        WFDIS_W::new(self)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    #[must_use]
    pub fn nblset(&mut self) -> NBLSET_W<22> {
        NBLSET_W::new(self)
    }
    ///Bit 31 - FMC controller enable
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<31> {
        FMCEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SRAM/NOR-Flash chip-select control register for bank 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr3](index.html) module
pub struct BCR3_SPEC;
impl crate::RegisterSpec for BCR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [bcr3::R](R) reader structure
impl crate::Readable for BCR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bcr3::W](W) writer structure
impl crate::Writable for BCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BCR3 to value 0x30d2
impl crate::Resettable for BCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x30d2;
}
