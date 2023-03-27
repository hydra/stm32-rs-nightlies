///Register `CRR3` reader
pub struct R(crate::R<CRR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRR3` writer
pub struct W(crate::W<CRR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRR3_SPEC>;
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
impl From<crate::W<CRR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BASEADDR` reader - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
pub type BASEADDR_R = crate::FieldReader<u8, u8>;
///Field `BASEADDR` writer - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
pub type BASEADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRR3_SPEC, u8, u8, 8, O>;
///Field `RSIZE` reader - size for region x
pub type RSIZE_R = crate::FieldReader<u8, u8>;
///Field `RSIZE` writer - size for region x
pub type RSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRR3_SPEC, u8, u8, 3, O>;
///Field `REN` reader - enable for region x
pub type REN_R = crate::BitReader<bool>;
///Field `REN` writer - enable for region x
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRR3_SPEC, bool, O>;
///Field `REMAPADDR` reader - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
pub type REMAPADDR_R = crate::FieldReader<u16, u16>;
///Field `REMAPADDR` writer - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
pub type REMAPADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRR3_SPEC, u16, u16, 11, O>;
///Field `MSTSEL` reader - AHB cache master selection for region x
pub type MSTSEL_R = crate::BitReader<bool>;
///Field `MSTSEL` writer - AHB cache master selection for region x
pub type MSTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRR3_SPEC, bool, O>;
///Field `HBURST` reader - output burst type for region x
pub type HBURST_R = crate::BitReader<bool>;
///Field `HBURST` writer - output burst type for region x
pub type HBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRR3_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 9:11 - size for region x
    #[inline(always)]
    pub fn rsize(&self) -> RSIZE_R {
        RSIZE_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 15 - enable for region x
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    pub fn remapaddr(&self) -> REMAPADDR_R {
        REMAPADDR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 28 - AHB cache master selection for region x
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 31 - output burst type for region x
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    #[must_use]
    pub fn baseaddr(&mut self) -> BASEADDR_W<0> {
        BASEADDR_W::new(self)
    }
    ///Bits 9:11 - size for region x
    #[inline(always)]
    #[must_use]
    pub fn rsize(&mut self) -> RSIZE_W<9> {
        RSIZE_W::new(self)
    }
    ///Bit 15 - enable for region x
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<15> {
        REN_W::new(self)
    }
    ///Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    #[must_use]
    pub fn remapaddr(&mut self) -> REMAPADDR_W<16> {
        REMAPADDR_W::new(self)
    }
    ///Bit 28 - AHB cache master selection for region x
    #[inline(always)]
    #[must_use]
    pub fn mstsel(&mut self) -> MSTSEL_W<28> {
        MSTSEL_W::new(self)
    }
    ///Bit 31 - output burst type for region x
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<31> {
        HBURST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ICACHE region 3 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crr3](index.html) module
pub struct CRR3_SPEC;
impl crate::RegisterSpec for CRR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [crr3::R](R) reader structure
impl crate::Readable for CRR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crr3::W](W) writer structure
impl crate::Writable for CRR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRR3 to value 0x0200
impl crate::Resettable for CRR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
