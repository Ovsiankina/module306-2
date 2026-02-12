"use server";

import { prisma } from "@/libs/db";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";

// ============== SHOPS ==============

export async function getAllShops() {
  try {
    const shops = await prisma.shop.findMany({
      where: { isActive: true },
      orderBy: { name: "asc" },
    });
    return shops;
  } catch (error) {
    console.error("Error getting shops:", error);
    return [];
  }
}

export async function getShopsByCategory(category: string) {
  try {
    const shops = await prisma.shop.findMany({
      where: { category, isActive: true },
      orderBy: { name: "asc" },
    });
    return shops;
  } catch (error) {
    console.error("Error getting shops:", error);
    return [];
  }
}

export async function getShopById(id: string) {
  try {
    const shop = await prisma.shop.findUnique({
      where: { id },
    });
    return shop;
  } catch (error) {
    console.error("Error getting shop:", error);
    return null;
  }
}

export async function getShopCategories() {
  try {
    const shops = await prisma.shop.findMany({
      where: { isActive: true },
      select: { category: true },
      distinct: ["category"],
    });
    return shops.map((s) => s.category);
  } catch (error) {
    console.error("Error getting categories:", error);
    return [];
  }
}

// ============== PARKING ==============

export async function getParkingStatus() {
  try {
    const parkings = await prisma.parking.findMany({
      orderBy: { name: "asc" },
    });
    return parkings;
  } catch (error) {
    console.error("Error getting parking status:", error);
    return [];
  }
}

// ============== MALL INFO ==============

export async function getMallInfo() {
  try {
    const info = await prisma.mallInfo.findFirst();
    return info;
  } catch (error) {
    console.error("Error getting mall info:", error);
    return null;
  }
}

// ============== GAME ==============

export async function canPlayGame() {
  const session = await getServerSession(authOptions);

  if (!session?.user?._id) {
    return { canPlay: false, reason: "not_logged_in", attemptsToday: 0 };
  }

  const userId = session.user._id;
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const tomorrow = new Date(today);
  tomorrow.setDate(tomorrow.getDate() + 1);

  // Check user's plays today
  const playsToday = await prisma.gamePlay.findMany({
    where: {
      userId,
      playDate: {
        gte: today,
        lt: tomorrow,
      },
    },
    orderBy: { playDate: "desc" },
  });

  const attemptsToday = playsToday.length;

  // User can play if:
  // - No plays today (first attempt)
  // - One play today and it was a loss (second chance)
  if (attemptsToday === 0) {
    return { canPlay: true, reason: "first_attempt", attemptsToday: 0 };
  }

  if (attemptsToday === 1 && !playsToday[0].won) {
    return { canPlay: true, reason: "second_chance", attemptsToday: 1 };
  }

  if (playsToday[0].won) {
    return { canPlay: false, reason: "already_won", attemptsToday };
  }

  return { canPlay: false, reason: "max_attempts", attemptsToday };
}

export async function playGame() {
  const session = await getServerSession(authOptions);

  if (!session?.user?._id) {
    return { success: false, error: "not_logged_in" };
  }

  const canPlayResult = await canPlayGame();
  if (!canPlayResult.canPlay) {
    return { success: false, error: canPlayResult.reason };
  }

  const userId = session.user._id;
  const today = new Date();
  today.setHours(0, 0, 0, 0);

  // Check daily prize pool
  let prizePool = await prisma.dailyPrizePool.findUnique({
    where: { date: today },
  });

  if (!prizePool) {
    prizePool = await prisma.dailyPrizePool.create({
      data: { date: today, totalPrizes: 10, prizesWon: 0 },
    });
  }

  const prizesRemaining = prizePool.totalPrizes - prizePool.prizesWon;

  // Win probability: 20% base chance, but only if prizes remain
  const winProbability = prizesRemaining > 0 ? 0.2 : 0;
  const won = Math.random() < winProbability;

  let voucher = null;

  if (won) {
    // Find an available voucher
    const availableVoucher = await prisma.voucher.findFirst({
      where: {
        wonByUserId: null,
        expiresAt: { gt: new Date() },
      },
      include: { shop: true },
    });

    if (availableVoucher) {
      // Award the voucher
      voucher = await prisma.voucher.update({
        where: { id: availableVoucher.id },
        data: {
          wonByUserId: userId,
          wonAt: new Date(),
        },
        include: { shop: true },
      });

      // Update prize pool
      await prisma.dailyPrizePool.update({
        where: { id: prizePool.id },
        data: { prizesWon: prizePool.prizesWon + 1 },
      });
    }
  }

  // Record the play
  await prisma.gamePlay.create({
    data: {
      userId,
      attempt: canPlayResult.attemptsToday + 1,
      won: !!voucher,
      prizeId: voucher?.id,
    },
  });

  return {
    success: true,
    won: !!voucher,
    voucher: voucher
      ? {
          code: voucher.code,
          value: voucher.value,
          shopName: voucher.shop.name,
          description: voucher.description,
        }
      : null,
    canPlayAgain: !voucher && canPlayResult.attemptsToday === 0,
  };
}

// ============== VISITOR TRACKING ==============

export async function trackVisitor(sessionId: string, userAgent?: string, referrer?: string) {
  try {
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    // Update or create visitor record
    await prisma.visitor.upsert({
      where: { sessionId },
      update: { pageViews: { increment: 1 } },
      create: {
        sessionId,
        userAgent: userAgent || "",
        referrer: referrer || "",
      },
    });

    // Update daily stats
    await prisma.visitorStats.upsert({
      where: { date: today },
      update: {
        totalPageViews: { increment: 1 },
      },
      create: {
        date: today,
        uniqueVisitors: 1,
        totalPageViews: 1,
      },
    });
  } catch (error) {
    console.error("Error tracking visitor:", error);
  }
}

export async function getVisitorStats(period: "day" | "month" | "year") {
  try {
    const now = new Date();
    let startDate: Date;

    switch (period) {
      case "day":
        startDate = new Date(now);
        startDate.setHours(0, 0, 0, 0);
        break;
      case "month":
        startDate = new Date(now.getFullYear(), now.getMonth(), 1);
        break;
      case "year":
        startDate = new Date(now.getFullYear(), 0, 1);
        break;
    }

    const stats = await prisma.visitorStats.findMany({
      where: {
        date: { gte: startDate },
      },
      orderBy: { date: "asc" },
    });

    const totalVisitors = stats.reduce((sum, s) => sum + s.uniqueVisitors, 0);
    const totalPageViews = stats.reduce((sum, s) => sum + s.totalPageViews, 0);

    return {
      period,
      totalVisitors,
      totalPageViews,
      stats,
    };
  } catch (error) {
    console.error("Error getting visitor stats:", error);
    return { period, totalVisitors: 0, totalPageViews: 0, stats: [] };
  }
}
