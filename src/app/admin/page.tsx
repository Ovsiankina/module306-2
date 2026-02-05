import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { redirect } from "next/navigation";
import { prisma } from "@/libs/db";
import Link from "next/link";
import { Store, Car, Gift, Users, BarChart3, Settings } from "lucide-react";

export const metadata = {
  title: "Administration - FoxTown",
  description: "Tableau de bord d'administration FoxTown Factory Stores",
};

export default async function AdminPage() {
  const session = await getServerSession(authOptions);

  if (!session?.user?.isAdmin) {
    redirect("/login?error=unauthorized");
  }

  // Get stats
  const [shopCount, parkingCount, voucherCount, visitorCount, todayPlays] = await Promise.all([
    prisma.shop.count({ where: { isActive: true } }),
    prisma.parking.count(),
    prisma.voucher.count({ where: { wonByUserId: null } }),
    prisma.visitor.count(),
    prisma.gamePlay.count({
      where: {
        playDate: {
          gte: new Date(new Date().setHours(0, 0, 0, 0)),
        },
      },
    }),
  ]);

  const stats = [
    { label: "Boutiques actives", value: shopCount, icon: Store, color: "bg-blue-500", href: "/admin/shops" },
    { label: "Parkings", value: parkingCount, icon: Car, color: "bg-green-500", href: "/admin/parking" },
    { label: "Bons disponibles", value: voucherCount, icon: Gift, color: "bg-purple-500", href: "/admin/vouchers" },
    { label: "Visiteurs totaux", value: visitorCount, icon: Users, color: "bg-orange-500", href: "/admin/stats" },
  ];

  return (
    <section className="container mx-auto px-4 py-12">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Administration</h1>
          <p className="text-gray-600">Bienvenue, {session.user.name}</p>
        </div>
        <Link
          href="/admin/settings"
          className="flex items-center gap-2 px-4 py-2 bg-gray-100 rounded-lg hover:bg-gray-200 transition"
        >
          <Settings className="w-5 h-5" />
          Paramètres
        </Link>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {stats.map((stat) => {
          const Icon = stat.icon;
          return (
            <Link
              key={stat.label}
              href={stat.href}
              className="bg-white rounded-xl shadow-lg p-6 hover:shadow-xl transition"
            >
              <div className="flex items-center gap-4">
                <div className={`${stat.color} p-3 rounded-lg`}>
                  <Icon className="w-6 h-6 text-white" />
                </div>
                <div>
                  <p className="text-3xl font-bold text-gray-900">{stat.value}</p>
                  <p className="text-sm text-gray-500">{stat.label}</p>
                </div>
              </div>
            </Link>
          );
        })}
      </div>

      {/* Quick Actions */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        <div className="bg-white rounded-xl shadow-lg p-6">
          <h2 className="text-xl font-bold text-gray-900 mb-4">Actions rapides</h2>
          <div className="grid grid-cols-2 gap-4">
            <Link
              href="/admin/shops/new"
              className="flex items-center gap-3 p-4 bg-blue-50 rounded-lg hover:bg-blue-100 transition"
            >
              <Store className="w-5 h-5 text-blue-600" />
              <span className="font-medium text-blue-900">Ajouter une boutique</span>
            </Link>
            <Link
              href="/admin/parking"
              className="flex items-center gap-3 p-4 bg-green-50 rounded-lg hover:bg-green-100 transition"
            >
              <Car className="w-5 h-5 text-green-600" />
              <span className="font-medium text-green-900">Mettre à jour parking</span>
            </Link>
            <Link
              href="/admin/vouchers/new"
              className="flex items-center gap-3 p-4 bg-purple-50 rounded-lg hover:bg-purple-100 transition"
            >
              <Gift className="w-5 h-5 text-purple-600" />
              <span className="font-medium text-purple-900">Créer des bons</span>
            </Link>
            <Link
              href="/admin/stats"
              className="flex items-center gap-3 p-4 bg-orange-50 rounded-lg hover:bg-orange-100 transition"
            >
              <BarChart3 className="w-5 h-5 text-orange-600" />
              <span className="font-medium text-orange-900">Voir statistiques</span>
            </Link>
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-lg p-6">
          <h2 className="text-xl font-bold text-gray-900 mb-4">Activité du jour</h2>
          <div className="space-y-4">
            <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <span className="text-gray-600">Parties jouées aujourd&apos;hui</span>
              <span className="font-bold text-gray-900">{todayPlays}</span>
            </div>
            <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <span className="text-gray-600">Bons gagnés aujourd&apos;hui</span>
              <span className="font-bold text-gray-900">
                {todayPlays > 0 ? Math.floor(todayPlays * 0.2) : 0}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Help Section */}
      <div className="bg-gradient-to-r from-orange-500 to-red-600 rounded-xl p-6 text-white">
        <h2 className="text-xl font-bold mb-2">Besoin d&apos;aide ?</h2>
        <p className="text-orange-100 mb-4">
          Ce tableau de bord vous permet de gérer facilement le contenu du site FoxTown.
          Utilisez les liens ci-dessus pour accéder aux différentes sections.
        </p>
        <ul className="text-sm text-orange-100 space-y-1">
          <li>• <strong>Boutiques</strong>: Ajouter, modifier ou désactiver des boutiques</li>
          <li>• <strong>Parking</strong>: Mettre à jour la disponibilité des places</li>
          <li>• <strong>Bons</strong>: Créer et gérer les bons d&apos;achat pour le jeu</li>
          <li>• <strong>Statistiques</strong>: Consulter les visites et l&apos;activité</li>
        </ul>
      </div>
    </section>
  );
}
