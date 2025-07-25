#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCPC` reader - Capture/compare preloaded control"]
pub type CcpcR = crate::BitReader;
#[doc = "Field `CCPC` writer - Capture/compare preloaded control"]
pub type CcpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - Capture/compare control update selection"]
pub type CcusR = crate::BitReader;
#[doc = "Field `CCUS` writer - Capture/compare control update selection"]
pub type CcusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1` reader - Output Idle state 1"]
pub type Ois1R = crate::BitReader;
#[doc = "Field `OIS1` writer - Output Idle state 1"]
pub type Ois1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - Output Idle state 1"]
pub type Ois1nR = crate::BitReader;
#[doc = "Field `OIS1N` writer - Output Idle state 1"]
pub type Ois1nW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&self) -> CcpcR {
        CcpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&self) -> CcusR {
        CcusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CcpcW<'_, Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CcusW<'_, Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&mut self) -> Ois1W<'_, Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> Ois1nW<'_, Cr2Spec> {
        Ois1nW::new(self, 9)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
