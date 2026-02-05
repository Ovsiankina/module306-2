import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

async function main() {
  // Clear existing data
  await prisma.productVariant.deleteMany();
  await prisma.product.deleteMany();

  // Sample products
  const products = [
    {
      name: "Classic White T-Shirt",
      description: "A comfortable classic white t-shirt made from 100% organic cotton. Perfect for everyday wear.",
      price: 29.99,
      category: "t-shirts",
      sizes: JSON.stringify(["XS", "S", "M", "L", "XL"]),
      image: JSON.stringify(["samples/tshirt-white-1.jpg"]),
      variants: [
        { priceId: "price_tshirt_white", color: "white", images: JSON.stringify(["samples/tshirt-white-1.jpg", "samples/tshirt-white-2.jpg"]) },
        { priceId: "price_tshirt_black", color: "black", images: JSON.stringify(["samples/tshirt-black-1.jpg"]) },
      ],
    },
    {
      name: "Slim Fit Jeans",
      description: "Modern slim fit jeans with stretch denim for maximum comfort. A wardrobe essential.",
      price: 79.99,
      category: "pants",
      sizes: JSON.stringify(["28", "30", "32", "34", "36"]),
      image: JSON.stringify(["samples/jeans-blue-1.jpg"]),
      variants: [
        { priceId: "price_jeans_blue", color: "blue", images: JSON.stringify(["samples/jeans-blue-1.jpg", "samples/jeans-blue-2.jpg"]) },
        { priceId: "price_jeans_black", color: "black", images: JSON.stringify(["samples/jeans-black-1.jpg"]) },
      ],
    },
    {
      name: "Wool Blend Sweater",
      description: "Cozy wool blend sweater perfect for cold weather. Features a classic crew neck design.",
      price: 89.99,
      category: "sweaters",
      sizes: JSON.stringify(["S", "M", "L", "XL"]),
      image: JSON.stringify(["samples/sweater-gray-1.jpg"]),
      variants: [
        { priceId: "price_sweater_gray", color: "gray", images: JSON.stringify(["samples/sweater-gray-1.jpg"]) },
        { priceId: "price_sweater_navy", color: "navy", images: JSON.stringify(["samples/sweater-navy-1.jpg"]) },
        { priceId: "price_sweater_burgundy", color: "burgundy", images: JSON.stringify(["samples/sweater-burgundy-1.jpg"]) },
      ],
    },
    {
      name: "Leather Jacket",
      description: "Premium genuine leather jacket with a timeless design. Built to last for years.",
      price: 299.99,
      category: "jackets",
      sizes: JSON.stringify(["S", "M", "L", "XL"]),
      image: JSON.stringify(["samples/jacket-black-1.jpg"]),
      variants: [
        { priceId: "price_jacket_black", color: "black", images: JSON.stringify(["samples/jacket-black-1.jpg", "samples/jacket-black-2.jpg"]) },
        { priceId: "price_jacket_brown", color: "brown", images: JSON.stringify(["samples/jacket-brown-1.jpg"]) },
      ],
    },
    {
      name: "Running Sneakers",
      description: "Lightweight running sneakers with advanced cushioning technology for maximum performance.",
      price: 129.99,
      category: "shoes",
      sizes: JSON.stringify(["7", "8", "9", "10", "11", "12"]),
      image: JSON.stringify(["samples/sneakers-white-1.jpg"]),
      variants: [
        { priceId: "price_sneakers_white", color: "white", images: JSON.stringify(["samples/sneakers-white-1.jpg"]) },
        { priceId: "price_sneakers_black", color: "black", images: JSON.stringify(["samples/sneakers-black-1.jpg"]) },
        { priceId: "price_sneakers_red", color: "red", images: JSON.stringify(["samples/sneakers-red-1.jpg"]) },
      ],
    },
    {
      name: "Cotton Hoodie",
      description: "Soft cotton hoodie with a kangaroo pocket. Great for casual outings or lounging at home.",
      price: 59.99,
      category: "hoodies",
      sizes: JSON.stringify(["XS", "S", "M", "L", "XL", "XXL"]),
      image: JSON.stringify(["samples/hoodie-gray-1.jpg"]),
      variants: [
        { priceId: "price_hoodie_gray", color: "gray", images: JSON.stringify(["samples/hoodie-gray-1.jpg"]) },
        { priceId: "price_hoodie_black", color: "black", images: JSON.stringify(["samples/hoodie-black-1.jpg"]) },
        { priceId: "price_hoodie_navy", color: "navy", images: JSON.stringify(["samples/hoodie-navy-1.jpg"]) },
      ],
    },
    {
      name: "Chino Pants",
      description: "Versatile chino pants that work for both casual and semi-formal occasions.",
      price: 69.99,
      category: "pants",
      sizes: JSON.stringify(["28", "30", "32", "34", "36", "38"]),
      image: JSON.stringify(["samples/chino-beige-1.jpg"]),
      variants: [
        { priceId: "price_chino_beige", color: "beige", images: JSON.stringify(["samples/chino-beige-1.jpg"]) },
        { priceId: "price_chino_navy", color: "navy", images: JSON.stringify(["samples/chino-navy-1.jpg"]) },
        { priceId: "price_chino_olive", color: "olive", images: JSON.stringify(["samples/chino-olive-1.jpg"]) },
      ],
    },
    {
      name: "Denim Jacket",
      description: "Classic denim jacket with a modern fit. A timeless piece for any wardrobe.",
      price: 99.99,
      category: "jackets",
      sizes: JSON.stringify(["S", "M", "L", "XL"]),
      image: JSON.stringify(["samples/denim-jacket-blue-1.jpg"]),
      variants: [
        { priceId: "price_denim_blue", color: "blue", images: JSON.stringify(["samples/denim-jacket-blue-1.jpg"]) },
        { priceId: "price_denim_lightblue", color: "light blue", images: JSON.stringify(["samples/denim-jacket-lightblue-1.jpg"]) },
      ],
    },
  ];

  for (const product of products) {
    const { variants, ...productData } = product;

    const createdProduct = await prisma.product.create({
      data: productData,
    });

    for (const variant of variants) {
      await prisma.productVariant.create({
        data: {
          ...variant,
          productId: createdProduct.id,
        },
      });
    }

    console.log(`Created product: ${product.name}`);
  }

  console.log("\nSeeding completed! Created", products.length, "products.");
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
