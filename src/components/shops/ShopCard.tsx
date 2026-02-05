"use client";

import Link from "next/link";
import { MapPin, Clock, Phone, ExternalLink } from "lucide-react";

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

interface ShopCardProps {
  shop: Shop;
}

export default function ShopCard({ shop }: ShopCardProps) {
  const floorLabel =
    shop.floor === 0
      ? "Rez-de-chaussée"
      : shop.floor > 0
      ? `Étage ${shop.floor}`
      : `Sous-sol ${shop.floor}`;

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow">
      <div className="aspect-video bg-gray-100 flex items-center justify-center">
        {shop.logo ? (
          <img
            src={`/${shop.logo}`}
            alt={shop.name}
            className="max-h-20 max-w-[80%] object-contain"
          />
        ) : (
          <span className="text-2xl font-bold text-gray-400">{shop.name}</span>
        )}
      </div>

      <div className="p-4">
        <div className="flex items-start justify-between mb-2">
          <h3 className="text-lg font-semibold text-gray-900">{shop.name}</h3>
          <span className="text-xs bg-purple-100 text-purple-800 px-2 py-1 rounded-full">
            {shop.category}
          </span>
        </div>

        <p className="text-sm text-gray-600 mb-3 line-clamp-2">
          {shop.description}
        </p>

        <div className="space-y-1 text-sm text-gray-500">
          <div className="flex items-center gap-2">
            <MapPin className="w-4 h-4" />
            <span>
              {floorLabel} - {shop.location}
            </span>
          </div>
          {shop.openingHours && (
            <div className="flex items-center gap-2">
              <Clock className="w-4 h-4" />
              <span>{shop.openingHours}</span>
            </div>
          )}
          {shop.phone && (
            <div className="flex items-center gap-2">
              <Phone className="w-4 h-4" />
              <span>{shop.phone}</span>
            </div>
          )}
        </div>

        {shop.websiteUrl && (
          <a
            href={shop.websiteUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="mt-4 inline-flex items-center gap-2 text-purple-600 hover:text-purple-800 text-sm font-medium"
          >
            <ExternalLink className="w-4 h-4" />
            Visiter le site officiel
          </a>
        )}
      </div>
    </div>
  );
}
