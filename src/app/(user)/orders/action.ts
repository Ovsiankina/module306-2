"use server";

import { prisma } from "@/libs/db";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { Session } from "next-auth";
import {
  EnrichedProducts,
  OrderDocument,
  ProductsDocument,
  VariantsDocument,
} from "@/types/types";
import {
  calculateExpectedDeliveryDate,
  generateRandomOrderNumber,
} from "@/helpers/orderModel";
import Stripe from "stripe";
import { emptyCart, getItems } from "@/app/(carts)/cart/action";

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

export const getUserOrders = async () => {
  try {
    const session: Session | null = await getServerSession(authOptions);
    const userId = session?.user._id;

    if (!userId) return null;

    const userOrders = await prisma.userOrders.findUnique({
      where: { userId },
      include: { orders: true },
    });

    if (userOrders && userOrders.orders && userOrders.orders.length > 0) {
      // Transform orders and parse JSON fields
      const transformedOrders = userOrders.orders.map((order) => ({
        ...order,
        _id: order.id,
        address: JSON.parse(order.address),
        products: JSON.parse(order.products),
        total_price: order.totalPrice,
      }));

      // Sort by purchase date (newest first)
      transformedOrders.sort((a, b) => {
        const dateA = new Date(a.purchaseDate);
        const dateB = new Date(b.purchaseDate);
        return dateB.getTime() - dateA.getTime();
      });

      return {
        ...userOrders,
        orders: transformedOrders,
      };
    }

    return userOrders;
  } catch (error) {
    console.error("Error getting orders:", error);
  }
};

export const getOrder = async (orderId: string) => {
  try {
    const session: Session | null = await getServerSession(authOptions);
    const userId = session?.user._id;

    if (!userId) return null;

    const userOrders = await prisma.userOrders.findUnique({
      where: { userId },
      include: { orders: true },
    });

    const orderFound = userOrders?.orders.find(
      (order) => order.id === orderId
    );

    if (!orderFound) {
      console.log("Order not found");
      return null;
    }

    const products: ProductsDocument[] = JSON.parse(orderFound.products);
    const address = JSON.parse(orderFound.address);

    const enrichedProducts = await Promise.all(
      products.map(async (product: ProductsDocument) => {
        const matchingProduct = await prisma.product.findUnique({
          where: { id: product.productId },
          include: { variants: true },
        });

        if (matchingProduct) {
          const transformed = transformProduct(matchingProduct);
          const matchingVariant = transformed.variants.find(
            (variant: VariantsDocument) => variant.color === product.color
          );
          if (matchingVariant) {
            return {
              productId: matchingProduct.id,
              name: matchingProduct.name,
              category: matchingProduct.category,
              image: [matchingVariant.images[0]],
              price: matchingProduct.price,
              purchased: true,
              color: product.color,
              size: product.size,
              quantity: product.quantity,
            };
          }
        }
        return null;
      })
    );

    const filteredEnrichedProducts = enrichedProducts.filter(
      (product) => product !== null
    );

    const enrichedOrder = {
      name: orderFound.name,
      email: orderFound.email,
      phone: orderFound.phone,
      address: address,
      products: filteredEnrichedProducts,
      orderId: orderFound.orderId,
      purchaseDate: orderFound.purchaseDate,
      expectedDeliveryDate: orderFound.expectedDeliveryDate,
      total_price: orderFound.totalPrice,
      orderNumber: orderFound.orderNumber,
      _id: orderFound.id,
    };

    return enrichedOrder;
  } catch (error) {
    console.error("Error getting order:", error);
    return null;
  }
};

export const saveOrder = async (data: Stripe.Checkout.Session) => {
  try {
    const userId = data.metadata?.userId;
    if (!userId || !data) {
      console.error("Missing information.");
      return null;
    }

    const cart = await getItems(userId);
    if (!cart) {
      console.error("Products or cart not found.");
      return null;
    }

    const products = cart.map((item) => ({
      productId: item.productId,
      quantity: item.quantity,
      size: item.size,
      color: item.color,
      image: item.image,
    }));

    const newOrderData = {
      name: data.customer_details?.name || "",
      email: data.customer_details?.email || "",
      phone: data.customer_details?.phone || null,
      address: JSON.stringify({
        line1: data.customer_details?.address?.line1,
        line2: data.customer_details?.address?.line2,
        city: data.customer_details?.address?.city,
        state: data.customer_details?.address?.state,
        postal_code: data.customer_details?.address?.postal_code,
        country: data.customer_details?.address?.country,
      }),
      products: JSON.stringify(products),
      orderId: data.id,
      totalPrice: data.amount_total || 0,
      expectedDeliveryDate: calculateExpectedDeliveryDate(),
      orderNumber: generateRandomOrderNumber(),
    };

    const userOrders = await prisma.userOrders.findUnique({
      where: { userId },
      include: { orders: true },
    });

    if (userOrders) {
      const orderIdMatch = userOrders.orders.some(
        (order) => order.orderId === data.id
      );
      if (!orderIdMatch) {
        await prisma.order.create({
          data: {
            ...newOrderData,
            userOrdersId: userOrders.id,
          },
        });
        console.log("Order successfully updated.");
      } else {
        console.info("This order has already been saved.");
      }
    } else {
      const newUserOrders = await prisma.userOrders.create({
        data: { userId },
      });
      await prisma.order.create({
        data: {
          ...newOrderData,
          userOrdersId: newUserOrders.id,
        },
      });
      console.info("New order document created and saved successfully.");
    }

    await emptyCart(userId);
  } catch (error) {
    console.error("Error saving the order:", error);
  }
};
