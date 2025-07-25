#[doc = "Register `CMAR5` reader"]
pub type R = crate::R<Cmar5Spec>;
#[doc = "Register `CMAR5` writer"]
pub type W = crate::W<Cmar5Spec>;
#[doc = "Field `MA` reader - Memory address"]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<'_, Cmar5Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA channel 5 memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmar5Spec;
impl crate::RegisterSpec for Cmar5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar5::R`](R) reader structure"]
impl crate::Readable for Cmar5Spec {}
#[doc = "`write(|w| ..)` method takes [`cmar5::W`](W) writer structure"]
impl crate::Writable for Cmar5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMAR5 to value 0"]
impl crate::Resettable for Cmar5Spec {}
