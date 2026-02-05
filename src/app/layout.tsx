import type { Metadata } from "next";
import { GeistSans } from "geist/font/sans";
import Providers from "./Providers";
import { Navbar } from "../components/common/Navbar";
import { Footer } from "../components/common/Footer";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/next";
import { Toaster } from "sonner";
import { Session, getServerSession } from "next-auth";
import { authOptions } from "@/libs/auth";

import "../styles/globals.css";

export const metadata: Metadata = {
  title: "FoxTown Factory Stores - Mendrisio",
  description: "FoxTown Factory Stores est le paradis du shopping avec 160 boutiques et 250 marques prestigieuses. Réductions de 30% à 70% toute l'année. Jouez à notre roue de la fortune pour gagner des bons d'achat!",
};

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session: Session | null = await getServerSession(authOptions);

  return (
    <html lang="fr">
      <Providers>
        <body className={GeistSans.className}>
          <Navbar session={session} />
          <main className="pointer-events-auto">
            {children}
            <Toaster position="top-right" />
            <Analytics />
            <SpeedInsights />
          </main>
          <Footer />
        </body>
      </Providers>
    </html>
  );
}
