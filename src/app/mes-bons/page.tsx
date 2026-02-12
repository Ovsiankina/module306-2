import { Suspense } from "react";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { redirect } from "next/navigation";
import { prisma } from "@/libs/db";
import { Skeleton } from "@/components/ui/skeleton";
import { Gift, Calendar, Store, CheckCircle, XCircle } from "lucide-react";
import { format } from "date-fns";
import { fr } from "date-fns/locale";

export const metadata = {
  title: "Mes Bons d'Achat - Les Galeries",
  description: "Consultez vos bons d'achat gagnés à la roue de la fortune.",
};

export default async function MesBonsPage() {
  const session = await getServerSession(authOptions);

  if (!session?.user?._id) {
    redirect("/login");
  }

  return (
    <section className="container mx-auto px-4 py-12">
      <div className="flex items-center gap-3 mb-2">
        <Gift className="w-8 h-8 text-purple-600" />
        <h1 className="text-3xl font-bold text-gray-900">Mes Bons d&apos;Achat</h1>
      </div>
      <p className="text-gray-600 mb-8">
        Retrouvez tous les bons que vous avez gagnés à la roue de la fortune
      </p>

      <Suspense fallback={<VouchersSkeleton />}>
        <UserVouchers userId={session.user._id} />
      </Suspense>
    </section>
  );
}

async function UserVouchers({ userId }: { userId: string }) {
  const vouchers = await prisma.voucher.findMany({
    where: { wonByUserId: userId },
    include: { shop: true },
    orderBy: { wonAt: "desc" },
  });

  if (vouchers.length === 0) {
    return (
      <div className="bg-white rounded-2xl shadow-lg p-12 text-center">
        <Gift className="w-16 h-16 text-gray-300 mx-auto mb-4" />
        <h2 className="text-xl font-semibold text-gray-900 mb-2">
          Aucun bon d&apos;achat
        </h2>
        <p className="text-gray-500 mb-6">
          Jouez à la roue de la fortune sur la page d&apos;accueil pour tenter de
          gagner des bons d&apos;achat!
        </p>
        <a
          href="/"
          className="inline-block bg-purple-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-purple-700 transition"
        >
          Jouer maintenant
        </a>
      </div>
    );
  }

  const activeVouchers = vouchers.filter(
    (v) => !v.isUsed && new Date(v.expiresAt) > new Date()
  );
  const expiredOrUsedVouchers = vouchers.filter(
    (v) => v.isUsed || new Date(v.expiresAt) <= new Date()
  );

  return (
    <div className="space-y-8">
      <div className="bg-gradient-to-r from-purple-600 to-indigo-700 rounded-2xl p-6 text-white">
        <div className="grid grid-cols-2 gap-4">
          <div className="text-center">
            <p className="text-4xl font-bold">{activeVouchers.length}</p>
            <p className="text-purple-100">Bons actifs</p>
          </div>
          <div className="text-center">
            <p className="text-4xl font-bold">
              {activeVouchers.reduce((sum, v) => sum + v.value, 0)} CHF
            </p>
            <p className="text-purple-100">Valeur totale</p>
          </div>
        </div>
      </div>

      {activeVouchers.length > 0 && (
        <div>
          <h2 className="text-xl font-bold text-gray-900 mb-4">Bons actifs</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {activeVouchers.map((voucher) => (
              <VoucherCard key={voucher.id} voucher={voucher} active />
            ))}
          </div>
        </div>
      )}

      {expiredOrUsedVouchers.length > 0 && (
        <div>
          <h2 className="text-xl font-bold text-gray-500 mb-4">
            Bons expirés ou utilisés
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {expiredOrUsedVouchers.map((voucher) => (
              <VoucherCard key={voucher.id} voucher={voucher} active={false} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function VoucherCard({
  voucher,
  active,
}: {
  voucher: {
    id: string;
    code: string;
    value: number;
    description: string;
    isUsed: boolean;
    wonAt: Date | null;
    expiresAt: Date;
    shop: { name: string };
  };
  active: boolean;
}) {
  const isExpired = new Date(voucher.expiresAt) <= new Date();

  return (
    <div
      className={`bg-white rounded-xl shadow-lg overflow-hidden ${
        !active ? "opacity-60" : ""
      }`}
    >
      <div
        className={`px-6 py-4 ${
          active
            ? "bg-gradient-to-r from-green-500 to-emerald-600"
            : "bg-gray-400"
        }`}
      >
        <div className="flex items-center justify-between">
          <span className="text-white font-bold text-2xl">
            {voucher.value} CHF
          </span>
          {voucher.isUsed ? (
            <span className="flex items-center gap-1 text-white/80 text-sm">
              <CheckCircle className="w-4 h-4" /> Utilisé
            </span>
          ) : isExpired ? (
            <span className="flex items-center gap-1 text-white/80 text-sm">
              <XCircle className="w-4 h-4" /> Expiré
            </span>
          ) : null}
        </div>
      </div>

      <div className="p-6">
        <div className="flex items-center gap-2 mb-3">
          <Store className="w-5 h-5 text-purple-600" />
          <span className="font-semibold text-gray-900">{voucher.shop.name}</span>
        </div>

        <p className="text-gray-600 text-sm mb-4">{voucher.description}</p>

        <div className="bg-gray-100 rounded-lg p-3 mb-4">
          <p className="text-xs text-gray-500 mb-1">Code à présenter en caisse</p>
          <p className="font-mono font-bold text-lg text-gray-900">
            {voucher.code}
          </p>
        </div>

        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Calendar className="w-4 h-4" />
          <span>
            {voucher.wonAt && (
              <>
                Gagné le{" "}
                {format(new Date(voucher.wonAt), "d MMMM yyyy", { locale: fr })}
              </>
            )}
            {" - "}
            Expire le{" "}
            {format(new Date(voucher.expiresAt), "d MMMM yyyy", { locale: fr })}
          </span>
        </div>
      </div>
    </div>
  );
}

function VouchersSkeleton() {
  return (
    <div className="space-y-8">
      <Skeleton className="h-32 w-full rounded-2xl" />
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {[1, 2, 3, 4].map((i) => (
          <Skeleton key={i} className="h-64 w-full rounded-xl" />
        ))}
      </div>
    </div>
  );
}
