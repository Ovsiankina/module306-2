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
    console.error("Error fetching products:", error);
    return [];
  }
}

/**
 * Fetch products by category with caching
 * Each category has its own cache entry (category arg becomes part of cache key)
 */
export async function getCategoryProducts(
  category: ProductCategory,
): Promise<ProductWithVariants[]> {
  "use cache";
  cacheTag("products", `category-${category}`);
  cacheLife("hours");

  try {
    const product = await prisma.product.findUnique({
      where: { id: _id },
      include: { variants: true },
    });
    if (!product) return null;
    return transformProduct(product);
  } catch (error) {
    console.error("Error fetching category products:", error);
    return [];
  }
}

/**
 * Fetch a single product by ID with caching
 * Each product has its own cache entry (productId arg becomes part of cache key)
 */
export async function getProduct(
  productId: number,
): Promise<ProductWithVariants | null> {
  "use cache";
  cacheTag("products", `product-${productId}`);
  cacheLife("hours");

  try {
    const product = await productsRepository.findById(productId);
    if (!product) return null;
    return productWithVariantsSchema.parse(product);
  } catch (error) {
    console.error("Error fetching product:", error);
    return null;
  }
}

/**
 * Fetch random products excluding a specific product
 * Note: This is dynamic (random) so it stays outside cache
 * It benefits from the cached getAllProducts() call
 */
export async function getRandomProducts(
  productIdToExclude: number,
): Promise<ProductWithVariants[]> {
  try {
    const allProducts = await getAllProducts();
    const filtered = allProducts.filter((p) => p.id !== productIdToExclude);
    const shuffled = filtered.sort(() => Math.random() - 0.5);
    return productWithVariantsSchema.array().parse(shuffled.slice(0, 6));
  } catch (error) {
    console.error("Error fetching random products:", error);
    return [];
  }
}

/**
 * Invalidates all product caches immediately
 * Call this after creating, updating, or deleting products
 * Uses updateTag for read-your-own-writes semantics (user sees changes immediately)
 */
export async function revalidateProducts(productId?: number): Promise<void> {
  // Always invalidate the general products tag
  updateTag("products");

  // If a specific product ID is provided, also invalidate that specific product
  if (productId) {
    updateTag(`product-${productId}`);
  }
}
