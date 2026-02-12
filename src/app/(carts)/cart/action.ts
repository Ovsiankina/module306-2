"use server";

import { revalidatePath } from "next/cache";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { Session } from "next-auth";
import { prisma } from "@/libs/db";
import { EnrichedProducts, VariantsDocument } from "@/types/types";

export type CartItem = {
  productId: string;
  size: string;
  variantId: string;
  quantity: number;
  price: number;
};

export type Cart = {
  userId: string;
  items: CartItem[];
};

// Helper function to transform Prisma product
function transformProduct(product: any) {
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

// Helper to get cart from database
async function getCartFromDb(userId: string): Promise<Cart | null> {
  const cartRecord = await prisma.cart.findUnique({
    where: { userId },
  });
  if (!cartRecord) return null;
  return {
    userId,
    items: JSON.parse(cartRecord.items),
  };
}

// Helper to save cart to database
async function saveCartToDb(userId: string, items: CartItem[]): Promise<void> {
  await prisma.cart.upsert({
    where: { userId },
    update: { items: JSON.stringify(items) },
    create: { userId, items: JSON.stringify(items) },
  });
}

export async function getItems(userId: string) {
  if (!userId) {
    console.error(`User Id not found.`);
    return undefined;
  }

  const cart = await getCartFromDb(userId);

  if (cart === null) {
    return undefined;
  }

  const updatedCart: EnrichedProducts[] = [];
  for (const cartItem of cart.items) {
    try {
      if (cartItem.productId && cartItem.variantId) {
        const matchingProduct = await prisma.product.findUnique({
          where: { id: cartItem.productId },
          include: { variants: true },
        });

        if (!matchingProduct) {
          console.error(
            `Product not found for productId: ${cartItem.productId}`,
          );
          continue;
        } else {
          const transformed = transformProduct(matchingProduct);
          const matchingVariant = transformed.variants.find(
            (variant: VariantsDocument) =>
              variant.priceId === cartItem.variantId,
          );
          const updatedCartItem: EnrichedProducts = {
            ...cartItem,
            color: matchingVariant?.color || "",
            category: matchingProduct.category,
            image: matchingVariant ? [matchingVariant.images[0]] : [],
            name: matchingProduct.name,
            purchased: false,
            _id: matchingProduct.id,
          };

          updatedCart.push(updatedCartItem);
        }
      }
    } catch (error) {
      console.error("Error getting product details:", error);
    }
  }

  const filteredCart = updatedCart.filter((item) => item !== null);

  return filteredCart;
}

export async function getTotalItems(session: Session | null) {
  if (!session?.user._id) return 0;

  const cart = await getCartFromDb(session.user._id);
  const total: number =
    cart?.items.reduce((sum, item) => sum + item.quantity, 0) || 0;

  return total;
}

export async function addItem(
  category: string,
  productId: string,
  size: string,
  variantId: string,
  price: number,
) {
  const session: Session | null = await getServerSession(authOptions);

  if (!session?.user._id) {
    console.error(`User Id not found.`);
    return;
  }

  const userId = session.user._id;
  const cart = await getCartFromDb(userId);

  let items: CartItem[] = [];

  if (!cart || !cart.items) {
    items = [
      {
        productId: productId,
        size: size,
        variantId: variantId,
        quantity: 1,
        price: price,
      },
    ];
  } else {
    let itemFound = false;

    items = cart.items.map((item) => {
      if (
        item.productId === productId &&
        item.variantId === variantId &&
        item.size === size
      ) {
        itemFound = true;
        item.quantity += 1;
      }
      return item;
    });

    if (!itemFound) {
      items.push({
        productId: productId,
        size: size,
        variantId: variantId,
        quantity: 1,
        price: price,
      });
    }
  }

  await saveCartToDb(userId, items);
  revalidatePath(`/${category}/${productId}`);
}

export async function delItem(
  productId: string,
  size: string,
  variantId: string,
) {
  const session: Session | null = await getServerSession(authOptions);
  const userId = session?.user._id;

  if (!userId) return;

  const cart = await getCartFromDb(userId);

  if (cart && cart.items) {
    const updatedItems = cart.items.filter(
      (item) =>
        !(
          item.productId === productId &&
          item.variantId === variantId &&
          item.size === size
        ),
    );

    await saveCartToDb(userId, updatedItems);
    revalidatePath("/cart");
  }
}

export async function delOneItem(
  productId: string,
  size: string,
  variantId: string,
) {
  try {
    const session: Session | null = await getServerSession(authOptions);
    const userId = session?.user._id;

    if (!userId) return;

    const cart = await getCartFromDb(userId);

    if (cart && cart.items) {
      const updatedItems = cart.items
        .map((item) => {
          if (
            item.productId === productId &&
            item.variantId === variantId &&
            item.size === size
          ) {
            if (item.quantity > 1) {
              item.quantity -= 1;
            } else {
              return null;
            }
          }
          return item;
        })
        .filter(Boolean) as CartItem[];

      await saveCartToDb(userId, updatedItems);
      revalidatePath("/cart");
    }
  } catch (error) {
    console.error("Error in delOneItem:", error);
  }
}

export const emptyCart = async (userId: string) => {
  try {
    const cart = await getCartFromDb(userId);

    if (cart && cart.items) {
      await saveCartToDb(userId, []);
      revalidatePath("/cart");
      console.log("Cart emptied successfully.");
    } else {
      console.log("Cart is already empty.");
    }
  } catch (error) {
    console.error("Error emptying cart:", error);
  }
};
