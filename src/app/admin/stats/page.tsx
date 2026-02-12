import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { redirect } from "next/navigation";
import { prisma } from "@/libs/db";
import Link from "next/link";
import { ArrowLeft, Users, Eye, Gift, Calendar } from "lucide-react";
import { format, subDays, startOfMonth, startOfYear } from "date-fns";
import { fr } from "date-fns/locale";

export const metadata = {
  title: "Statistiques - Admin FoxTown",
};

export default async function AdminStatsPage() {
  const session = await getServerSession(authOptions);

  if (!session?.user?.isAdmin) {
    redirect("/login?error=unauthorized");
  }

  const today = new Date();
  today.setHours(0, 0, 0, 0);

  const startOfThisMonth = startOfMonth(today);
  const startOfThisYear = startOfYear(today);

  // Get visitor stats
  const [todayStats, monthStats, yearStats] = await Promise.all([
    prisma.visitorStats.findUnique({ where: { date: today } }),
    prisma.visitorStats.aggregate({
      where: { date: { gte: startOfThisMonth } },
      _sum: { uniqueVisitors: true, totalPageViews: true },
    }),
    prisma.visitorStats.aggregate({
      where: { date: { gte: startOfThisYear } },
      _sum: { uniqueVisitors: true, totalPageViews: true },
    }),
  ]);

  // Get game stats
  const [todayPlays, todayWins, monthPlays, monthWins] = await Promise.all([
    prisma.gamePlay.count({ where: { playDate: { gte: today } } }),
    prisma.gamePlay.count({ where: { playDate: { gte: today }, won: true } }),
    prisma.gamePlay.count({ where: { playDate: { gte: startOfThisMonth } } }),
    prisma.gamePlay.count({ where: { playDate: { gte: startOfThisMonth }, won: true } }),
  ]);

  // Get voucher stats
  const [totalVouchers, usedVouchers, wonVouchers] = await Promise.all([
    prisma.voucher.count(),
    prisma.voucher.count({ where: { isUsed: true } }),
    prisma.voucher.count({ where: { wonByUserId: { not: null } } }),
  ]);

  // Get daily stats for chart (last 7 days)
  const last7Days = await Promise.all(
    Array.from({ length: 7 }, (_, i) => {
      const date = subDays(today, 6 - i);
      return prisma.visitorStats.findUnique({ where: { date } }).then((stats) => ({
        date: format(date, "dd/MM", { locale: fr }),
        visitors: stats?.uniqueVisitors || 0,
        pageViews: stats?.totalPageViews || 0,
      }));
    })
  );

  return (
    <section className="container mx-auto px-4 py-12">
      <div className="flex items-center gap-4 mb-8">
        <Link
          href="/admin"
          className="p-2 hover:bg-gray-100 rounded-lg transition"
        >
          <ArrowLeft className="w-5 h-5" />
        </Link>
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Statistiques</h1>
          <p className="text-gray-600">Vue d&apos;ensemble de l&apos;activité du site</p>
        </div>
      </div>

      {/* Period Selector */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-4">
            <Calendar className="w-5 h-5 text-blue-600" />
            <h2 className="font-semibold text-gray-900">Aujourd&apos;hui</h2>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-500">Visiteurs</span>
              <span className="font-bold">{todayStats?.uniqueVisitors || 0}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">Pages vues</span>
              <span className="font-bold">{todayStats?.totalPageViews || 0}</span>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-4">
            <Calendar className="w-5 h-5 text-green-600" />
            <h2 className="font-semibold text-gray-900">Ce mois</h2>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-500">Visiteurs</span>
              <span className="font-bold">{monthStats._sum.uniqueVisitors || 0}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">Pages vues</span>
              <span className="font-bold">{monthStats._sum.totalPageViews || 0}</span>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-4">
            <Calendar className="w-5 h-5 text-purple-600" />
            <h2 className="font-semibold text-gray-900">Cette année</h2>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-500">Visiteurs</span>
              <span className="font-bold">{yearStats._sum.uniqueVisitors || 0}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">Pages vues</span>
              <span className="font-bold">{yearStats._sum.totalPageViews || 0}</span>
            </div>
          </div>
        </div>
      </div>

      {/* Chart - Simple Bar Chart */}
      <div className="bg-white rounded-xl shadow-lg p-6 mb-8">
        <h2 className="text-xl font-bold text-gray-900 mb-6">Visiteurs (7 derniers jours)</h2>
        <div className="flex items-end justify-between gap-2 h-48">
          {last7Days.map((day, i) => {
            const maxVisitors = Math.max(...last7Days.map((d) => d.visitors), 1);
            const height = (day.visitors / maxVisitors) * 100;

            return (
              <div key={i} className="flex-1 flex flex-col items-center gap-2">
                <span className="text-sm font-medium text-gray-900">{day.visitors}</span>
                <div className="w-full bg-gray-100 rounded-t relative" style={{ height: "160px" }}>
                  <div
                    className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-blue-600 to-blue-400 rounded-t transition-all duration-500"
                    style={{ height: `${height}%` }}
                  />
                </div>
                <span className="text-xs text-gray-500">{day.date}</span>
              </div>
            );
          })}
        </div>
      </div>

      {/* Game Stats */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-6">
            <Gift className="w-6 h-6 text-orange-600" />
            <h2 className="text-xl font-bold text-gray-900">Jeu - Roue de la Fortune</h2>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="bg-orange-50 rounded-lg p-4 text-center">
              <p className="text-3xl font-bold text-orange-600">{todayPlays}</p>
              <p className="text-sm text-orange-700">Parties aujourd&apos;hui</p>
            </div>
            <div className="bg-green-50 rounded-lg p-4 text-center">
              <p className="text-3xl font-bold text-green-600">{todayWins}</p>
              <p className="text-sm text-green-700">Gains aujourd&apos;hui</p>
            </div>
            <div className="bg-blue-50 rounded-lg p-4 text-center">
              <p className="text-3xl font-bold text-blue-600">{monthPlays}</p>
              <p className="text-sm text-blue-700">Parties ce mois</p>
            </div>
            <div className="bg-purple-50 rounded-lg p-4 text-center">
              <p className="text-3xl font-bold text-purple-600">{monthWins}</p>
              <p className="text-sm text-purple-700">Gains ce mois</p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-lg p-6">
          <div className="flex items-center gap-3 mb-6">
            <Gift className="w-6 h-6 text-purple-600" />
            <h2 className="text-xl font-bold text-gray-900">Bons d&apos;achat</h2>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <span className="text-gray-600">Total bons créés</span>
              <span className="text-2xl font-bold text-gray-900">{totalVouchers}</span>
            </div>
            <div className="flex items-center justify-between p-4 bg-green-50 rounded-lg">
              <span className="text-green-700">Bons gagnés</span>
              <span className="text-2xl font-bold text-green-600">{wonVouchers}</span>
            </div>
            <div className="flex items-center justify-between p-4 bg-blue-50 rounded-lg">
              <span className="text-blue-700">Bons utilisés</span>
              <span className="text-2xl font-bold text-blue-600">{usedVouchers}</span>
            </div>
            <div className="flex items-center justify-between p-4 bg-orange-50 rounded-lg">
              <span className="text-orange-700">Bons disponibles</span>
              <span className="text-2xl font-bold text-orange-600">
                {totalVouchers - wonVouchers}
              </span>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
