import { prisma } from "@/libs/db";
import { NextResponse } from "next/server";
import bcrypt from "bcryptjs";

export async function POST(request: Request) {
  try {
    const { name, email, password, phone } = await request.json();

    if (password.length < 6) {
      return NextResponse.json(
        { message: "Password must be at least 6 characters" },
        { status: 400 },
      );
    }

    const userFound = await prisma.user.findUnique({
      where: { email },
    });

    if (userFound) {
      return NextResponse.json(
        { message: "Email already exists" },
        { status: 409 },
      );
    }

    const hashedPassword = await bcrypt.hash(password, 12);

    const savedUser = await prisma.user.create({
      data: {
        name,
        email,
        phone: phone || "",
        password: hashedPassword,
      },
    });

    console.log(savedUser);

    return NextResponse.json(
      {
        name: savedUser.name,
        email: savedUser.email,
        createdAt: savedUser.createdAt,
        updatedAt: savedUser.updatedAt,
      },
      { status: 201 },
    );
  } catch (error: any) {
    console.error("Error during signup:", error);
    return NextResponse.json({ message: error.message || "Server error" }, { status: 400 });
  }
}

export async function PUT(request: Request) {
  try {
    const { userId, name, email, password, phone, address } =
      await request.json();

    if (password && password.length < 6) {
      return NextResponse.json(
        { message: "Password must be at least 6 characters" },
        { status: 400 },
      );
    }

    const userToUpdate = await prisma.user.findUnique({
      where: { id: userId },
    });

    if (!userToUpdate) {
      return NextResponse.json({ message: "User not found" }, { status: 404 });
    }

    const updateData: any = {};
    if (name) updateData.name = name;
    if (email) updateData.email = email;
    if (phone) updateData.phone = phone;
    if (password) {
      const hashedPassword = await bcrypt.hash(password, 12);
      updateData.password = hashedPassword;
    }

    const updatedUser = await prisma.user.update({
      where: { id: userId },
      data: updateData,
    });

    console.log(updatedUser);

    return NextResponse.json(
      {
        message: "User updated successfully",
        updatedUser: {
          id: updatedUser.id,
          name: updatedUser.name,
          email: updatedUser.email,
          createdAt: updatedUser.createdAt,
          updatedAt: updatedUser.updatedAt,
        },
      },
      { status: 200 },
    );
  } catch (error: any) {
    console.error("Error during user update:", error);
    return NextResponse.json({ message: error.message || "Server error" }, { status: 400 });
  }
}

export async function DELETE(request: Request) {
  try {
    const { userId } = await request.json();

    const user = await prisma.user.findUnique({
      where: { id: userId },
    });

    if (!user) {
      return NextResponse.json({ message: "User not found" }, { status: 404 });
    }

    await prisma.user.delete({
      where: { id: userId },
    });

    return NextResponse.json(
      { message: "User deleted successfully" },
      { status: 200 },
    );
  } catch (error) {
    console.error("Error during user/cart item deletion:", error);
    return NextResponse.error();
  }
}
