"use client";

import { useState } from "react";
import ShopCard from "./ShopCard";

interface Shop {
  id: string;
  name: string;
  description: string;
  category: string;
  floor: number;
  location: string;
  websiteUrl: string;
  logo: string;
  image: string;
  openingHours: string;
  phone: string;
}

interface ShopListProps {
  shops: Shop[];
  categories: string[];
}

export default function ShopList({ shops, categories }: ShopListProps) {
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);

  const filteredShops = selectedCategory
    ? shops.filter((shop) => shop.category === selectedCategory)
    : shops;

  return (
    <div>
      <div className="flex flex-wrap gap-2 mb-6">
        <button
          onClick={() => setSelectedCategory(null)}
          className={`px-4 py-2 rounded-full text-sm font-medium transition ${
            selectedCategory === null
              ? "bg-purple-600 text-white"
              : "bg-gray-100 text-gray-700 hover:bg-gray-200"
          }`}
        >
          Toutes ({shops.length})
        </button>
        {categories.map((category) => {
          const count = shops.filter((s) => s.category === category).length;
          return (
            <button
              key={category}
              onClick={() => setSelectedCategory(category)}
              className={`px-4 py-2 rounded-full text-sm font-medium transition ${
                selectedCategory === category
                  ? "bg-purple-600 text-white"
                  : "bg-gray-100 text-gray-700 hover:bg-gray-200"
              }`}
            >
              {category} ({count})
            </button>
          );
        })}
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {filteredShops.map((shop) => (
          <ShopCard key={shop.id} shop={shop} />
        ))}
      </div>

      {filteredShops.length === 0 && (
        <p className="text-center text-gray-500 py-8">
          Aucune boutique trouvée dans cette catégorie.
        </p>
      )}
    </div>
  );
}
