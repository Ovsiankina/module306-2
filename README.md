# Centre Commercial - E-commerce Template

A modern e-commerce template built with Next.js 14, Prisma with SQLite, and Cloudinary for image management.

## Quick Start

```bash
# Install dependencies
npm install

# Initialize the database and seed sample data
npm run seed

# Start the development server
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the app.

## Features

- Responsive Modern Design
- User Authentication (Google OAuth + Credentials)
- Product Catalog with Categories
- Shopping Cart & Wishlist
- Product Search
- User Profile Management
- Image Management with Cloudinary
- Order History
- SQLite Database (no external DB required)

## Database Setup

This project uses **SQLite** with **Prisma ORM** for simplicity. No external database is required.

### Initialize Database

```bash
# Push schema to database (creates tables)
npx prisma db push

# Generate Prisma client
npx prisma generate
```

### Seed Sample Data

```bash
npm run seed
```

This creates 8 sample products with variants.

### View/Edit Database

```bash
npx prisma studio
```

Opens a visual database editor at http://localhost:5555

## Adding Products

### Option 1: Using Prisma Studio

1. Run `npx prisma studio`
2. Click on the `Product` table
3. Click "Add record"
4. Fill in the fields:
   - `name`: Product name
   - `description`: Product description
   - `price`: Price as a number (e.g., 29.99)
   - `category`: Category slug (e.g., "t-shirts", "pants", "shoes")
   - `sizes`: JSON array as string (e.g., `["S", "M", "L", "XL"]`)
   - `image`: JSON array of image paths (e.g., `["products/tshirt-1.jpg"]`)
5. Save and add variants in the `ProductVariant` table

### Option 2: Using the Seed Script

Edit `prisma/seed.ts` and add new products to the `products` array:

```typescript
{
  name: "Your Product Name",
  description: "Product description here",
  price: 49.99,
  category: "category-slug",
  sizes: JSON.stringify(["S", "M", "L"]),
  image: JSON.stringify(["path/to/image.jpg"]),
  variants: [
    {
      priceId: "unique_price_id",
      color: "blue",
      images: JSON.stringify(["path/to/blue-variant.jpg"])
    },
  ],
}
```

Then run:
```bash
npm run seed
```

### Option 3: Programmatically

```typescript
import { prisma } from "@/libs/db";

const product = await prisma.product.create({
  data: {
    name: "New Product",
    description: "Description",
    price: 59.99,
    category: "category",
    sizes: JSON.stringify(["S", "M", "L"]),
    image: JSON.stringify(["image-path.jpg"]),
  },
});

await prisma.productVariant.create({
  data: {
    productId: product.id,
    priceId: "variant_price_id",
    color: "red",
    images: JSON.stringify(["red-variant.jpg"]),
  },
});
```

## Adding Images

This project uses **Cloudinary** for image hosting and optimization.

### Setup Cloudinary

1. Create a free account at [cloudinary.com](https://cloudinary.com)
2. Get your credentials from the Dashboard
3. Add to your `.env` file:

```env
CLOUDINARY_CLOUD_NAME=your_cloud_name
CLOUDINARY_API_KEY=your_api_key
CLOUDINARY_API_SECRET=your_api_secret
```

### Upload Images

1. Go to your Cloudinary Media Library
2. Upload your product images
3. Copy the public ID (path) of the image
4. Use this path in your product data

**Example:**
- Upload `tshirt-blue.jpg` to Cloudinary
- Cloudinary gives you public ID: `products/tshirt-blue`
- Use `"products/tshirt-blue"` in your product's image field

### Image Path Format

Images are stored as paths without the full URL. The app constructs the full Cloudinary URL automatically.

```typescript
// In your product data:
image: JSON.stringify(["products/my-product-image"])

// The app will load from:
// https://res.cloudinary.com/YOUR_CLOUD_NAME/image/upload/products/my-product-image
```

## Environment Variables

Create a `.env` file in the root directory:

```env
# Database (SQLite - default path)
DATABASE_URL="file:./prisma/dev.db"

# Authentication
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret
NEXTAUTH_SECRET=your_random_secret
NEXTAUTH_URL=http://localhost:3000

# Image Storage (Cloudinary)
CLOUDINARY_CLOUD_NAME=your_cloudinary_name
CLOUDINARY_API_KEY=your_cloudinary_key
CLOUDINARY_API_SECRET=your_cloudinary_secret

# Stripe (for payments - optional)
STRIPE_SECRET_KEY=your_stripe_secret
NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY=your_stripe_publishable_key
```

### Generate NextAuth Secret

```bash
npx auth secret
```

## Project Structure

```
src/
├── app/                    # Next.js App Router
│   ├── api/               # API endpoints
│   ├── [category]/        # Product category pages
│   ├── (carts)/           # Cart & Wishlist pages
│   ├── (user)/            # User pages (orders, profile)
│   └── actions.ts         # Server actions
├── components/            # React components
│   ├── products/          # Product components
│   ├── cart/              # Cart components
│   ├── account/           # Auth components
│   └── ui/                # UI components
├── libs/                  # Utilities
│   ├── db.ts              # Prisma client
│   └── auth.ts            # NextAuth config
└── types/                 # TypeScript types

prisma/
├── schema.prisma          # Database schema
├── seed.ts                # Seed script
└── dev.db                 # SQLite database file
```

## Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start development server |
| `npm run build` | Build for production |
| `npm run start` | Start production server |
| `npm run seed` | Seed database with sample data |
| `npm run lint` | Run ESLint |

## Deployment

### Vercel (Recommended)

1. Push your code to GitHub
2. Import project in Vercel
3. Add environment variables
4. Deploy

**Note:** For production, consider using a hosted database like PostgreSQL instead of SQLite.

## License

MIT License
