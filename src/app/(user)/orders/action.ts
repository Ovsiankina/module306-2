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
import { getUser } from "@/lib/auth/server";
import {
  orderWithDetailsSchema,
  insertOrderItemSchema,
  insertCustomerInfoSchema,
  insertOrderProductSchema,
} from "@/lib/db/drizzle/schema";
import type {
  OrderItem,
  CustomerInfo,
  OrderProduct,
  OrderWithDetails,
  MinimalCartItem,
} from "@/lib/db/drizzle/schema";

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
    console.error("Unexpected error fetching orders:", error);
    if (error instanceof Error) {
      console.error("Error stack:", error.stack);
    }
    return null;
  }
};

export const getOrder = async (
  orderId: OrderItem["id"],
): Promise<OrderWithDetails | null> => {
  try {
    const user = await getUser();
    const userId = user?.id;

    if (!userId) {
      console.info("No user found when fetching order");
      return null;
    }

    const order = await ordersRepository.findById(orderId);

    if (!order || order.userId !== userId) {
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
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("Error fetching order:", errorMessage);
    return null;
  }
};

/**
 * Create order item in database
 */
export async function createOrderItem(
  userId: string,
  orderNumber: number,
): Promise<OrderItem> {
  const deliveryDate = new Date();
  deliveryDate.setDate(deliveryDate.getDate() + 7);

  const orderItemToSave = insertOrderItemSchema.parse({
    userId,
    deliveryDate,
    orderNumber,
  });

  const order = await ordersRepository.create(orderItemToSave);

  if (!order) {
    throw new Error("Error creating order");
  }

  return order;
}

/**
 * Save customer info from Stripe session
 */
export async function saveCustomerInfo(
  orderId: number,
  session: Stripe.Checkout.Session,
): Promise<CustomerInfo | null> {
  const customerInfoToSave = insertCustomerInfoSchema.parse({
    orderId,
    name: session.customer_details?.name || "Unknown",
    email: session.customer_details?.email || "unknown@email.com",
    phone: session.customer_details?.phone || undefined,
    address: {
      line1: session.customer_details?.address?.line1 || "",
      line2: session.customer_details?.address?.line2,
      city: session.customer_details?.address?.city || "",
      state: session.customer_details?.address?.state,
      postal_code: session.customer_details?.address?.postal_code || "",
      country: session.customer_details?.address?.country || "",
    },
    stripeOrderId: session.id,
    totalPrice: session.amount_total || 0,
  });

  const customerInfo = await ordersRepository.addCustomerInfo(
    orderId,
    customerInfoToSave,
  );

  if (!customerInfo) {
    throw new Error("Error saving customer info");
  }

  return {
    id: customerInfo.id,
    orderId: customerInfo.orderId,
    name: customerInfo.name,
    email: customerInfo.email,
    phone: customerInfo.phone,
    address: customerInfo.address,
    stripeOrderId: customerInfo.stripeOrderId,
    totalPrice: customerInfo.totalPrice,
    createdAt:
      customerInfo.createdAt?.toISOString() ?? new Date().toISOString(),
    updatedAt:
      customerInfo.updatedAt?.toISOString() ?? new Date().toISOString(),
  };
}

export async function saveOrderProducts(
  orderId: number,
  lineItems: Stripe.LineItem[],
  cartItems: MinimalCartItem[],
): Promise<OrderProduct[]> {
  const orderProductsData = lineItems
    .map((lineItem) => {
      const cartItem = cartItems.find(
        (item) => item.stripeId === lineItem.price?.id,
      );

      if (!cartItem) {
        console.warn(`No cart item found for price ID: ${lineItem.price?.id}`);
        return null;
      }

      return insertOrderProductSchema.parse({
        orderId,
        variantId: cartItem.variantId,
        quantity: lineItem.quantity || 1,
        size: cartItem.size,
      });
    })
    .filter((item): item is NonNullable<typeof item> => item !== null);

  if (orderProductsData.length === 0) {
    throw new Error("No valid order products to save");
  }

  const savedProducts = await ordersRepository.addProducts(
    orderId,
    orderProductsData,
  );

  return savedProducts.map((p) => ({
    id: p.id,
    orderId: p.orderId,
    variantId: p.variantId,
    quantity: p.quantity,
    size: p.size as "XS" | "S" | "M" | "L" | "XL" | "XXL",
    createdAt: p.createdAt?.toISOString() ?? new Date().toISOString(),
    updatedAt: p.updatedAt?.toISOString() ?? new Date().toISOString(),
  }));
}
