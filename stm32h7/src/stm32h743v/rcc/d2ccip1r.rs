///Register `D2CCIP1R` reader
pub struct R(crate::R<D2CCIP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D2CCIP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D2CCIP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D2CCIP1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `D2CCIP1R` writer
pub struct W(crate::W<D2CCIP1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D2CCIP1R_SPEC>;
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
impl From<crate::W<D2CCIP1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D2CCIP1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SAI1SEL` reader - SAI1 and DFSDM1 kernel Aclk clock source selection
pub type SAI1SEL_R = crate::FieldReader<u8, SAI1SEL_A>;
///SAI1 and DFSDM1 kernel Aclk clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    ///0: pll1_q selected as peripheral clock
    Pll1Q = 0,
    ///1: pll2_p selected as peripheral clock
    Pll2P = 1,
    ///2: pll3_p selected as peripheral clock
    Pll3P = 2,
    ///3: I2S_CKIN selected as peripheral clock
    I2sCkin = 3,
    ///4: PER selected as peripheral clock
    Per = 4,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI1SEL_A> {
        match self.bits {
            0 => Some(SAI1SEL_A::Pll1Q),
            1 => Some(SAI1SEL_A::Pll2P),
            2 => Some(SAI1SEL_A::Pll3P),
            3 => Some(SAI1SEL_A::I2sCkin),
            4 => Some(SAI1SEL_A::Per),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pll1Q`
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI1SEL_A::Pll1Q
    }
    ///Checks if the value of the field is `Pll2P`
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI1SEL_A::Pll2P
    }
    ///Checks if the value of the field is `Pll3P`
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI1SEL_A::Pll3P
    }
    ///Checks if the value of the field is `I2sCkin`
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL_A::I2sCkin
    }
    ///Checks if the value of the field is `Per`
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI1SEL_A::Per
    }
}
///Field `SAI1SEL` writer - SAI1 and DFSDM1 kernel Aclk clock source selection
pub type SAI1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D2CCIP1R_SPEC, u8, SAI1SEL_A, 3, O>;
impl<'a, const O: u8> SAI1SEL_W<'a, O> {
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pll1Q)
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pll2P)
    }
    ///pll3_p selected as peripheral clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pll3P)
    }
    ///I2S_CKIN selected as peripheral clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2sCkin)
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Per)
    }
}
///Field `SAI23SEL` reader - SAI2 and SAI3 kernel clock source selection
pub use SAI1SEL_R as SAI23SEL_R;
///Field `SPI123SEL` reader - SPI/I2S1,2 and 3 kernel clock source selection
pub use SAI1SEL_R as SPI123SEL_R;
///Field `SAI23SEL` writer - SAI2 and SAI3 kernel clock source selection
pub use SAI1SEL_W as SAI23SEL_W;
///Field `SPI123SEL` writer - SPI/I2S1,2 and 3 kernel clock source selection
pub use SAI1SEL_W as SPI123SEL_W;
///Field `SPI45SEL` reader - SPI4 and 5 kernel clock source selection
pub type SPI45SEL_R = crate::FieldReader<u8, SPI45SEL_A>;
///SPI4 and 5 kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI45SEL_A {
    ///0: APB clock selected as peripheral clock
    Apb = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: HSE selected as peripheral clock
    Hse = 5,
}
impl From<SPI45SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI45SEL_A) -> Self {
        variant as _
    }
}
impl SPI45SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI45SEL_A> {
        match self.bits {
            0 => Some(SPI45SEL_A::Apb),
            1 => Some(SPI45SEL_A::Pll2Q),
            2 => Some(SPI45SEL_A::Pll3Q),
            3 => Some(SPI45SEL_A::HsiKer),
            4 => Some(SPI45SEL_A::CsiKer),
            5 => Some(SPI45SEL_A::Hse),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Apb`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == SPI45SEL_A::Apb
    }
    ///Checks if the value of the field is `Pll2Q`
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI45SEL_A::Pll2Q
    }
    ///Checks if the value of the field is `Pll3Q`
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI45SEL_A::Pll3Q
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI45SEL_A::HsiKer
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI45SEL_A::CsiKer
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI45SEL_A::Hse
    }
}
///Field `SPI45SEL` writer - SPI4 and 5 kernel clock source selection
pub type SPI45SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D2CCIP1R_SPEC, u8, SPI45SEL_A, 3, O>;
impl<'a, const O: u8> SPI45SEL_W<'a, O> {
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Apb)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::CsiKer)
    }
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Hse)
    }
}
///Field `SPDIFSEL` reader - SPDIFRX kernel clock source selection
pub type SPDIFSEL_R = crate::FieldReader<u8, SPDIFSEL_A>;
///SPDIFRX kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIFSEL_A {
    ///0: pll1_q selected as peripheral clock
    Pll1Q = 0,
    ///1: pll2_r selected as peripheral clock
    Pll2R = 1,
    ///2: pll3_r selected as peripheral clock
    Pll3R = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
}
impl From<SPDIFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIFSEL_A) -> Self {
        variant as _
    }
}
impl SPDIFSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SPDIFSEL_A {
        match self.bits {
            0 => SPDIFSEL_A::Pll1Q,
            1 => SPDIFSEL_A::Pll2R,
            2 => SPDIFSEL_A::Pll3R,
            3 => SPDIFSEL_A::HsiKer,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Pll1Q`
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPDIFSEL_A::Pll1Q
    }
    ///Checks if the value of the field is `Pll2R`
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SPDIFSEL_A::Pll2R
    }
    ///Checks if the value of the field is `Pll3R`
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == SPDIFSEL_A::Pll3R
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPDIFSEL_A::HsiKer
    }
}
///Field `SPDIFSEL` writer - SPDIFRX kernel clock source selection
pub type SPDIFSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D2CCIP1R_SPEC, u8, SPDIFSEL_A, 2, O>;
impl<'a, const O: u8> SPDIFSEL_W<'a, O> {
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::Pll1Q)
    }
    ///pll2_r selected as peripheral clock
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::Pll2R)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::Pll3R)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::HsiKer)
    }
}
///Field `DFSDM1SEL` reader - DFSDM1 kernel Clk clock source selection
pub type DFSDM1SEL_R = crate::BitReader<DFSDM1SEL_A>;
///DFSDM1 kernel Clk clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SEL_A {
    ///0: rcc_pclk2 selected as peripheral clock
    RccPclk2 = 0,
    ///1: System clock selected as peripheral clock
    Sys = 1,
}
impl From<DFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1SEL_A {
        match self.bits {
            false => DFSDM1SEL_A::RccPclk2,
            true => DFSDM1SEL_A::Sys,
        }
    }
    ///Checks if the value of the field is `RccPclk2`
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == DFSDM1SEL_A::RccPclk2
    }
    ///Checks if the value of the field is `Sys`
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == DFSDM1SEL_A::Sys
    }
}
///Field `DFSDM1SEL` writer - DFSDM1 kernel Clk clock source selection
pub type DFSDM1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, D2CCIP1R_SPEC, DFSDM1SEL_A, O>;
impl<'a, const O: u8> DFSDM1SEL_W<'a, O> {
    ///rcc_pclk2 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::RccPclk2)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn sys(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::Sys)
    }
}
///Field `FDCANSEL` reader - FDCAN kernel clock source selection
pub type FDCANSEL_R = crate::FieldReader<u8, FDCANSEL_A>;
///FDCAN kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL_A {
    ///0: HSE selected as peripheral clock
    Hse = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: pll2_q selected as peripheral clock
    Pll2Q = 2,
}
impl From<FDCANSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL_A) -> Self {
        variant as _
    }
}
impl FDCANSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FDCANSEL_A> {
        match self.bits {
            0 => Some(FDCANSEL_A::Hse),
            1 => Some(FDCANSEL_A::Pll1Q),
            2 => Some(FDCANSEL_A::Pll2Q),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL_A::Hse
    }
    ///Checks if the value of the field is `Pll1Q`
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL_A::Pll1Q
    }
    ///Checks if the value of the field is `Pll2Q`
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == FDCANSEL_A::Pll2Q
    }
}
///Field `FDCANSEL` writer - FDCAN kernel clock source selection
pub type FDCANSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D2CCIP1R_SPEC, u8, FDCANSEL_A, 2, O>;
impl<'a, const O: u8> FDCANSEL_W<'a, O> {
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Hse)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Pll1Q)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Pll2Q)
    }
}
///Field `SWPSEL` reader - SWPMI kernel clock source selection
pub type SWPSEL_R = crate::BitReader<SWPSEL_A>;
///SWPMI kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWPSEL_A {
    ///0: pclk selected as peripheral clock
    Pclk = 0,
    ///1: hsi_ker selected as peripheral clock
    HsiKer = 1,
}
impl From<SWPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SWPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SWPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWPSEL_A {
        match self.bits {
            false => SWPSEL_A::Pclk,
            true => SWPSEL_A::HsiKer,
        }
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == SWPSEL_A::Pclk
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SWPSEL_A::HsiKer
    }
}
///Field `SWPSEL` writer - SWPMI kernel clock source selection
pub type SWPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, D2CCIP1R_SPEC, SWPSEL_A, O>;
impl<'a, const O: u8> SWPSEL_W<'a, O> {
    ///pclk selected as peripheral clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(SWPSEL_A::Pclk)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SWPSEL_A::HsiKer)
    }
}
impl R {
    ///Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 6:8 - SAI2 and SAI3 kernel clock source selection
    #[inline(always)]
    pub fn sai23sel(&self) -> SAI23SEL_R {
        SAI23SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection
    #[inline(always)]
    pub fn spi123sel(&self) -> SPI123SEL_R {
        SPI123SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - SPI4 and 5 kernel clock source selection
    #[inline(always)]
    pub fn spi45sel(&self) -> SPI45SEL_R {
        SPI45SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:21 - SPDIFRX kernel clock source selection
    #[inline(always)]
    pub fn spdifsel(&self) -> SPDIFSEL_R {
        SPDIFSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 24 - DFSDM1 kernel Clk clock source selection
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 28:29 - FDCAN kernel clock source selection
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - SWPMI kernel clock source selection
    #[inline(always)]
    pub fn swpsel(&self) -> SWPSEL_R {
        SWPSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<0> {
        SAI1SEL_W::new(self)
    }
    ///Bits 6:8 - SAI2 and SAI3 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn sai23sel(&mut self) -> SAI23SEL_W<6> {
        SAI23SEL_W::new(self)
    }
    ///Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn spi123sel(&mut self) -> SPI123SEL_W<12> {
        SPI123SEL_W::new(self)
    }
    ///Bits 16:18 - SPI4 and 5 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn spi45sel(&mut self) -> SPI45SEL_W<16> {
        SPI45SEL_W::new(self)
    }
    ///Bits 20:21 - SPDIFRX kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn spdifsel(&mut self) -> SPDIFSEL_W<20> {
        SPDIFSEL_W::new(self)
    }
    ///Bit 24 - DFSDM1 kernel Clk clock source selection
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W<24> {
        DFSDM1SEL_W::new(self)
    }
    ///Bits 28:29 - FDCAN kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<28> {
        FDCANSEL_W::new(self)
    }
    ///Bit 31 - SWPMI kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn swpsel(&mut self) -> SWPSEL_W<31> {
        SWPSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC Domain 2 Kernel Clock Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [d2ccip1r](index.html) module
pub struct D2CCIP1R_SPEC;
impl crate::RegisterSpec for D2CCIP1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [d2ccip1r::R](R) reader structure
impl crate::Readable for D2CCIP1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [d2ccip1r::W](W) writer structure
impl crate::Writable for D2CCIP1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets D2CCIP1R to value 0
impl crate::Resettable for D2CCIP1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
