import { NextResponse } from "next/server";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { prisma } from "@/libs/db";

export async function GET() {
  const session = await getServerSession(authOptions);

  if (!session?.user?.isAdmin) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  const parkings = await prisma.parking.findMany({
    orderBy: { name: "asc" },
  });

  return NextResponse.json(parkings);
}

export async function PUT(request: Request) {
  const session = await getServerSession(authOptions);

  if (!session?.user?.isAdmin) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  try {
    const parkings = await request.json();

    for (const parking of parkings) {
      await prisma.parking.update({
        where: { id: parking.id },
        data: {
          availableSpaces: parking.availableSpaces,
          totalSpaces: parking.totalSpaces,
          isOpen: parking.isOpen,
        },
      });
    }

    return NextResponse.json({ success: true });
  } catch (error) {
    console.error("Error updating parking:", error);
    return NextResponse.json({ error: "Failed to update" }, { status: 500 });
  }
}
