"use client";

import { Car } from "lucide-react";

interface Parking {
  id: string;
  name: string;
  totalSpaces: number;
  availableSpaces: number;
  floor: string;
  isOpen: boolean;
}

interface ParkingStatusProps {
  parkings: Parking[];
}

export default function ParkingStatus({ parkings }: ParkingStatusProps) {
  return (
    <div className="bg-white rounded-2xl shadow-lg p-6">
      <div className="flex items-center gap-3 mb-4">
        <Car className="w-6 h-6 text-purple-600" />
        <h2 className="text-xl font-bold text-gray-900">Parking</h2>
      </div>

      <div className="space-y-4">
        {parkings.map((parking) => {
          const percentage =
            (parking.availableSpaces / parking.totalSpaces) * 100;
          const statusColor =
            percentage > 30
              ? "bg-green-500"
              : percentage > 10
              ? "bg-yellow-500"
              : "bg-red-500";

          return (
            <div key={parking.id} className="space-y-2">
              <div className="flex justify-between items-center">
                <div>
                  <p className="font-medium text-gray-900">{parking.name}</p>
                  <p className="text-sm text-gray-500">
                    {parking.floor !== ""
                      ? `Niveau ${parking.floor}`
                      : "Extérieur"}
                  </p>
                </div>
                <div className="text-right">
                  {parking.isOpen ? (
                    <>
                      <p className="font-bold text-lg">
                        {parking.availableSpaces}
                      </p>
                      <p className="text-xs text-gray-500">
                        places libres
                      </p>
                    </>
                  ) : (
                    <span className="text-red-600 font-medium">Fermé</span>
                  )}
                </div>
              </div>

              {parking.isOpen && (
                <div className="w-full bg-gray-200 rounded-full h-2">
                  <div
                    className={`${statusColor} h-2 rounded-full transition-all`}
                    style={{ width: `${percentage}%` }}
                  />
                </div>
              )}
            </div>
          );
        })}
      </div>

      <p className="text-xs text-gray-400 mt-4 text-center">
        Mis à jour en temps réel
      </p>
    </div>
  );
}
