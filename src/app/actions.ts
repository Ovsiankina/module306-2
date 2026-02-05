"use server";

import { prisma } from "@/libs/db";
import { EnrichedProducts, VariantsDocument } from "@/types/types";

// Helper function to transform Prisma product to EnrichedProducts format
function transformProduct(product: any): EnrichedProducts {
  return {
    ...product,
    _id: product.id,
    sizes: JSON.parse(product.sizes),
    image: JSON.parse(product.image),
    variants: product.variants.map((v: any) => ({
      ...v,
      images: JSON.parse(v.images),
    })),
  };
}

export const getAllProducts = async () => {
  try {
    const products = await prisma.product.findMany({
      include: { variants: true },
    });
    return products.map(transformProduct);
  } catch (error) {
    console.error("Error getting products:", error);
    throw new Error("Failed to fetch category products");
  }
};

export const getCategoryProducts = async (category: string) => {
  try {
    const products = await prisma.product.findMany({
      where: { category },
      include: { variants: true },
    });
    return products.map(transformProduct);
  } catch (error) {
    console.error("Error getting products:", error);
    throw new Error("Failed to fetch category products");
  }
};

export const getRandomProducts = async (productId: string) => {
  const shuffleArray = (array: EnrichedProducts[]) => {
    let shuffled = array.slice();
    for (let i = shuffled.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
    }
    return shuffled;
  };

  try {
    const allProducts = await prisma.product.findMany({
      include: { variants: true },
    });
    const transformedProducts = allProducts.map(transformProduct);
    const shuffledProducts = shuffleArray(transformedProducts);
    const randomProducts = shuffledProducts
      .filter((product) => product._id !== productId)
      .slice(0, 6);
    return randomProducts;
  } catch (error) {
    console.error("Error getting products:", error);
    throw new Error("Failed to fetch random products");
  }
};

export const getProduct = async (_id: string) => {
  try {
    const product = await prisma.product.findUnique({
      where: { id: _id },
      include: { variants: true },
    });
    if (!product) return null;
    return transformProduct(product);
  } catch (error) {
    console.error("Error getting product:", error);
  }
};
