import { Suspense } from "react";
import { getAllShops, getMallInfo } from "../mall-actions";
import { Skeleton } from "@/components/ui/skeleton";
import { MapPin, Phone, ExternalLink } from "lucide-react";
import Link from "next/link";

export const metadata = {
  title: "Plan du Centre - FoxTown Factory Stores",
  description: "Consultez le plan du centre commercial FoxTown et trouvez facilement vos boutiques préférées sur 4 niveaux.",
};

export default async function PlanPage() {
  return (
    <section className="container mx-auto px-4 py-12">
      <h1 className="text-3xl font-bold text-gray-900 mb-2">Plan du Centre</h1>
      <p className="text-gray-600 mb-8">
        160 boutiques réparties sur 4 niveaux - Réductions de 30% à 70%
      </p>

      <Suspense fallback={<PlanSkeleton />}>
        <FloorPlan />
      </Suspense>
    </section>
  );
}

async function FloorPlan() {
  const [shops, mallInfo] = await Promise.all([getAllShops(), getMallInfo()]);

  // FoxTown floor structure with color coding
  const floors = [
    { level: 3, name: "Level 3", color: "from-green-500 to-green-600", colorName: "Green", textColor: "text-green-600", bgColor: "bg-green-100" },
    { level: 2, name: "Level 2", color: "from-blue-500 to-blue-600", colorName: "Blue", textColor: "text-blue-600", bgColor: "bg-blue-100" },
    { level: 1, name: "Level 1", color: "from-red-500 to-red-600", colorName: "Red", textColor: "text-red-600", bgColor: "bg-red-100" },
    { level: 0, name: "Level 0", color: "from-yellow-500 to-yellow-600", colorName: "Yellow", textColor: "text-yellow-600", bgColor: "bg-yellow-100" },
  ];

  return (
    <div className="space-y-8">
      {/* Legend */}
      <div className="bg-white rounded-lg shadow-lg p-6">
        <h2 className="text-lg font-semibold mb-4">Légende des niveaux</h2>
        <div className="flex flex-wrap gap-4">
          {floors.map((floor) => (
            <div key={floor.level} className="flex items-center gap-2">
              <div className={`w-6 h-6 rounded bg-gradient-to-r ${floor.color}`} />
              <span className="text-sm font-medium">{floor.name}</span>
            </div>
          ))}
        </div>
      </div>

      {/* PDF Plan Download */}
      <div className="bg-gradient-to-r from-purple-600 to-indigo-700 rounded-lg p-6 text-white">
        <div className="flex flex-col md:flex-row items-center justify-between gap-4">
          <div>
            <h2 className="text-xl font-bold mb-1">Plan interactif complet</h2>
            <p className="text-purple-100">Téléchargez le plan détaillé du centre avec tous les emplacements</p>
          </div>
          <a
            href="/assets/plan-foxtown.pdf"
            target="_blank"
            className="inline-flex items-center gap-2 bg-white text-purple-700 px-6 py-3 rounded-lg font-medium hover:bg-purple-50 transition"
          >
            <ExternalLink className="w-5 h-5" />
            Voir le plan PDF
          </a>
        </div>
      </div>

      {/* Floors */}
      <div className="grid gap-6">
        {floors.map((floor) => {
          const floorShops = shops.filter((shop) => shop.floor === floor.level);
          if (floorShops.length === 0) return null;

          // Group shops by category
          const shopsByCategory = floorShops.reduce((acc, shop) => {
            if (!acc[shop.category]) acc[shop.category] = [];
            acc[shop.category].push(shop);
            return acc;
          }, {} as Record<string, typeof floorShops>);

          return (
            <div
              key={floor.level}
              className="bg-white rounded-lg shadow-lg overflow-hidden"
            >
              <div className={`bg-gradient-to-r ${floor.color} px-6 py-4`}>
                <div className="flex items-center justify-between">
                  <div>
                    <h2 className="text-xl font-bold text-white">{floor.name}</h2>
                    <p className="text-white/80 text-sm">
                      {floorShops.length} boutique{floorShops.length > 1 ? "s" : ""}
                    </p>
                  </div>
                  <span className={`px-3 py-1 rounded-full text-sm font-medium bg-white/20 text-white`}>
                    {floor.colorName}
                  </span>
                </div>
              </div>

              <div className="p-6">
                {Object.entries(shopsByCategory).map(([category, categoryShops]) => (
                  <div key={category} className="mb-6 last:mb-0">
                    <h3 className={`text-sm font-semibold ${floor.textColor} uppercase tracking-wide mb-3`}>
                      {category}
                    </h3>
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                      {categoryShops.map((shop) => (
                        <div
                          key={shop.id}
                          className={`flex items-start gap-3 p-3 rounded-lg ${floor.bgColor} hover:shadow-md transition`}
                        >
                          <div className={`flex-shrink-0 w-10 h-10 rounded-full bg-white flex items-center justify-center ${floor.textColor}`}>
                            <span className="text-xs font-bold">{shop.location}</span>
                          </div>
                          <div className="flex-1 min-w-0">
                            <div className="flex items-start justify-between">
                              <h4 className="font-semibold text-gray-900 truncate">
                                {shop.name}
                              </h4>
                              {shop.websiteUrl && (
                                <a
                                  href={shop.websiteUrl}
                                  target="_blank"
                                  rel="noopener noreferrer"
                                  className={`${floor.textColor} hover:opacity-70`}
                                >
                                  <ExternalLink className="w-4 h-4" />
                                </a>
                              )}
                            </div>
                            {shop.phone && (
                              <a
                                href={`tel:${shop.phone}`}
                                className="flex items-center gap-1 text-xs text-gray-500 hover:text-gray-700"
                              >
                                <Phone className="w-3 h-3" />
                                {shop.phone}
                              </a>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </div>

      {/* Services Info */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="bg-purple-50 rounded-lg p-6">
          <h3 className="font-semibold text-purple-900 mb-3">Services disponibles</h3>
          <ul className="space-y-2 text-sm text-purple-700">
            <li className="flex items-center gap-2">
              <MapPin className="w-4 h-4" /> Infopoint - Point d&apos;information
            </li>
            <li className="flex items-center gap-2">
              <MapPin className="w-4 h-4" /> Bureau de change (Tichange)
            </li>
            <li className="flex items-center gap-2">
              <MapPin className="w-4 h-4" /> Tax Free Refund Point
            </li>
            <li className="flex items-center gap-2">
              <MapPin className="w-4 h-4" /> WiFi gratuit dans toutes les zones communes
            </li>
          </ul>
        </div>

        <div className="bg-orange-50 rounded-lg p-6">
          <h3 className="font-semibold text-orange-900 mb-3">Expériences uniques</h3>
          <ul className="space-y-2 text-sm text-orange-700">
            <li className="flex items-center gap-2">
              <span className="text-lg">🎰</span> Casino Admiral Mendrisio
            </li>
            <li className="flex items-center gap-2">
              <span className="text-lg">🎭</span> The Sense Gallery - Réalité multisensorielle
            </li>
            <li className="flex items-center gap-2">
              <span className="text-lg">🍽️</span> 9 bars et restaurants
            </li>
            <li className="flex items-center gap-2">
              <span className="text-lg">🎁</span> FoxPrivilege - Programme de fidélité
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}

function PlanSkeleton() {
  return (
    <div className="space-y-6">
      <Skeleton className="h-24 w-full rounded-lg" />
      <Skeleton className="h-32 w-full rounded-lg" />
      {[1, 2, 3, 4].map((i) => (
        <div key={i} className="bg-white rounded-lg shadow-lg overflow-hidden">
          <Skeleton className="h-16 w-full" />
          <div className="p-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {[1, 2, 3, 4, 5, 6].map((j) => (
              <div key={j} className="flex gap-3">
                <Skeleton className="w-10 h-10 rounded-full" />
                <div className="flex-1">
                  <Skeleton className="h-5 w-24 mb-2" />
                  <Skeleton className="h-4 w-16" />
                </div>
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
