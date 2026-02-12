"use server";

import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { Session } from "next-auth";
import { revalidatePath } from "next/cache";
import { prisma } from "@/libs/db";

export type WishlistItem = {
  productId: string;
};

export type Wishlists = {
  userId: string;
  items: WishlistItem[];
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

// Helper to get wishlist from database
async function getWishlistFromDb(userId: string): Promise<Wishlists | null> {
  const wishlistRecord = await prisma.wishlist.findUnique({
    where: { userId },
  });
  if (!wishlistRecord) return null;
  return {
    userId,
    items: JSON.parse(wishlistRecord.items),
  };
}

// Helper to save wishlist to database
async function saveWishlistToDb(userId: string, items: WishlistItem[]): Promise<void> {
  await prisma.wishlist.upsert({
    where: { userId },
    update: { items: JSON.stringify(items) },
    create: { userId, items: JSON.stringify(items) },
  });
}

export async function addItem(productId: string) {
  const session: Session | null = await getServerSession(authOptions);

  if (!session?.user._id) {
    console.error(`User Id not found.`);
    return;
  }

  const userId = session.user._id;
  const wishlists = await getWishlistFromDb(userId);

  let items: WishlistItem[] = [];

  if (!wishlists || !wishlists.items) {
    items = [{ productId }];
  } else {
    let itemFound = false;

    items = wishlists.items.map((item) => {
      if (item.productId === productId) {
        itemFound = true;
      }
      return item;
    });

    if (!itemFound) {
      items.push({ productId });
    }
  }

  await saveWishlistToDb(userId, items);
  revalidatePath("/wishlist");
}

export async function getItems(userId: string) {
  if (!userId) {
    console.error(`User Id not found.`);
    return null;
  }

  const wishlist = await getWishlistFromDb(userId);

  if (wishlist === null) {
    console.error("wishlist not found.");
    return null;
  }

  const updatedWishlist = [];
  for (const wishlistItem of wishlist.items) {
    try {
      if (wishlistItem.productId) {
        const matchingProduct = await prisma.product.findUnique({
          where: { id: wishlistItem.productId },
          include: { variants: true },
        });

        if (!matchingProduct) {
          console.error(
            `Product not found for productId: ${wishlistItem.productId}`,
          );
          continue;
        } else {
          updatedWishlist.push(transformProduct(matchingProduct));
        }
      }
    } catch (error) {
      console.error("Error getting product details:", error);
    }
  }

  const filteredWishlist = updatedWishlist.filter((item) => item !== null);

  return filteredWishlist;
}

export async function getTotalWishlist() {
  const session: Session | null = await getServerSession(authOptions);

  if (!session?.user._id) return undefined;

  const wishlists = await getWishlistFromDb(session.user._id);

  if (wishlists === null) {
    return undefined;
  }

  return wishlists;
}

export async function delItem(productId: string) {
  const session: Session | null = await getServerSession(authOptions);
  const userId = session?.user._id;

  if (!userId) {
    console.error("User not found.");
    return;
  }

  const wishlists = await getWishlistFromDb(userId);

  if (wishlists && wishlists.items) {
    const updatedItems = wishlists.items.filter(
      (item) => item.productId !== productId
    );

    await saveWishlistToDb(userId, updatedItems);
    revalidatePath("/wishlist");
  }
}
