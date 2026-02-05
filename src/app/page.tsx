import { Suspense } from "react";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import {
  getAllShops,
  getShopCategories,
  getParkingStatus,
  getMallInfo,
  canPlayGame,
} from "./mall-actions";
import ShopList from "@/components/shops/ShopList";
import ParkingStatus from "@/components/parking/ParkingStatus";
import WheelOfFortune from "@/components/game/WheelOfFortune";
import { Skeleton } from "@/components/ui/skeleton";

export default async function Home() {
  return (
    <section className="pt-8 pb-16">
      <Suspense fallback={<HeroSkeleton />}>
        <HeroSection />
      </Suspense>

      <div className="container mx-auto px-4 mt-12">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="lg:col-span-2">
            <Suspense fallback={<GameSkeleton />}>
              <GameSection />
            </Suspense>
          </div>
          <div>
            <Suspense fallback={<ParkingSkeleton />}>
              <ParkingSection />
            </Suspense>
          </div>
        </div>
      </div>

      <div className="container mx-auto px-4 mt-16">
        <h2 className="text-3xl font-bold text-gray-900 mb-8">Nos Boutiques</h2>
        <Suspense fallback={<ShopsSkeleton />}>
          <ShopsSection />
        </Suspense>
      </div>
    </section>
  );
}

async function HeroSection() {
  const mallInfo = await getMallInfo();

  return (
    <div className="bg-gradient-to-r from-purple-700 to-indigo-800 text-white">
      <div className="container mx-auto px-4 py-16">
        <h1 className="text-4xl md:text-5xl font-bold mb-4">
          {mallInfo?.name || "Centre Commercial"}
        </h1>
        <p className="text-xl text-purple-100 mb-6 max-w-2xl">
          {mallInfo?.description ||
            "Bienvenue dans votre centre commercial préféré!"}
        </p>
        <div className="flex flex-wrap gap-6 text-purple-100">
          {mallInfo?.address && (
            <div>
              <span className="font-semibold">Adresse:</span> {mallInfo.address}
            </div>
          )}
          {mallInfo?.openingHours && (
            <div>
              <span className="font-semibold">Horaires:</span>{" "}
              {mallInfo.openingHours}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

async function GameSection() {
  const session = await getServerSession(authOptions);
  const playStatus = await canPlayGame();

  return (
    <WheelOfFortune
      session={session}
      initialCanPlay={playStatus.canPlay}
      initialReason={playStatus.reason}
    />
  );
}

async function ParkingSection() {
  const parkings = await getParkingStatus();

  return <ParkingStatus parkings={parkings} />;
}

async function ShopsSection() {
  const [shops, categories] = await Promise.all([
    getAllShops(),
    getShopCategories(),
  ]);

  return <ShopList shops={shops} categories={categories} />;
}

function HeroSkeleton() {
  return (
    <div className="bg-gradient-to-r from-purple-700 to-indigo-800 text-white">
      <div className="container mx-auto px-4 py-16">
        <Skeleton className="h-12 w-96 mb-4 bg-purple-600" />
        <Skeleton className="h-6 w-full max-w-2xl mb-6 bg-purple-600" />
        <div className="flex gap-6">
          <Skeleton className="h-5 w-48 bg-purple-600" />
          <Skeleton className="h-5 w-64 bg-purple-600" />
        </div>
      </div>
    </div>
  );
}

function GameSkeleton() {
  return (
    <div className="bg-gradient-to-br from-purple-600 to-blue-500 rounded-2xl p-8">
      <Skeleton className="h-8 w-48 mx-auto mb-4 bg-purple-400" />
      <Skeleton className="h-64 w-64 mx-auto rounded-full bg-purple-400" />
      <Skeleton className="h-12 w-48 mx-auto mt-6 bg-purple-400" />
    </div>
  );
}

function ParkingSkeleton() {
  return (
    <div className="bg-white rounded-2xl shadow-lg p-6">
      <Skeleton className="h-8 w-32 mb-4" />
      <div className="space-y-4">
        {[1, 2, 3].map((i) => (
          <div key={i} className="space-y-2">
            <Skeleton className="h-6 w-full" />
            <Skeleton className="h-2 w-full" />
          </div>
        ))}
      </div>
    </div>
  );
}

function ShopsSkeleton() {
  return (
    <div>
      <div className="flex gap-2 mb-6">
        {[1, 2, 3, 4].map((i) => (
          <Skeleton key={i} className="h-10 w-24 rounded-full" />
        ))}
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {[1, 2, 3, 4, 5, 6, 7, 8].map((i) => (
          <div key={i} className="bg-white rounded-lg shadow-md p-4">
            <Skeleton className="aspect-video mb-4" />
            <Skeleton className="h-6 w-3/4 mb-2" />
            <Skeleton className="h-4 w-full mb-1" />
            <Skeleton className="h-4 w-2/3" />
          </div>
        ))}
      </div>
    </div>
  );
}
