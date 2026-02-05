"use client";

import { useState, useEffect, useTransition } from "react";
import Link from "next/link";
import { ArrowLeft, Car, Save } from "lucide-react";
import { toast } from "sonner";

interface Parking {
  id: string;
  name: string;
  totalSpaces: number;
  availableSpaces: number;
  floor: string;
  isOpen: boolean;
}

export default function AdminParkingPage() {
  const [parkings, setParkings] = useState<Parking[]>([]);
  const [isPending, startTransition] = useTransition();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("/api/admin/parking")
      .then((res) => res.json())
      .then((data) => {
        setParkings(data);
        setLoading(false);
      })
      .catch(() => {
        toast.error("Erreur lors du chargement");
        setLoading(false);
      });
  }, []);

  const updateParking = (id: string, field: keyof Parking, value: number | boolean) => {
    setParkings((prev) =>
      prev.map((p) => (p.id === id ? { ...p, [field]: value } : p))
    );
  };

  const saveChanges = async () => {
    startTransition(async () => {
      try {
        const res = await fetch("/api/admin/parking", {
          method: "PUT",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(parkings),
        });

        if (res.ok) {
          toast.success("Parking mis à jour avec succès!");
        } else {
          toast.error("Erreur lors de la mise à jour");
        }
      } catch {
        toast.error("Erreur de connexion");
      }
    });
  };

  if (loading) {
    return (
      <section className="container mx-auto px-4 py-12">
        <div className="animate-pulse space-y-4">
          <div className="h-8 bg-gray-200 rounded w-1/3"></div>
          <div className="h-64 bg-gray-200 rounded"></div>
        </div>
      </section>
    );
  }

  return (
    <section className="container mx-auto px-4 py-12">
      <div className="flex items-center gap-4 mb-8">
        <Link
          href="/admin"
          className="p-2 hover:bg-gray-100 rounded-lg transition"
        >
          <ArrowLeft className="w-5 h-5" />
        </Link>
        <div className="flex-1">
          <h1 className="text-3xl font-bold text-gray-900">Gestion du Parking</h1>
          <p className="text-gray-600">Mettez à jour la disponibilité des places en temps réel</p>
        </div>
        <button
          onClick={saveChanges}
          disabled={isPending}
          className="flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition disabled:opacity-50"
        >
          <Save className="w-5 h-5" />
          {isPending ? "Enregistrement..." : "Enregistrer"}
        </button>
      </div>

      <div className="grid gap-6">
        {parkings.map((parking) => {
          const percentage = (parking.availableSpaces / parking.totalSpaces) * 100;

          return (
            <div key={parking.id} className="bg-white rounded-xl shadow-lg p-6">
              <div className="flex items-start justify-between mb-6">
                <div className="flex items-center gap-4">
                  <div className="p-3 bg-green-100 rounded-lg">
                    <Car className="w-6 h-6 text-green-600" />
                  </div>
                  <div>
                    <h2 className="text-xl font-bold text-gray-900">{parking.name}</h2>
                    <p className="text-gray-500">Niveau {parking.floor}</p>
                  </div>
                </div>
                <label className="flex items-center gap-2 cursor-pointer">
                  <span className="text-sm text-gray-600">Ouvert</span>
                  <input
                    type="checkbox"
                    checked={parking.isOpen}
                    onChange={(e) => updateParking(parking.id, "isOpen", e.target.checked)}
                    className="w-5 h-5 rounded border-gray-300 text-green-600 focus:ring-green-500"
                  />
                </label>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Places disponibles
                  </label>
                  <input
                    type="number"
                    min={0}
                    max={parking.totalSpaces}
                    value={parking.availableSpaces}
                    onChange={(e) =>
                      updateParking(parking.id, "availableSpaces", parseInt(e.target.value) || 0)
                    }
                    className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent text-2xl font-bold text-center"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Capacité totale
                  </label>
                  <input
                    type="number"
                    min={1}
                    value={parking.totalSpaces}
                    onChange={(e) =>
                      updateParking(parking.id, "totalSpaces", parseInt(e.target.value) || 1)
                    }
                    className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent text-2xl font-bold text-center"
                  />
                </div>
              </div>

              <div className="mt-6">
                <div className="flex justify-between text-sm text-gray-500 mb-2">
                  <span>Occupation</span>
                  <span>{Math.round(100 - percentage)}%</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-4">
                  <div
                    className={`h-4 rounded-full transition-all ${
                      percentage > 30
                        ? "bg-green-500"
                        : percentage > 10
                        ? "bg-yellow-500"
                        : "bg-red-500"
                    }`}
                    style={{ width: `${percentage}%` }}
                  />
                </div>
              </div>
            </div>
          );
        })}
      </div>

      <div className="mt-8 bg-blue-50 rounded-lg p-6">
        <h3 className="font-semibold text-blue-900 mb-2">Conseil</h3>
        <p className="text-blue-700 text-sm">
          Mettez à jour régulièrement le nombre de places disponibles pour offrir
          une information précise aux visiteurs. Les changements sont visibles
          immédiatement sur la page d&apos;accueil et la page parking.
        </p>
      </div>
    </section>
  );
}
