import { Suspense } from "react";
import { getParkingStatus, getMallInfo } from "../mall-actions";
import { Skeleton } from "@/components/ui/skeleton";
import { Car, Clock, CreditCard, Info } from "lucide-react";

export const metadata = {
  title: "Parking - Les Galeries",
  description: "Consultez la disponibilité des places de parking au centre commercial Les Galeries.",
};

export default async function ParkingPage() {
  return (
    <section className="container mx-auto px-4 py-12">
      <h1 className="text-3xl font-bold text-gray-900 mb-2">Parking</h1>
      <p className="text-gray-600 mb-8">
        Plus de 1000 places de parking pour votre confort
      </p>

      <Suspense fallback={<ParkingSkeleton />}>
        <ParkingContent />
      </Suspense>
    </section>
  );
}

async function ParkingContent() {
  const parkings = await getParkingStatus();

  const totalSpaces = parkings.reduce((sum, p) => sum + p.totalSpaces, 0);
  const totalAvailable = parkings.reduce((sum, p) => sum + p.availableSpaces, 0);

  return (
    <div className="space-y-8">
      <div className="bg-gradient-to-r from-purple-600 to-indigo-700 rounded-2xl p-8 text-white">
        <div className="flex items-center gap-4 mb-6">
          <Car className="w-12 h-12" />
          <div>
            <h2 className="text-2xl font-bold">Disponibilité en temps réel</h2>
            <p className="text-purple-100">
              Mis à jour automatiquement toutes les minutes
            </p>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-6">
          <div className="bg-white/10 rounded-xl p-6 text-center">
            <p className="text-4xl font-bold">{totalAvailable}</p>
            <p className="text-purple-100">Places disponibles</p>
          </div>
          <div className="bg-white/10 rounded-xl p-6 text-center">
            <p className="text-4xl font-bold">{totalSpaces}</p>
            <p className="text-purple-100">Capacité totale</p>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {parkings.map((parking) => {
          const percentage =
            (parking.availableSpaces / parking.totalSpaces) * 100;
          const statusColor =
            percentage > 30
              ? "text-green-600 bg-green-100"
              : percentage > 10
              ? "text-yellow-600 bg-yellow-100"
              : "text-red-600 bg-red-100";
          const barColor =
            percentage > 30
              ? "bg-green-500"
              : percentage > 10
              ? "bg-yellow-500"
              : "bg-red-500";

          return (
            <div
              key={parking.id}
              className="bg-white rounded-xl shadow-lg overflow-hidden"
            >
              <div className="p-6">
                <div className="flex items-start justify-between mb-4">
                  <div>
                    <h3 className="font-bold text-lg text-gray-900">
                      {parking.name}
                    </h3>
                    <p className="text-sm text-gray-500">
                      Niveau {parking.floor}
                    </p>
                  </div>
                  {parking.isOpen ? (
                    <span
                      className={`px-3 py-1 rounded-full text-sm font-medium ${statusColor}`}
                    >
                      {parking.availableSpaces} places
                    </span>
                  ) : (
                    <span className="px-3 py-1 rounded-full text-sm font-medium text-red-600 bg-red-100">
                      Fermé
                    </span>
                  )}
                </div>

                {parking.isOpen && (
                  <>
                    <div className="w-full bg-gray-200 rounded-full h-3 mb-2">
                      <div
                        className={`${barColor} h-3 rounded-full transition-all duration-500`}
                        style={{ width: `${percentage}%` }}
                      />
                    </div>
                    <p className="text-xs text-gray-400 text-right">
                      {parking.availableSpaces} / {parking.totalSpaces} places
                    </p>
                  </>
                )}
              </div>
            </div>
          );
        })}
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-4">
            <div className="w-10 h-10 rounded-full bg-purple-100 flex items-center justify-center">
              <Clock className="w-5 h-5 text-purple-600" />
            </div>
            <h3 className="font-semibold text-gray-900">Horaires</h3>
          </div>
          <ul className="space-y-2 text-sm text-gray-600">
            <li>Lundi - Samedi: 7h00 - 22h00</li>
            <li>Dimanche: 9h00 - 20h00</li>
          </ul>
        </div>

        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-4">
            <div className="w-10 h-10 rounded-full bg-purple-100 flex items-center justify-center">
              <CreditCard className="w-5 h-5 text-purple-600" />
            </div>
            <h3 className="font-semibold text-gray-900">Tarifs</h3>
          </div>
          <ul className="space-y-2 text-sm text-gray-600">
            <li>1ère heure: Gratuite</li>
            <li>Heures suivantes: 2.00 CHF/heure</li>
            <li>Forfait journée: 15.00 CHF</li>
          </ul>
        </div>

        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-4">
            <div className="w-10 h-10 rounded-full bg-purple-100 flex items-center justify-center">
              <Info className="w-5 h-5 text-purple-600" />
            </div>
            <h3 className="font-semibold text-gray-900">Informations</h3>
          </div>
          <ul className="space-y-2 text-sm text-gray-600">
            <li>Hauteur max: 2.10m</li>
            <li>Places PMR disponibles</li>
            <li>Bornes de recharge électrique</li>
          </ul>
        </div>
      </div>
    </div>
  );
}

function ParkingSkeleton() {
  return (
    <div className="space-y-8">
      <Skeleton className="h-64 w-full rounded-2xl" />
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {[1, 2, 3].map((i) => (
          <Skeleton key={i} className="h-40 w-full rounded-xl" />
        ))}
      </div>
    </div>
  );
}
